# 2022-07-9

### 学习进度

- 速读 [ Rust语言圣经 ](https://course.rs/first-try/slowly-downloading.html)
- 争取1-2天搞定，然后开始做练习，感觉速度慢了

### 知识积累
#### 下载慢添加镜像
在 $HOME/.cargo/config.toml 添加以下内容：
```
[source.crates-io]
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
```
#### 下划线开头可忽略未使用的变量 let _x = 5;

https://course.rs/basic/base-type/index.html
