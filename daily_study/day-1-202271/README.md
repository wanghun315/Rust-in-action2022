# 2022-07-01

### Rust 依赖包安装

- [ 安装Rust依赖包 ](https://www.rust-lang.org/zh-CN/tools/install])
- 设置path 使用命令判断是否安装成功
> rustc --version

### Rust 环境搭建（vs code）

- 安装vs code
- 下载 Rust插件（插件工具栏搜索Rust插件）
- 创建Rust工作空间
- 创建main.rs
```
fn main() {
    println!("Hello, world!");
}
```
- 配置环境变量
```
{
    "rust.mode": "rls",
    "rust.cargoHomePath": "%CARGO_HOME%",
    "rust.cargoPath":"%RUSTBINPATH%\\cargo.exe",
    "rust.racerPath":"%RUSTBINPATH%\\racer.exe",
    "rust.rls":"%RUSTBINPATH%\\rls.exe",
    "rust.rustfmtPath":"%RUSTBINPATH%\\rustfmt.exe",
    "rust.rustup":"%RUSTBINPATH%\\rustup.exe",
    "rust.rustLangSrcPath": "%RUST_SRC_PATH%",
    "rust.executeCargoCommandInTerminal": true,
    "workbench.statusBar.feedback.visible": false,
    "rust.actionOnSave": "build",
    "debug.allowBreakpointsEverywhere": true,
    "rust-client.disableRustup": true,
}
```
- 设置编译环境
```
{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "name": "rust",
            "type": "cppvsdbg",
            "request": "launch",
            //"program": "enter program name, for example ${workspaceFolder}/a.exe",
            "program": "./target/debug/Output.exe",
            "args": [],
            "stopAtEntry": false,
            "cwd": "${workspaceFolder}",
            "environment": [],
            "externalConsole": true,
        }
    ]
}
```
- 运行 出现了
> Hello, world!


