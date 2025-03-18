# witd

A tool to watch a set of directories or files and run a command.

## Install
```
cargo install witd
```

## Example Usage
```
cargo witd -w=src/ -i=target/ cargo test
```

## Flags
- `-h || --help` will output help
- `-w=DIR || --watch=DIR` will include the given directory in the watch list
- `-i=DIR || --ignore=DIR` will ignore the given directory in the watch list

