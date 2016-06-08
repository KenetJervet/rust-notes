# Rust笔记

这里是最精髓的内容。

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
let x: i32 = 5;  // 禁止自动类型推断
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
let mut m = [1, 2, 3]; // m: mut [i32; 3]

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

### 借用规则解决的问题

#### 无效迭代

```rust
let mut v = vec![1, 2, 3];  // A: 可变绑定

for i in &v {  // B: 不变引用。回忆一下为什么不能用for i in v，因为移动语义
    println!("{}", i);
    v.push(34);  // 报错，可变借用与B处冲突
}
```

#### 生命周期结束后的使用

```rust
let y: &i32;
{
    let x = 5;
    y = &x;
}

println!("{}", y);  // 报错，被引用的x比y的存活周期短
```

甚至于，下面的代码也会报错：

```rust
let y: &i32;
let x = 5;
y = &x;

println!("{}", y);
```

虽然作用域相同，但销毁的顺序与创建的顺序相反。上面的代码中``y``比``x``的生命长显然是不对的。

## 生命期

### 生命期

生命期用来解决释放资源后对资源的使用。

定义函数时可以指定引用参数的生命期。

```rust
// implicit
fn foo(x: &i32) {
}

// explicit
fn bar<'a>(x: &'a i32) {
}
```

``<>``中的是一个泛型参数。生命期是其中一种。当然可以有多个生命期参数：

```rust
fn bar<'a, 'b>(...)
```

然后参数类型中显式写明生命期类型：

```rust
...(x: &'a i32)
```

如果要加``mut``，则：

```rust
...(x: &'a mut i32)
```

### 结构体中的生命期

当结构体中包含引用时，就必须使用显式的生命周期。

```rust
struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let y = &5; // this is the same as `let _y = 5; let y = &_y;`
    let f = Foo { x: y };

    println!("{}", f.x);
}
```

保证了对``Foo``的引用不可能比其中``x``引用活得更长。

### impl块中的生命期

```rust
struct Foo<'a> {
    x: &'a i32,
}

impl<'a> Foo<'a> {
    fn x(&self) -> &'a i32 { self.x }
}

fn main() {
    let y = &5; // this is the same as `let _y = 5; let y = &_y;`
    let f = Foo { x: y };

    println!("x is: {}", f.x());
}
```

``impl<'a>``定义了生命期``'a``，而后Foo使用。

### 多个生命期

不消多说，语法和你想的一样。

### 与作用域连起来看

```rust
struct Foo<'a> {
    x: &'a i32,
}

fn main() {
    let x;                    // -+ x goes into scope
                              //  |
    {                         //  |
        let y = &5;           // ---+ y goes into scope
        let f = Foo { x: y }; // ---+ f goes into scope
        x = &f.x;             //  | | error here
    }                         // ---+ f and y go out of scope
                              //  |
    println!("{}", x);        //  |
}                             // -+ x goes out of scope
```

问题：为什么struct一定要有生命期标识？

### 'static

``'static``指示在整个程序运行期间存活的变量。

```rust
let x: &'static str = "Hello, world";
```

```rust
static FOO: i32 = 5;
let x: &'static i32 = &FOO;
```

### 生命期的省略

输入生命期指的是函数参数的生命期；输出生命期指的是函数返回值中的生命期。

1. 参数中每个省略的生命期都成为一个独立的生命期参数。
2. 如果只有一个输入生命期，不论是否省略，这个生命期会被指派到返回值中所有省略了的生命期。
3. 如果有多个输入生命期，而其中一个是``&self``或``&mut self``，则``self``的生命期会被指派到返回值中所有省略了的生命期。
4. 除上述情况外，省略生命期均报错。

```rust
fn print(s: &str); // elided
fn print<'a>(s: &'a str); // expanded

fn debug(lvl: u32, s: &str); // elided
fn debug<'a>(lvl: u32, s: &'a str); // expanded

// In the preceding example, `lvl` doesn’t need a lifetime because it’s not a
// reference (`&`). Only things relating to references (such as a `struct`
// which contains a reference) need lifetimes.

fn substr(s: &str, until: u32) -> &str; // elided
fn substr<'a>(s: &'a str, until: u32) -> &'a str; // expanded

fn get_str() -> &str; // ILLEGAL, no inputs

fn frob(s: &str, t: &str) -> &str; // ILLEGAL, two inputs
fn frob<'a, 'b>(s: &'a str, t: &'b str) -> &str; // Expanded: Output lifetime is ambiguous

fn get_mut(&mut self) -> &mut T; // elided
fn get_mut<'a>(&'a mut self) -> &'a mut T; // expanded

fn args<T:ToCStr>(&mut self, args: &[T]) -> &mut Command; // elided
fn args<'a, 'b, T:ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command; // expanded

fn new(buf: &mut [u8]) -> BufWriter; // elided
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>; // expanded
```

## 可变性

可变的变量绑定：

```rust
let mut x = 5;
x = 6;
```

并不改变x所在的值，而改变x所指的位置。
很像C中的``const *``

可变引用：

```rust
let mut x = 5;
let y = &mut x;
```

很像C中的``* const``

同时使用两种引用：

```rust
let mut x = 5;
let mut y = &mut x;
```

``mut``是模式的一部分：

```rust
let (mut x, y) = (5, 6);

fn foo(mut x: i32) {
    ...
}
```

### 内部与外部可变性

说一个东西的可变性，一般说的是它是``外部可变性``，比如：

```rust
use std::sync::Arc

let x = Arc::new(5);
let y = x.clone();  // 此处更新引用计数
```

上面对引用计数的更新是在内部进行的。

下面这种情况：

```rust
use std::cell::RefCell;

let x = RefCell::new(42);

let y = x.borrow_mut();
```

是内部可变性，调用``borrow_mut``将创建对内部数据的``&mut``引用。

``RefCell``将Rust的借用规则推迟到运行时，当违背借用规则时，将``panic!``掉：

```rust
use std::cell::RefCell;

let x = RefCell::new(42);

let y = x.borrow_mut();
let z = x.borrow_mut();
```

### 域级别的可变性

可变性只适用于绑定（``let mut``）和借用（``&mut``），因此，下面这种不成立：

```rust
struct Point {
    x: i32,
    mut y: i32  // 错误
}
```

结构体的可变性取决于对它的绑定：

```rust
struct Point {
    x: i32,
    y: i32,
}

let mut a = Point { x: 5, y: 6 };

a.x = 10;

let b = Point { x: 5, y: 6};

b.x = 10; // error: cannot assign to immutable field `b.x`
```

使用``Cell<T>``来获得域级别的可变性：

```rust
use std::cell::Cell;

struct Point {
    x: i32,
    y: Cell<i32>,
}

let point = Point { x: 5, y: Cell::new(6) };

point.y.set(7);

println!("y: {:?}", point.y);
```

## 结构体

```rust
struct Point {
    x: i32,
    y: i32
}

fn main() {
    let p = Point { x: 0, y: 0 };
    println!("The origin is at ({}, {})", origin.x, origin.y);
}
```

### 更新

```rust
struct Point3d {
    x: i32,
    y: i32,
    z: i32
}

let mut point = Point3d { x: 0, y: 0, z: 0 };
point = Point3d { y: 1, .. point };
```

同样适用于非``mut``绑定。（很像Haskell的record的说。。。）

### 元组结构体

一个元组和结构体的混合结构：

```rust
struct Color(i32, i32, i32)
```
说白了，结构体就是Haskell中的记录，而元组结构体就是Haskell中非记录语法的数据。

人家说了，有一种情况下元组结构体会比较有用，就是当只有一个元素的时候（``newtype``模式云云）。。。
尼玛还能跟Haskell再像一点么！！！！！

### 空结构体

```rust
struct Electron;

let x = Electron;
```

尼玛还能跟Haskell再像一点么！！！！！

## 枚举

```rust
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32},
    Write(String),
}
```

尼玛还能跟Haskell的data再像一点么！！！！！
用的时候倒是有点区别：

```rust
let x: Message = Message::Move { x: 3, y: 4 };
```

但是比较傻逼的是，不能使用模式对枚举进行解构：

```rust
let Message::ChangeColor(r, g, b) = msg;  // 错误
```

通过实现相等关系，或使用``match``可以解决这一限制。

### 将枚举构造器用作函数

这有什么好说的。。。

## 匹配

使用``match``关键字：

```rust
let x = 5;

match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    4 => println!("four"),
    5 => println!("five"),
    _ => println!("something else"),
}
```

与Haskell不同，条件必须是穷尽的，否则会编译失败。

``match``是表达式，似乎显而易见。

### 对枚举进行匹配

```rust
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn quit() { /* ... */ }
fn change_color(r: i32, g: i32, b: i32) { /* ... */ }
fn move_cursor(x: i32, y: i32) { /* ... */ }

fn process_message(msg: Message) {
    match msg {
        Message::Quit => quit(),
        Message::ChangeColor(r, g, b) => change_color(r, g, b),
        Message::Move { x: x, y: y } => move_cursor(x, y),
        Message::Write(s) => println!("{}", s),
    };
}
```

## 模式

### 多重模式

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

### 解构

结构体的几种解构方法：

```rust
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };

match origin {
    Point { x, y } => println!("({},{})", x, y),
}
```

```rust
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };

match origin {
    Point { x: x1, y: y1 } => println!("({},{})", x1, y1),
}
```

```rust
struct Point {
    x: i32,
    y: i32,
}

let origin = Point { x: 0, y: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```

### 空白绑定

比如匹配一个``Result<T, E>``：

```rust
match some_value {
    Ok(value) => println!("got a value: {}", value),
    Err(_) => println!("an error occurred"),
}
```

使用``..``来忽略多个值，如：

```rust
enum OptionalTuple {
    Value(i32, i32, i32),
    Missing,
}

let x = OptionalTuple::Value(5, -2, 3);

match x {
    OptionalTuple::Value(..) => println!("Got a tuple!"),
    OptionalTuple::Missing => println!("No such luck."),
}
```

### 绑定中的引用和可变引用

```rust
let x = 5;

match x {
    ref r => println!("Got a reference to {}", r),
}
```

```rust
let mut x = 5;

match x {
    ref mut mr => println!("Got a mutable reference to {}", mr),
}
```

为什么不是``&r``和``&mut r``呢？
因为作用是不同的，``&r``用来匹配一个引用，而``ref``是用来从数据处获得一个引用。

### 区间

```rust
let x = 1;

match x {
    1 ... 5 => println!("one through five"),
    _ => println!("anything"),
}
```

### 创建对模式的绑定

```rust
let x = 1;

match x {
    e @ 1 ... 5 => println!("got a range element {}", e),
    _ => println!("anything"),
}
```

Haskell即视感愈发明显。。。

要在多重匹配上创建绑定，则必须在每一个匹配上分别创建：

```rust
let x = 5;

match x {
    e @ 1 ... 5 | e @ 8 ... 10 => println!("got a range element {}", e),
    _ => println!("anything"),
}
```

是的你没看错，``e@(1 ... 5 | 8 ... 10)``是不行的，嗯就是这么傻逼。。。

### 守卫

```rust
let x = 4;
let y = false;

match x {
    4 | 5 if y => println!("yes"),
    _ => println!("no"),
}
```

好吧这次和Haskell有点不像。

上面的``if``作用于``4 | 5``，而且也不能写作``4 | (5 if y) => ...``。嗯就是这么傻逼。。。
