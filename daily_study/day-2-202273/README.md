# 2022-07-03

### 学习进度

- 今日进行了[ Rust练习实践 ](https://zh.practice.rs/),巩固之前自学内容

### 知识积累

-  u16::checked_add(251, 8).unwrap();
```
/*
checked_add的方式有两种
第一种为u16::checked_add(数值，数值),相加
第二种类型直接调用checked_add()

checked_*：返回的类型是Option<_>，当出现溢出的时候，返回值是None；
saturating_*：返回类型是整数，如果溢出，则给出该类型可表示范围的“最大/最小”值；
wrapping_*：直接抛弃已经溢出的最高位，将剩下的部分返回
*/
let i = 100_i8;
println!("checked {:?}", i.checked_add(i));
println!("saturating {:?}", i.saturating_add(i));
println!("wrapping {:?}", i.wrapping_add(i));

/*
unwrap()方法源代码如下
pub fn unwrap(self) -> T {
  match self {
        Ok(t) => t,
        Err(e) => unwrap_failed("called `Result::unwrap()` on an `Err` value", e),
    }
}
相关类型：unwrap_or 、 unwrap_or_else 或 unwrap_or_default 。
不建议使用，因为会出现Panics（程序终止）
*/ 
let x:Option<&str> = None;
assert_eq!(x.unwrap(), "air"); // fails
```

- 获取数组
```
use std::ops::{Range, RangeInclusive};
Range{ start: 1, end: 5 }) 	// 不包含5
RangeInclusive::new(1, 5)	// 包含5
```
