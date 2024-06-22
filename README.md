# rust-study

## Foreword & Introduction

## Chapter 1: Getting Started

## Chapter 2: Practices: Guessing Game

## Chapter 3: Common Programming Concepts

## Chapter 4: Ownership

## Chapter 5: Structs

结构体：将相关联的数据片段联系起来并命名它们，这样可以使得代码更加清晰  
impl块：定义与你的类型相关联的函数  
方法：一种关联函数，指定结构体的实例所具有的行为

### section 1: 定义和实例化Structs


### section 2: 使用Structs的一个例子Rectangle


### section 3: Method Syntax

#### 方法method和函数functions
相似点：都使用fn 关键字和名称声明，都拥有参数和返回值。  
不同点：方法在结构体或者是枚举或者trait对象的上下文中被定义，第一个参数总是self，代表调用该方法的结构体实例。

#### 定义方法
定义Rectangle结构体上的area方法：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

//Everything within this impl block will be associated with the Rectangle type.
//在函数signature以及对应parameter中修改成self
//&self 来替代 rectangle: &Rectangle，&self 实际上是 self: &selg的缩写
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

impl块：implementation  
在一个 `impl` 块中 `Self` 类型是 `impl` 块的类型的别名。方法的第一个参数必须有一个名为 `self` 的 `Self` 类型的参数。因此我们在第一个参数位置上只用 `self` 这个名字来简化。

**注意，我们仍然需要在 `self` 前面使用 `&` 来表示这个方法借用了 `Self` 实例**，使得方法可以选择获得 `self` 的所有权（只希望能够读取结构体中的数据，而非写入）。  

不可变地借用 `self`：通常用在当方法将 `self` 转换成别的实例的时候，想要防止调用者在转换之后使用原始的实例的情况。  

可变地借用 `self`：需要将第一个参数修改为 `&mut self`），就跟其他参数一样。(可以回到Chapter 4浏览所有权相关内容)

**方法的名字和结构的字段struct‘s fields可以重名**
```rust
impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    //width和Rectangle里的width字段重名，但后面有括号()所以是方法
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
}
```
**特例：`getters`方法：**
1. 方法的名字和结构的字段重名
2. 被定义为只返回字段中的值，而不做其他事情
3. 把字段变成私有的，但方法是公共的，这样就可以**把对字段的只读访问作为该类型公共 API 的一部分**。（第七章中会探讨此问题）

#### method代替function的优点：

1. 不需要在每个函数签名中重复 self 的类型之外
2. 具有良好的组织性（将某个类型实例能做的所有事情都一起放入 impl 块中，而不是让将来的用户在我们的库中到处寻找其相关的function）

#### 对象调用方法 & 指针上调用方法

对象调用方法： 

`C/C++`中：`对象.方法`

指针调用方法： 

`C/C++`中：`指针->方法`，需要dereference(解引用指针)，即 `object` 是一个指针，那么 `object->something()` 就像 `(*object).something()` 一样  

`Rust`中：automatic referencing and dereferencing(自动引用和解引用)使得在 `object.something()` 调用方法时，Rust 会自动为 `object` 添加 `&`、`&mut` 或 `*` 以便使 object 与方法签名匹配。

```rust
//因为方法有一个明确的接收者 ———— self 的类型
//在给出接收者和方法名的前提下，Rust可以判断是方法仅仅读取（&self）做出修改（&mut self）或者是获取所有权（self）
//对方法接收者的隐式借用让所有权在实践中更友好
p1.distance(&p2);
(&p1).distance(&p2);
````

#### 多参数方法

练习**can_hold方法**： 让一个 Rectangle 的实例获取另一个 Rectangle 实例，如果 self （第一个 Rectangle）能完全包含第二个长方形则返回 true；否则返回 false

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

#### Associated Functions关联函数

1. 在 `impl`块中被定义的函数
2. 关联：与 `impl`后面命名类型相关
3. 和方法不一样，因为第一参数不是 `self`，不作用于一个结构体的实例
4. 调用：使用结构体名和 `::` 语法来调用这个关联函数  
   `::` 语法用于关联函数和模块创建的命名空间namespaces created by modules(Chapter 7)
5. `String::from` 是一个关联函数，defined on the String type

```rust
//关键字 Self 在函数的返回类型中代指在 impl 关键字后出现的类型，在这里是 Rectangle
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
//调用：：let sq = Rectangle::square(3);
```

#### 多impl模块 Multiple impl Blocks

```
//可以将其放在一个impl块中，仅说明当前语法有效，第十章在泛型和trait时会看到更多有用的impl块
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```
---

## Chapter 6: Enums and Pattern Matching

枚举eunmerations，即enums：列举可能的成员（variants）来定义一个类型 

### section 1: 定义 Enum

>#### Structs & Enums
>
>-结构体：字段和数据整合为一组。例如：Rectangle结构体中有width和height两个字段
>
>-枚举：声明某个值是一个集合中的一员。例如：形状集合中有Rectangle，有Circle，Triangle...

#### 定义枚举
>处理IP地址(IPv4（version four）和 IPv6（version six）)：枚举出所有可能的值，任何一个 IP 地址要么是 IPv4 的要么是 IPv6 的，而且不能两者都是。**枚举值只可能是其中一个成员**。

```rust
//IpAddKind是一个自定义数据类型
//v4,v6是variants（枚举的成员）
enum IpAddrKind {
    V4,
    V6,
}
```
#### 实例枚举值

```rust
    //创建IpAddKind两个不同成员的实例
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
```

```rust
//定义一个函数来获取任何IpAddKind
fn route(ip_kind: IpAddrKind) {}
//使用任意成员来调用这个函数
fn main() {
  route(IpAddKind::V4);
  route(IpAddKind::V6);
}
```

```rust
    enum IpAddrKind {
        V4,
        V6,
    }

    //IP 地址的数据和 IpAddrKind 成员存储在一个 struct 中
    //使用Struct将kind和address打包，使得枚举成员和值相关联
    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
```

更简洁的方式，直接将数据附加到枚举的每个成员上：

```rust
    //IpAddr 枚举的新定义表明了 V4和V6 成员都关联了 String 值
    enum IpAddr {
        V4(String),
        V6(String),
    }

    //the name of each enum variant that we define also becomes a function that constructs an instance of the enum.
    //IpAddr::V4() takes a String argument and returns an instance of the IpAddr type
    //automatically get this constructor function defined as a result of defining the enum
    let home = IpAddr::V4(String::from("127.0.0.1"));

    let loopback = IpAddr::V6(String::from("::1"))；
```
枚举的另一优势：**每个成员可以处理不同类型和数量的数据**：
```rust
    //IPv4 版本的 IP 地址总是含有四个值在 0 和 255 之间的数字部分，即四个u8
    //IPv6 地址仍然表现为一个 String
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

```

#### the standard library has a definition(to store IP addresses and encode)

```rust
//embeds the address data inside the variants in the form of two different structs
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

//枚举成员中可以放入不同数据类型：字符串、数字类型、结构体、枚举
enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

#### Message枚举练习
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

//等同于以下几个struct，但用eunm能将其组合在Message类型下
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
```
**在枚举上使用impl定义方法**
```rust
    impl Message {
        fn call(&self) {
            // 方法...
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
```

#### Option：an enum defined by the standard library

`Option`：相对于空值的优势（要么有值，要么无值）
>如果请求一个非空列表的第一项，会得到一个值，如果请求一个空的列表，就什么也不会得到。
>  
>从类型系统的角度来表达这个概念就意味着编译器需要检查是否处理了所有应该处理的情况，这样就可以避免在其他编程语言中非常常见的 bug。

空值（Null ）是一个值，它代表没有值，一个因为某种原因目前无效或缺失的值。在有空值的语言中，变量总是这两种状态之一：空值和非空值。Rust没有空值功能，但拥有一个可以**编码存在或不存在概念的枚举**。

```rust
enum Option<T> {
    None,
    Some(T),
}
```

1. `Option<T>`是常规的枚举，Some(T) 和 None 是 Option<T> 的成员。
1. `Option<T>` 枚举包含在了 prelude 之中，不需要将其显式引入作用域。
2. `Option<T>`成员也是如此，可以不需要 Option:: 前缀来直接使用 Some 和 None。
3. `<T>` 意味着 Option 枚举的 Some 成员可以包含任意类型的数据，同时每一个用于 `T` 位置的具体类型使得 `Option<T>` 整体作为不同的类型。

```rust
    let some_number = Some(5);//some_number 的类型是 Option<i32>
    let some_char = Some('e');//some_char 的类型是 Option<char>

    let absent_number: Option<i32> = None;//需要指定 Option 整体的类型
```

**在对 Option<T> 进行运算之前必须将其转换为 T**：(捕获到空值最常见的问题之一:假设某值不为空但实际上为空的情况)

```rust
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y;
```
```rust
$ cargo run
   Compiling enums v0.1.0 (file:///projects/enums)
error[E0277]: cannot add `Option<i8>` to `i8`
 --> src/main.rs:5:17
  |
5 |     let sum = x + y;
  |                 ^ no implementation for `i8 + Option<i8>`
  |
  = help: the trait `Add<Option<i8>>` is not implemented for `i8`
  = help: the following other types implement trait `Add<Rhs>`:
            <i8 as Add>
            <i8 as Add<&i8>>
            <&'a i8 as Add<i8>>
            <&i8 as Add<&i8>>

For more information about this error, try `rustc --explain E0277`.
error: could not compile `enums` (bin "enums") due to 1 previous error
```

`Option<T>` 枚举拥有大量用于各种情况的方法，将其从 Some 成员中取出 T 的值来使用它：你可以查看它的文档。

### section 2: match 控制流结构

**`match`**：allows you to compare a value against **a series of patterns** and then execute code based on which pattern matches.

**patterns**:literal values, variable names, 通配符wildcards, and many other things (第十八章：不同种类的模式以及他们的作用)

#### 模拟验钞机

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    //if表达式必须返回一个布尔值，而match可以是返回任何类型的
    match coin {
        //每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 match 表达式的返回值
        Coin::Penny => {
            println!("Lucky penny!");
            1  
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### Patterns that bind to values 绑定值的模式：如何从枚举成员成员中提取值

>1999 年到 2008 年间，美国在 25 美分的硬币的一侧为 50 个州的每一个都印刷了不同的设计。其他的硬币都没有这种区分州的设计，所以只有这些 25 美分硬币有特殊的价值。可以将这些信息加入我们的 enum，通过改变 Quarter 成员来包含一个 State 值

```rust
#[derive(Debug)] // 这样可以立刻看到州的名称
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {state:?}!");
            25
        }
    }
}
```

#### Matching with `Option<T>`

> **在 `Option<T>` 中从 `Some` 中取出其内部的 `T` 值**
> 
> 编写一个函数，它获取一个 Option<i32> ，如果其中含有一个值，将其加一。如果其中没有值，函数应该返回 None 值，而不尝试执行任何操作。
>

```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```

>**注意：匹配是穷尽exhaustive的，包含所有可能性的，必须穷举到最后的可能性来使代码有效。** 
>
>```rust
>    fn plus_one(x: Option<i32>) -> Option<i32> {
>        match x {
>            Some(i) => Some(i + 1),
>        }
>    }
>```
>
>我们没有处理 None 的情况，所以这些代码会造成一个 bug。
>
>```rust
>$ cargo run
>   Compiling enums v0.1.0 (file:///projects/enums)
>error[E0004]: non-exhaustive patterns: `None` not covered
> --> src/main.rs:3:15
>  |
>3 |         match x {
>  |               ^ pattern `None` not covered
>  |
>note: `Option<i32>` defined here
> --> >/rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:572:1
> ::: /rustc/9b00956e56009bab2aa15d7bff10916599e3d6d6/library/core/src/option.rs:576:5
>  |
>  = note: not covered
>  = note: the matched value is of type `Option<i32>`
>help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
>  |
>4 ~             Some(i) => Some(i + 1),
>5 ~             None => todo!(),
>  |
>
>For more information about this error, try `rustc --explain E0004`.
>error: could not compile `enums` (bin "enums") due to 1 previous error
>```

#### 通配模式和 _ 占位符 Catch-all Patterns and the _ Placeholder

>正在玩一个游戏，如果你掷出骰子的值为 3，角色不会移动，而是会得到一顶新奇的帽子。如果你掷出了 7，你的角色将失去新奇的帽子。对于其他的数值，你的角色会在棋盘上移动相应的格子。

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
```

>当你掷出的值不是 3 或 7 的时候，你必须再次掷出。这种情况下我们不需要使用这个值，所以我们改动代码使用 `_` 来替代变量 `other`
>
>**`_` ：可以匹配任意值而不绑定到该值。**这告诉 Rust 我们不会使用这个值，所以 Rust 也不会警告我们存在未使用的变量。

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn reroll() {}
```
>如果你掷出 3 或 7 以外的值，你的回合将无事发生。
>我们可以使用单元值（在“元组类型”一节中提到的空元组）作为 _ 分支的代码

```rust
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
```

### section 3: if let简洁控制流

`if let`：处理只匹配一个模式的值而忽略其他模式的情况。

```rust
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }
```

更简洁的编写方式：

```rust
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }
```
else的使用

```rust
    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state:?}!"),
        _ => count += 1,
    }

    //简化表达if let和else
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    } else {
        count += 1;
    }

```

---

## Chapter 7: Managing Growing Projects with Packages, Crates, and Modules

**将代码分解为多个模块和多个文件来组织代码**：通过对相关功能进行分组和划分不同功能的代码，你可以清楚在哪里可以找到实现了特定功能的代码，以及在哪里可以改变一个功能的工作方式。

- Rust 提供了将包分成多个 crate，将 crate 分成模块，以及通过指定绝对或相对路径从一个模块引用另一个模块中定义的项的方式。
- 通过使用 use 语句将路径引入作用域，这样在多次使用时可以使用更短的路径。
- 模块定义的代码默认是私有的，不过可以选择增加 pub 关键字使其定义变为公有。
- 对于一个由一系列相互关联的包组成的超大型项目，**Cargo提供了workspaces“工作空间”**(Chapter 14).

### section 1: Packages and Crates

<img width="477" alt="image" src="https://github.com/estherea1/rust-study/assets/153347674/eeecf268-860b-42a7-a4a5-81320d69763d">

**Package：**
- multiple binary crates 多个二进制crate项
- an optionally one library crate 一个可选的crate库
- `Cargo.toml` 文件定义了包的信息和其包含的crate。

**crate：Rust 在编译时最小的代码单位。** 
>如果你用 rustc 而不是 cargo 来编译一个文件（第一章我们这么做过），编译器还是会将那个文件认作一个 crate。crate 可以包含模块，模块可以定义在其他文件，然后和 crate 一起编译。

**crate两种形式**：
1. **二进制项 a binary crate **：可以被编译为可执行程序，比如一个命令行程序或者一个 web server。它们**必须有一个 `main` 函数**来定义当程序被执行的时候所需要做的事情。目前我们所创建的 crate 都是二进制项。
2. **库 a library crate **：**没有 `main` 函数**，它们也不会编译为可执行程序，它们提供一些诸如函数之类的东西，使其他项目也能使用这些东西。大多数时间说的crate 指的都是库，这与其他编程语言中 library 概念一致。

**crate root**：一个源文件，Rust编译器以它为起始点，并构成你的 crate 的根模块。 通常，对于一个库 crate 而言是src/lib.rs，对于一个二进制 crate 而言是src/main.rs。

**包是一个整体，包含了多个crate。通过Cargo.toml文件管理。** Cargo 就是一个包含构建你代码的二进制项的包。Cargo 也包含这些二进制项所依赖的库。其他项目也能用 Cargo 库来实现与 Cargo 命令行程序一样的逻辑。

`Cargo.toml`：包的配置文件，包含包的元数据和依赖项。
>```rust
>[package]
>name = "my_package"
>version = "0.1.0"
>edition = "2018"
>
>[dependencies]
>serde = "1.0"
>```

`src/main.rs`：一个与包同名的二进制 crate 的 crate 根，包含可执行程序的入口点。

`src/lib.rs`：与包同名的库crate，包含库的代码，且 src/lib.rs 是 crate 根。crate 根文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目。

`src/bin/other_bin.rs`: 其他二进制crate，可以包含额外的可执行程序.每个 src/bin 下的文件都会被编译成一个独立的二进制 crate。

### section 2: Defining Modules to Control Scope and Privacy

**模块系统 the module syste**：Rust 中管理代码的组织，包括哪些内容可以被公开，哪些内容作为私有部分，以及程序每个作用域中的名字。

- **包（Packages）**：Cargo 的一个功能，它允许你构建、测试和分享 crate。
- **Crates**：一个模块的树形结构，它形成了库或二进制项目。
- **模块（Modules）和 use**：允许你控制作用域和路径的私有性。
- **路径（path）**：一个命名例如结构体、函数或模块等项的方式。

#### Modules Cheat Sheet

```shell
backyard
├── Cargo.lock
├── Cargo.toml
└── src
    ├── garden
    │   └── vegetables.rs
    ├── garden.rs
    └── main.rs //crate根文件
```

**声明模块Declaring modules：使用`mod`关键字声明一个模块。模块可以包含函数、结构体、枚举、常量、类型别名、其他模块等。**

在 crate 根文件中，可以声明一个新模块 src/main.rs：
```rust
use crate::garden::vegetables::Asparagus;
pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");
}
```
`mod garden;` 声明了一个叫做garden的模块，编译器会在下列路径中寻找模块代码：  
1. 内联，在大括号中，当`mod garden`后方不是一个分号而是一个大括号
2. 在文件 src/garden.rs
3. 在文件 src/garden/mod.rs

**声明子模块Declaring submodules**：你可能在src/garden.rs中定义了mod vegetables。在除了 crate 根节点以外的其他文件中，你可以定义子模 src/garden.rs：
```rust
//pub mod vegetables;意味着在src/garden/vegetables.rs中的代码也应该被包括
pub mod vegetables;
```
编译器会在以父模块命名的目录中寻找子模块代码:
1. 内联，在大括号中，当mod vegetables后方不是一个分号而是一个大括号
2. 在文件 src/garden/vegetables.rs
3. 在文件 src/garden/vegetables/mod.rs

src/garden/vegetables.rs：
```rust
#[derive(Debug)]
pub struct Asparagus {}
```

**模块中的代码路径 Paths to code in modules**：  
绝对路径: 从crate根开始的路径；相对路径: 从当前模块开始的路径。

**私有 vs 公用 Private vs public**:   
一个模块里的代码默认对其父模块私有，模块和模块中的所有项（函数、结构体等）都是私有的。为了使一个模块公用，应当在声明时使用pub mod替代mod。为了使一个公用模块内部的成员公用，应当在声明前使用pub。

```
// 声明一个模块名为 my_module
mod my_module {
    pub fn my_function() {
        println!("Hello from my_module!");
    }

    fn private_function() {
        println!("This is a private function.");
    }

    pub fn public_function() {
        println!("This is a public function.");

    // 声明一个子模块名为 my_submodule
    pub mod my_submodule {
        pub fn my_sub_function() {
            println!("Hello from my_submodule!");
        }
    }
}

// 使用绝对路径
fn main() {
    crate::my_module::my_function();
    crate::my_module::my_submodule::my_sub_function();

    // 不能调用私有函数
    // my_module::private_function();

    // 可以调用公有函数
    my_module::public_function();
    my_module::my_submodule::public_sub_function();
}

// 使用相对路径
mod another_module {
    use super::my_module;

    pub fn call_functions() {
        my_module::my_function();
        my_module::my_submodule::my_sub_function();
    }
}
```

**use 关键字**: `use` 关键字用于将模块路径引入作用域，便于访问模块中的项。它可以简化长路径的使用。

```rust
// 将模块路径引入作用域
use crate::my_module::my_function;
use crate::my_module::my_submodule::my_sub_function;

fn main() {
    // 现在可以直接调用函数
    my_function();
    my_sub_function();
}
```

#### Grouping Related Code in Modules：前后台的模拟

>在餐饮业，餐馆中会有一些地方被称之为 前台（front of house），还有另外一些地方被称之为 后台（back of house）。前台是招待顾客的地方，在这里，店主可以为顾客安排座位，服务员接受顾客下单和付款，调酒师会制作饮品。后台则是由厨师工作的厨房，洗碗工的工作地点，以及经理做行政工作的地方组成。
>要以这种方式构造我们的 crate，我们可以将其函数组织成嵌套模块。

src/lib.rs：
```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

使用模块组织代码：`src/main.rs` 和 `src/lib.rs` 叫做 `crate root`。之所以这样叫它们是因为这两个文件的内容都分别在 crate 模块结构的根组成了一个名为 crate 的模块，该结构被称为 模块树（module tree）。

```shell
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

### section 3: Paths for Referring to an Item in the Module Tree

#### 模块中的代码路径

1. 绝对路径: the full path starting from a crate root;
   1. for code from an external crate 外部crate：the absolute path begins with the **crate name**
   2. for code from the current crate 当前crate： it starts with the **literal crate**
3. 相对路径: starts from the current module and **uses `self`, `super`, or an identifier **in the current module.
4. **注意：父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用它们父模块中的项**。学会使用pub关键字

```rust
mod front_of_house {
    //模块上的 pub 关键字只允许其父模块引用它，而不允许访问内部代码
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}
```

#### super 开始的相对路径

`super`：从父模块开始构建相对路径，而不是从当前模块或者 crate 根开始。

>模拟了厨师更正了一个错误订单，并亲自将其提供给客户的情况。
>
>back_of_house 模块中的定义的 fix_incorrect_order 函数通过指定的 super 起始的 deliver_order 路径，来调用父模块中的 deliver_order 函数。

```rust
fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
```

#### 创建公有的结构体和枚举 Making Structs and Enums Public

>这个例子模拟的情况是，在一家餐馆中，顾客可以选择随餐附赠的面包类型，但是厨师会根据季节和库存情况来决定随餐搭配的水果。餐馆可用的水果变化是很快的，所以顾客不能选择水果，甚至无法看到他们将会得到什么水果。

```rust
mod back_of_house {
    //公有结构，私有字段
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // 在夏天订购一个黑麦土司作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意更换想要面包的类型
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 如果取消下一行的注释代码不能编译；
    // 不允许查看或修改早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");
}
```

**结构体默认内容全部是私有的，除非使用 pub 关键字，枚举成员默认共有**：

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

### section 4: Bringing Paths Into Scope with the use Keyword

使用`use`将模块引入scope（通过 use 引入作用域的路径也会检查私有性）：

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

同于`use`语句的作用域

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        // the shortcut no longer applies within the customer module:
        //hosting::add_to_waitlist();
        //使用super关键字
        super::hosting::add_to_waitlist();
    }
}
```

#### 创建惯用的use路径

使用 `use` 将函数的父模块引入作用域：必须在调用函数时指定父模块，这样可以清晰地表明函数不是在本地定义的，同时使完整路径的重复度最小化。

>使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。
>
>将 HashMap 结构体引入二进制 crate 作用域的习惯用法。
>
>```rust
>use std::collections::HashMap;
>fn main() {
>    let mut map = HashMap::new();
>    map.insert(1, 2);
>}
>```
>
>例外：使用 use 语句将两个具有相同名称的项带入作用域。 
>如何将两个具有相同名称但不同父模块的 Result 类型引入作用域，以及如何引用它们。
>
>```rust
>use std::fmt;
>use std::io;
>
>fn function1() -> fmt::Result {
>    // --snip--
>}
>
>fn function2() -> io::Result<()> {
>    // --snip--
>}
>```

#### 使用as关键字提供新的名称

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

#### 使用pub use 重导re-exporting出名称

**重导出（re-exporting）**：将一个名称导入了当前作用域，还允许别人把它导入他们自己的作用域。

在这个修改之前，外部代码需要使用路径`restaurant::front_of_house::hosting::add_to_waitlist()` 来调用 `add_to_waitlist` 函数。现在这个 `pub use` 从根模块重导出了 `hosting` 模块，外部代码现在可以使用路径 `restaurant::hosting::add_to_waitlist`。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

//使用pub use使名称可从新作用域中被导入至任何代码
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

#### Using External Packages 使用外部包

crates.io 上有很多 Rust 社区成员发布的包，将其引入你自己的项目都需要一道相同的步骤：**在 Cargo.toml 列出它们并通过 use 将其中定义的项引入项目包的作用域中**。

注意 std 标准库对于你的包来说也是外部 crate。因为标准库随 Rust 语言一同分发，无需修改 Cargo.toml 来引入 std，不过**需要通过 use 将标准库中定义的项引入项目包的作用域中来引用它们**，比如我们使用的 HashMap(`use std::collections::HashMap;
`)。

#### Using Nested Paths to Clean Up Large use Lists 嵌套路径来消除大量的`use`行 

```rust
// --snip--
use std::cmp::Ordering;
use std::io;
// --snip--

//指定嵌套的路径在一行中将多个带有相同前缀的项引入作用域
// --snip--
use std::{cmp::Ordering, io};
// --snip--
```

```rust
use std::io;
use std::io::Write;

//用self关键字表示第一个路径，将部分重复的路径合并为一个 use 语句
use std::io::{self, Write};
```

#### The Glob Operator 通过glob运算符将所有的公有定义引入作用域 

如果希望将一个路径下所有公有项引入作用域，可以指定路径后跟 *，glob 运算符。  
**使用 glob 运算符时请多加小心！Glob 会使得我们难以推导作用域中有什么名称和它们是在何处定义的。**

```rust
use std::collections::*;
```

> glob 运算符经常用于测试模块 tests 中，这时会将所有内容引入作用域（第十一章）。

### section 5: Separating Modules into Different Files

```shell
crate src/lib.rs
 └── front_of_house
 │   ├── hosting
 │   │   ├── add_to_waitlist
 │   │   └── seat_at_table
 │   └── serving
 │       ├── take_order
 │       ├── serve_order
 │       └── take_payment
 └── back_of_house
```

src/lib.rs
```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}
```

src/front_of_house.rs
```rust
pub mod hosting ;
```

src/front_of_house/hosting.rs
```rust
pub fn add_to_waitlist() {}
```

---

## Chapter 8: Common Collections

collections:不同于数组和元组类型，集合指向的数据是存储在堆上的（这意味着数据的数量不必在编译时就已知，并且还可以随着程序的运行增长或缩小）。

常用的三个集合：
1. vector：一个挨着一个地储存一系列数量可变的值
2. string：字符的集合
3. hash map：允许我们将值与一个特定的键（key）相关联

### section 1: Storing Lists of Values with Vectors

`Vec<T>`：
1. 允许我们在一个单独的数据结构中储存多于一个的值
2. 在内存中相邻排列
3. 只能存储相同类型的值
4. Vec<T> 是一个由标准库提供的类型，它可以存放任何类型

> 文件中的文本行或者是购物车种商品的价格

#### 新建vector

```rust
    //新建一个空的 vector 来储存 i32 类型的值
    let v: Vec<i32> = Vec::new();

    //通常会给初始值，rust会推断存储值得类型，所以用vec! marco来创建新的vector
    let v = vec![1, 2, 3];
```

#### 更新vector

使用push方法增加元素：

```rust
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
```

#### 读取vector的元素

1. 通过索引
2. 使用get方法：通过索引作为参数调用get方法，获得一个Option<T>，需要经过match输出。当引用访问超过vector元素时，不会导致panic，更友好。

```rust
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }
```

#### vector中的所有权和借用

**不能在相同作用域中同时存在可变和不可变引用的规则**

```rust
    //v可变
    let mut v = vec![1, 2, 3, 4, 5];

    //获取不可变引用
    let first = &v[0];

    //v后增加元素
    v.push(6);

    //引用不可变first，将会报错
    println!("The first element is: {first}");
```

为什么第一个元素的引用会关心 vector 结尾的变化？不能这么做的原因是由于 vector 的工作方式：在 vector 的结尾增加新元素时，在没有足够空间将所有元素依次相邻存放的情况下，可能会要求分配新内存并将老的元素拷贝到新的空间中。这时，第一个元素的引用就指向了被释放的内存。借用规则阻止程序陷入这种状况。

#### 遍历vector中的元素

使用`for`遍历vector中的元素

```rust
    //不可变
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    //可变
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        //为了修改可变引用所指向的值，在使用 += 运算符之前必须使用解引用运算符（*）获取 i 中的值（第十五章）
        *i += 50;
    }
```

**注意：因为借用检查器的规则，无论可变还是不可变地遍历一个 vector 都是安全的。如果在for 循环体内插入或删除项，都会得到编译错误。for 循环中获取的 vector 引用阻止了同时对 vector 整体的修改**

#### 使用枚举存储多种类型

vector只能储存相同类型值，当需要在 vector 中储存不同类型值时，我们可以定义并使用一个枚举。

> 假如我们想要从电子表格的一行中获取值，而这一行的有些列包含数字，有些包含浮点值，还有些是字符串.
> 
>我们可以定义一个枚举，其成员会存放这些不同类型的值，同时所有这些枚举成员都会被当作相同类型：那个枚举的类型。接着可以创建一个储存枚举值的 vector，这样最终就能够储存不同类型的值了。

```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

编译时需要准确的知道这个 vector 中允许什么类型：因为需要明确储存每个元素到底需要多少内存

`pop`和`push`类型，`pop`会移除并返回vector的最后一个元素。

#### Dropping a Vector Drops Its Elements 丢弃vector时也会丢弃其他的元素

```rust
    {
        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here
```

当 vector 被丢弃时，所有其内容也会被丢弃，这意味着这里它包含的整数将被清理。借用检查器确保了任何 vector 中内容的引用仅在 vector 本身有效时才可用。

### section 2: Storing UTF-8 Encoded Text with Strings

字符串：字节的集合+方法

字符串字面值 string literals:
```rust
let s1 = "Hello, world!";
```
1. 只读数据段中的不可变字符串
2. 用双引号创建
3. 类型：`&str`
4. 存储位置：编译时确定并存储在只读数据段中
   
字符串 String:
```rust
let mut s2 = String::from("Hello, world!");
```
1. 可变的，可以追加、删除或者修改内容
2. 类型：`String`
3. 存储位置：在堆上分配内存，运行时动态分配和管理
4. 所有权：`String`类型拥有其内容的所有权，可以移动和转移
5. 由 Rust 标准库提供，而不是编入核心语言，它是一种可增长、可变、可拥有、UTF-8 编码的字符串类型

字符串切片 String slice `&str`类型：
```rust
let s3: &str = &s2[0..5];
```
1. 不可变，对原始字符串的一部分引用
2. 类型：`&str`
3. 存储位置：可以引用堆上的String或只读数据段中的字符串字面值(它们是一些对储存在别处的 UTF-8 编码字符串数据的引用)
4. 引用类型：字符串切片是一个引用类型，不能拥有其内容的所有权
5. 通常以被借用的形式出现，&str
6. 由于字符串字面值被储存在程序的二进制输出中，因此字符串字面值也是字符串 slices

```rust
fn main() {
    // 字符串字面值
    let s1 = "Hello, world!";
    println!("字符串字面值: {}", s1);

    // String 类型
    let mut s2 = String::from("Hello, world!");
    s2.push_str(" Welcome to Rust!");
    println!("String 类型: {}", s2);

    // 字符串切片
    let s3: &str = &s2[0..5];
    println!("字符串切片: {}", s3);

    // 字符串切片可以引用字符串字面值
    let s4: &str = "Hello, world!";
    println!("字符串切片（引用字面值）: {}", s4);
}
```

#### 新建字符串

`String`被实现为一个带有一些额外保证、限制和功能的字节 vector 的封装。

```rust
    //新建一个空的String
    let mut s = String::new();
```

```rust
    let data = "initial contents";

    //使用to_string方法初始化数据
    let s = data.to_string();

    // 该方法也可直接用于字符串字面值：
    let s = "initial contents".to_string();
```

```rust
    //使用String::from 函数创建String
    let s = String::from("initial contents");
```

#### 更新字符串

String 的大小可以增加，其内容也可以改变，和`Vec`一样。。

1. 使用push_str：采用字符串 slice，因为我们并不需要获取参数的所有权
   
   ```rust
        let mut s = String::from("foo");
        s.push_str("bar");
   ```
   
    ```rust
    //执行这两行代码之后，s 将会包含 foobar。push_str 方法采用字符串 slice，因为我们并不需要获取参数的所有权。例如，在将 s2 的内容附加到 s1 之后还能使用它
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");
    ```

2. push附加字符串
    
    ```rust
    let mut s = String::from("lo");
    s.push('l');
    ```
3. 使用 `+` 或 `format! marco` 来拼接 String 值
   
    ```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // 注意 s1 被移动了，不能继续使用
    ```

    `+`运算符使用了`add`函数，所以使用`+`时第二个参数要引用

    ```rust
        fn add(self, s: &str) -> String {
    ```

    `add` 函数的 `s` 参数：只能将 `&str` 和 `String` 相加，不能将两个 `String` 值相加。`&s2`的类型是 `&String`, 而不是 `add` 第二个参数所指定的 `&str`。但是 `add` 调用中使用 `&s2` 是因为 `&String` 可以被 强转（coerced）成 `&str`。

    **Deref 强制转换（deref coercion）**： `&s2` 变成了 `&s2[..]`

    `let s3 = s1 + &s2;` 看起来就像它会复制两个字符串并创建一个新的字符串，而实际上这个语句会获取 `s1` 的所有权，附加上从 `s2` 中拷贝的内容，并返回结果的所有权。换句话说，它看起来好像生成了很多拷贝，不过实际上并没有：这个实现比拷贝要更高效。

    ```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    ```

    `format!` 与 `println!` 的工作原理相同，不过不同于将输出打印到屏幕上，它返回一个**带有结果内容的 String**。这个版本就好理解的多，**format! marco生成的代码使用引用所以不会获取任何参数的所有权**。

    ```rust
    //使用format!
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    ```
#### 索引字符串

>```rust
>    //报错
>    let s1 = String::from("hello");
>    let h = s1[0];
>```

**内部表现**：String 是一个 Vec<u8> 的封装。（详细请参考book）

**Bytes and Scalar Values and Grapheme Clusters 字节、标量值和字形簇**（详细请参考book）

#### 字符串slice

> 字符串索引应该返回的类型是不明确的：字节值、字符、字形簇或者字符串 slice。
>
> 为了更明确索引并表明你需要一个字符串 slice，相比使用 [] 和单个值的索引，可以使用 **[] 和一个 range** 来创建含特定字节的字符串 slice。

```rust
let hello = "Здравствуйте";
//注意因为上面的字母是两个字节长，所以[0..1]会panic
let s = &hello[0..4];
```

#### 遍历字符串的方法

**操作字符串每一部分的最好的方法是明确表示需要字符还是字节。** 

**chars方法**：

```rust
for c in "Зд".chars() {
    println!("{c}");
}
```

**bytes方法**：

```rust
for b in "Зд".bytes() {
    println!("{b}");
}
```

#### 其余方法

标准库提供了很多围绕 String 和 &str 构建的功能，来帮助我们正确处理这些复杂场景。

`contains` 来搜索一个字符串

`replace` 将字符串的一部分替换为另一个字符串

### section 3: Storing Keys with Associated Values in Hash Maps

hash map: `HashMap<K, V>`储存了一个键类型 K 对应一个值类型 V 的映射。

#### 新建一个hashmap

```rust
    use std::collections::HashMap;

    //HashMap::new()创建
    let mut scores = HashMap::new();

    //insert()插入键值对
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
```

1. 数据存储在堆上
2. 所有的键对，值对都必须是相同类型

#### 访问hashmap中的值

通过get方法获取对应键的值：

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    let score = scores.get(&team_name).copied().unwrap_or(0);
```

**`get`方法返回`Option<&V`>**，如果某个键在哈希 map 中没有对应的值，`get` 会返回 None。

程序中通过调用`copied`方法来**获取一个`Option<i32>`而不是 `Option<&i32>`**，接着调用`unwrap_or`在`scores`中没有该键所对应的项时将其设置为零。

#### 遍历hashmap键值对

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
```

#### 所有权问题

>对于像 i32 这样的实现了 Copy trait 的类型，其值可以拷贝进哈希 map。
>
>对于像 String 这样拥有所有权的值，其值将被移动而哈希 map 会成为这些值的所有者

```rust
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // 这里 field_name 和 field_value 不再有效!
```

#### 更新hashmap

**键值对数量增长，但唯一的键只能关联唯一的值。**

> 当我们想要改变哈希 map 中的数据时，必须决定如何处理一个键已经有值了的情况。
> 
> 可以选择完全无视旧值并用新值代替旧值。
> 可以选择保留旧值而忽略新值，并只在键没有对应值时增加新值。
> 或者可以结合新旧两值。

1. 覆盖旧值
```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    //直接覆盖
    scores.insert(String::from("Blue"), 25);

    //:?
    println!("{scores:?}");
```

2. 键没有对应值时增加新值 

`entry`：它获取我们想要检查的键作为参数，返回值是一个枚举，Entry，它代表了可能存在也可能不存在的值。

```rust
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{scores:?}");
```

3. 更新旧值

> 计数一些文本中每一个单词分别出现了多少次。
> 
> 我们使用哈希 map 以单词作为键并递增其值来记录我们遇到过几次这个单词。
> 
> 如果是第一次看到某个单词，就插入值 0

```rust
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        //or_insert 方法返回这个键的值的一个可变引用（&mut V）
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
```
注意：
1. **遍历哈希 map 会以任意顺序进行**
2. HashMap 默认使用一种叫做 SipHash 的哈希函数，它可以抵御涉及哈希表（hash table）的拒绝服务（Denial of Service, DoS）攻击。
3. SipHash 并不是可用的最快的算法，但有更高的安全性。
4. 如果性能监测显示此哈希函数非常慢，可以指定一个不同的 hasher 来切换为其它函数。
5. hasher 是一个实现了 BuildHasher trait 的类型。(第十章)
6. crates.io 有其他人分享的实现了许多常用哈希算法的 hasher 的库。

---

## Chapter 9: Error Handling

Error
1. 可恢复的recoverable：文件未找到的错误（报告问题并重试操作）
2. 不可恢复的unrecoverable：访问一个超过数组末端的位置

Result<T,E>：处理可恢复的错误  
panic! marco：遇到不可恢复错误时停止执行

### section 1: Unrecoverable Errors with panic!

**造成panic两种情况**：
1. 执行会造成代码 panic 的操作（比如访问超过数组结尾的内容）
2. 显式调用 panic! 宏

**panic作用**：
1. 打印错误信息
2. 展开并清理栈数据（通过环境变量可以打印调用堆栈call stack以便于定位panic的原因）
   > **栈展开或者终止Unwinding the Stack or Aborting in Response to a Panic**
   > 
   > 当出现 panic 时，程序默认会开始 展开（unwinding），这意味着 Rust 会回溯栈并清理它遇到的每一个函数的数据，不过这个回溯并清理的过程有很多工作。
   >
   >另一种选择是直接 终止（abort），这会不清理数据就退出程序。
   >
   > 那么程序所使用的内存需要由操作系统来清理。
   >
   >如果你需要项目的最终二进制文件越小越好，panic 时通过在 Cargo.toml 的 [profile] 部分增加 panic = 'abort'，可以由展开切换为终止。例如，如果你想要在 release 模式中 panic 时直接终止：
   >
   >```rust
   >[profile.release]
   >panic = 'abort'
   >```
3. 退出

```rust
fn main() {
    panic!("crash and burn");
}
```

最后两行包含 panic! 调用造成的错误信息。第一行显示了 panic 提供的信息并指明了源码中 **panic 出现的位置：src/main.rs:2:5** 表明这是 **src/main.rs 文件的第二行第五个字符**。

在这个例子中，被指明的那一行是我们代码的一部分，而且查看这一行的话就会发现 panic! 宏的调用。在其他情况下，panic! 可能会出现在我们的代码所调用的代码中。**错误信息报告的文件名和行号可能指向别人代码中的 panic! 宏调用，而不是我们代码中最终导致 panic! 的那一行**。

```shell
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at src/main.rs:2:5:
crash and burn
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

#### panic! backtrace

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

C语言：读取数据结构之后的值是**未定义行为（undefined behavior）**。你会得到任何对应数据结构中这个元素的内存位置的值，甚至是这些内存并不属于这个数据结构的情况。这被称为**缓冲区溢出（buffer overread）**，并可能会导致安全漏洞，比如攻击者可以像这样操作索引来读取储存在数据结构之后不被允许的数据。

Rust：报错。note行提醒我们可以**设置 RUST_BACKTRACE 环境变量**来得到一个 backtrace。**backtrace 是一个执行到目前位置所有被调用的函数的列表**。
```rust
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`
thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

阅读 backtrace 的关键是从**头开始读直到发现你编写的文件**。

这就是问题的发源地。**这一行往上是你的代码所调用的代码；往下则是调用你的代码的代码。**这些行可能包含核心 Rust 代码，标准库代码或用到的 crate 代码。

```rust
//将 RUST_BACKTRACE 环境变量设置为任何不是 0 的值来获取 backtrace
$ RUST_BACKTRACE=1 cargo run
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', src/main.rs:4:5
stack backtrace:
   0: rust_begin_unwind
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/std/src/panicking.rs:584:5
   1: core::panicking::panic_fmt
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:142:14
   2: core::panicking::panic_bounds_check
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/panicking.rs:84:5
   3: <usize as core::slice::index::SliceIndex<[T]>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:242:10
   4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/slice/index.rs:18:9
   5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/alloc/src/vec/mod.rs:2591:9
   6: panic::main
             at ./src/main.rs:4:5
   7: core::ops::function::FnOnce::call_once
             at /rustc/e092d0b6b43f2de967af0887873151bb1c0b18d3/library/core/src/ops/function.rs:248:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
```

为了获取带有这些信息的 backtrace，必须启用 debug 标识。当不使用 --release 参数运行 cargo build 或 cargo run 时 debug 标识会默认启用，就像这里一样。

### section 2: Recoverable Errors with Result

在 Rust 中，许多错误并没有严重到需要程序停止执行，而是可以进行恢复处理。

**Result 枚举类型提供了一种方式来处理这些可恢复的错误**。Result 类型有两个成员：**Ok 和 Err，分别代表操作成功和失败的结果**。

```rust
enum Result<T, E> {
    Ok(T),//T 代表成功时返回的数据类。
    Err(E),//E 代表失败时返回的错误类型
}
```

Result 类型允许我们在函数可能失败的地方使用，**返回成功或失败的信息**。例如，文件打开操作可能会成功返回文件句柄，或失败返回错误信息。

```rust
use std::fs::File;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

#### 匹配不同的错误类型

不同的错误需要不同的处理方式。通过 **ErrorKind 枚举**，我们可以区分错误类型，并根据错误类型采取不同的措施：

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```

#### 使用`unwrap`和`expect`简化错误处理

`unwrap`：在 Result 是 Ok 时返回内部值，否则调用 panic!。

`expect`：类似于 unwrap，但允许传递自定义的 panic! 信息。

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

```rust
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

#### 传播错误：当一个函数调用可能会失败的操作时，可以选择将错误返回给调用者进行处理。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

#### 使用`?`传播错误

? 运算符是一个**简洁的方式来处理可能会返回错误的操作**。它的作用是在一个可能失败的操作后面，用来**简化 match 表达式**。如果操作成功，? 会返回成功的值；如果操作失败，它会将错误返回给调用者。**? 运算符只能在返回类型是 Result 或 Option 的函数中使用**。

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

`File::open("hello.txt")?` 和 `read_to_string(&mut username)?` 会在失败时立即返回错误，而不是用 match 进行显式的错误处理。这使得代码更加简洁和易读。

`?` 运算符背后的工作原理如下：

如果调用成功，返回 Ok(T) 的值。
如果调用失败，返回 Err(E)，并将错误传递给调用者。

#### 在不同函数中使用`?`

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

// Result<(), Box<dyn std::error::Error>>，表示它可能会返回一个错误
// Box<dyn Error> 理解为 “任何类型的错误”
//在返回 Box<dyn Error> 错误类型 main 函数中对 Result 使用 ? 是允许的，因为它允许任何 Err 值提前返回
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let username = read_username_from_file()?;
    println!("Username: {}", username);
    Ok(())
}
```

### section 3: To panic! or Not to panic!

使用 panic! 的场景：

1. 原型或示例代码prototype code：快速验证概念
   
    当你编写一个示例来展示一些概念时，在拥有健壮的错误处理代码的同时也会使得例子不那么明确。例如，调用一个类似 unwrap 这样可能 panic! 的方法可以被理解为一个你实际希望程序处理错误方式的占位符，它根据其余代码运行方式可能会各不相同。
    
2. 测试代码：明确指示测试失败
3. 遇到不可恢复的错误时：如非法状态或严重逻辑错误

使用 Result 的场景：

1. 处理可恢复的错误：如文件未找到、网络超时
2. 给调用者选择处理错误的方法：让调用者决定如何处理错误

利用 Rust 的类型系统来确保值的有效性，提供错误处理的指导方针，以增强代码的健壮性。

1. 利用类型系统确保值有效性，减少运行时错误
2. 提供明确的错误信息和处理方法，提升代码健壮性

---

## Chapter 10: Generic Types, Traits, and Lifetimes

- 泛性generics：允许定义可用于多种具体类型的代码。例如，Option<T>, Vec<T> 等。

    ```rust
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
    }
    ```

- Trait：定义共同行为，可与泛型结合，限制泛型只接受特定行为的类型。

- 生命周期：确保引用的有效性，允许借用值时编译器仍能检查引用的有效性。

### section 1: Generic Data Types

泛型： create definitions for items like function signatures or structs

类型命名规范：UpperCamelCase驼峰式命名法

```rust
fn largest<T>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");
}
```

```shell
$ cargo run
   Compiling chapter10 v0.1.0 (file:///projects/chapter10)
error[E0369]: binary operation `>` cannot be applied to type `&T`
 --> src/main.rs:5:17
  |
5 |         if item > largest {
  |            ---- ^ ------- &T
  |            |
  |            &T
  |
help: consider restricting type parameter `T`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
  |             ++++++++++++++++++++++

For more information about this error, try `rustc --explain E0369`.
error: could not compile `chapter10` (bin "chapter10") due to 1 previous error
```

std::cmp::PartialOrd，这是个 trait

largest 的函数体不能适用于 T 的所有可能的类型

因为在函数体需要比较 T 类型的值，不过它只能用于我们知道如何排序的类型。为了开启比较功能，标准库中定义的 `std::cmp::PartialOrd` trait 可以实现类型的比较功能（查看附录 C 获取该 trait 的更多信息）。依照帮助说明中的建议，我们**限制 T 只对实现了 PartialOrd 的类型有效后**代码就可以编译了，因为标准库为 i32 和 char 实现了 PartialOrd。

#### 结构体定义的泛型

<>定义结构体，包含一个或者多个泛型参数类型字段。

```rust
struct Point<T> {
    //x 和 y 都是 相同类型的，无论它具体是何类型
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    //报错，因为x和y要有相同类型
    let wont_work = Point { x: 5, y: 4.0 };
}
```

#### 使用多个泛型类型参数

我们修改 Point 的定义为拥有两个泛型类型 `T` 和 `U`。其中字段 x 是 T 类型的，而字段 y 是 U 类型的

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

#### 枚举定义中的泛型

和结构体类似，枚举也可以在成员中存放泛型数据类型。例如标准库提供的 Option<T> 枚举：

```rust
enum Option<T> {
    Some(T),
    None,
}

//多泛型
//T:std::fs::File
//E:std::io::Error
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

#### 方法定义中的泛型

```rust
struct Point<T> {
    x: T,
    y: T,
}

//在 impl 后面声明 T，这样就可以在 Point<T> 上实现的方法中使用 泛型T ，而不是具体类型
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

**为泛型指定限制constraint**：其他 T 不是 f32 类型的 Point<T> 实例则没有定义此方法

```rust
//可以选择为 Point<f32> 实例实现方法，而不是为泛型 Point 实例
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

**多泛型的应用**：

```rust
struct Point<X1, Y1> {
    x: X1,
    y: Y1,
}

impl<X1, Y1> Point<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

#### 泛型代码的性能：

泛型类型参数是否会有运行时消耗？**泛型并不会使程序比具体类型运行得慢**。

**单态化monomorphization：**一个通过填充编译时使用的具体类型，将通用代码转换为特定代码的过程。能够保证效率。

> 创建泛型函数的步骤：编译器寻找所有泛型代码被调用的位置并使用泛型代码针对具体类型生成代码。
>
> ```rust
> let integer = Some(5);
> let float = Some(5.0);
> ```
>
> 单态化：编译器会读取传递给 Option<T> 的值并发现有两种 Option<T>：一个对应 i32 另一个对应 f64。为此，它会将泛型定义 Option<T> 展开为两个针对 i32 和 f64 的定义，接着将泛型定义替换为这两个具体的定义。

### section 2: Traits: Defining Shared Behavior

**trait——类似于接口interfaces**
1. 定义了某个特定类型拥有可能与其他类型共享的功能。
2. 可以通过 trait 以一种抽象的方式定义共同行为。
3. 可以使用 trait bounds 指定泛型是任何拥有特定行为的类型
4. 可以对不同类型调用相同的方法：共享相同的行为

#### 定义trait

一个类型的行为由其可供调用的方法构成。如果可以**对不同类型调用相同的方法**的话，这些类型就可以共享相同的行为了。trait 定义是一种**将方法签名组合起来的方法**，目的是**定义一个实现某些目的所必需的行为的集合**。

> 例如，这里有多个存放了不同类型和属性文本的结构体：结构体 NewsArticle 用于存放发生于世界各地的新闻故事，而结构体 Tweet 最多只能存放 280 个字符的内容，以及像是否转推或是否是对推友的回复这样的元数据。
>
> 我们想要创建一个名为 aggregator 的多媒体聚合库用来显示可能储存在 NewsArticle 或 Tweet 实例中的数据摘要。为了实现功能，每个结构体都要能够获取摘要，这样的话就可以调用实例的 summarize 方法来请求摘要。

```rust
pub trait Summary {
    fn summarize(&self) -> String;
    //trait 体中可以有多个方法：一行一个方法签名且都以分号结尾，均无实现。
}
```

#### 为类型实现trait

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

**相干性coherence/孤儿规则orphan rule：**这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现

#### 默认实现

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

#### trait作为参数

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

#### trait bound语法

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

```

#### 通过+指定多个trait bound

```rust
pub fn notify<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

#### 通过where简化trait bound

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // 函数体
}
```

#### 返回实现了trait的类型

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}
```

#### 使用trait bound有条件的实现方法

```rust
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

### section 3: Validating References with Lifetimes

#### 生命周期：已经使用过的泛型

**1. 生命周期确保引用如预期一直有效**  
生命周期用于确保引用在其作用域内始终有效，从而避免悬垂引用。

**2. 使用泛型生命周期参数来注明它们的关系**  
通过生命周期标注，确保不同引用之间的关系在整个生命周期内保持有效。

#### 避免了dangling references悬垂引用

悬垂引用：导致程序引用了非预期引用的数据。

```rust
fn main() {
    //一个外部作用域和一个内部作用域
    //声明了没有初始值的变量，该变量存在于外部作用域
    let r;

    //如果尝试在给它一个值之前使用这个变量，会出现一个编译时错误，这就说明了 Rust 确实不允许空值
    {
        let x = 5;
        r = &x;
    }

    //r 引用的值在尝试使用之前就离开了作用域
    println!("r: {r}");
}
```

#### 借用检查器borrow checker

Rust 的借用检查器在编译时**检查引用是否有效**，确保在引用存在时，**被引用的数据不会被释放**。

一个有效的引用:数据比引用有着更长的生命周期

#### 函数中的泛型生命周期

```rust
//该函数签名表明 x 和 y 必须拥有相同的生命周期 'a，返回值也同样拥有 'a 生命周期
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

无法编译：**增加泛型生命周期**参数来定义**引用间的关**系以便借用检查器可以进行分析。

#### 生命周期注解语法

生命周期注解：
1. 不改变任何引用的生命周期的长
2. 描述了多个引用生命周期相互的关系，而不影响其生命周期
3. 指定了泛型生命周期后函数也能接受任何生命周期的引用
4. 使用单引号加小写字母表示，如：`'a`。
5. 生命周期参数注解位于引用的 & 之后，并有一个空格来将引用类型与生命周期注解分隔开

```rust
//本身没有多少意义，多个引用的泛型生命周期参数如何相互联系的
&i32        // 引用
&'a i32     // 带有显式生命周期的引用
&'a mut i32 // 带有显式生命周期的可变引用
```

#### 函数签名中的生命周期注解

为了在函数签名中使用生命周期注解，需要在函数名和参数列表间的尖括号中声明泛型生命周期（lifetime）参数，就像泛型类型（type）参数一样。

```rust
//这两个参数和返回的引用存活的一样久
//两个参数和返回的引用的生命周期是相关的
//longest 函数返回的引用的生命周期与函数参数所引用的值的生命周期的较小者一致
//泛型生命周期 'a 的具体生命周期等同于 x 和 y 的生命周期中较小的那一个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

#### 深入理解生命周期：指定生命周期参数的正确方式依赖函数实现的具体功能。

生命周期参数取决于函数的具体实现方式，确保引用在预期范围内有效。

```rust
//如果将 longest 函数的实现修改为总是返回第一个参数而不是最长的字符串 slice，就不需要为参数 y 指定一个生命周期
fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}
```

#### 结构体定义中的生命周期注解

我们定义的结构体全都包含拥有所有权的类型。也可以定义包含引用的结构体，不过这需要为结构体定义中的每一个引用添加生命周期注解。

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

结构体中的引用字段需要生命周期注解，以确保在整个结构体生命周期内引用有效。

#### 生命周期省略 Lifetime Elision

某些情况下，编译器可以自动推断生命周期，避免手动标注。

input lifetimes输入生命周期:函数或方法的参数的生命周期

output lifetimes输出生命周期：返回值的生命周期

lifetime elision rule:
1. 编译器为每一个引用参数都分配一个生命周期参数。有两个引用参数的函数就有两个不同的生命周期参数，依此类推。
2. 如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数：fn foo<'a>(x: &'a i32) -> &'a i32。
3. 如果方法有多个输入生命周期参数并且其中一个参数是 &self 或 &mut self，说明是个对象的方法 (method)，那么所有输出生命周期参数被赋予 self 的生命周期。

#### 方法定义中的生命周期注解

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

#### 静态生命周期

'static 生命周期指的是引用在整个程序运行期间都有效。

```rust
//所有的字符串字面值都拥有 'static 生命周期
let s: &'static str = "I have a static lifetime.";

```

#### 结合泛型类型参数、trait bounds 和生命周期

```rust
//该函数接受三个参数，其中两个有相同生命周期 'a，并且 ann 必须实现 Display trait。
use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---

## Chapter 11 Automated Tests

假设我们可以编写一个叫做 add_two 的将传递给它的值加二的函数。它的签名有一个整型参数并返回一个整型值。当实现和编译这个函数时，Rust 会进行所有目前我们已经见过的类型检查和借用检查，例如，这些检查会确保我们不会传递 String 或无效的引用给这个函数。Rust 所 不能 检查的是这个函数是否会准确的完成我们期望的工作：返回参数加二后的值，而不是比如说参数加 10 或减 50 的值！这也就是测试出场的地方。

我们可以编写测试断言，比如说，当传递 3 给 add_two 函数时，返回值是 5。无论何时对代码进行修改，都可以运行测试来确保任何现存的正确行为没有被改变。

测试是一项复杂的技能：虽然不能在一个章节的篇幅中介绍如何编写好的测试的每个细节，但我们还是会讨论 Rust 测试功能的机制。我们会讲到编写测试时会用到的注解和宏，运行测试的默认行为和选项，以及如何将测试组织成单元测试和集成测试。

### section 1: How to write tests

测试函数体通常执行如下三种操作：

1. 设置任何所需的数据或状态
2. 运行需要测试的代码
3. 断言assert其结果是我们所期望的

编写测试的功能：test属性、宏、should_panic属性

#### 测试函数剖析

在 Rust 中，**测试是通过带有 #[test] 属性注解的函数来实现的**。这些函数专门用于测试代码中的功能和行为。当你运行 cargo test 命令时，Rust 的测试框架会**自动识别并运行这些标记为测试的函数**。每个测试函数应包含断言（如 assert!、assert_eq!）来验证代码是否按预期工作。

属性（attribute）是关于 Rust 代码片段的元数据。derive 属性就是一个例子

函数--->测试函数
#[test] fn

运行**cargo test**：Rust 会构建一个测试执行程序用来调用被标注的函数，并报告每一个测试是通过还是失败。Cargo 新建一个库项目时，它会自动为我们生成一个测试模块和一个测试函数。这个模块提供了一个编写测试的模板，为此每次开始新项目时不必去查找测试函数的具体结构和语法了。

ok：所有都通过
1 passed：通过数量
0 failed;：失败数量
0 ignored;：忽略
0 measured：针对性能测试的benchmark tests
0 filtered out：无过滤
Doc-tests adder：所有文档测试结果

当测试函数中出现 panic 时测试就失败了。每一个测试都在一个新线程中运行，当主线程发现测试线程异常了，就将对应测试标记为失败。

#### 使用assert! marco来检查结果

**assert! 宏用于验证条件是否为真：**

```rust
Copy code
#[test]
fn test_assert() {
    assert!(1 + 1 == 2);
}
```

#### 使用assert_eq! 和 assert_ne! marco来测试相等

```rust
//相等
#[test]
fn test_assert_eq() {
    assert_eq!(2 + 2, 4);
}

//不相等
#[test]
fn test_assert_ne() {
    assert_ne!(2 + 2, 5);
}
```

> 在一些语言和测试框架中，断言两个值相等的函数的参数被称为 **expected 和 actual**，而且指定参数的顺序非常重要。然而在 Rust 中，它们则叫做 **left 和 right**，同时指定期望的值和被测试代码产生的值的顺序并不重要。这个测试中的断言也可以写成 assert_eq!(add_two(2), 4)，这时失败信息仍同样是 assertion failed: `(left == right)`。
>
> **assert_ne! 宏**在传递给它的两个值不相等时通过，而在相等时失败。在代码按预期运行，**我们不确定值 会 是什么，不过能确定值绝对 不会 是什么的时候**，这个宏最有用处。例如，如果一个函数保证会以某种方式改变其输出，不过这种改变方式是由运行测试时是星期几来决定的，这时最好的断言可能就是函数的输出不等于其输入。

#### 自定义失败信息

```rust
pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        //
        assert!(
            result.contains("Carol"),
            //自定义部分
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
```

```test
#[test]
fn test_custom_message() {
    let result = 2 + 2;
    assert_eq!(result, 4, "计算结果错误: {} 不是4", result);
}
```

#### 使用should_panic 检查panic

```rust
#[test]
#[should_panic(expected = "less than or equal to 100")]
fn greater_than_100() {
    Guess::new(200);
}
```

should_panic 属性中 expected 参数提供的值是 Guess::new 函数 panic 信息的子串。我们可以指定期望的整个 panic 信息。

expected 信息的选择取决于 panic 信息有多独特或动态，和你希望测试有多准确。在这个例子中，错误信息的子字符串足以确保函数在 else if value > 100 的情况下运行。

#### 将Result<T, E>用于测试

1. 不能对这些使用 Result<T, E> 的测试使用 #[should_panic] 注解。
2. 为了断言一个操作返回 Err 成员，不要使用对 Result<T, E> 值使用问号表达式（?）。而是使用 **assert!(value.is_err())**。

```rust
#[test]
fn test_result() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("计算错误"))
    }
}
```

### section 2: Controllong how tests are run

cargo test
1. 在测试模式下编译代码并运行生成的测试二进制文件
2. 默认行为是并发运行所有的测试，并截获测试运行过程中产生的输出，阻止它们被显示出来，使得阅读测试结果相关的内容变得更容易

命令行参数：  
cargo test --help：提示 cargo test 的有关参数

cargo test -- --help：提示在分隔符之后使用的有关参数

#### 并行或连续的运行测试

当运行多个测试时，Rust 默认使用线程来并行运行。因为测试是在同时运行的，你应该确保测试不能相互依赖，或依赖任何共享的状态，包括依赖共享的环境，比如当前工作目录或者环境变量。

> 举个例子，每一个测试都运行一些代码，假设这些代码都在硬盘上创建一个 test-output.txt 文件并写入一些数据。接着每一个测试都读取文件中的数据并断言这个文件包含特定的值，而这个值在每个测试中都是不同的。因为所有测试都是同时运行的，一个测试可能会在另一个测试读写文件过程中修改了文件。那么第二个测试就会失败，并不是因为代码不正确，而是因为测试并行运行时相互干扰。**一个解决方案是使每一个测试读写不同的文件；另一个解决方案是一次运行一个测试**。

如果你不希望测试并行运行，或者想要更加精确的控制线程的数量，可以传递 --test-threads 参数和希望使用线程的数量给测试二进制文件：

```rust
//这里将测试线程设置为 1，告诉程序不要使用任何并行机制。
//这也会比并行运行花费更多时间，不过在有共享的状态时，测试就不会潜在的相互干扰了
$ cargo test -- --test-threads=1
```

#### 显示函数输出

- 测试通过：只会看见说明测试通过的提示行
- 测试失败：所有标准输出和其他错误信息

```rust
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {a}");
    10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);
    }
}
```

`cargo test -- --show-output` 显示成功测试的输出

#### 通过指定名字来运行部分测试

1. 运行单个测试
    不能像这样指定多个测试名称；只有传递给 cargo test 的第一个值才会被使用。不过有运行多个测试的方法。
2. 过滤运行多个测试
    我们可以指定部分测试的名称，任何名称匹配这个名称的测试会被运行。例如，因为头两个测试的名称包含 add，可以通过 cargo test add 来运行这两个测试：

#### 除非制定否则忽略某些测试

使用ignore属性来标记耗时的测试并排除它们，cargo test时会忽略ignore的fn。

cargo test -- --ignored：只运行被忽略的测试

cargo test -- --include-ignored：不管是否忽略都要运行全部测试

```rust
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // 需要运行一个小时的代码
}
```

### section 3: test organization

1. **单元测试 unit tests**：向于更小而更集中，在隔离的环境中一次测试一个模块，或者是测试私有接口。单元测试独立地验证库的不同部分，也能够测试私有函数实现细节。
2. **集成测试 integration tests**：对于库来说则完全是外部的。检查多个部分是否能结合起来正确地工作，并像其他外部代码那样测试库的公有 API。它们与其他外部代码一样，通过相同的方式使用你的代码，**只测试公有接口**而且每个测试都有可能会测试多个模块。

#### 单元测试

> 单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码，以便于快速而准确地验证某个单元的代码功能是否符合预期。
> 
> 单元测试与它们要测试的代码**共同存放在位于 src 目录下相同的文件中**。
> 
> 规范是在每个文件中**创建包含测试函数的 tests 模块，并使用 cfg(test) 标注模块**。

1. 测试模块和#[cfg(test)]
   
    测试模块的 #[cfg(test)] 注解告诉 Rust 只在执行 cargo test 时才编译和运行测试代码。
    
    cargo build：不编译运行#[cfg(test)]测试代码。可以节省编译时间，能减少编译产生的文件的大小。

    单元测试位于与源码相同的文件中，所以你需要使用 #[cfg(test)] 来指定它们不应该被包含进编译结果中与之对应的集成测试因为位于另一个文件夹，所以它们并不需要 #[cfg(test)] 注解。

    cfg(configuration)：表示接下来的项，只有在给定特定配置选项时，才会被包含。在这种情况下，配置选项是 test，即 Rust 所提供的用于编译和运行测试的配置选项。通过使用 cfg 属性，Cargo 只会在我们主动使用 cargo test 运行测试时才编译测试代码。这包括测试模块中可能存在的帮助函数，以及标注为 #[test] 的函数。

2. 测试私有函数

    Rust 的私有性规则确实允许你测试私有函数。

    ```rust
    pub fn add_two(a: i32) -> i32 {
        internal_adder(a, 2)
    }

    fn internal_adder(a: i32, b: i32) -> i32 {
        a + b
    }

    #[cfg(test)]
    mod tests {
        // “路径用于引用模块树中的项” 部分所说，子模块的项可以使用其上级模块的项。
        use super::*;

        #[test]
        fn internal() {
            assert_eq!(4, internal_adder(2, 2));
        }
    }
    ```

#### 集成测试

为了创建集成测试，你需要**先创建一个 tests 目录**。Cargo 知道如何去寻找这个目录中的集成测试文件。接着可以随意在这个目录中创建任意多的测试文件，**Cargo 会将每一个文件当作单独的 crate 来编译**。

1. tests目录

    并不需要将 tests/integration_test.rs 中的任何代码标注为 #[cfg(test)]

    三个部分：单元测试、集成测试、文档测试

2. 集成测试中的子模块

    随着集成测试的增加，你可能希望在 tests 目录创建更多文件以便更好地组织它们，例如**根据测试的功能来将测试分组**。

    > 当你有一些在多个集成测试文件都会用到的帮助函数，而你尝试按照第七章 “将模块移动到其他文件” 部分的步骤将它们提取到一个通用的模块中时， tests 目录中文件行为的不同就会凸显出来。
    > 
    > 例如，如果我们可以创建 一个tests/common.rs 文件并创建一个名叫 setup 的函数，我们希望这个函数能被多个测试文件的测试函数调用

    tests/common.rs
    ```rust
    pub fn setup() {
        // setup code specific to your library's tests would go here
    }
    ```

    我们并不想要common 出现在测试结果中显示 running 0 tests 。我们只是希望其能被其他多个集成测试文件中调用罢了。

    为了不让 common 出现在测试输出中，我们**将创建 tests/common/mod.rs ，而不是创建 tests/common.rs** 。现在项目目录结构看起来像这样：

    ```shell
    ├── Cargo.lock
    ├── Cargo.toml
    ├── src
    │   └── lib.rs
    └── tests
        ├── common
        │   └── mod.rs
        └── integration_test.rs
    ```

    不要将 common 看作一个集成测试文件，将 setup 函数代码移动到 tests/common/mod.rs 并删除 tests/common.rs 文件之后，测试输出中将不会出现这一部分。tests 目录中的子目录不会被作为单独的 crate 编译或作为一个测试结果部分出现在测试输出中。

    ```rust
    use adder;

    mod common;

    #[test]
    fn it_adds_two() {
        common::setup();
        assert_eq!(4, adder::add_two(2));
    }
    ```

3. 二进制crate的集成测试

    如果项目是二进制 crate 并且只包含 src/main.rs 而没有 src/lib.rs，这样就不可能在 tests 目录创建集成测试并**使用 extern crate 导入 src/main.rs 中定义的函数**。只有库 crate 才会向其他 crate 暴露了可供调用和使用的函数；二进制 crate 只意在单独运行。

    这就是许多 Rust 二进制项目使用一个简单的 src/main.rs 调用 src/lib.rs 中的逻辑的原因之一**。因为通过这种结构，集成测试 就可以 通过 extern crate 测试库 crate 中的主要功能了**，而如果这些重要的功能没有问题的话，src/main.rs 中的少量代码也就会正常工作且不需要测试。

---

## Chapter 12 I/O project: building a command line program

创建grep：Globally search a Regular Expression adn Print

1. 获取一个文件路径和一个字符串作为参数
2. 读取文件并找到其中包含字符串参数的行
3. 打印出这些行

> 在这个过程中，我们会展示如何让我们的命令行工具利用很多命令行工具中用到的终端功能。读取环境变量来使得用户可以配置工具的行为。打印到标准错误控制流（stderr）而不是标准输出（stdout），例如这样用户可以选择将成功输出重定向到文件中的同时仍然在屏幕上显示错误信息。

包含内容：
- 代码组织（使用 第七章 学习的模块）
- vector 和字符串（第八章，集合）
- 错误处理（第九章）
- 合理的使用 trait 和生命周期（第十章）
- 测试（第十一章）
- 闭包（第十三章）
- 迭代器&trait对象（第十七章）

---

## Chapter 13 Functional Language Features: Iterators and Closures

函数式编程(functional programming)：将函数作为参数值或其他函数的返回值、将函数赋值给变量以供之后执行等等。

- 闭包（Closures），一个可以储存在变量里的类似函数的结构
- 迭代器（Iterators），一种处理元素序列的方式

### section 1: Closures: Anonymous Functions that Capture Their Environment

### section 2: Processing a Series of Items with Iterators

### section 3: Improving Our I/O Project

### section 4: Comparing Performance: Loops vs. Iterators

---

## Useful Links

1. [如何书写Markdown][markdown]
2. [Rust程序设计语言 英文版][rust_EN]
3. [Rust程序设计语言 简体中文版][rust_CN]
4. [How to write a git commit message][git_commit]
5. [git][git]

[markdown]: https://www.markdownguide.org/basic-syntax/ "相关Markdown格式链接"
[rust_EN]: https://doc.rust-lang.org/book/title-page.html "Rust英文book"
[rust_CN]: https://kaisery.github.io/trpl-zh-cn/title-page.html "Rust中文book"
[git_commit]:https://cbea.ms/git-commit/ "怎么写git commit"
[git]:https://github.com/ZenithalHourlyRate/learn-git-the-not-so-super-hard-way/blob/master/git.pdf "git操作"