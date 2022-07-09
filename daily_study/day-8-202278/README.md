# 2022-07-8

### 学习进度

- 今日继续学习[ 简单英语学Rust ](https://kumakichi.github.io/easy_rust_chs/Chapter_58.html)

### 知识积累
#### 测试
```
// 需要使用 cargo test 来运行测例
#[test]
fn test(){
    assert_eq!(2, 2);
}
// 完整测例
#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn one_test(){
        assert_eq!(1, 3);
    }
    ...
    #[test]
    #[should_painic] // 崩溃点
    fn panics_not_right(){
        println("");
    }
    // 到达should_painic就崩溃，测试完成
}
```
#### 外部引入
- 在Cargo.toml中有[dependencies]标签
```
[package]
name = "demo_rust"
version = "0.0.0"
authors = ["wang"]
ediion = "2022"

[dependencies]
rand = "0.7.3"
// 代码使用
use rand;
let random_u16 = rand::random::<u16>();
println!("{}", random_u16);
// rand的另一种使用
use rand::{thread_rng, Rng};
let mut number_maker = thread_rng();
//随机生成1到11之间的数字
println!("{}", number_maker.gen_range(1, 11));
```
##### rayon
- .iter(), .iter_mut(), into_iter()在rayon中是这样写的:
- .par_iter(), .par_iter_mut(), par_into_iter(). 所以你只要加上par_，你的代码就会变得快很多。(par的意思是 "并行")
```
use rayon::prelude::*; // Import rayon
// .enumerate()来获取每个数字的索引
// .par_iter_mut() 来遍历
fn main() {
    let mut my_vec = vec![0; 200_000];
    my_vec.par_iter_mut().enumerate().for_each(|(index, number)| *number+=index+1); // add par_ to iter_mut
    println!("{:?}", &my_vec[5000..5005]);
}
```
##### serde
- JSON YAML等格式相互转换
```
// Serialize, Deserialize
#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}
```
##### regex 
- 正则表达式
##### chrono
- 时间功能的库

#### 标准库
##### 数组
- 不能使用for，可使用.iter()和&来切片
- 解构
```
let my_city = ["WeiFang", "BeiJing"];
for city in &my_city{
}
for city in my_city.iter(
)
```
##### char
- 可用.escape_unicode() 获取Unicode号码
- char::from(99) 或者char::try_from().unwarp_or('-')
##### 证书
- 不会崩溃 .checked_add(), .checked_sub(), .checked_mul(), .checked_div()
- 实现Add
```
use std::ops::Add; // first bring in Add

#[derive(Debug, Copy, Clone, PartialEq)] // PartialEq is probably the most important part here. You want to be able to compare numbers
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    // 相加的时候另一个的数值
    type Output = Self; // Remember, this is called an "associated type": a "type that goes together".
                        // In this case it's just another Point

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
let p1 = Point{1, 2};
let p2 = Point{1, 2};
p1+p2 就调用了Add
```
- Sub Mul Div 基本一样，+=、-=、*=和/=，只要加上Assign:AddAssign、SubAssign、MulAssign和DivAssign即可
- % 被称为 Rem, - 被称为 Neg
##### 浮点数
- f32和f64
- .floor(): 给你下一个最低的整数.
- .ceil(): 给你下一个最高的整数。
- .round(): 如果小数部分大于等于0.5，返回数值加1;如果小数部分小于0.5，返回相同数值。这就是所谓的四舍五入，因为它给你一个 "舍入"的数字(一个数字的简短形式)。
- .trunc():只是把小数点号后的部分截掉。Truncate是 "截断"的意思。
- .fold(): 用来得到最高或者最低的数
```
    let my_vec = vec![8.0_f64, 7.6, 9.4, 10.0, 22.0, 77.345, 10.22, 3.2, -7.77, -10.0];
    let maximum = my_vec.iter().fold(f64::MIN, |current_number, next_number| current_number.max(*next_number)); // Note: start with the lowest possible number for an f64.
    let minimum = my_vec.iter().fold(f64::MAX, |current_number, next_number| current_number.min(*next_number)); // And here start with the highest possible number
    println!("{}, {}", maximum, minimum);
```
##### bool
- 可以将bool变成一个整数
```
    let true_false = (true, false);
    println!("{} {}", true_false.0 as u8, true_false.1 as i32);
```
- 。then() 闭包
```
    let (tru, fals) = (true.then(|| 8), false.then(|| 8));
    println!("{:?}, {:?}", tru, fals);
```
##### Vec
- sort() 升序排列，最小的在前
- sort_unstable() 更快速的排序
- dedup() 去重
##### String 与Vec类似
- .push() 加入一个char
- .push_str() 加入一个&str
##### OsString和CString
- std::ffi 外部函数接口
- OsString 非unicode
##### Mem
- std::mem
- .size_of() .size_of_val()和.drop()
```
use std::mem;

fn main() {
    println!("{}", mem::size_of::<i32>());
    let my_array = [8; 50];
    println!("{}", mem::size_of_val(&my_array));
    let mut some_string = String::from("You can drop a String because it's on the heap");
    mem::drop(some_string);
    // some_string.clear();   If we did this it would panic
}
打印结果
4
200
```
- swap() 交换两个变量
```
mem::swap(&mut T, &mut T)
```
- replace()
```
pub fn replace<T>(dest: &mut T, mut src: T) -> T {
    swap(dest, &mut src);
    src
}
```
- take() 与replace类似
```
    let mut number_vec = vec![8, 7, 0, 2, 49, 9999];
    let mut new_vec = vec![];

    number_vec.iter_mut().for_each(|number| {
        let taker = mem::take(number);
        new_vec.push(taker);
    });
```
##### Prelude
- 让println!()可以不用use std即可
##### Time
- Instant::now()获得系统时间
- 与sleep联合使用
```
use std::time::Duration;
use std::thread::sleep;

fn main() {
    let three_seconds = Duration::from_secs(3);
    println!("I must sleep now.");
    sleep(three_seconds);
    println!("Did I miss anything?");
}
```
- column!() 当前列数
- file!() 文件的名称
- line!() 当前行
- module_path!() 当前模块名称。
#### 编写宏
- 使用macro_rules!{}
- 接受一个输入然后给一个输出
```
macro_rules! give_six{
    (6)=>{6};
    ()=>{println!("None")}
}
let six = give_size(6); //返回6
give_size(); // 打印 None
// 2. 改进
// $input:expr 对于一个表达式，给他起一个变量名$input
/*
    expr还可替换：
    item: an Item
    block: a BlockExpression
    stmt: a Statement without the trailing semicolon (except for item statements that require semicolons)
    pat: a Pattern
    expr: an Expression
    ty: a Type
    ident: 标识符 an IDENTIFIER_OR_KEYWORD
    path: a TypePath style path
    tt: 表示树 a TokenTree (a single token or tokens in matching delimiters (), [], or {})
    meta: an Attr, the contents of an attribute
    lifetime: a LIFETIME_TOKEN
    vis: a possibly empty Visibility qualifier
    literal: matches -?LiteralExpression
*/
macro_rules! might_print{
    ($input:expr)=>{
        println!("{}", $input)
    }
}
```

#### 编译
- cargo run 建立并运行
- cargo build 构建程序
- cargo build --release 和 cargo run --release 发布模式
- cargo check 检查代码
- cargo new 项目名 新建项目
- cargo clean 清除
- cargo doc 创建

#### 接受用户输入
- std::io::stdin
```
use std::io;
fn main(){
    let mut input_string = String::new();
    io::stdIn().read_line(&mut input_string).unwarp();
    println!("{}", input_string);
}
```
- std::env::args()环境参数
```
use std::env::args;

fn main() {
    let input = args();

    for entry in input {
        println!("You entered: {}", entry);
    }
}
// 当运行cargo run ****时，****作为参数可以通过args()获得
```
- std::env::vars() 获得系统变量
```
    for item in std::env::vars() {
        println!("{:?}", item);
    }
    // 获得单个变量
    env!("USER")
    option_env!("ROOT").unwrap_or("Can't find ROOT"))
```
- main添加返回值
```
use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    fs::File::create("myfilename.txt")?.write_all(b"Let's put this in the file")?;
    Ok(())
}
```
- 操作文件
```
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn main() -> std::io::Result<()> {
    fs::write(文件名, 内容);
    let mut calvin_file = OpenOptions::new()
        .append(true).read(true).open(文件名)?;
    calvin_file.write_all(内容)?;
    write!(&mut calvin_file, 内容)?;
    fs::read_to_string(文件名)?;
    Ok(())
}
```
