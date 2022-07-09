# 2022-07-7

### 学习进度

- 今日继续学习[ 简单英语学Rust ](https://kumakichi.github.io/easy_rust_chs/Chapter_47.html)

### 知识积累
#### impl Trait
- 与泛型类似
```
// 入参
// 类似与 fn prints_it<T:String+std::fmt::Display>>(input: T){}
fn prints_it(input: impl Into<String> + std::fmt::Display){
    println!("{}", input);
}
fn main(){
    prints_it("hi");
    prints_it(String::from("Hi"));
}
```
-闭包
```
// 使用impl声明一个闭包 FnMut(i32) 得到的就是number
fn returns_closure(input: &st)->impl FnMut(i32)->i32{
    match input{
        "double" => |mut number|{
            number += 2;
            number
        }
    }
}
fn main(){
    let mut doubles = returns_closure("double");
    doubles(10);
}
```
- Arc 原子引用计数器
```
use std::sync::{Arc, Mutex};
fn main(){
    let my_number = Arc::new(Mutex::new(0));
    let mut handle_vec = Vec![]; // JoinHandles will go int here
    for _ in 0..2{
        let my_number = Arc::Clone(&my_number);
        let handle = std::thread::spawn(move || {
            for _ in 0..10{
                *my_number.lock().unwrap() += 1;
            }
        });
        handle_vec.push(handle);
    }
    handle_vec.into_iter().for_each(|handle| handle.join().unwrap());
    println!("{"?}", my_number); // Mutex{data: 20}
}
```
- Channels
```rust 
// 生产者和消费者需要绑定在一起
pub fn channel<T>()->(Sender<T>, Recever<T>)
// 声明
use std::sync::mpsc::{channel, Sender, Receiver};
let (s, r):(Sender<i32>, Receiver<i32>) = channel();
// 简写
use std::sync::mpsc::channel;
let (sender, receiver) = channel();
sender.send(5);
receiver.recv();

//线程中使用
use std::sync::mpsc::channel;

fn main() {
    let (sender, receiver) = channel();
    let sender_clone = sender.clone();

    std::thread::spawn(move|| { // move sender in
        sender.send("Send a &str this time").unwrap();
    });

    std::thread::spawn(move|| { // move sender_clone in
        sender_clone.send("And here is another &str").unwrap();
    });
    // 打印内容看谁最后完成
    println!("{}", receiver.recv().unwrap());
    // 如何同时打印两个信息
    // 使用 let mut handle_vec = vec![];保存线程返回变量
    // for _ in handle_vec{} // 遍历中使用receiver.recv().unwarp();可同时显示两个
}
```

##### 属性
```
#[derive(Debug)] 影响下一行代码
#![derive(Debug)] 将影响整个空间
```
- 写了没有使用的代码，仍然会编译
```
// #[allow(dead_code)] 和 #[allow(unused_variables)]
// struct 只是声明，没有使用
#![allow(dead_code)]
#![allow(unused_variables)]
struct Struct1 {} // Create five structs
struct Struct2 {}
struct Struct3 {}
struct Struct4 {}
struct Struct5 {}

fn main() {
    let char1 = 'ん'; // and four variables. We don't use any of them but the compiler is quiet
    let char2 = ';';
    let some_str = "I'm just a regular &str";
    let some_vec = vec!["I", "am", "just", "a", "vec"];
}
```
- 创建结构和枚举派生
```
#[derive(Debug)]
struct HoldsAString {
    the_string: String,
}

fn main() {
    let my_string = HoldsAString {
        the_string: "Here I am!".to_string(),
    };
    println!("{:?}", my_string);
}
```
- 配置
```
#[cfg()]的意思是配置 ,例如
#[cfg(test)] // 告诉编译器不要运行他们，除非是在测试
#[cfg(target_os = "windows")] // windows环境下运行
#![no_std] // 不要引入std的标准库
```
#### Box
- 使用Box时，可以把一个类型放在堆上而不是栈上
- 成为智能指针
```rust
//1
let my_box = Box::new(1); // This is Box<i32> 
let an_integer = *my_box; // this is i32

//2
struct List{ item: Option<Box<List>>,}
impl List{
    fn new() -> List{
        List{
            item: Som(Box::new(List{item: None})),
        }
    }
}
fn main(){
    let mut my_list = List::new();
}
```
##### Box包裹trait
```
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct ErrorOne;

impl Error for ErrorOne {} // Now it is an error type with Debug. Time for Display:

impl fmt::Display for ErrorOne {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the first error!") // All it does is write this message
    }
}


#[derive(Debug)] // Do the same thing with ErrorTwo
struct ErrorTwo;

impl Error for ErrorTwo {}

impl fmt::Display for ErrorTwo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "You got the second error!")
    }
}

// Make a function that just returns a String or an error
fn returns_errors(input: u8) -> Result<String, Box<dyn Error>> { // With Box<dyn Error> you can return anything that has the Error trait

    match input {
        0 => Err(Box::new(ErrorOne)), // Don't forget to put it in a box
        1 => Err(Box::new(ErrorTwo)),
        _ => Ok("Looks fine to me".to_string()), // This is the success type
    }

}

fn main() {

    let vec_of_u8s = vec![0_u8, 1, 80]; // Three numbers to try out

    for number in vec_of_u8s {
        match returns_errors(number) {
            Ok(input) => println!("{}", input),
            Err(message) => println!("{}", message),
        }
    }
}
```
#### 默认值
```rust
let default_i8 : i8 = Default::default();       // 0
let default_str : String = Default::default();  // ""
let default_bool : bool = Default::default();   // false
// struct默认值
#[derive(Debug)]
struct Character {
    name: String,
    age: u8,
    height: u32,
    weight: u32,
    lifestate: LifeState,
}

#[derive(Debug)]
enum LifeState {
    Alive,
    Dead,
    NeverAlive,
    Uncertain,
}

impl Character {
    fn new(name: String, age: u8, height: u32, weight: u32, alive: bool) -> Self {
        Self {
            name,
            age,
            height,
            weight,
            lifestate: if alive {
                LifeState::Alive
            } else {
                LifeState::Dead
            },
        }
    }
}

impl Default for Character {
    fn default() -> Self {
        Self {
            name: "Billy".to_string(),
            age: 15,
            height: 170,
            weight: 70,
            lifestate: LifeState::Alive,
        }
    }
}

fn main() {
    let character_1 = Character::default();

    println!(
        "The character {:?} is {:?} years old.",
        character_1.name, character_1.age
    );
}
```
- 建造者模式
```
// 注意 mut self而不是&mut self
// 改变自身的height，并返回自身
fn height(mut self, height: u32) -> Self {   
    self.height = height;
    self
}
// 调用Character::default().height(180)
```
#### Deref和DerefMut
- 加上Deref 后，在使用new等返回self都只会返回Deref中的设置信息
```
use std::ops::Deref;
#[derie(Debug)]
struct HoldsANumber(u8);
impl Deref for HoldsANumber{
    type Target = u8;
    fn deref(&self)->&Self::Target{
        &self.0
    }
}
let my_num = HoldsANUmber(20);
println!("{:?}", *my_num + 20)
// 标准库实例
use std::ops::Deref;
struct DerefExample<T>{
    value:T
}
impl<T> Deref for DerefExample<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target{
        &self.alue
    }
}
fn main(){
    let x = DerefExample{ value:'a'};
    assert_eq!('a', *x);
}
```
- checked_sub() 安全的减法，整数可直接使用,如果失败返回None，不会崩溃
- DerefMut 使用前必须实现Deref
```
use std::ops::{Deref, DerefMut};

struct HoldsANumber(u8);

impl HoldsANumber {
    fn prints_the_number_times_two(&self) {
        println!("{}", self.0 * 2);
    }
}

impl Deref for HoldsANumber {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HoldsANumber { // You don't need type Target = u8; here because it already knows thanks to Deref
    fn deref_mut(&mut self) -> &mut Self::Target { // Everything else is the same except it says mut everywhere
        &mut self.0
    }
}

fn main() {
    let mut my_number = HoldsANumber(20);
    *my_number = 30; // DerefMut lets us do this
    println!("{:?}", my_number.checked_sub(100));
    my_number.prints_the_number_times_two();
}
```
#### Create和模块
```
// 声明一个mod
mod print_things{
    use std::fmt::Display;
    //必须加pub
    pub fn prints_one<T: Display>(input: T){
        println!("{}", input);
    }
}
fn main(){
    use create::print_thines::prints_one;
    prints_one(6);   
}
```
