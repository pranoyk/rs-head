# rs-head

This projects implemets the cli command `head` that prints the first few lines of each FILE to standard output.

### usage

```sh
./main {file_path}
```

### features

- [x] print 10 lines by default.
- [ ] implement -c flag to print first NUM bytes of a file.
- [x] implement -n flag to print first NUM lines of a file instead of 10 by default.
- [x] implement --help flag to show provide usage of this tool.