## Difference between Tar+gz with Rust libraries and system `tar` command

## Run

```
$ git clone <url>
$ cd tar_comparison
$ cargo run --release
```

Read the output, all files generated are stored in a local `tmp/` directory for inspection.

## Expected

I expect that using [compressing a directory with Rust libraries](https://github.com/rust-lang-nursery/rust-cookbook/blob/752b035c1ae1646c999b4d7c39c78d67215d5893/src/compression/tar/tar-compress.md) produces a similar sized file to `cd <from-dir> && tar czf <to-filename.tar.gz> *` (lower filesize is better).

## Actual

```
## Comparing tar+gz at the same time using RUST and system

- RUST tar+gzip /Users/rschneeman/Documents/projects/tmp/tar_comparison/tmp/2024-06-06-17-32-23-327069000/rust_tar_gzip_one_operation.tar.gz
- Done
- System tar+gzip /Users/rschneeman/Documents/projects/tmp/tar_comparison/tmp/2024-06-06-17-32-23-327069000/system_tar_gzip_one_operation.tar.gz
  - Running tar+gzip command: "bash" "-c" "cd /Users/rschneeman/Documents/projects/tmp/tar_comparison/tmp/2024-06-06-17-32-23-327069000/source && tar -czf /Users/rschneeman/Documents/projects/tmp/tar_comparison/tmp/2024-06-06-17-32-23-327069000/system_tar_gzip_one_operation.tar.gz *"
  - stdout:
  - stderr:
  - tar+gzip command succeeded
- Done
 - system_tar_gz size: 21337548
 - rust_tar_gz size: 40482499

## system_tar_gz is smaller than rust_tar_gz (tar+gzip)
 :(
```

Shelling out to `tar -czf` produces a file that is 0.525x the size as using Rust + Gzip in an atomic operation.
