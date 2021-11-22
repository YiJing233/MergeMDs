# MergeMDs

平常用Typora写日报，每天一个文件，于是每个月都有很多MD文件，而当我想把他们合并起来成一个合订集就变得很困难

恰好最近在学Rust，就写一个cli程序练手

## 使用方法

windows: 打开文件夹，执行 `cargo run -- main .\example`，在父级文件夹出现output.md文件

macos：打开文件夹，执行 `cargo run -- main ./example`，在父级文件夹出现output.md文件

（尚且是第一版，还有改动空间）

预期实现效果：

```bash
// macos
$ cat *.md > output.md
```

```bash
// windows
$ type *.md > output.md
```