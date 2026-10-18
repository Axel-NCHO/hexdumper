## Hex dump / Binary file inspector

**Goal:** Read raw bytes and display them.

**Scope:** Single-file CLI tool. Read-only.

**Features (must):**

- `hexdump [options] FILE`
- Canonical hex+ASCII output (like `hexdump -C`): offset | 16 bytes hex | ASCII.
- Flags: `-n LEN` (byte count), `-s OFFSET` (skip), `-o/-d/-x` (octal/dec/hex 16-bit words).

- Read from stdin if `-` or no file.

**Stack:** 

- Rust with `std::fs::File` + `Read`

**Deliverable:** Single static binary + README with examples.

**Done criteria:** 

- Can hexdump a 1 GB file with constant memory
- Correct output on at least 5 known files verified against `xxd`

**Stretch:** `--diff` mode comparing two files byte-by-byte.



### How does it work?

```mermaid
stateDiagram-v2
    [*] --> set_params
    set_params --> open_file
    open_file --> skip_bytes
    skip_bytes --> [*] : EOF
    skip_bytes --> print_index
    print_index --> read_2_bytes
    read_2_bytes --> print_2_bytes_in_base
    print_2_bytes_in_base --> append_2_bytes_in_ascii
    append_2_bytes_in_ascii --> print_ascii : read = 16
    append_2_bytes_in_ascii --> print_ascii : reached LEN
    append_2_bytes_in_ascii --> read_2_bytes : read < LEN & read < 16
    print_ascii --> print_index : read < LEN
    print_ascii --> [*] : read = LEN
    
state "Set params
       OFFSET = -s or 0
       LEN = -n or -1
       BASE = -o/-d/-x or 16 " as set_params
state "Open FILE" as open_file
state "Skip OFFSET bytes" as skip_bytes
state "Print index" as print_index
state "Read 2 bytes" as read_2_bytes
state "Print 2 bytes in BASE" as print_2_bytes_in_base
state "Append 2 bytes in ASCII" as append_2_bytes_in_ascii
state "Print |ASCII| as reset ASCII" as print_ascii
```
