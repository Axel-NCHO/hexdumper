use clap::Parser;
use std::{
    fmt::Write as FmtWrite,
    fs::File,
    io::{self, BufReader, ErrorKind, Read, Seek, Write},
    path::PathBuf,
};

const LINE_LEN: usize = 16;
const READ_BUFFER_SIZE: usize = 64 * 1024;

#[derive(Parser)]
#[command(name = "hexdumper")]
struct Cli {
    #[arg(short = 's', long, default_value_t = 0)]
    offset: u64,

    #[arg(short = 'n', long, default_value_t = usize::MAX)]
    len: usize,

    #[arg(
        short = 'C',
        long,
        conflicts_with_all = ["two_bytes_hexadecimal", "two_bytes_decimal", "two_bytes_octal"]
    )]
    canonical: bool,

    #[arg(
        short = 'x',
        long,
        conflicts_with_all = ["canonical", "two_bytes_decimal", "two_bytes_octal"]
    )]
    two_bytes_hexadecimal: bool,

    #[arg(
        short = 'd',
        long,
        conflicts_with_all = ["canonical", "two_bytes_hexadecimal", "two_bytes_octal"]
    )]
    two_bytes_decimal: bool,

    #[arg(
        short = 'o',
        long,
        conflicts_with_all = ["canonical", "two_bytes_decimal", "two_bytes_hexadecimal"]
    )]
    two_bytes_octal: bool,

    file: PathBuf,
}

impl Cli {
    fn base(&self) -> Base {
        if self.two_bytes_decimal {
            Base::Decimal
        } else if self.two_bytes_octal {
            Base::Octal
        } else {
            Base::Hexadecimal
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Base {
    Hexadecimal,
    Decimal,
    Octal,
}

impl Base {
    fn format_word(self, out: &mut String, bytes: &[u8; 2]) {
        let value = u16::from_le_bytes(*bytes);

        match self {
            Self::Hexadecimal => write!(out, "{value:04x} "),
            Self::Decimal => write!(out, "{value:05} "),
            Self::Octal => write!(out, "{value:06o} "),
        }
        .expect("Cannot display word");
    }

    fn format_byte(self, out: &mut String, byte: u8) {
        match self {
            Self::Hexadecimal => write!(out, "{byte:02x} "),
            Self::Decimal => write!(out, "{byte:05} "),
            Self::Octal => write!(out, "{byte:03o} "),
        }
        .expect("Cannot display byte")
    }
}

fn main() {
    let cli = Cli::parse();

    let mut file = File::open(&cli.file).expect("Cannot open file");
    match file.seek(io::SeekFrom::Start(cli.offset)) {
        Ok(_) => {}
        Err(err) if err.kind() == ErrorKind::UnexpectedEof => return,
        Err(err) => panic!("Cannot skip {} bytes: {err}", cli.offset),
    }

    let mut reader = BufReader::with_capacity(READ_BUFFER_SIZE, file);

    let stdout = io::stdout();
    let mut output = io::BufWriter::with_capacity(READ_BUFFER_SIZE, stdout.lock());

    let base = cli.base();

    if cli.canonical {
        dump_canonical(&mut reader, &mut output, base, cli.len)
    } else {
        dump_two_bytes(&mut reader, &mut output, base, cli.len)
    }
    .expect("Error while dumping file");

    output.flush().unwrap();
}

// Reads at most `buf.len()` bytes, handling short reads.
// Returns zero only when EOF is reached before any bytes are read.
fn read_line<R: Read>(reader: &mut R, buf: &mut [u8]) -> io::Result<usize> {
    let mut filled = 0;

    while filled < buf.len() {
        match reader.read(&mut buf[filled..])? {
            0 => break,
            n => filled += n,
        }
    }

    Ok(filled)
}

fn dump_canonical<R: Read, W: Write>(
    reader: &mut R,
    output: &mut W,
    base: Base,
    total_len: usize,
) -> io::Result<()> {
    let mut bytes = [0u8; LINE_LEN];
    let mut line = String::with_capacity(128);
    let mut ascii = [b'.'; LINE_LEN];
    let mut total_read = 0usize;
    let mut index = 0usize;

    while total_read < total_len {
        let wanted = LINE_LEN.min(total_len - total_read);
        let n = read_line(reader, &mut bytes[..wanted])?;

        if n == 0 {
            break;
        }

        line.clear();
        write!(&mut line, "{index:08x} ").expect("Cannot write line index");

        for &byte in &bytes[..n] {
            base.format_byte(&mut line, byte);
        }

        for (dst, &byte) in ascii.iter_mut().zip(&bytes[..n]) {
            *dst = if byte.is_ascii_graphic() || byte == b' ' {
                byte
            } else {
                b'.'
            };
        }

        line.push('|');
        unsafe {
            line.push_str(std::str::from_utf8_unchecked(&ascii[..n]));
        }
        line.push_str("|\n");

        output.write_all(line.as_bytes())?;

        total_read += n;
        index += n;
    }

    Ok(())
}

fn dump_two_bytes<R: Read, W: Write>(
    reader: &mut R,
    output: &mut W,
    base: Base,
    total_len: usize,
) -> io::Result<()> {
    let mut bytes = [0u8; LINE_LEN];
    let mut line = String::with_capacity(96);
    let mut total_read = 0usize;
    let mut index = 0usize;

    while total_read < total_len {
        let wanted = LINE_LEN.min(total_len - total_read);
        let n = read_line(reader, &mut bytes[..wanted])?;

        if n == 0 {
            break;
        }

        line.clear();
        write!(&mut line, "{index:08x} ").expect("Cannot write index");

        for chunk in bytes[..n].chunks(2) {
            let word = [chunk[0], chunk.get(1).copied().unwrap_or(0)];
            base.format_word(&mut line, &word);
        }

        line.push('\n');
        output.write_all(line.as_bytes())?;

        total_read += n;
        index += n;
    }

    Ok(())
}
