本章覆盖有：

- 各种各样的内存分配，性能特性，局限性
- 如何给一个对象指定那种内存分配
- 引用和Box的区别

## The Various Kinds of Allocation

要理解Rust语言，也可以说其它系统语言，例如C语言，对于理解内存分配是重要的，例如静态分配(static allocation)，栈分配(stack allocation)，堆分配(heap allocation)。

本章完全致力于该类问题。另外，我们将看到四种内存分配：

- In processor registers
- Static
- In the stack
- In the heap

在C或C++语言，静态分配指的是全局变量和使用`static`关键字声明的变量；栈分配是所有non-static本地变量，以及函数参数；堆分配则是调用了C语言标准库`malloc`函数的，或C++操作符的。

## Linear Addressing

任何计算机硬件，有一块可读和可写内存，即RAM，它由一系列长字节构成，由它们的地址访问。内存第一个字节位置为0，最后一个字节的位置等于内存长度减一。

为了简明起见，目前有两种类型的计算机：

	- 同一时刻单一线程，该进程直接使用物理内存地址。这称为“实时内存系统(real-memory systems)”。
	- 多道程序操作系统，为每个进程提供一个虚拟地址空间。这类称为“虚拟内存系统(virtual-memory systems)”

对于第一种计算机类型，现在仅作为控制器使用，没有实质上的操作系统(所以这类也称为“裸机bare-metal system”)，或者是一个系统驻留在系统的一小块，这种内存操作系统的地址跟应用程序的差不多大。

对于第二种计算机类型，访问内存的能力由操作系统控制，它运行在一个privileged mode(即权限模式，也称为protected mode，或kernel mode)，它将内存的一部分分配给各个正在运行的进程。

至此，在多道程序操作系统，进程“看见”的内存跟操作系统“看见”的内存不一样。操作系统是一个shell，或称为壳。操作系统给予进程权限访问的内存有200多个字节，操作系统满足这种需求由该驻留的进程实现。也就是说，一段机器地址300到499的内存，操作系统和这个分配了200字节的进程通信，但不是和内存的开始地址300通信。实质上每个进程有一个特定的地址空间，称为“virtual”，操作系统对物理内存的映射，称为“real”。

实际上，当一个进程访问操作系统的内存，操作系统只是保留该进程内存空间的一部分，不会有真实内存提供给该进程。因此，对于非常大的内存部分，这个分配也非常快。

只要进程尝试访问内存，即使是初始化为0，操作系统意识到进程是访问虚拟内存片段以及没有映射真实内存，立即为虚拟内存响应真实内存片段。

因此，进程的处理没有直接作用在真实的内存上，而是作用在操作系统提供的虚拟内存上，虚拟内存(虚拟存储)包含对真实地址的映射。

实际上，通常一个单一进程的虚拟内存甚至大于计算机的整个实时内存。例如，计算机可以有一个10亿字节的物理内存，对于该计算机上跑4个进程，每个进程可以有30亿字节的虚拟内存空间。如果所有的虚拟内存映射到真实内存，要处理这种情况，要求有12亿字节的内存。相反，虚拟内存的大部分字节没有被映射到真实内存；仅实际上被用于进程的字节才被映射到真实内存。只要进程开始使用它们的内存地址空间片段，以及没有被映射到真实内存时，操作系统为该虚拟内存片段响应真实的内存。

因此，每次当一个进程访问地址，不论是读或写，如果该地址属于一个虚拟内存片段(实际上叫做“页”)被驻留并映射真实内存的对应片段，进程立即访问这个真实内存；相反，如果这个驻留的页没有被映射，在允许访问之前，操作系统踢掉(kicks in)这个页，机制上叫做“page fault”，通过这种机制，操作系统分配一个真实内存页并将其映射到包含访问地址的虚拟内存页上；若是访问地址不属于进程内存空间部分上的驻留页，会出现地址错误(通常称为“segmentation fault”)。通常，地址错误导致进程的立即中止。

当然，如果程序使用内存太过随意，操作系统需要花费大量时间来做mapping，导致处理的巨大下降，甚至会由于内存不足而中断。

因此，在现代计算机中，都是单进程和多进程集于一身，每个进程“看到”它自己的内存像字节数组一样。一种是真实内存，一种是虚拟内存，但无论它是一个连续地址空间(contiguours address space)，或通常所说的“线性地址(linear addressing)”。区别于旧的计算机系统，现在计算机使用了一个“分段(segmented)”地址空间，编程者使用起来更麻烦。

所有这些都是为了曾清，在一个虚拟内存系统中，操作系统对内存分配管理的操作，是由虚拟内存到真实内存的映射。尽管现在还没有讨论跟多关于内存的分配，我们这里将内存分配定义为：由进程“发现”了驻留内存的一个片段，并关联这个片段到一个对象的操作。

## Static Allocation

尽管，有各种各样的内存分配机制。

最简单的内存分配机制是静态分配(static allocation)。根据这种机制，编译器决定了程序的每个对象需要多少个字节，以及安全地从地址空间获取相应的字节序列。因此，每个变量的地址在编译期确定。下面是一些例子：

```rust
static _A: u32 = 3;
static _B: i32 = -1_000_000;
static _C: f64 = 5.7e10;
static _D: u8 = 200;
```

`static`关键字类似于`let`，都用于声明一个变量，选择性地初始化。

`static`和`let`的不同在于：

- `static`使用了静态分配，`let`使用了栈分配。
- `static`要求显式指定变量的类型，在`let`中不是必须的。
- 常规代码不能改变一个静态变量的值，即使用了`mut`指定。因此，基于安全考虑，Rust中的静态变量通常是immutable的。
- 代码风格上要求静态变量的命名仅能包含大写字母，以及用下划线划分。违反这个规则，编译器会报一个警告。

上面四点，这里我们仅看第一个，分配的方式。

`_A`和`_B`变量带有4个字节，`_C`8个字节，`_D`带有1个字节。如果进程的开始地址是0，编译器会给`_A`分配地址0，`_B`地址是4，`_C`地址是8，`_D`地址是16，总共在编译期分配了17个字节。

当程序开始执行，进程向操作系统访问使用17个字节的内存。然后，在执行期间，不会有更多的内存请求被处理。当进程结束，进程的所有内存会自动释放给操作系统。

静态分配的缺陷是不能创建递归函数。进一步讲，如果一个函数的参数和本地变量是静态指派的，它们只有一份拷贝，当递归函数调用自身，它不能为这些参数和本地变量提供另一份拷贝。

静态分配的另一个缺陷是所有字程序的所有变量被分配在程序的开始，如果程序包含很多变量，但实际执行仅使用了一小部分，大多数变量作了无用的分配，造成该程序的内存饥渴。

典型地，`static`变量的修改是不安全的(unsafe)。

因此，在Rust中，`static`使用得不是特别多。

然后，静态分配被广泛用于其他两种数据：所有可执行二进制代码(executable binary code)，以及所有字符串字面量。


## Stack Allocation

由于静态分配的不足，Rust将对象指派到“stack”里面，每次使用`let`关键字声明变量，每次一个参数被传递给一个函数调用。这里所谓的“stack”是每个进程地址空间的片段。

实际上，每个线程也有一个stack，而不是每个进程都有一个stack。如果操作系统支持线程，则每个程序运行，一个进程被创建，一个线程会被创建并在该进程内部运行。之后，在同一个进程内部，可以创建和启动其它线程。每一次一个线程被创建(包含进程的主线程)，会请求操作系分配一份地址空间片段，它是线程的stack。在真实内存系统(裸机)中，仅会有一个stack被创建用于执行程序。

每个线程保留栈末端的地址。典型地，值较高的一端被认为是堆栈的底部，值较低的一端被认为是堆栈的顶部。

让我们看如下代码，类似前面那个，但使用了栈分配而不是静态分配：

```rust
let _a: u32 = 3;
let _b: i32 = -1_000_000;
let _c: f64 = 5.7e10;
let _d: u8 = 200;
```

该段代码仅有一个线程。现在假设这个线程有一个仅100个字节的stack，地址范围在`[500, 600)`。当程序运行，这4个变量从栈的底部开始分配，即从600开始。

因此，如图11-1所示，变量`_a`会占领地址596-599地址4个字节，变量`_b`会占领地址592-595地址4个字节，变量`_c`会占领地址584-591地址8个字节，变量`_d`会仅占领地址583。

![Figure 11-1](/img/chap11/Figure_11_1.png)

然后，如果你需要标识一个对象的地址，你必须总是使用最低位地址。因此，我们说`_a`的地址是596，`_b`的地址是592，`_c`的地址是584，`_d`的地址是583。

单词“stack”引用中国盆碟的字面理解，我们不可能在stack的中间插入一个碟(dish)，又或者从中间移除一个碟。仅能在stack的顶层添加一个碟，又或者在stack不为空，从顶层移除一个碟。

类似地，栈分配的特性是，你仅能在栈的顶部添加、或删除元素。

栈分配(allocation)或重新分配(deallocation)是非常高效的，因为它们由地址最后一个元素的添加或删除构成，该地址为stack的顶部。这个地址称为“栈指示器，栈点，指针 stack pointer”，它一直保存在处理器寄存器中，直到出现上下文切换，才将控制交由另一个线程。

stack的局限仅在于，栈顶的地址分配和重新分配。进一步讲，当一个对象被添加到stack，这个对象可以进行读和写，即使有其它对象被添加，只是读写操作不会增加或减少该对象的大小。

当一个函数调用，会给它的所有参数和所有本地变量分配足够的地址栈空间。这种分配通过这些对象大小总数的栈指针递减的方式处理，当执行的函数中止后，这个栈空间被重新分配，并以同样的值增加栈指针。因此，当一个函数返回，在函数调用之前栈指针被用来储存该值。

然而，一个函数在一段程序中可能从很多个栈点被调用，这种栈点可能有不同的大小。因此，任何函数的参数和本地变量会根据函数的调用情况分配在不同的位置。下面是一个例子：

```rust
fn f1(x1: i32) {
	let y1 = 2 + x1;
}
fn f2(x2: i32) {
	f1(x2 + 7);
}
let k = 20;
f1(k + 4);
f2(30);
```

让我们顺着这段代码的执行。看看下面表格的每个步骤对栈地址的描述：

| Operation   |   1      2      3      4    | Description                                   |
|:-----------:|:---------------------------:|:---------------------------------------------:|
|  k   ->     |   20                        | `main`入口调用，将本地变量`k`的值20添加到栈   |
|  x1  ->     |   20     24                 | `f1`方法被调用，将参数`x1`的值24添加到栈      |
|  y1  ->     |   20     24     26          | `f1`执行，将本地变量`y1`的值26添加到栈        |
|  <-  y1     |   20     24                 | `f1`结束，将它的本地变量`y1`的值26从栈中移除  |
|  <-  x1     |   20                        | `f1`结束，将它的参数`x1`的值24从栈中移除      |
|  x2  ->     |   20     30                 | `f2`方法被调用，将参数`x2`的值30添加到栈      |
|  x1  ->     |   20     30     37          | `f1`方法被调用，将参数`x1`的值37添加到栈      |
|  y1  ->     |   20     30     37     39   | `f1`执行，将本地变量`y1`的值39添加到栈        |
|  <-  y1     |   20     30     37          | `f1`结束，将它的本地变量`y1`的值39从栈中移除  |
|  <-  x1     |   20     30                 | `f1`结束，将它的参数`x1`的值37从栈中移除      |
|  <-  x2     |   20                        | `f2`结束，将它的参数`x2`的值30从栈中移除      |
|  <-  k      |                             | `main`方法结束，将本地变量`k`的值20从栈中移除 |

实际上，不论函数在哪里调用，栈上添加数据，不论函数在哪里结束，这份数据从栈上移除。这里函数`f1`被调用了两次，这里`f1`生成的机器码不会已绝对地址作为它参数和本地变量的参考。相反，它使用的地址关联这个“栈点(stack pointer)”。初始化时，这个栈点包含这个栈地址的底部。在机器码中，栈分配的变量的地址被关联这个站点(stack pointer)。让我们再复述上面这个例子。

下表展示了，每个操作，该栈点所指向的绝对地址，SP表示“stack pointer”：

| Operation   |   1      2      3      4    | Stack pointer |  x1     |   y1      |
|:-----------:|:---------------------------:|:-----------------------------------:|
|  k   ->     |   20                        | base          |         |           |
|  x1  ->     |   20     24                 | base - 4      |         |           |
|  y1  ->     |   20     24     26          | base - 12     | SP + 4  | SP        |
|  <-  y1     |   20     24                 | base - 12     | SP + 4  | SP        |
|  <-  x1     |   20                        | base - 12     |         |           |
|  x2  ->     |   20     30                 | base - 4      |         |           |
|  x1  ->     |   20     30     37          | base - 8      |         |           |
|  y1  ->     |   20     30     37     39   | base - 16     | SP + 4  | SP        |
|  <-  y1     |   20     30     37          | base - 16     | SP + 4  | SP        |
|  <-  x1     |   20     30                 | base - 8      |         |           |
|  <-  x2     |   20                        | base - 4      |         |           |
|  <-  k      |                             | base          |         |           |


在程序的开始，SP值位于栈底部地址，栈的内容未被定义，以及变量`x1`和`y1`目前未被定义。

当系统调用主函数时，SP变成了`base - 4`，因为`main`函数没有参数，仅有一个本地变量`k`，它占4个字节。

当函数`f1`第一次被调用时，SP变成了`base - 12`，因为`f1`有一个参数，`x1`，以及一个本地变量`y1`，每个占4个字节。

`y1`的创建和销毁没有改变SP，因为它函数调用时已经设置了适当的值。

当函数`f1`结束，SP在函数调用前被存储在值中，此时为`base - 4`。

当函数`f2`被调用，SP变成`base - 8`，以为参数`x2`增加了4个字节。

当`f1`再一次被调用，SP变成了`base - 16`，因为它按前一次的大小递减了8个字节。

当`f1`、`f2`和`main`函数结束后，SP递增，先变成`base - 8`，然后`base - 4`，然后`base`。

最后两列展示了，在函数`f1`中，参数`x1`的值总是SP - 4；以及本地变量`y1`的值总是SP自身。


## Limitations of Stack Allocation

栈分配非常高效和方便，但有一些局限：

- 栈的大小通常有限。它的大小由操作系统决定，并可以由某些程序压缩，但在数量级上，只有几兆的字节。
- Rust仅允许在栈上分配那些编译时已经知道大小的类型，例如基本数据类型和数组，不能对诸如vector这种运行期确定大小的类型进行栈空间的分配。
- 不能显式地在栈上分配或再分配对象。任何变量的自动分配，都需要函数签名被调用实现。即使是声明在函数的内部块，也只能有该函数结束进行再分配，这种行为这种行为不能被覆盖。

对于第二点，我们实际上声明了本地变量`Vec<_>`，这个对象会被分配到这个栈上，但在背后，这个对象会在栈之外分配一些内存。

对于第一点，可以构建一个例子超出栈的容量。

注意：下面的代码最后在虚拟机执行，因为它会强制系统的重启。

下面是一个超栈容量的例子，它会触发“statck overflow”：

```rust
const SIZE: usize = 100_000;
const N_ARRAY: usize = 1_000_000;
fn create_array() -> [u8; SIZE] { [0u8; SIZE] }
fn recursive_func(n: usize) {
let a = create_array();
println!("{} {}", N_ARRAY - n + 1, a[0]);
if n > 1 { recursive_func(n - 1) }
}
recursive_func(N_ARRAY);
```























