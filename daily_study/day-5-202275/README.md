# 2022-07-5

### 学习进度

- 今日继续学习[ 简单英语学Rust ](https://kumakichi.github.io/easy_rust_chs/Chapter_35.html)
- 学习速度加快
### 知识积累
- 链式方法 collect
>let new_vec = (1..=10).collect::<Vec<i32>>();  OR let new_vec: Vec<i32> = (1..=10).collect();

>Vec![].into_iter().skip(3).take(4).collect::<Vec<i32>>(); skip(3)-跳过3个循环从第四个开始；take(4)-获得四个数组

- 迭代器
>.iter() 引用的迭代器 .iter_mut() 可变引用的迭代器  .into_iter() 值的迭代器(不是引用)
```
let vector1 = vec![1, 2, 3];
// iter()  引用迭代，执行完毕还活着
vector1.iter().map(|x| x + 1).collect::<Vec<i32>>();
// into_iter() 值迭代，按值进行迭代，会损坏变量，做完该方法后就不能用了
let vector2 = vector1.into_iter().map(|x| x * 10).collect::<Vec<i32>>();//此时vector1变为了None
// iter_mut() 可变因此不需要collect来创建新的map
vector1.iter_mut().for_each(|x| *x +=100);
```
- next() 
```
#[derive(Debug, Clone)]
struct Library {
    library_type: LibraryType,
    books: Vec<String>,
}

#[derive(Debug, Clone)]
enum LibraryType {
    City,
    Country,
}

impl Library {
    fn add_book(&mut self, book: &str) {
        self.books.push(book.to_string());
    }

    fn new() -> Self {
        Self {
            library_type: LibraryType::City,
            // most of the time
            books: Vec::new(),
        }
    }
}

impl Iterator for Library {
    type Item = String;

    fn next(&mut self) -> Option<String> {
        match self.books.pop() {
            Some(book) => Some(book + " is found!"), // Rust allows String + &str
            None => None,
        }
    }
}

fn main() {
    let mut my_library = Library::new();
    my_library.add_book("The Doom of the Darksword");
    my_library.add_book("Demian - die Geschichte einer Jugend");

    for item in my_library.clone() { // we can use a for loop now. Give it a clone so Library won't be destroyed
        println!("{}", item);
    }
}
```
- lambda 使用|_| 其中_可谓正常参数
```
1.  let my_closure = |x: i32| println!("{}", x); 等价与 fn my_closure(let x:i32){prinln!("{}", x);}
2. .map() Vec![1,2].iter().map(|number| number * 2).collect::<Vec<i32>>(); 
3. .enumerate():给出一个带有索引号和元素的迭代器 
    Vec![1,2] .iter().enumerate().for_each(|(index, number)|println!("{0}P{1}", index, number);
4. .zip() 两个迭代器合并
    let some_numbers = vec![0, 1, 2, 3, 4, 5]; // a Vec<i32>
    let some_words = vec!["zero", "one", "two", "three", "four", "five"]; // a Vec<&str>

    let number_word_hashmap = some_numbers
        .into_iter()                 // now it is an iter
        .zip(some_words.into_iter()) // inside .zip() we put in the other iter. Now they are together.
        .collect::<HashMap<_, _>>();    
    number_word_hashmap.get(&2).unwarp() == two;
5. .char_indices() 按字节进行遍历
6. .filter(|b| b.lenght()<5).collection::<Vec<&str>> // 获得b，并判断b是否小于5; 保留你想保留的元素    
7. .filter_map() filter和map的集合，闭包返回Option
8. and_then() Option类型的进出
9. and() 
10 .any() all() 第一个遍历对了就返回，第二个全部查完
11. .find() 返回Option类型
12. cycle() 单循环
13. .fold()
截至：https://kumakichi.github.io/easy_rust_chs/Chapter_38.html
```
- dbg! 宏，类似于println！
```
inspect 类似于dbg!
  .inspect(|first_item| {
  println!("The item is: {}", first_item);
  match **first_item % 2 { // first item is a &&i32 so we use **
  0 => println!("It is even."),
  _ => println!("It is odd."),
  }
  })
```
> 截止：https://kumakichi.github.io/easy_rust_chs/Chapter_39.html
