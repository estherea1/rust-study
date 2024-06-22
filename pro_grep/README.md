# 创建grep：Globally search a Regular Expression adn Print

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

## 接受命令行参数

> **Crates.io 上有一些现成的库可以帮助我们接受命令行参数**，这里我们自己实现。

### Reading the Argument Values 读取参数值

`std::env::args`：返回一个迭代器（iterator）———— 传递给程序的命令行参数的 

- 迭代器生成一系列的值
- 可以在迭代器上调用 collect 方法将其转换为一个集合：比如包含所有迭代器产生元素的vector

`std::env::args_os`：`std::env::args`在任何参数包含无效Unicode字符时会panic。`std::env::args_os`返回OsString值而不是String值。出于简单考虑使用了 std::env::args，因为 **OsString 值每个平台都不一样而且比 String 值处理起来更为复杂**。

"target/debug/minigrep":二进制文件的名称。

这与 C 中的参数列表的行为一致：让程序使用在执行时调用它们的名称。如果要在消息中**打印它**或者根据用于**调用程序的命令行别名更改程序**的行为，通常可以**方便地访问程序名称**，不过考虑到本章的目的，我们将忽略它并只保存所需的两个参数。

### 将参数值保存进变量

用数组`&args[1]``&args[2]`读取参数

## 读取文件

增加读取由 file_path 命令行参数指定的文件的功能。

## 重构改进模块性和错误处理

### 四个问题

1. main：多个任务，最好能分离出功能以便每个函数就负责一个任务。
2. query和file_path：程序中的配置变量；contents：执行程序逻辑。`main`函数的增长会引入更多的变量到作用域，很难追踪每个变量。**将配置变量组织近一个结构**。
3. 打开文件失败用expect打印出具体的错误信息。读取文件失败原因：文件不存在，没有打开文件权限。
4. 将所有的错误处理都放在一处也有助于确保我们打印的错误信息对终端用户来说是有意义的。如果没有指定足够的参数来运行程序，index out of bounds错误。

### 问题一：Separation of Concerns for Binary Projects

Rust 社区开发出的**关注分离的指导**：处理二进制程序`main`函数**负责多个任务**的组织问题。

1. 将程序拆分成 main.rs 和 lib.rs 并将**程序的逻辑**放入 lib.rs 中。
2. 当命令行解析逻辑比较小时，可以保留在 main.rs 中。
3. 当命令行解析开始变得复杂时，也同样将其从 main.rs 提取到 lib.rs 中。

保留在`main`函数中的责任应该被限制为：
- 使用**参数值**调用命令行**解析逻辑**
- 设置任何**其他的配置**
- 调用 **lib.rs 中的 run 函数**
- 如果 run 返回错误，则**处理这个错误**

**main.rs 处理程序运行，而 lib.rs 处理所有的真正的任务逻辑**。**因为不能直接测试 main 函数，这个结构通过将所有的程序逻辑移动到 lib.rs 的函数中使得我们可以测试它们**。仅仅保留在 main.rs 中的代码将足够小以便阅读就可以验证其正确性。让我们遵循这些步骤来重构程序。

#### 提取参数解析器 Extracting the Argument Parser

将解析参数的功能提取到一个`main`将会调用的函数中，为将命令行解析逻辑一大弄到src/lib.rs。采用小的、增量的步骤进行重构。在做出这些改变之后，再次运行程序并验证参数解析是否仍然正常。经常验证你的进展是一个好习惯，这样在遇到问题时能帮助你定位问题的成因。

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, file_path) = parse_config(&args);

    // --snip--
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}
```

### 问题二：组合配置值 Grouping Configuration Values

> `parse_config(args: &[String])`返回的元组`(query, file_path)`,将元组拆成了独立的部分。这是一个我们可能没有进行正确抽象的信号。
>
> `parse_config` 名称的 config 部分，它暗示了我们返回的**两个值是相关的并都是一个配置值的一部分**。
>
> 将这两个值组合进元组之外并没有表达这个数据结构的意义：我们可以**将这两个值放入一个结构体**并给每个字段一个有意义的名字。这会让未来的维护者更容易理解不同的值如何相互关联以及它们的目的。
>
> **基本类型偏执（primitive obsession）**：在复杂类型更为合适的场景下使用基本类型的反模式。

```rust
struct Config {
    query: String,
    file_path: String,
}
```

```rust
//返回了引用 args 中 String 值的字符串 slice
fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let file_path = &args[2];

    (query, file_path)
}

//返回Config包含拥有所有权的 String 值
fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let file_path = args[2].clone();

    Config { query, file_path }
}
```

> clone方法 
> 
> 比储存字符串数据的引用消耗更多的时间和内存。不过拷贝数据使得代码显得更加直白因为**无需管理引用的生命周期**，所以在这种情况下牺牲一小部分性能来换取简洁性的取舍是值得的。
> 
> 由于其运行时消耗，倾向于避免使用 clone 来解决所有权问题。**在第一轮编写时拥有一个可以工作但有点低效的程序要比尝试过度优化代码更好一些**。

#### 创建一个Config的构造函数

增加`Config`结构体来描述`query`和`file_path`的相关性，并能够从`parse_config`函数中将这些值的名称作为结构体字段名称返回。

`parse_config`目的：创建一个 Config 实例，将`parse_config`从一个普通函数变为一个叫做`new`的与结构体关联的函数。

> 做出这个改变使得代码更符合习惯：可以像标准库中的`String`调用 `String::new`来创建一个该类型的实例那样，将`parse_config`变为一个与`Config`关联的`new`函数。

### 问题三：修复错误处理

如果`args`vector包含少于3个项并尝试访问vector中索引1或索引2的值会造成程序panic。

`index out of bounds: the len is 1 but the index is 1`是一个针对程序员的错误信息，然而这并不能真正帮助终端用户理解发生了什么和他们应该做什么。

调用`panic!`或者返回`Result<Config, &'static str>`，在成功时带有一个`Config`实例而在出现错误时带有一个 `&'static str`。回忆一下第十章 “静态生命周期” 中讲到 **&'static str 是字符串字面值的类型**，也是目前的错误信息。

把函数名从new改为build，因为许多程序员希望 new 函数永远不会失败。当 `Config::new` 与 `main` 交流时，可以使用 `Result` 类型来表明这里存在问题。

#### 调用Config::build并处理错误

接着修改 `main` 将 `Err` 成员转换为对用户更友好的错误，而不是 `panic!` 调用产生的关于 `thread 'main'` 和 `RUST_BACKTRACE` 的文本。

为了处理错误情况并打印一个对用户友好的信息，我们需要更新 `main` 函数来处理现在 `Config::build` 返回的 `Result`。另外还需要手动实现原先由`panic!`负责的工作，即**以非零错误码退出命令行工具的工作**。

**非零的退出状态是一个惯例信号，用来告诉调用程序的进程：该程序以错误状态退出了**。

`unwrap_or_else`，它定义于标准库的 `Result<T, E>` 上。使用 `unwrap_or_else` 可以进行一些自定义的非 panic! 的错误处理。

当Result是Ok时，这个方法的行为类似于 unwrap：它**返回Ok内部封装的值**。

然而，当其值是Err时，该方法会调用一个**闭包（closure）**，也就是一个我们定义的**作为参数传递给 unwrap_or_else 的匿名函数**。unwrap_or_else会将**Err的内部值（静态字符串）传递给闭包中位于两道竖线间的参数`err`**。闭包中的代码在其运行时可以使用这个`err`值。

`use std::process`：在错误的情况闭包中将被运行的代码只有两行：我们打印出了 err 值，接着调用了 std::process::exit。**process::exit 会立即停止程序并将传递给它的数字作为退出状态码**。**基于 panic! 的错误处理，除了不会再得到所有的额外输出了**。

### 问题一：从main提取逻辑

**提取一个叫做`run`的函数来存放目前`main`函数中不属于设置配置或处理错误的所有逻辑**。一旦完成这些，`main`函数将简明得足以通过观察来验证，而我们将能够为所有其他逻辑编写测试。

目前我们只进行小的增量式的提取函数的改进。我们仍将在src/main.rs中定义这个函数。

### 问题三：从run函数中返回错误

通过将剩余的逻辑分离进`run`函数而不是留在`main`中，像`Config::build`那样改进错误处理。不再通过expect允许程序 panic，run函数将会在出错时返回一个 esult<T, E>。

```rust
use std::error::Error;

// --snip--

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //不同于遇到错误就 panic!，? 会从函数中返回错误值并让调用者来处理它
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
```

将 run 函数的返回类型变为 Result<(), Box<dyn Error>>。之前这个函数返回 unit 类型 ()，现在它仍然保持作为 Ok 时的返回值。

对于错误类型，使用了trait对象`Box<dyn Error>`（在开头使用了 `use`语句将`std::error::Error`引入作用域）。

**Box<dyn Error>意味着函数会返回实现了Error trait的类型**，不过**无需指定具体将会返回的值的类型**。这提供了在不同的错误场景可能有不同类型的错误返回值的灵活性。这也就是dyn，它是 “动态的”（“dynamic”）的缩写。

run 函数签名中声明成功类型返回值是 ()，这意味着需要将 unit 类型值包装进 Ok 值中。**Ok(()) 一开始看起来有点奇怪，不过这样使用 () 是惯用的做法，表明调用 run 函数只是为了它的副作用；函数并没有返回什么有意义的值**。

#### 从main中run返回的错误

```rust
if let Err(e) = run(config) {
        println!("错误：{e}");
        process::exit(1);
    }
```

我们使用`if let`来检查`run`是否返回一个`Err`值，不同于 `unwrap_or_else`，并在出错时调用`process::exit(1)`。run并不返回像Config::build返回的Config实例那样需要unwrap的值。**因为run在成功时返回()，而我们只关心检测错误，所以并不需要 unwrap_or_else 来返回未封装的值，因为它只会是()。**不过两个例子中 if let 和 unwrap_or_else 的函数体都一样：打印出错误并退出。

### 问题一：将代码拆分到库crate

所有不是 main 函数的代码从 src/main.rs 移动到新文件 src/lib.rs 中：

- run 函数定义
- 相关的 use 语句
- Config 的定义
- Config::build 函数定义

## 采用测试驱动开发完善库的功能

遵循**测试驱动开发（Test Driven Development, TDD**的模式来逐步增加 minigrep 的搜索逻辑。它遵循如下步骤：

1. 编写一个失败的测试，并运行它以确保它失败的原因是你所期望的。
2. 编写或修改足够的代码来使新的测试通过。
3. 重构刚刚增加或修改的代码，并确保测试仍然能通过。
4. 从步骤 1 开始重复！

TDD：众多编写软件的方法之一，有助于驱动代码的设计。

**在编写能使测试通过的代码之前编写测试有助于在开发过程中保持高测试覆盖率**。

> 我们将测试驱动实现实际在文件内容中搜索查询字符串并返回匹配的行示例的功能。
>
> search函数：获取一个需要查询的字符串和用来查询的文本，并只会返回包含请求的文本行

### 编写失败测试

```rust
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    Ok(())
}

//定义一个显式生命周期 'a 并用于 contents 参数和返回值
// vector 中应该包含引用参数 contents（而不是参数query）slice 的字符串 slice
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        //双引号之后的反斜杠，这告诉 Rust 不要在字符串字面值内容的开头加入换行符）我们断言 search 函数的返回值只包含期望的那一行
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
```

### 编写测试通过的代码

修复实现search：

- 遍历内容的每一行文本。
- 查看这一行是否包含要搜索的字符串。
- 如果有，将这一行加入列表返回值中。
- 如果没有，什么也不做。
- 返回匹配到的结果列表

#### 使用lines方法遍历每一行

`lines`：一行一行遍历字符串的方法。lines 方法返回一个迭代器。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    for line in contents.lines() {
        // do something with line
    }
}
```

#### 用查询字符串搜索每一行

`contains`
```rust
if line.contains(query) {
            // 对文本行进行操作
}
```

#### 存储匹配的行

为此可以在 for 循环之前创建一个可变的 vector 并调用 push 方法在 vector 中存放一个 line。在 for 循环之后，返回这个 vector。

```rust
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
```

### 在run中使用search函数

```rust
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}
```

## 处理环境变量

> 用户可以通过设置环境变量来设置搜索是否是大小写敏感的。
> 
> 可以将其设计为一个命令行参数并要求用户每次需要时都加上它，或者使用环境变量。
> 
> 这允许用户**设置环境变量一次之后在整个终端会话中所有的搜索都将是大小写不敏感的**。

### 编写一个大小写不敏感search函数的失败测试（TDD）

增加一个新函数 search_case_insensitive，并将会在环境变量有值时调用它。

我们将为新的大小写不敏感搜索函数新增一个测试函数，并将老的测试函数从 one_result 改名为 case_sensitive 来更清楚的表明这两个测试的区别。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

### 实现search_case_insensitive

虽然 to_lowercase 可以处理基本的 Unicode，但它不是 100% 准确。如果编写真实的程序的话，我们还需多做一些工作，不过这一部分是关于环境变量而不是 Unicode 的，所以这样就够了。

```rust
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
```

### 环境变量

```rust
use std::env;
// --snip--

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        //调用 env::var 函数并传递我们需要寻找的环境变量名称，IGNORE_CASE
        //env::var 返回一个 Result
        //Result在环境变量被设置时返回包含其值的 Ok 成员，并在环境变量未被设置时返回 Err 成员
        //Result 的 is_ok 方法来检查环境变量是否被设置，这也就意味着我们 需要 进行一个大小写不敏感的搜索
        //如果IGNORE_CASE 环境变量没有被设置为任何值，is_ok 会返回 false 并将进行大小写敏感的搜索
        //我们并不关心环境变量所设置的 值，只关心它是否被设置了，所以检查 is_ok 而不是 unwrap、expect 或任何我们已经见过的 Result 的方法
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
```

**shell中设置环境变量并运行程序:**

```zsh
$ IGNORE_CASE=1 cargo run to poem.txt
```

一些程序允许对**相同配置同时使用参数 和 环境变量**。在这种情况下，程序来决定**参数和环境变量的优先级**。作为一个留给你的测试，尝试通过一个命令行参数或一个环境变量来控制大小写敏感搜索。并在运行程序时遇到矛盾值时决定命令行参数和环境变量的优先级。

## 将错误信息输出到标准错误而不是标准输出

`println!`：将输出写到了终端。

标准输出：standard output, stdout, 对应一般信息

标准错误：standard error, stderr, 错误信息

这种区别允许用户选择将程序**正常输出定向到一个文件**中并仍将**错误信息打印到屏幕上**

### 检查错误应该写入何处

`$ cargo run > output.txt`

命令行程序被期望将错误信息发送到标准错误流，这样即便选择将标准输出流重定向到文件中时仍然能看到错误信息。目前我们的程序并不符合期望；相反我们将看到它将错误信息输出保存到了文件中！

我们通过 > 和文件路径 output.txt 来运行程序，我们期望重定向标准输出流到该文件中。在这里，我们没有传递任何参数，所以会产生一个错误：

`Problem parsing arguments: not enough arguments`

### 将错误打印到标准错误

标准库提供了 eprintln! 宏来打印到标准错误流，所以将两个调用 println! 打印错误信息的位置替换为 eprintln!：

```rust
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
```