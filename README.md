## strange touch

CLI for Windows to modify creation time of a file to any timestamp

```bash
$ strange-touch -h
Usage: strange-touch.exe [OPTIONS] <PATH>

Arguments:
  <PATH>  

Options:
  -t, --timestamp <TIMESTAMP>  
  -d, --datetime <DATETIME>    
  -h, --help                   Print help
  -V, --version                Print version

$ strange-touch ./file.txt -d '2025-11-07 22:00:00'
```
