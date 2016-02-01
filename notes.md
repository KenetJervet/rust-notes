# Rust笔记

闲话不多说，这里是最精髓的内容，no bullshit.

## 变量绑定

```rust
let x = 5;
```

### 模式

在let语句中，左侧并非变量名，而是模式。

```rust
let (x, y) = (1, 2);
```

### 类型标记

```rust
let x: i32 = 5;  // 自动类型推断
```

### 可变性

默认不可变，使用``mut``则可变。

```rust
let mut x = 5;
x = 10;
```

### 绑定的初始化

绑定在使用之前必须初始化。否则编译失败。

### 作用域和隐藏

大括号标记作用域块。函数定义也是一个块。块内绑定可以隐藏块外绑定，如：

```rust
let x: i32 = 8;
{
    let x = 12;  // 隐藏外部绑定
}
```

隐藏和可变性是完全不同的概念（是否使用let来区分）。

## 函数

```rust
fn func(x: i32) -> i32 {
    x + 1
}
```

1. 函数参数***必须声明类型***
2. 若无返回值，可省略``->``及之后的部分
3. 若有返回值，最后一行为返回值，且***不能加分号***

### 表达式和语句

简言之，表达式有值，语句没有值。

1. let是语句
2. 可变绑定赋值是表达式（y = 6），其值为``()``
3. 表达式语句：使用``;``将表达式转换为语句。

### 提前返回

```rust
return x;
```

用在最后被视为傻逼编码风格。

### 发散函数

```rust
fn diverge() -> ! {
    panic!("This function never returns");
}
```

似乎就是Haskell中的``_|_``

顺便，使用RUST_BACKTRACE可以看到调用栈。对直接执行和``cargo run``都生效。

### 函数指针

```rust
fn plus_one(i: i32) -> i32 {
    i + 1
}

let f: fn(i32) -> i32 = plus_one;
// 或
let f = plus_one
```

调用方式还是一样的：

```rust
let six = f(5);
```

## 基本类型

### 布尔

类型：``bool``
表示：``true``/``false``

### 字符

类型：``char``
表示：``'狗'``
代表任意Unicode字符

### 数值类型

类型：``i8``、``i16``、``i32``、``i64``、``u8``、``u16``、``u32``、``u64``
    ``f32``、``f64``、``isize``、``usize``

其中，``isize``和``usize``为变长类型（由机器决定），其他的类型不论在何处环境下都是固定长度的。

### 数组

```rust
let x = [1, 2, 3]; // x: [i32; 3]
let mut m = [1, 2, 3]; // m: mut [m32; 3]

// 初始化的简便方法：
let x20 = [0; 20]; // a: [i32; 20]，全部初始化为0
```

取数组长度用``len()``
访问元素使用下标，语法和其他很多语言相同：

```rust
let a = [1, 2, 3];

println!("a has {} elements", a.len());
println!("the second is {}", a[1]);
```

### 切片

非拷贝的数据结构部分访问方式。

```rust
let a = [1, 2, 3, 4, 5];
let complete = &a[..];  // 所有元素
let middle = &a[1..4];  // 只包含2, 3, 4
```

### 字符串

类型： ``str``

非固定大小类型。这里不做过多介绍。

### 元组

```rust
let x = (1, "hello");
let x: (i32, &str) = (1, "hello");
```

使用解构``let``访问元组

```rust
let (x, y, z) = (1, 2, 3);
```

另外：

```rust
(0,);  // 单元素元组
(0);  // 0
```

除``let``外，还可使用元组索引：

```rust
let tuple = (0, 1, 2);

let x = tuple.0;
let y = tuple.1;
let z = tuple.2;
```

### 函数

函数类型在讲函数指针时已经有所认识。

## 注释

### 单行注释

```rust
// 我是单行注释
```

### 多行注释

文档中并未说明，但根据[这个提交](https://github.com/rust-lang/rust/commit/244ea680820c205461ad5af979c0a722372e6dc6)，应该支持C风格的多行注释。

```rust
/* 我是
多行注释 */
```

### 文档注释

```rust
/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #     x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
```

### 用于箱、模块、函数的文档注释

```rust
//! # The Rust Standard Library
//!
//! The Rust Standard Library provides the essential runtime
//! functionality for building portable Rust software.
```

常用于箱根``lib.rs``或模块根``mod.rs``中。

### 生成HTML文档

使用``rustdoc``生成，还可以运行代码示例当作测试。

## 条件判断

### if

```rust
if x == 5 {
    println!("x is five");
} else if x == 6 {
    println!("x is six");
} else {
    println!("x is not five nor six");
}
```

比较特殊的是，在Rust中，``if``是一个表达式，也就是说，可以这么搞：
```rust
let x = 5;
let y = if x == 5 { 10 } else { 20 }
```
当没有``else``子句时，if表达式的值为``()``

## 循环

### loop

表示无限循环。

```rust
loop {
    println("Loop forever");
}
```

### while

```rust
let mut done = false;
while !done {
    ...
}
```

```rust
while true {
    ...
}
```
和
```rust
loop {
    ...
}
```
作用相同，显然用后者更好。

### for

Rust中的``for``类似于Python中的``for``，而非C或Java。

```rust
for x in 0..10 {
    println!("{}", x);
}
```

更抽象的表达为：
```rust
for var in expression {
    code
}
```
其中，``expression``必须可以通过``IntoIterator``转换为迭代器。

Rust中没有C中对应的``for``循环。

### enumerate

Rust中的``enumerate``函数也类似于Python中的内置函数``enumerate``，当需要获取当前迭代次数时使用：

```rust
for (i, j) in (5..10).enumerate() {
    println!("x = {} and y = {}", i, j);
}
```

可以用在区间或迭代器上。

### break 和 continue

和C一样，用作语句。

```rust
let mut x = 5;

loop {
    x += x - 3;

    println!("{}", x);

    if x % 5 == 0 { break; }
}
```

```rust
for x in 0..10 {
    if x % 2 == 0 { continue; }

    println!("{}", x);
}
```

### 循环标签

```rust
'outer: for x in 0..10 {
    'inner: for y in 0..10 {
        if x % 2 == 0 { continue 'outer; } // continues the loop over x
        if y % 2 == 0 { continue 'inner; } // continues the loop over y
        println!("x: {}, y: {}", x, y);
    }
}
```

这个实在很好用(｡♥‿♥｡)


## 所有权

Rust的特色功能。

之后说的这些功能都是在编译时完成的，没有任何运行时的消耗。

### 所有权

在Rust中，变量绑定使得变量拥有被绑定对象的``所有权``。这意味着，当绑定所在作用域消失后，被绑定的资源将被释放。

```rust
fn foo() {
    let v = vec![1, 2, 3];
}
```

当v被创建时，创建一个新的``Vec<T>``于堆中。当脱离v所在作用域时，堆中所占用的资源也一并销毁。

### 移动语义

Rust确保任何资源只有一个绑定。

```rust
let v = vec![1, 2, 3];
let v2 = v;
```

之后再执行：

```rust
println!("v[0] is {}", v[0]);
```

将会报错：

```
error: use of moved value: `v`
println!("v[0] is: {}", v[0]);
```

### Copy类型

``Copy``是一个特性（trait）。实现了该特性的数据不受上面的讨论的移动约束。当重新绑定时，创建了一个完整的拷贝。

所有的基本类型都实现了``Copy``，比如：

```rust
fn main() {
    let a = 5;

    let _y = double(a);
    println!("{}", a);
}

fn double(x: i32) -> i32 {
    x * 2
}
```
可以正常编译。

有一种傻逼的办法，可以还回所有权：

```rust
let v = vec![1, 2, 3];

fn foo(v: Vec<i32>) -> Vec<i32> {
    // do stuff with v
    
    v
}

let v = foo(v);
// ...
```

很傻逼。

## 引用和借用

### 引用

直接看代码：

```rust
fn foo(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    // do stuff with v1 and v2

    // return the answer
    42
}

let v1 = vec![1, 2, 3];
let v2 = vec![1, 2, 3];

let answer = foo(&v1, &v2);

// we can use v1 and v2 here!
```

我们把``&T``类型称为“引用”。

注意！引用是**不可变**的。

```rust
fn foo(v: &Vec<i32>) {
     v.push(5);
}

let v = vec![];

foo(&v);
```

报错：

```
error: cannot borrow immutable borrowed content `*v` as mutable
v.push(5);
^
```

### 可变引用

``&mut T``类型的引用是可变的，如：

```rust
let mut x = 5;
{
    let y = &mut x;
    *y += 1;
}
println!("{}", x);
```

其中，``x``必须也标记为``mut``。
``*``用于访问被引用的对象，与C相似。

另外，若把上面的``{}``拿掉，会报错：

```
error: cannot borrow `x` as immutable because it is also borrowed as mutable
    println!("{}", x);
                   ^
note: previous borrow of `x` occurs here; the mutable borrow prevents
subsequent moves, borrows, or modification of `x` until the borrow ends
        let y = &mut x;
                     ^
note: previous borrow ends here
fn main() {

}
^
```

请看下面的规则：

### 规则

1. 借用必须维持在与所有者相同或更低（内层）的作用域中。
2. 借用只能是下面中的一种：
  * 一个或多个引用（``&T``）
  * 一个且仅有一个可变引用（``&mut T``）
  
比较这两段代码：

```rust
let mut x = 5;

let y = &mut x;    // -+ &mut borrow of x starts here
                   //  |
*y += 1;           //  |
                   //  |
println!("{}", x); // -+ - try to borrow x here
                   // -+ &mut borrow of x ends here
```

```rust
let mut x = 5;

{
    let y = &mut x; // -+ &mut borrow starts here
    *y += 1;        //  |
}                   // -+ ... and ends here

println!("{}", x);  // <- try to borrow x here
```
