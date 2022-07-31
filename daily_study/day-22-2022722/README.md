# 2022-07-22

## 学习进度

- 继续进行lab0-2的文档解读

## 知识积累
### 应用程序设计
1. 项目结构
- user/src/bin
```rust
    // 引用user目录下的lib.rs
    #[macro_use]
    extern crate user_lib
    
    // lib.rs
    // 设置了用户库的入口点:_start:
    #[no_mangle]
    #[link_section = ".text.entry"]
    pub extern "C" fn _start() -> !{
        clear_bss();
        exit(main());
    }
    // 使用link_section宏建_start函数编译后的汇编代码放到.text.entry中
    // clear() 清零.bss段
    #![feature(linkage)] //启用弱链接特性
    
    #[linkage="weak"]
    #[no_mangle]
    fn main() -> i32{
        panic!("cannot find main");
    }
    // 标志位弱链接，当bin目录下存在main符号时，连接器会使用bin目录下的函数作为main
```
2. 内存布局
- 将程序的物理地址调整为0x80400000，三个程序都被加载到这个物理地址上
- 将_start所在的.text.entry放在整个程序的开头0x80400000
