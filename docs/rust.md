## :bookmark: Rust基础

### 简介

**Rust** -- 一种静态类型 _(statically typed)_, 没有垃圾回收 _(garbage collector, GC)_ 的语言

### 注释

#### 常规注释
```rust
// 行注释
/*
    块注释
*/
```

#### 文档注释
```rust
/*! 
//! 
/// 
```

*TODO: 关于注释待补充 😜*

### 变量和常量的规则

声明变量 **变量默认不可变(immutable)**

```rust
let variable = 0;
// variable = 1;
```
不能对不可变变量二次赋值() 可以在变量前修饰 `mut` 使变量可变. 还向coder表明其他代码可能会改变这个变量的值
```rust
let mut vaiable = 0;
variable  = 1;
```

变量重复声明(隐藏/ Shadowing)
```rust
let mut variable = "Hello Rust, I'm yakir !";
// variable = variable.len(); // 不允许修改变量类型
let variable = 0;
let variable = String::new(); // variable 隐藏之前声明的同名变量
let mut variable = "Bonjour !";
```

声明常量
```rust
const YEAR: u32 = 2018;
```
常量声明须指定常量值类型(这里为 `u32`)

### 内建类型之一: 标量(scalar)

**标量**类型代表单读一个值. Rust 有四种标量类型: 整型、浮点型、布尔型和字符型

#### 整型

Rust 默认数字类型是 `i32`  
|长度|有符号|无符号|
|:---|:---|:---|
|8-bit|i8|u8|
|16-bit|i16|u16|
|32-bit|i32|u32|
|64-bit|i64|u64|
|arch|isize|usize|

#### 浮点型
Rust 默认浮点型是 `f64`
|长度|浮点型|表示|
|:---|:---|:---|
|32-bit|f32|单精度浮点|
|64-bit|f64|双精度浮点|

#### 布尔型
`bool` 类型只有两个值 `true` 和 `false`

#### 字符型
`char` 类型代表一个 Unicode 标量(Unicode Scalar Value 包含 `U+0000` 到 `U+D7FF` 和 `U+E000` 到 `U+10FFFF` 在内的值)

### 内建类型之二: 复合(compound)

#### 元组
将一个或多个不同类型的值组合成一个复合类型的主要方式  
创建使用元组  
```rust
let tuple: (i32, f32, f64, bool, u8) = (1, -2.1, 3.0, false, 1);
let bt2: f64 = tuple.2.into();
println!("{}", bt2);
let bt3: bool = tuple.3;
println!("{}", bt3);
```

#### 数组
将一个或者多个同一类型的值集合, 数组元素的类型必须一致, 数组一旦声明长度不可变. 
```rust
let array = [0, 1, 2, 3, 4, 5, 6];
println!("0:{} 5:{}", array[0], array[5]);
```
如果访问数组索引之外的数据, 编译时会提示: `index out of bounds`, 运行时程序会异常


### 函数

函数声明  
```rust
fn zero() -> u8 {
    0
}
```
返回数字 **0**, 函数体是表达式 `0`

```rust
fn fun(param: u32) -> u32 {
  return param + 100;
}
```
入参和函数返回类型都是 `u32`, 函数体是语句 `return param + 100;`

使用元组返回多个值
```rust
fn fun_tuple(val: String) -> (String, usize) {
    let len = val.len();
    (val, len)
}

// 使用函数返回的元组
// 方式一 
let fun_tuple_res: (String, usize) = fun_tuple(String::from("yakir"));
println!("{} {}", fun_tuple_res.0, fun_tuple_res.1);
// 方式二
let (val0, val1) = fun_tuple(String::from("yakir"));
println!("{} {}", val0, val1);
```

#### 函数体语句(Statements)与表达式(Expressions)

```rust
let sum = {
    let one = 1;
    one + 1
};
```
语句代码块中表达式  
```rust
{
    let one = 1;
    one + 1
}
```
返回 `2`, 返回值绑定到变量 `sum` 上, 表达式中 `one + 1` 结束时不包含 `;` 表明**表达式不包含分号(;)**. 函数和表达式都可以返回值


### 流程控制

#### if 表达式
```rust
if <逻辑表达式> {
    // 做一些事情
}
```
因为`if`是一个**表达式**, 所以表达式的执行结果是有返回值的, 可以将`if`的执行结果赋给变量(**确保`if`表达式的每个分支返回结果是同一类型**)  
```rust
let hi = if val == 0 {
    "你好 !"
}else if val == 1 {
    "Bonjour !"
}else if val == 2 {
    "Hello !"
} else {
    "🤩"
};
println!("{}", hi);
```

#### loop 重复执行

```rust
loop {
    println!("loop");
}
```

运行之后需要手动停止程序

#### while 循环

`while` 条件循环
```rust
let mut loop_nu = 3;
while loop_nu > 0 {
    println!("while 条件循环 {}", loop_nu);
    loop_nu = loop_nu - 1;
}
```

`while` 集合迭代循环 _(条件循环的一种使用场景)_
```rust
let array = [0, 1, 2, 3, 4, 5, 6];
let mut index = 0;
while index < 7 {
    println!("数组元素 while 循环 {}", array[index]);
    index = index + 1;
}
```

条件 `loop_nu > 0` 成立的时候执行循环体, 条件不满足的时候退出循环

#### for 循环

`for` 循环迭代集合
```rust
let array = [0, 1, 2, 3, 4, 5, 6];
for ele in array.iter() {
    println!("数组元素 {}", ele);
}
```

`for` 循环 `Rang`
```rust
for number in (1..4) {
    println!("{}", number);
}
```
`(1..4)` 使用了标准库中 `Range`, 用来生成从一个数字到另一个数字之前的所有数字序列


### 所有权(ownership)

#### 堆(heap)和栈(stack)
- 栈
  栈以放入值的顺序存储, 以相反的顺序取出值(先进后出 \ last in, first out)  
  添加数据叫**进栈**(pushing onto the stack), 移出数据叫**出栈**(弹栈 \ popping off the stack)

- 堆
  对于在编译时数据大小不定的数据存储在堆中. 当向堆中存储数据时, 程序请求一定大小的空间, 操作系统在堆中找到空间并标记为已使用, 返回这块空间的**指针**(pointer). 这个过程称作在**堆中分配内存**(allocating on the heap).  
  获取到的**指针存储在栈中**, 需要访问堆中的数据时, 可以通过指针获取

当调用一个函数, 传递给函数的值(包括可能指向堆上数据的指针)和函数的局部变量被压入战中. 当函数结束时, 这些值被移出栈(弹栈).  
所有权系统需要处理的问题, 记录何处的代码在使用堆上的数据, 最小化堆上的冗余数据的数量以及清理堆上不再使用的数据(避免耗尽空间).  


#### 所有权规则

1. 每一个值都有一个**所有者** _(owner)_ 变量
2. 值有且只能有一个所有者
3. 当所有者(变量)离开作用域, 这个值将被丢弃

#### 变量作用域(scope)

```rust
{
    let var = "Hello";
}
```
变量 `var` 的作用域是在 `{}` 之间, 在作用域中变量是有效的, 离开作用域变量是无效的

#### String 类型

使用 `String` 的 `from` 函数, 从字符串字面量创建 `String`
```rust
let name = String::from("yakir");
```

修改字符串
```rust
let mut name = String::from("yakir");
name.push_str("Chen");  // 字符串追加 "yakirChen"
```
第一部分在编译时可以确定变量所占内存空间的大小.  
第二部分在运行时修改字符串.
  - 程序需要在运行时向操作系统请求内存空间
  - 处理完`String`之后需要将内存返还给操作系统

变量在离开作用域之后, 它所占用的内存会被自动释放. 当变量离开作用域, Rust会调用一个特殊的函数 `drop` 

`String`由三部分组成 1. 一个指向存放字符串内容内存的指针, 2. 长度, 3. 容量

变量与数据交互一: 移动  
```rust
let s1 = String::from("yakir");
let s2  = s1;
```
当把s1赋值给s2的时候 从 _**栈上**_ 拷贝了 `s1` 变量(s1的指针、长度、容量), 并 _**没有复制**_ 指针指向的 _**堆上的数据**_ .  
这个时候两个指针指向了堆中的同一块数据, `s1` 和 `s2` 离开了作用域, 那变量离开了作用域都会尝试释放相同的一块内存, 会产生一个二次释放(double free)的错误.  
`Rust`为保证内存安全, 这个场景中 `Rust` 认为 `s1` 不再有效(变量不能再使用), 因此 `s1` 就不用清理


变量与数据交互二: 克隆  
```rust
let s1 = String::from("yakir");
let s2  = s1.clone();
```
创建一个新的变量绑定克隆的数据

只在栈上的数据: 拷贝  
对于一些在编译时就已知存储大小的类型被存储在栈上, 拷贝实际的值是快速的, 且不会使变量被拷贝后原值无效化  
Rust 有一个 `Copy` 特性(trait)的特殊注解, 可以作用在类似整型这样直接存储在栈上的类型  
可以使用`Copy`特性的类型  
- 所有整数类型
- 布尔类型
- 所有浮点数类型
- 元组, 当且仅当元组包含的数据类型也是`Copy`的时候

#### 所有权和函数

将值传递给函数时可能会发生移动或者复制, 将变量的值赋给变量时**变量的所有权也随着转移**

```rust
fn main(){
    let age = 18;
    fun_int(age);
    println!("{}", age);

    let yakir: String = String::from("yakir");
    fun_str(yakir);
}
pub fn fun_int(val: u32) -> u32 {
    val
}

pub fn fun_str(val: String) -> String {
    val
}
```
在调用`fun_str`函数之后, 变量 `yakir` 的值发生了移动(就像复制语句一样)

#### 引用与借用

##### 引用(references)  
把变量传递给函数的时候在变量名前添加 `&` 引用符号, 同时定义函数的时候函数签名声明参数类型是引用(**`&String`**)  
```rust
pub fn fun_str_ref(val: &String) -> usize {
    val.len()
}
let yakir_ref: String = String::from("yakirChen");
fun_str_ref(&yakir_ref);
```
实质上我的理解😁, 入参`&String val`是一个指向变量 `String yakir_ref` 的指针, 变量 `yakir_ref` 包含对字符串 "yakirChen" 相关的信息(指针、长度、容量)

解引用(dereferencing)  
与引用 `&` 相对操作是解引用 `*`

将获取引用作为函数参数称为**借用(borrowing)**, _**借用的变量在函数体中是不能修改的**_

##### 可变引用


## :dart: Rust高级

.  
.  
.  



---
Reference  
[Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn)
