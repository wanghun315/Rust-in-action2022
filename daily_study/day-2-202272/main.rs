/*
声明了一个常数
*/
const NUMBER_OF_MONTHS:u32 = 12;
// 创建了一个静态的string数组，注意string和&str的不同，以及声明数组的方法
static SEASONS:[&str;4] = ["Spring", "Summer", "Fall", "Winter"];
fn main() {
    for i in 0..SEASONS.len(){
        println!("the SEASONS is {}", SEASONS[i]);
    }
    println!("Hello, world! the year is {} month", NUMBER_OF_MONTHS);
    // 可变引用
    mut_link();
    // 函数引用声明周期
    let country = String::from("Chian");
    print_country(country);
    /*
     再次调用时会报错
     print_country(country);
     原因：
     1.先声明了String类型的country，并赋值
     2.将country作为参数传给print_country后（注意传的是String 非&String），该变量会在函数内部释放，因此导致country无法使用
     3.解决方案：将print_country 加上返回值或者加上& ,例如print_count_ref
    */
    // 使用&方式 传入地址没事
    let mut count_ref = String::from("China");
    print_count_ref(&mut count_ref);
    print_count_ref(&mut count_ref);
    // 基本类型没事
    let test:u8 = 1;
    print_int(test);
    print_int(test);

    // 数组
    let array1 = ["One", "Two"]; // type is [&str; 2]
    let array2 = ["One", "Two", "Five"]; // type is [&str;3]
    let array3 = ["a";10]; //10个a的数组
    /*
    索引号从0开始（不是1）
    索引范围是不包含的（不包括最后一个数字，例如[2..5] 只有第二个，第三个，第四个）
    如果包含则改写为 &array3[2..=5]
    */
    let three_to_five = &array3[2..5]; // 索引从2到第5，不含5

    // 向量 功能多，但是速度比数组慢
    let name1 = String::from("wang");
    let name2 = String::from("zhao");
    let mut my_vec = Vec::new();
    my_vec.push(name1);
    my_vec.push(name2);
    //创建 i32的向量 使用vec!创建
    let int_vec = vec![8, 10, 30];
    // 查看向量的空间 -- 动态扩充的
    println!("{}", int_vec.capacity());
    // 可通过设置向量大小来加快向量速度---向量重新申请空间拖慢了速度
    // Vec::with_capacity 声明了8个长度的向量，此时capacity会大于等于8
    // 必须加mut
    let mut mun_vec = Vec::with_capacity(8);
    mun_vec.push('a');
    // 使用.into()将数组变为向量  数组转向量
    let my_vec2: Vec<_> = [9,10,20].into();

    // 元组 使用()来表示
    // 可以存任何类型
    let random_tuple = ("this is string", 8, vec!['a'], 'b', [8,80,800], 7.7);
    print!("{:?}, {:?}", random_tuple.0, random_tuple.2);
    let str_vec = vec!["one", "two", "three"];
    let(_,_,variable) = (str_vec[0], _, str_vec[3]);
    // 只获得数组的一部分数值
    print!("variable is {}", variable);

    // 结构体 struct
    struct Colour(u8, u8, u8);
    struct SizeAndColour{
        size:u32,
        colour:Colour,
        name:String
    }

    let colour = Colour(8,9,8);
    let size:u32 = 100;
    let name = String::from("name");
    // 不同名称时需要写两次，例如size：size
    let sac = SizeAndColour{
        size,
        colour,
        name
    };

    // 枚举
    enum ThingsInTheSky{
        Sun, Starts,
    }
    let sky = ThingsInTheSky::Starts; 
    
    // 控制流
    if true{
        print!("if is true");
    }
    let my_number = 5;
    if my_number==5 || my_number!=4{

    }else if my_number == 3 && my_number >10{

    }else{

    }
    // 可将match替代if，类似于swith
    let second_num =match my_number {
        0 => print!("0"),
        1 => print!("1"),
        _ => print!("other"),
    };
    //也可 match(string1, string2){ ("key1","key2")=>println!(""), _=>println(""),}

    // 循环
    // loop
    let mut counter = 0;
    'first_loop: loop{
        counter += 1;
        if counter > 9{
            'second_loop: loop{
                counter += 3;
                if counter == 50{
                    // 跳出第一个循环
                    break 'first_loop;
                }
            }
        }
    }
    let my_number = loop{
        counter += 1;
        if counter % 53 == 3{
            // 复制给my_number
            break counter;
        }
    };
    //while
    while counter < 5{
        counter += 1;
    }
    //for
    for number in 0..3{
        //循环0，1，2
    }
    for _ in 0..=3  {
        //循环0, 1, 2, 3
    }




}

fn print_count_ref(country_name: &mut String){
    // 加入mut为可修改数值，main方法可同步
    country_name.push_str(" is lager");
    println!("{}", country_name);
}

fn print_int(test:u8){
    println!("input number {}", test);
}

/*
可变引用赋值
声明类型的范围
一个& 对应一个*
*/
fn mut_link() -> String {
    //可赋值的i32类型
    let mut my_number = 8;
    // 将可赋值的地址给num_ref
    let num_ref = &mut my_number;
    // 数值加10 堆中的数据变为了8+10
    *num_ref += 10;
    // &my_number 与 num_ref地址一致，因此堆数据相同
    println!("{0}", my_number);

    let second_number = 800;
    let triple_referrnce = &&&second_number;
    println!("Seconde_number = triple_reference? {0}", second_number == ***triple_referrnce);
    
    // country为局部变量，当作为地址返回后，数值会取消，因此地址中会无数据
    let country = String::from("China");
    let count_ref = &country;
    (*count_ref).to_string()
}

fn print_country(country_name: String){
    println!("{country_name}", country_name = country_name);
}
