# 2022-07-6

### 学习进度

- 今日继续学习[ 简单英语学Rust ](https://kumakichi.github.io/easy_rust_chs/Chapter_39.html)

### 知识积累

#### 1.&str
##### 字符串
- 直接声明时，生命周期为整个程序，直接写进了二进制中，类型为&'static str',意为字符串字元有一个叫static的生命期
##### 借用str 
- 作为String的引用来使用，无生命期，例如 fn print_str(my_str: &str){}，调用为print_str(&String::format("string""));

#### 生命期
- 引用的生命期不能比这个对象长
```
fn returns_str() -> &str{
    return "I am" // ---错误，因为这个值声明是在方法内，引用出去后是null
}
// 改进
fn returns_str() -> &‘static str{
    return "I am" // ---正确，返回了字符串字面量，该值整个程序有效
}
```
##### struct使用
```
// 尝试写struct City<'a>和name: &'a str
// 意思是 name可以存活到与City一样的生命期
// a可以为任意值
#[derive(Debug)]
struct City<'a>{
    name: &'a str,
    date_f: u32,
}
let my_city = City{
    name: &"China".to_string(),
    date_f: 2022,
}
```

### struct 加上方法
```
// 不仅仅struct要与name同一个生命期，impl也要一致
struct Adventurer<'a>{
    name: &'a str,
    hit_points: u32,
}
impl Adventer<'_>{
    fn take_damage(&mut self){
        slef.hit_points -= 20;
        println!("{} has {} hit", slef.name, self.hit_points);
    }
}
fn main(){
    let mut bi = Adventurer{
        name:"Billy",
        hit_points: 100,
    }
    bi.take_damage();
}
```

#### 内部可变性
- 不用mut也可修改数据
##### Cell
```
// 给出的是值而不是引用
use std::cell::Cell;
struct PhoneModel{
    on_sale: Cell<bool>,
}
fn main(){
    let huawei_phone = PhoneModel{
        on_sale： Cell::new(true),
    };
    huawei_phone.on_sale.set(false);
}
```
##### RefCell
```
// 引用单元格，引用
// 小心使用，他是运行后编译
use std::cell::RefCell;
struce User{
    active: RefCell<bool>,
}
fn main(){
    let user_l = user{
        active: RefCell::new(true),
    };
    println!("{}", user_l.active);

    user_l.active.borrow();
    user_l.active.borrow_mut(); //引出后就不能再borrow_mut()了
    user_l.active.replace(false);

}
```

##### Mutex
- 线程安全，只让一个线程修改它（有lock（））
```
use std::sync::Mutex;
let my_mutex = Mutex::new(5);
// 加锁
let mut mutex_changer = my_mutex.lock().unwarp();
// 加锁后修改，也可大括号括起来，括号外自己解锁（超出加锁范围）
*mutex_changer = 6;
// 解锁
std::mem::drop(mutex_changer);
// 判断是否可用
if let Ok(value) = my_mutex.try_lock(){
    //没加锁
}else{
    // 加锁
}

// 直接赋值，不需要解锁
*my_mutex.lock().unwarp() = 6;
```

##### RwLock
- 读写锁 类似于Mutex和RefCell
- 多个.read() 可以，一个.write() 可以
```
use std::sync::RwLock;
let my_rwlock = RwLock::new(5);
let read1 = my_rwlock.read().unwrap(); // 多个read都可以
let read2 = my_rwlock.read().unwrap();// 多个read都可以
drop(read1); 
drop(read2); // 两个read()都进行了解锁，因此可以write()
let mut write1 = my_rwlock.write().unwarp();
*write1 = 6;
drop(write1); // 解锁后可继续操作

if let Ok(mut number) = my_rwlock.try_write(){
    *number += 10
}else{
    // 锁住了
}
```

#### Cow
##### 枚举，写时克隆
````
// 源码： 当类型为B时，返回 Borrowed(&'a B), 否则 Owned(<B as ToOwned>:Owned)
pub enum Cow<'a, B> where B: 'a + ToOwned + ?Size,{
    Borrowed(&'a B),
    Owned(<B as ToOwned>:Owned),
}
// Cow<'static, str>判断时为
// Cow::Borrowed(message) : message 为 实际值
// Cow::Owned(message) : message 为 实际值
//注意，返回的数值是str
fn modulo_3(input: u8) -> Cow<'static, str> {
    match input % 3 {
        0 => "Remainder is 0".into(),
        1 => "Remainder is 1".into(),
        remainder => format!("Remainder is {}", remainder).into(), // 这个为String，因此会变成Cow::Owned()
    }
}

fn main() {
    for number in 1..=6 {
        match modulo_3(number) {
            Cow::Borrowed(message) => println!("{} went in. The Cow is borrowed with this message: {}", number, message),
            Cow::Owned(message) => println!("{} went in. The Cow is owned with this message: {}", number, message),
        }
    }
}
````

#### 类型别名
```
//1
type CharacterVec = Vec<char>;
//2
type SkipFourTakeFive<'a> = std::iter::Take<std::iter::Skip<std::slice::Iter<'a, char>>>;

fn returns<'a>(input: &'a Vec<char>) -> SkipFourTakeFive {
    input.iter().skip(4).take(5)
}
//3
use std::iter::{Task, Skip};
use std::slice::Iter;
fn returns<'a>(input: &'a Vec<char>) -> Take<Skip<Iter<'a, char>>> {
    input.iter().skip(4).take(5)
}
//4
struct File(String); // File is a wrapper around String

fn main() {
    let my_file = File(String::from("I am file contents"));
    let my_string = String::from("I am file contents");
    println!("{}", my_file == my_string);  // ⚠️ 错误，不能比较，因为类型不对
    println!("{}", my_file.0 == my_string);//      正确
}
//5 enum枚举简单调用
enum MapDirection{
    North, East,
}
fn main(){
    use MapDirection::*;
    //后面可直接使用 MapDirction
    match direction{
        North => println!(),
        East => println!();
    }
    // 也可以进行改名
    use MapDirection::{
        North as N,
        East as E,
    }
}
```
#### todo! 宏
- 还未想好如何操作，但是又要运行，可以用 todo!()
```
fn get_book(book: &Book) -> Option<String> {
    todo!() // todo means "I will do it later, please be quiet"
}
```
### Rc
- 使用数量变为0，才会消失
```
    use std::rc:Rc;
    let history = Rc::new("".to_string());
    let h2 = history.clone() // Rc的计数变为了2
    println!("{}", Rc::strong_count(&history)); //强引用数量
    Rc::downgrade(&item) 来创建弱引用 //如果只有弱引用，则变量会消失
    Rc::weak_count(&item) 查看弱引用数
```

#### 多线程
- std::thread::spawn 创建线程
````
fn main(){
    //当主程序结束，线程还未结束时，会报异常
    std::thread::spawn(||{
        println!("");
    });
    // 改进 将线程绑定到变量上
    let handle = std::thread::spawn(||{
        println!("");
    });
    handle.join();
    
    // 取值
    let mut my_str = String::form("");
    let handle = std::thread::spawn(move || { // 使用move来获得线程外的数值
        println!("{}", my_str);
    });
    handle.join();
}
````
#### 闭包
- FnOnce: 取整个值
- FnMut ： 取一个可变引用
- Fn    : 取一个普通引用
```
    // fn
    let my_str = String::form("string");
    let my_closure = || println!("{}", my_str);
    my_closure();   //可多次调用    
    my_closure(); // String没有实现copy，因此是Fn
    // FnMut
    let mut my_str = String::form("string");
    let mut my_closure = || {
        my_str.push_str(" clone");
        println!("{}", my_str);
    }
    my_closure();   //可多次调用，my_str会新增clone字符串
    my_closure();   // 如果改变my_str, 就变成了FnMut
    //  FnOnce
    let my_vec: Vec<i32> = vec![8, 9, 10];
    let my_closure = || {
        my_vec
            .into_iter() // into_iter takes ownership
            .map(|x| x as u8) // turn it into u8
            .map(|x| x * 2) // multiply by 2
            .collect::<Vec<u8>>() // collect into a Vec
    };
    let new_vec = my_closure(); //只能使用一次
```
- 闭包的伟大
```rust
/*
    fn all<F> 有一个通用类型F
    （&mut self, f:F） &mut Self 说明这是一个方法；f:F 变量名和类型
    闭包部分：
    F: FnMut(Self::Item)->bool
    闭包是FnMut，可以改变值，改变Self::Item的值，必须返回true或者false
*/
fn all<F>(&mut sel, f: F)->bool where F:FnMut(Self::Item)->bool,

```
