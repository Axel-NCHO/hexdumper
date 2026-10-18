## Hexdumper: a binary file inspector

> This tool was written for learning purposes

### Goal

Read raw bytes and display them

### Scope

Single-file CLI tool. Read-only.

### Features

- `hexdumper [options] FILE`
- Canonical hex+ASCII output (like `hexdump -C`): offset | 16 bytes hex | ASCII.
- Flags: `-n LEN` (byte count), `-s OFFSET` (skip), `-o/-d/-x` (octal/dec/hex 16-bit words).

See 
```shell 
cargo run -- --help
```

### How does it work?

```mermaid

stateDiagram-v2
  [*] --> set_params
  set_params --> open_file
  open_file --> seek_offset
  seek_offset --> [*] : Seek fails / beyond EOF
  seek_offset --> init_buffers : Success

  init_buffers --> select_mode
  select_mode --> canonical : -C
  select_mode --> two_bytes : Default / -x / -d / -o

  state "Canonical mode" as canonical {
      [*] --> read_chunk_c
      read_chunk_c --> format_bytes : Bytes read > 0
      read_chunk_c --> [*] : EOF / LEN reached
      format_bytes --> format_ascii
      format_ascii --> write_line_c
      write_line_c --> read_chunk_c : Continue
  }

  state "Two-byte mode" as two_bytes {
      [*] --> read_chunk_2
      read_chunk_2 --> format_words : Bytes read > 0
      read_chunk_2 --> [*] : EOF / LEN reached
      format_words --> write_line_2
      write_line_2 --> read_chunk_2 : Continue
  }

  canonical --> flush_output : Done
  two_bytes --> flush_output : Done
  flush_output --> [*]

  state "Parse OFFSET, LEN, BASE and mode" as set_params
  state "Open input file" as open_file
  state "Seek to OFFSET" as seek_offset
  state "Initialize fixed-size input and output buffers" as init_buffers
  state "Read up to 16 bytes, bounded by remaining LEN" as read_chunk_c
  state "Format bytes in BASE" as format_bytes
  state "Build ASCII representation" as format_ascii
  state "Append complete line to output buffer" as write_line_c
  state "Read up to 16 bytes, bounded by remaining LEN" as read_chunk_2
  state "Format little-endian 16-bit words in BASE" as format_words
  state "Append complete line to output buffer" as write_line_2
  state "Flush buffered output" as flush_output
```
