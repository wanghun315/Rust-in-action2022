# 2022-07-01

### Rust 依赖包安装

- [ 安装Rust依赖包 ](https://www.rust-lang.org/zh-CN/tools/install])
- 设置path 使用命令判断是否安装成功
> rustc --version
> cargo --version
- 创建项目
> cargo new 项目名称
- 编译
> cargo build
- 运行
> cargo run

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

### 注释方式

- 单行注释 （ \\\\ ）
- 范围注释 （\\\* 开始  结束\*\\ ）


### 类型

- 基本类型 整型（有符号和无符号）和char（字符型）
> 有符号 i8 i16 i32 i128 和 isize 例如 let _number: i8 = 100;
> 
> 无符号 u8 u16 u32 u128 和 usize 例如 let _number: u8 = 100;
> 
> char ansi码和unicode码来识别的 
>
> 整型和char互转  
```
let _number = 100;
println!("{}", _number as u8 as char)
输出为 d
```
- char长度问题 len和count
```
// len()为字节的大小 chars().count() 为汉字或英文的长度
let slice = "中国人";
println!("Slice is {} bytes and Word is {}", slice.len(), slice.chars().count());
```
> Slice is 12 bytes and Word is 3

