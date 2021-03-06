本章覆盖有：

- 为什么需要匿名函数，如何编写匿名函数，如何访问它定义的变量
- 这些“闭包”，如何声明和调用

## The Need for "Disposable" Functions

Rust对数组的升序实现，

```rust
let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
arr.sort();
print!("{:?}", arr);
```

结果将输出：“`[0, 1, 4, 7, 8, 10, 12, 45]`”。

但标准库里面没有提供降序的函数；你需要调用`sort_by`函数，将它的一个引用传递给一个比较函数。这种函数接受两个记录，并返回一个indication：

```rust
let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
use std::cmp::Ordering;
fn desc(a: &i32, b: &i32) -> Ordering {
	if a < b { Ordering::Greater }
	else if a > b { Ordering::Less }
	else { Ordering::Equal }
}
arr.sort_by(desc);
print!("{:?}", arr);
```

`desc`函数返回了一个标准库中定义的类型：

```rust
enum Ordering { Less, Equal, Greater }
```

这种方式生效，但有几点诟病。

首先，`desc`函数定义仅用于一处。标准库函数`sort_by`接收一个函数入参。这个入参需要是一个匿名函数，这个函数也仅用于一处。

另外，虽然类型规范对于变量声明是可选的，但对于参数和函数的返回值是必须的。这些规范，可以像函数名一样，方便地在其它语句或程序调用。但当你需要写一个函数仅在它声明的地方调用，这种规范几乎是匿名的。因此，声明和调用一个行内匿名的、由参数和返回值推断的类型的函数，将会是一个便利的特性。

另一个诟病是需要给函数体花括号闭合。通常函数会包含几条语句，因此不是所有匿名的带上花括号闭合。相反，匿名函数通常只有一条单一表达式，可以不用写闭合。

## Capturing the Environment

本章我们所陈述的所有内容，对于其它大多数语言也是适用的，包括C语言。但Rust函数有一个额外的不寻常限制：它不能访问任何外部声明的变量。你可以访问`static`的，你可以访问`constants`的，但不能访问栈分配的变量(也就是用`let`声明的变量)。例如，下面例子是不合法的：

```rust
let two = 2.;
fn print_double(x: f64) {
	print!("{}", x * two);
}
print_double(17.2);
```

编译出错：“can't capture dynamic environment in an fn item.”

“dynamic environment”意味着一系列变量在函数调用时才生效。所以，它是“dynamic”的，这些变量核能在某些语句生效，在其它语句失效。“capture the environment”意味着能够访问这些变量。

相反，下面是有效的：

```rust
const TWO: f64 = 2.;
fn print_double(x: f64) {
	print!("{}", x * TWO);
}
print_double(17.2);
```

或者这样写

```rust
static TWO: f64 = 2.;
fn print_double(x: f64) {
	print!("{}", x * TWO);
}
print_double(17.2);
```

这种限制有一个很好的理由：外部变量可以有效地进入函数的程序接口，但是从函数签名中看不出来，因此它对理解代码产生误导。

但当一个函数仅能在它定义的地方调用，访问外部变量并不能降低理解难度，因为这些外部变量在声明语句已经生效。

因此，我们特性的需求是：一个行内匿名函数，带类型推断；一个单一表达式作为函数体；可以捕获任何有效变量。

## Closures

闭包，说白了就是引用了自由变量的函数。这个被引用的自由变量将和这个函数一同存在，即使离开了创造它的环境也不例外。

闭包的调用出现在它定义的地方。实际上，你也可以定义一个闭包，尽管类型规范可行，实际上典型使用闭包的场景并不多。下面是使用闭包实现排序的一种方式：

```rust
let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
use std::cmp::Ordering;
let desc = |a: &i32, b: &i32| -> Ordering {
	if a < b { Ordering::Greater }
	else if a > b { Ordering::Less }
	else { Ordering::Equal }
};
arr.sort_by(desc);
print!("{:?}", arr);
```

跟前面不同的是：

- 使用了`let`关键字代替`fn`。
- 闭包名后面带有`=`号。
- 函数的参数由`(`和`)`，在闭包中变为`|`(管道)标志。
- 闭包声明带有分号`;`。

至此，我们说过，闭包声明和调用都在同一个地方，类型和大括号(braces)是可选的。因此，上面可以简化一下：

```rust
let mut arr = [4, 8, 1, 20, 0, 45, 12, 7];
use std::cmp::Ordering;
arr.sort_by(|a, b|
	if a < b { Ordering::Greater }
	else if a > b { Ordering::Less }
	else { Ordering:: Equal });
print!("{:?}", arr);
```

有很多简洁的写法。标准库早以包含有`cmp`函数("compare"的简写)；该函数根据两个参数比较返回一个`Ordering`值。下面写法是等价的：

```rust
arr.sort();
arr.sort_by(|a, b| a.cmp(b));
```

因此，要想得到一个反转的顺序，你可以用下面的方式：

```rust
arr.sort_by(|a, b| (&-*a).cmp(&-*b));
arr.sort_by(|a, b| b.cmp(a));
```

完整代码：

```rust
let mut arr = [4, 8, 1, 10, 0, 45, 12, 7];
arr.sort_by(|a, b| b.cmp(a));
print!("{:?}", arr);
```

同时也删除了`use`指令，因为这里不再需要。

## Other Examples

下面是以6种方式调用闭包的例子：

```rust
let factor = 2;
let multiply = |a| a * factor;
print!("{}", multiply(13));
let multiply_ref: &(Fn(i32) -> i32) = & multiply;
print!(
	" {} {} {} {} {}",
	(*multiply_ref)(13),
	multiply_ref(13),
	(|a| a * factor)(13),
	(|a: i32| a * factor)(13),
	|a| -> i32 { a * factor }(13));
```

将会打印输出：“`26 26 26 26 26 26`”。

该程序提供6种不同风格的闭包调用。每个调用都接收一个`i32`的命名参数`a`；由变量`factor`作乘积；这里的入参总是13，所以结果总是26。

在第二行，声明了一个闭包，它根据参数`a`和返回值进行类型推断。闭包内访问内部变量`factor`，即捕获了自由变量。以及初始化了变量`multiply`，它的类型由推断得出。

在第三行，闭包指派到`multiply`变量的调用和函数类似。

在第四行，声明的闭包的地址被用于初始化`multiply_ref`变量。该变量的类型可以被推断，但已经被明确指定。这里的`Fn`表示它是一个函数类型。每个函数都有一个类型，它有它的参数和返回值确定。表达式`Fn(i32) -> i32`表示“该函数的类型是：接收一个`i32`参数，返回一个`i32`”。该类型表达式又符号`&`处理，因为它是一个“reference to a function”，不是“a function”。

在第七行，函数的引用被反向引用，获得一个函数，以及调用这个函数。

在第八行，函数不进行反引用被调用，因为该函数调用会隐式地反引用处理。

最后三条语句，声明了匿名闭包并进行调用。第一条语句可以推断出参数类型和返回类型；第二条制定了参数类型并推断返回类型；第三条指定了返回类型，推断出参数类型。

注意参数13被传递到闭包总是用小括号括住。为了避免表达式`(13)`与前面部分的闭包出现迷惑，某些情况下闭包表达式也需要用小括号括起来。与此，在最后一条语句，闭包的语句体，要与返回类型规范区别开来，需要用花括号(braces)。

当闭包包含几个语句时，花括号是必须的，例如：

```rust
print!(
	"{}",
	(|v: &Vec<i32>| {
		let mut sum = 0;
		for i in 0..v.len() {
			sum += v[i];
		}
		sum
	})(&vec![11, 22, 34]));
```

这里将打印输出“67”，即向量元素求和。

这里需要制定参数的类型，不然编译器无法推断，以及抛出一个错误信息“`the type of this value must be known in this context`”，定义在表达式`v.len()`上。