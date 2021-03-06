本章覆盖有：

- 如何通过命令行参数启动程序
- 如何给操作系统返回一个状态码
- 如何获取和设置进程环境变量
- 如何处理运行时错误
- 如何在控制台读取键盘输入信息并打印输出
- 原生类型如何转换为字符串
- 如何读写二进制文件
- 如何按行读取文本文件

## Command-Line Arguments

通过命令行输入的最基本形式是，

```rust
let command_line: std::env::Args = std::env::args();
for argument in command_line {
	println!("[{}]", argument);
}
```

该程序被编译用来创建一个文件，它通过命令行“./main first second”，它将输出：

```rust
[./main]
[first]
[second]
```

标准库中定义的`args`返回命令行参数的迭代。这种迭代器的类型是`Args`，它产生`String`值。第一个产生的值是程序名，它用路径访问。其它则是程序参数。

任何空白块会被移除，所以如果你想保留，可以用引号，`./main " first argument" "second argument "`，它将打印：

```rust
[./main]
[ first argument]
[second argument ]
```

该程序可以简化为，

```rust
for a in std::env::args() {
	println!("[{}]", a);
}
```

## Process Return Code

退出程序的最基本形式是返回码，

```rust
std::process::exit(107);
```

当调用`exit`函数是程序立即退出，并返回启动进程数字107。

在类Unix系统中，可以通过`echo $?`得到上一次输入内容，要在Windows，则输入`echo %errorlevel%`。


## Environment Variables

另外一种最常见的输入/输出的形式是环境变量，

```rust
for var in std::env::vars() {
	println!("[{}]=[{}]", var.0, var.1);
}
```

该程序将给逐行打印输出每个变量。然后，要读或写这些特殊环境变量，可以，

```rust
print!("[{:?}]", std::env::var("abcd"));
std::env::set_var("abcd", "This is the value");
print!(" [{:?}]", std::env::var("abcd"));
```

该程序可能输出：`[Err(NotPresent)] [Ok("This is the value")]`。首先是，环境变量`abcd`不存在，因此调用`var`函数时，返回`Result`类型的一个`Err`值，这种错误的特定类型是枚举`NotPresent`。

因为在当前程序中又给这个环境变量设置了值，即使用了`set_var`函数。所以，下一次获取时，得到内部变量`Ok`类型的值。

一段类似的程序如下，

```rust
print!("{}",
	if std::env::var("abcd").is_ok() {
		"Already defined"
	} else {
		"Undefined"
	}
);
std::env::set_var("abcd", "This is the value");
print!(", {}.", match std::env::var("abcd") {
	Ok(value) => value,
	Err(err) => format!("Still undefined: {}", err),
})
```

结果将打印：`Undefined, This is the value.`。


## Reading from the Console

对于面向命令行的编程，最典型地方式是从控制台输入，读取其输入信息。这种输入可能被另一段进程重定向为读取一个文件的方式。

```rust
let mut line = String::new();
println!("{:?}", std::io::stdin().read_line(&mut line));
println!("[{}]", line);
```

当该段程序启动时，它将等待你从键盘的输入，直到回车。例如，键入“Hello”，然后按回车，将会打印，

```
Ok(6)
[Hello
]
```

`stdin`函数为对当前进程的标准输入流返回一个句柄(handle)。这个句柄上，可以调用`read_line`函数。它等待标准输入流的行尾(end-of-line)或卷尾(end-of-file)字符的输入，并读取当前输入缓冲区提供的所有字符。这种读取可能失败，因为同时可能会有其它进程在读取。

如果读取成功，读取到的字符丢到一个字符串对象，指派给`line`变量，作为参数的形式，接收这个可变对象的引用，`read_line`函数返回一个`Ok`结果对象，该对象的数据是按字节的数字读取的。注意这个数字是6，所以“Hello"是5个字节，但还包含一个行尾(end-of-line)控制字符。实际上，当`line`变量被输出后，中括号另起一样输出，因为行尾(end-of-line)字符在最后一行被打印出来了。

如果`read_line`函数不能从标准输入流读取字符串，它返回一个`Err`结果对象，以及不会更改变量`line`的值。

让我们看看标准输入流读取几行时发生了什么，

```rust
let mut text = format!("First: ");
let inp = std::io::stdin();
inp.read_line(&mut text).unwrap();
text.push_str("Second: ");
inp.read_line(&mut text).unwrap();
println!("{}: {} bytes", text, text.len());
```

运行该程序，键入“eè€”，回车，键入“Hello”，回车，将打印，

```
First: eè€
Second: Hello
: 28 bytes
```

首先，注意到字符串输入了三行，因为它包含两个行尾字符串。另外，它包含7字节的ASCII字符串“First: ”，以及8字节的ASCII字符串“Second: ”。“Hello”也是一个ASCII字符串，包含5个字节。另外“eè€”字符串包含6个字节，所以我们一共有7+6+1+8+5+1=28字节。

然后，让我们看看`text`变量的文本如何构建起来的。注意`read_line`函数将键入行追加到参数指定的对象上，而不是重写它。`text`变量由“First: ”初始化。然后，在第三行，首次键入的行被追加到文本中。然后，在第四行，字符串字面量“Second：”追加到变量。最后，在第五行，第二次键入的行再次被追加的内容中。

其三，注意到当函数`read_line`从缓冲区读取时，它会清空缓冲区，这样再次读取缓冲区时不会重复读取缓冲区的内容。

其四，注意每次调用`read_line`后，后面都会调用`unwrap`，但它的返回值可以忽略。

所以这个调用可以省略，

```rust
let mut text = format!("First: ");
let inp = std::io::stdin();
inp.read_line(&mut text);
text.push_str("Second: ");
inp.read_line(&mut text).unwrap();
println!("{}: {} bytes", text, text.len());
```

然而，当这段程序被编译，编译输出，对于两处调用的`read_line`，会警告：`"unused `std::result::Result` which must be used".`。它意味着`read_line`返回个`Result`的值，以及这个值被忽略或不被使用。Rust中认为忽略`Result`类型的值是危险的，因为这种类型可能表示一个运行时错误，所以程序逻辑不会统计这种错误。这在生产环境是危险的，但它也不适用于调试代码，因为它隐藏了你需要寻找的错误。

因此，在调试代码时，最好总是在最后加上`.unwrap()`从句。

在生产环境代码，问题并不是那么简单。


## Proper Runtime Error Handling

在真实软件世界，大部分函数调用返回一个`Result`类型值。这类函数称为“不可靠，fallible”函数，即正常返回`Ok`，异常情况返回`Err`。

在C++、Java以及其他面向对象语言中，标准错误的处理技术基于“异常”这一概念，并有`throw`、`try`、`catch`这些关键字。在Rust中没有这些东西；所有错误处理基于`Result`类型，以及`match`语句匹配。

假设，典型地，你写一个函数`f`，要实现它的功能，会调用几个不可靠函数，`f1`、`f2`、`f3`和`f4`。这些函数可能会返回错误，或者成功结果。希望如果某个函数失败，应该立即将错误信息返回给`f`函数，若是成功，则传递给下一个函数继续执行。

一个可能的写法是，

```rust
fn f1(x: i32) -> Result<i32, String> {
	if x == 1 {
		Err(format!("Err. 1"))
	} else {
		Ok(x)
	}
}
fn f2(x: i32) -> Result<i32, String> {
	if x == 2 {
		Err(format!("Err. 2"))
	} else {
		Ok(x)
	}
}
fn f3(x: i32) -> Result<i32, String> {
	if x == 3 {
		Err(format!("Err. 3"))
	} else {
		Ok(x)
	}
}
fn f4(x: i32) -> Result<i32, String> {
	if x == 4 {
		Err(format!("Err. 4"))
	} else {
		Ok(x)
	}
}
fn f(x: i32) -> Result<i32, String> {
	match f1(x) {
		Ok(result) => {
			match f2(result) {
				Ok(result) => {
					match f3(result) {
						Ok(result) => f4(result),
						Err(err_msg) => Err(err_msg),
					}
				}
				Err(err_msg) => Err(err_msg),
			}
		Err(err_msg) => Err(err_msg),
		}
	}
}
match f(2) {
	Ok(y) => println!("{}", y),
	Err(e) => println!("Error: {}", e),
}
match f(4) {
	Ok(y) => println!("{}", y),
	Err(e) => println!("Error: {}", e),
}
match f(5) {
	Ok(y) => println!("{}", y),
	Err(e) => println!("Error: {}", e),
}
```

结果将打印：

```
Error: Err. 2
Error: Err. 4
5
```

这种写法肯定不方便，可以替换为行内写法，

```rust
fn f(x: i32) -> Result<i32, String> {
	let result1 = f1(x);
	if result1.is_err() { return result1; }
	let result2 = f2(result1.unwrap());
	if result2.is_err() { return result2; }
	let result3 = f3(result2.unwrap());
	if result3.is_err() { return result3; }
	f4(result3.unwrap())
}
```

这种写法是将结果写入临时变量中，结果值通过`is_err`函数检测。失败则返回，成功则`unwrap`出真实结果。

下面是另一种等价`f`的实现，

```rust
fn f(x: i32) -> Result<i32, String> {
	f4(f3(f2(f1(x)?)?)?)
}
```

这里的问号是一个特定的宏(macro)，诸如“`e?`”的表达式，如果“`e`”是泛型类型“`Result<T,E>`”，宏展开为表达式“`match e { Some(v) => v, _ => return e }`”；相反，如果“`e`”是泛型类型“`Option<T>`”，宏展开为表达式“`match e { Ok(v) => v, _ => return e }`”。换言之，这种宏语法将“`Some`”或“`Ok`”的参数，进行转换，或返回包含的函数的一个值。

它仅能作用于类型为“`Result<T,E>`”或“`Option<T>`”的表达式中，所以也仅能作用于有恰当返回值类型的函数内部。如果闭合函数返回值类型是“`Result<T1,E>`”，问号宏仅能作用于类型“`Result<T2,E>`”的表达式，其中“`T2`”可以和“`T1`”不同，但“`E`”必须相同；以及，如果闭合函数返回值类型是“`Option<T1>`”，问号宏仅能作用于类型“`Option<T2>`”的表达式。

因此，要构建一个稳健的错误处理模式。每个函数包含对一个不可靠(fallible)函数的调用，应该是一个fallible函数或使用“`match`”语句处理“`Result`”结果值。在最先的一种示例代码中，每个不可靠函数的调用，都应该用问号宏来传递错误条件。“`main`”函数不可能是falliable的，所以在调用链中，应该用“`match`”语句处理“`Result`”的值。


## Writing to the Console

我们一直用“`print`”或“`println`”宏来打印消息。然而，你也可以直接用库函数将信息输出到控制台。

```rust
use std::io::Write;
//ILLEGAL: std::io::stdout().write("Hi").unwrap();
//ILLEGAL: std::io::stdout().write(String::from("Hi")).unwrap();
std::io::stdout().write("Hello ".as_bytes()).unwrap();
std::io::stdout().write(String::from("world").as_bytes()).unwrap();
```

结果将打印：“`Hello world`”。

“`stdout`”标准库函数返回一个句柄处理当前进程的标准输出流，“`write`”可以实现。

“`write`”函数不能直接打印静态字符串，也不能打印动态字符串，当然数字、常见组合对象也不能。

“`write`”函数接收一个“`&[u8]`”类型，它是字节切片的一个引用。这些字节会打印为控制台的UTF-8字符串。所以如果打印的对象不是UTF-8格式的切片字节，首先你需要转换。

为了将静态字符串和动态字符串转换为一个字节切片的引用，你可以使用“`as_bytes`”函数。该函数返回字符串第一个字节的地址，以及字符串对象的字节数。由于这个字节数早已经包含在字符串对象的头部，所以这个函数极其高效。

最后，注意到“`write`”函数返回一个“`Result`”类型值，表示它是一个不可靠函数(fallible function)。如果你确定它不可能是fail，最好调用“`unwrap`”函数获取其返回值。

## Converting a Value to a String

如果你希望将其它值类型打印为文本，可以使用“`to_string`”函数，它被定义在所有原生类型。

```rust
let int_str: String = 45.to_string();
let float_str: String = 4.5.to_string();
let bool_str: String = true.to_string();
print!("{} {} {}", int_str, float_str, bool_str);
```

将会打印：“`45 4.5 true`”。

`to_string`函数指派一个字符串对象，头部会放在栈，内容放在堆。因此，它不是高效的。

## File Input/Output

Rust提供了对于二进制文件或文本的读写，

```rust
use std::io::Write;
let mut file = std::fs::File::create("data.txt").unwrap();
file.write_all("eè€".as_bytes()).unwrap();
```

第二行调用了`create`函数，在当前文件目录下，创建一个“data.txt”的文件。该函数是falliable的，如果创建文件成功，它返回刚创建文件的句柄。

最后一行调用了`write_all`函数，对新创建的文件写入某些字节，注意“eè€”有6个字节，包含行尾结束符。

若要读取刚刚创建的文件“`data.txt`”，可以，

```rust
use std::io::Read;
let mut file = std::fs::File::open("data.txt").unwrap();
let mut contents = String::new();
file.read_to_string(&mut contents).unwrap();
print!("{}", contents);
```

打印输出：“eè€”。

第二行调用了`open`函数，打开当前目录下的"data.txt"文件。如果文件不存在或不可访问则fail。如果成功，则指派给一个`file`变量处理该函数。

第4行调用`read_to_string`函数，将文件内容读取到一个变量，由一个可变对象引用传递。

最后一行将文本内容打印输出。

对于文件拷贝，但是如果文件太大，是不可能将所有东西都塞到一个字符串。它要求读写分段处理。但分段处理并不高效。

下面是一个拷贝文件的高效实现，

```rust
use std::io::Read;
use std::io::Write;
let mut command_line: std::env::Args = std::env::args();
command_line.next().unwrap();
let source = command_line.next().unwrap();
let destination = command_line.next().unwrap();
let mut file_in = std::fs::File::open(source).unwrap();
let mut file_out = std::fs::File::create(destination).unwrap();
let mut buffer = [0u8; 4096];
loop {
	let nbytes = file_in.read(&mut buffer).unwrap();
	file_out.write(&buffer[..nbytes]).unwrap();
	if nbytes < buffer.len() { break; }
}
```

该段程序启动时必须传入两个命令行参数。第一个参数是源文件，第二个参数是目标文件。

从第3到第6行，将第一个命令行参数指派给`source`变量，第二个命令行参数指派给`destination`变量。

后面两行，将源文件打开，指派给变量`file_in`，创建文件指派给变量`file_out`。

然后将一个4096字节缓存指派到栈。

最后，用一个循环，重复地将一个4096byte的chunk，从源文件，写入到目标文件。缓冲区由多少字节，就自动读取多少字节。如果剩余文件片段不够长，读取少于4096的字节，或者0个字节。

读取的字节被塞到了`nbytes`变量。

对于超过4096字节的大文件，首次迭代读取4096个字节，然后继续迭代读取。对于小于4096字节的文件，迭代一次就可以了。

任何情况下，读取的字节数量，就是写入缓冲的字节数。因此，缓冲区切片由开始位置到读取字节数长度。

如果读取的字节数小于缓冲区长度，循环结束，因为已经达到输入文件的末尾。

注意，这里不用显式关闭文件。只要文件处理结束，文件自动关闭，以及存储和释放所有内部临时缓冲区。


## Processing Text Files

对于文本文件的处理，例如我们想知道有多少行，有多少空白，我们可以，

```rust
let mut command_line = std::env::args();
command_line.next();
let pathname = command_line.next().unwrap();
let counts = count_lines(&pathname).unwrap();
println!("file: {}", pathname);
println!("n. of lines: {}", counts.0);
println!("n. of empty lines: {}", counts.1);

fn count_lines(pathname: &str) -> Result<(u32, u32), std::io::Error> {
    use std::io::BufRead;

    let f = std::fs::File::open(pathname)?;
    let f = std::io::BufReader::new(f);
    let mut n_lines = 0;
    let mut n_empty_lines = 0;
    for line in f.lines() {
        n_lines += 1;
        if line?.trim().len() == 0 {
            n_empty_lines += 1;
        }
    }
    Ok((n_lines, n_empty_lines))
}
```

这里用到了`BufReader`。它会将输入放入缓冲区。创建一个“`BufReader`”对象后，不需要显式使用`File`对象，新创建的对象被指派给另一个变量`f`，它会覆盖原来的变量。

当两个计数器`n_lines`和`n_empty_lines`被声明以及初始化后。

进入循环体对文件内容进行统计。`BufReader`类型提供了`line`函数，它是一个迭代生产者，返回所在行的一个迭代器。注意Rust的迭代器是lazy的；所以，每次迭代，循环体将下一行塞到`line`变量。

以及文件读取是包含副作用的，所以`line`的值是一个`Result<String, std::io::Error>`类型值。因此，带上问号宏`?`获取它的真是字符串值，或者是返回的一个I/O错误。

循环体内，`n_lines`统计行数，`n_empty_lines`将空白和0长度行进行统计，它调用了`trim`函数。

最后一个语句返回成功值：`Ok`。它包含两个计数器。

