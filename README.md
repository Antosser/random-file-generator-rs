# Random File Generator
Program to create large file consisting of random data

## Why
This implementation is very fast and has no memory leaks, so you can create files __blazingly__ fast

## Installation
1. Install Rust
2. Run `cargo install rfgen`
3. Run `rfgen <Arguments>`

## Usage
```
Program to create large file consisting of random data

Usage: rfgen.exe [OPTIONS] <SIZE>

Arguments:
  <SIZE>  The size of each file. Suffixes: KB, KiB, MB, MiB, GB, GiB, TB, TiB

Options:
  -a, --amount <AMOUNT>  The amount of files of the specified size [default: 1]
  -h, --help             Print help
  -V, --version          Print version
```

## Result
The result will be files called 0000, 0001, 0002, etc.  
Eg
```
~> rfgen 10MB -a 10
~> ls
╭───┬──────┬──────┬─────────┬──────────╮
│ # │ name │ type │  size   │ modified │
├───┼──────┼──────┼─────────┼──────────┤
│ 0 │ 0000 │ file │ 10.0 MB │ now      │
│ 1 │ 0001 │ file │ 10.0 MB │ now      │
│ 2 │ 0002 │ file │ 10.0 MB │ now      │
│ 3 │ 0003 │ file │ 10.0 MB │ now      │
│ 4 │ 0004 │ file │ 10.0 MB │ now      │
│ 5 │ 0005 │ file │ 10.0 MB │ now      │
│ 6 │ 0006 │ file │ 10.0 MB │ now      │
│ 7 │ 0007 │ file │ 10.0 MB │ now      │
│ 8 │ 0008 │ file │ 10.0 MB │ now      │
│ 9 │ 0009 │ file │ 10.0 MB │ now      │
╰───┴──────┴──────┴─────────┴──────────╯
```
