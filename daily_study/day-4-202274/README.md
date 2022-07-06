# 2022-07-04

### 学习进度

- 今日继续学习[ 简单英语学Rust ](https://kumakichi.github.io/easy_rust_chs/Chapter_27.html),巩固之前自学内容
- 学习速度加快
### 知识积累

- 调用 struct 或 enum 上的函数，请使用 impl 块
- 1.struct使用impl调用
```
// 声明struct结构体
struct Animal {
    age: u8
}
// 实现结构体
impl Animal {
    // 声明new方法，返回自己，类似于this
    fn new() -> Self {
        Self {
            age:10,
        }
    }
    // 自我赋值
    fn change_to(&mut self) {
        age = 11;
    }
}
fn main() {
    // 实例化
    let mut new_animal = Animal::new();
    new_animal.change_to();
}
```
- 2.enum使用impl调用
```
// 声明枚举
enum Mood {
    Good,
    Bad,
    Sleepy,
}
// 实例化
impl Mood {
    fn check(&self) {
        match self {
            Mood::Good => println!("Feeling good!"),
            Mood::Bad => println!("Eh, not feeling so good"),
            Mood::Sleepy => println!("Need sleep NOW"),
        }
    }
}
// 调用
fn main() {
    let my_mood = Mood::Sleepy;
    my_mood.check();
}
 ```
- truct或enum，如果你想使用{:?}来打印，你需要给它Debug，所以我们将这样做:如果你在结构体或枚举上面写了#[derive(Debug)]，那么你就可以用{:?}来打印。这些带有#[]的信息被称为属性
```
use std::fmt::Debug;
引用后打印的使用{:?}
use std::fmt::Display;
引用后打印的使用{}
```
- 解构
```
struct Person{
    name:String,
}
let per = Person{name:"Wang".to_string()};
let Person{
    name, // 或者name:a  前者可直接使用name，后者使用a变量 
}=pre;
```
- 引用 *和& 以及点引用
但是用&后，可直接使用点来获得数值
```
struct Item {
    number: u8,
}
fn main() {
    let item = Item {
        number: 8,
    };
    let reference_item = &item;
    reference_item.number == 8 // true
}
```
- 泛型
```
// 1.基础
fn return_number<T>(number: T) -> T {
    number
}
// 2.Debug模式
use std::fmt::Debug; // Debug is located at std::fmt::Debug. So now we can just write 'Debug'.
fn print_number<T: Debug>(number: T) { // <T: Debug> is the important part
    println!("Here is your number: {:?}", number);
}
// Display模式+多个泛型+声明PartialOrd进行两个数比较
use std::fmt::Display;
use std::cmp::PartialOrd;
fn compare_and_display<T: Display, U: Display + PartialOrd>(statement: T, num_1: U, num_2: U) {
    println!("{}! Is {} greater than {}? {}", statement, num_1, num_2, num_1 > num_2);
}
```
- 异常 (unwrap 类似于拆盲盒，指不定就有None)
- 1.Option 
```
// 判断后返回None或者Some(数值)
fn take_fifth(value: Vec<i32>) -> Option<i32> {
    if value.len() < 5 {
        None
    } else {
        Some(value[4])
    }
}
// 比对Option
fn handle_option(my_option: Option<i32>) {
    match item {
      Some(number) => println!("Found a {}!", number),
      None => println!("Found a None!"),
    }
}
// 比对Option也可通过
if take_fifth(vec![1,2]).is_some() {
    take_fifth(vec![1,2]).unwarp();//该值一定不是None
} 
```
- 2.Result
```
// Option大约是Some或None(有值或无值)。
// Result大约是Ok或Err(还好的结果，或错误的结果)。
enum Option<T> {
    None,
    Some(T),
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
// 从city中遍历
while let Some(information) = city.pop(){} 
// 将information转化为i32 并判断是否错误
if let Ok(number) = information.parse::<i32>() {}
  ```

- HaspMap 和 BTreeMap 用法一致，区别：BTreeMap 默认排序(key)
```
use std::collections::HashMap;
let mut population: HashMap<u32, u32> = HashMap::new();
population.insert(1,3);
for (key, value) in population{}
population[key]  //获得value，不过容易出现None     返回的是数值
population.get(key) //当key么有value时，会出现None 返回的是Option
// pub fn entry(&mut self, key: K) -> Entry<K, V>
/* 
fn or_insert(self, default: V) -> &mut V { // 🚧
    match self {
        Occupied(entry) => entry.into_mut(),
        Vacant(entry) => entry.insert(default),
    }
}
*/ 
let address = population.entry(key).or_insert(value)
*address == value //true
// 相同key的value数组
population.entry(key).or_insert(Vec::new()).push(value)
```
- HashSet和BTreeSet 只有key的HashMap，类似于java的List （use std::collections::HashSet;）
- 二叉堆 BinaryHeap （use std::collections::BinaryHeap;）
```
BinaryHeap::new();
push() pop()
每次拿最大的
```
- VecDeque 类似于Vec，拿取两端数据时比Vec快
```
 VecDeque::from(vec![0; 600000]);
 my_vec.pop_front();// 只能从头或者从尾拿
```
- ？ 防止程序崩溃
- panic! 中断宏
```
// 断言
assert!(bool, msg): 如果()里面的部分不是真的, 程序就会崩溃.
assert_eq!(key1, key2, msg):()里面的两个元素必须相等。
assert_ne!(key1, key2, msg):()里面的两个元素必须不相等。(ne表示不相等)
```
- expect 可等价替换unwrap，不过expect（msg）,出现错误会显示msg
- trait（特性）
```
struct Wizard {}
struct Ranger {}

trait FightClose {
    fn attack_with_sword(&self, opponent: &mut Monster) {
        opponent.health -= 10;
        println!(
            "You attack with your sword. Your opponent now has {} health left.",
            opponent.health
        );
    }
    fn attack_with_hand(&self, opponent: &mut Monster) {
        opponent.health -= 2;
        println!(
            "You attack with your hand. Your opponent now has {} health left.",
            opponent.health
        );
    }
}
impl FightClose for Wizard {}
impl FightClose for Ranger {}
fn main() {
    let radagast = Wizard {};
    let aragorn = Ranger {};
    radagast.attack_with_sword(&mut uruk_hai);
    aragorn.attack_with_bow(&mut uruk_hai, 8);
}
```
- Form trait
```
use std::convert::From;

struct EvenOddVec(Vec<Vec<i32>>);

impl From<Vec<i32>> for EvenOddVec {
    fn from(input: Vec<i32>) -> Self {
        let mut even_odd_vec: Vec<Vec<i32>> = vec![vec![], vec![]]; // A vec with two empty vecs inside
                                                                    // This is the return value but first we must fill it
        for item in input {
            if item % 2 == 0 {
                even_odd_vec[0].push(item);
            } else {
                even_odd_vec[1].push(item);
            }
        }
        Self(even_odd_vec) // Now it is done so we return it as Self (Self = EvenOddVec)
    }
}

fn main() {
    let bunch_of_numbers = vec![8, 7, -1, 3, 222, 9787, -47, 77, 0, 55, 7, 8];
    let new_vec = EvenOddVec::from(bunch_of_numbers);

    println!("Even numbers: {:?}\nOdd numbers: {:?}", new_vec.0[0], new_vec.0[1]);
}
```
- AsRef
```
use std::fmt::{Debug, Display}; // add Debug

fn print_it<T>(input: T) // Now this line is easy to read
where
    T: AsRef<str> + Debug + Display, // and these traits are easy to read
{
    println!("{}", input)
}

fn main() {
    print_it("Please print me");
    print_it("Also, please print me".to_string());
}
```
### 截止到35
