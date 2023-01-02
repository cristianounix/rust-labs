## Rust Labs




Compiling using rust compiler

```
$ rustc file.ts
```



### Using cargo

Generating binary

```
$ cargo build  ( Dev )
$ cargo build --release ( Relese version )
```

Check size

```
$ du -h filebinary
```



Build optimization flags on Cargo.tml (Build Profile)

```
[package]

[dependencies]

[profile.release]
strip = true
apt-level = "2"
lto = true
codegen-units = 1
panic = "abort"

```



Formatting code

```
$ cargo fmt
```

Combile and run

```
$ cargo run
```

Watch changes and compiling

```
$ cargo install cargo-watch
$ cargo watch -x run
```


## Rust RPL

```
$ irust
```

## Extras

Hexdump 


You can use hexdump to view de binary content.

```
$ hexdump binaryfile
```


