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
2. 可变绑定赋值是表达式（y = 6），其值为()
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
