# LAS - Last Access Scanner

A very simple command line utility written in Rust for scanning directories and files and print the number of days passed since the creation, modification and last access times stored in the entry's metadata

# Installation:

```
git clone [url of this repository]
cd las
cargo run --release -- <directory path> [-m max_depth -s 0 | 1 | f | t | false | true]
```

where:
- max_depth is the number of recursion level
- s to skip hidden files and directories
