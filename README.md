[![crates.io](https://img.shields.io/crates/v/nu_plugin_compress.svg)](https://crates.io/crates/nu_plugin_compress)
[![docs.rs](https://docs.rs/nu_plugin_compress/badge.svg)](https://docs.rs/nu_plugin_compress)

## nu_plugin_compress
A nushell plugin for compression and decompression, supporting zstd, gzip, bzip2, and xz. Require nushell >= `0.98.0`.

## Status

Supported compression formats include:

|Type|Compress Command|Decompress Command|
|--|--|--|
|gzip|to gz|from gz|
|zstd|to zst|from zst|
|xz|to xz|from xz|
|bzip2|to bz2|from bz2|

The compression command can specify the level parameter.


### Installation
```shell
cargo install nu_plugin_compress
plugin add ~/.cargo/bin/nu_plugin_compress
plugin use compress
```

### Usage
```shell
# compress and save
"hello" | to gz | save hello.gz
# decompress and print
open hello.gz | decode
```