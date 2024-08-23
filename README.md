# PEX
`pex` is a simple parameter extractor written in Rust.

### Installation
```console
cargo install --git https://github.com/m333rl1n/pex.git
```

### Usage
```console
$ pex -h
USAGE: pex [strings]
```
1. Simple usage:
```console
$ curl -s https://domain.tld/file.js | pex # or `pex strings` to extract all javascript strings
```
![image](https://github.com/user-attachments/assets/44b925b6-7796-402f-9349-cf949bc3d804)

2. Advanced usage:
```console
$ cat urls.txt | concurl && find out/* -type f -exec cat {} + | pex
```
