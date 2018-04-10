//! main.rs
//! author: yakirChen

// use std::io;

// use gen_rand_nu::numeric;

fn main() {
    let rand_nu = fun(100);
    println!("1~100 随机数 {}", rand_nu);

    let nu = 100;
    let nu = nu + 1;
    println!("let 声明同名变量 {}", nu);;

    let mut name = "yakir".to_string();
    println!("hello {} !", &mut name);
    // io::stdin().read_line(&mut name).expect("Cann't Get Name !");

    let name = "yakirChen";
    println!("hello {} ! ", name);

    let mut variable = "Hello Rust, I'm yakir !";
    println!("{}", &mut variable);
    let variable = &mut variable.len();
    println!("variable字符串长度 {}", variable);
    let variable = String::new();
    println!("{}", variable);
    let mut variable = "Bonjour !";
    println!("{}", &mut variable);

    let float: f32 = -0.1;
    println!("{}", float);

    let boolean: bool = true;
    println!("{}", boolean);

    let tuple: (i32, f32, f64, bool, u8) = (1, -2.1, 3.0, false, 1);

    let bt2: f64 = tuple.2.into();
    println!("{}", bt2);
    let bt3: bool = tuple.3;
    println!("{}", bt3);

    let array = [0, 1, 2, 3, 4, 5, 6];
    println!("0:{} 5:{}", array[0], array[5]);
    // println!("10:{}", array[10]); 数组越界

    println!("{}", zero());

    let sum = {
        let one = 1;
        one + 1
    };
    println!("{}", sum);

    say_hello(String::from("chinese"));
    say_hello(String::from("french"));
    say_hello(String::from("english"));
    say_hello(String::from("esperanto"));

    let mut loop_nu = 3;
    while loop_nu > 0 {
        println!("while 条件循环 {}", loop_nu);
        loop_nu = loop_nu - 1;
    }

    for ele in array.iter() {
        println!("数组元素 for 循环 {}", ele);
    }

    let mut index = 0;
    while index < 7 {
        println!("数组元素 while 循环 {}", array[index]);
        index = index + 1;
    }

    // for number in (1..4).rev() {
    //     println!("{}!", number);
    // }
    // println!("LIFTOFF!!!");

    for number in 1..4 {
        println!("{}", number);
    }

    let mut name = String::from("yakir");
    name.push_str("Chen");
    println!("{}", &mut name);

    let my_name = name;
    // 这个时候 name 无效 所以 println!("{}", &mut name); 在编译时会出错
    println!("{}", my_name);

    let my_name_clone = my_name.clone();
    println!("源: {} clone: {}", my_name, my_name_clone);

    let age = 18;
    fun_int(age);

    let yakir: String = String::from("yakir");
    fun_str(yakir);

    // 使用函数返回的元组
    // 方式一
    let fun_tuple_res: (String, usize) = fun_tuple(String::from("yakir"));
    println!("{} {}", fun_tuple_res.0, fun_tuple_res.1);
    // 方式二
    let (val0, val1) = fun_tuple(String::from("yakir"));
    println!("{} {}", val0, val1);

    let yakir_ref: String = String::from("yakir");
    fun_str_ref(&yakir_ref);

    let mut yakir_ref_change: String = String::from("yakir");
    fun_str_ref_change(&mut yakir_ref_change);
}

pub fn fun(param: u32) -> u32 {
    return param + 100;
}

pub fn zero() -> u8 {
    0
}

fn fun_tuple(val: String) -> (String, usize) {
    let len = val.len();
    (val, len)
}

pub fn say_hello(val: String) {
    let hi = if val == "chinese" {
        "你好 !"
    } else if val == "french" {
        "Bonjour !"
    } else if val == "english" {
        "Hello !"
    } else {
        "🤩"
    };
    println!("{}", hi);
}

pub fn fun_int(val: u32) -> u32 {
    val + 1
}

pub fn fun_str(val: String) -> String {
    val
}

/// 变量引用
pub fn fun_str_ref(val: &String) -> usize {
    val.len()
}

/// 尝试修改借用的变量
// pub fn fun_str_ref_change(val: &String) {
//     val.push_str(", world");
// }
pub fn fun_str_ref_change(val: &mut String) {
    val.push_str("Chen");
}
