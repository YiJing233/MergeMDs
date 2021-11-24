# MergeMDs

平常用Typora写日报，每天一个文件，于是每个月都有很多MD文件，而当我想把他们合并起来成一个合订集就变得很困难

恰好最近在学Rust，就写一个cli程序练手

## 使用方法

前提是配置好rustup和cargo

`git clone https://github.com/YiJing233/MergeMDs.git`

windows: 进入项目，执行 `cargo run -- main .\example`，在父级文件夹出现output.md文件

macos：进入项目，执行 `cargo run -- main ./example`，在父级文件夹出现output.md文件

（尚且是第一版，还有改动空间）

预期实现效果：

```bash
// macos
$ cat *.md > output.md
// 或
$ cat *.md >> output.md // 追加写入
```

```bash
// windows
$ type *.md > output.md
// 或
$ type *.md >> output.md 
```

### TODO：
- 文件追加写入时 开头携带"\n"
- 完成应用 CLI化 和 package 发布
- 单测
- 合理的错误处理
- 补充 CLI 工具的帮助信息


