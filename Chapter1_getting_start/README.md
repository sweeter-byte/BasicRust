# Getting Start!

## set up Rust
in Linux, set up rustup:
```sh
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

update:
```sh
rustup update
```

You can get official book:
```sh
rustup doc
```


## "Hello, world!"

## Hello cargo

Cargo 是 Rust 的构建系统和包管理器，可以构建代码、下载依赖库并编译这些库。

使用 cargo 创建新项目:
```sh
cargo new hello_cargo
cd hello_cargo
```
--

进入 hello_cargo 目录并列出文件。将会看到 Cargo 生成了两个文件和一个目录：一个 Cargo.toml 文件，一个 src 目录，以及位于 src 目录中的 main.rs 文件。

Cargo.toml文件使用 TOML (Tom's Obvious, Minimal Language) 格式，这是 Cargo 配置文件的格式。接下来的三行设置了 Cargo 编译程序所需的配置：项目的名称、项目的版本以及要使用的 Rust 版本。

最后一行，[dependencies]，是罗列项目依赖的 section 的开始。在 Rust 中，代码包被称为 crates。这个项目并不需要其他的 crate，不过在第二章的第一个项目会用到依赖，那时会用得上这个 section。

--

构建项目:
```sh
cargo build
```

这个命令会创建一个可执行文件 target/debug/hello_cargo 

执行文件:
```sh
./target/debug/hello_cargo
```

--

或者构建后立马运行:
```sh
cargo run
```

--

快速检查代码确保其可以编译，但并不产生可执行文件:

```sh
cargo check
```

### 发布（release）构建

当项目最终准备好发布时，可以使用` cargo build --release `来优化编译项目。这会在 target/release 而不是 target/debug 下生成可执行文件。这些优化可以让 Rust 代码运行的更快，不过启用这些优化也需要消耗更长的编译时间。这也就是为什么会有两种不同的配置：一种是为了开发，你需要快速且频繁地重新构建；另一种是为用户构建最终程序，它们不会经常重新构建，并且希望程序运行得越快越好。如果你在基准测试代码的运行时间，请确保运行` cargo build --release `并使用 target/release 下的可执行文件进行测试。

[点击这里](https://doc.rust-lang.org/cargo/)提供了更多与` cargo `有关的信息