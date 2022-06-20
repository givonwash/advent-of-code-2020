# Advent-of-Code 2020

Using the Advent-of-Code 2020 to polish up on my Rust skills

## Repo Layout

```
.
├── README.md       # this document
└── day{N}          # Rust binary for dayN where 1 <= N <= 25
    ├── Cargo.lock
    ├── Cargo.toml
    ├── input       # contains the input for the day's challenge
    └── src
        └── main.rs
```

## Running the Binary

### Using `nix`

From the root of this repository

```bash
cat './day{N}/input/{input_filename}.txt' | nix run '.#day{N}'
```

### Using Cargo

From inside the `day{N}` directory.

```bash
cat './input/{input_filename}.txt' | cargo run
```
