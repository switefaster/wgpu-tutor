# 引言

__阅读本文需要[Rust](https://www.rust-lang.org)基础，以及一小部分[Rust Async](https://rust-lang.github.io/async-book/)的知识，未满足要求者请阅读[Rust Book](https://doc.rust-lang.org/book/)__

> __警告：__ 切忌将本教程当作一本正经的教程，否则后果自负。文中笔者可能会使用模糊不清甚至粗俗的语言，若有不适，请适度阅读。

---

 > 在遥远的梦想之地，让 Rust 有一个安全的，不使用`unsafe`的图形库是很多人长远的理想，无数的前辈们前仆后继[gfx](https://github.com/gfx-rs/gfx/tree/pre-ll)，[glium](https://github.com/glium/glium)，[vulkano](https://github.com/vulkano-rs/vulkano)……一时间，仿佛诸神黄昏。然，或许是这个问题对人类而言太过困难，或许是神明的旨意，英雄们或落幕，或仍有不足……漫漫长夜，笼罩着Rust图形学库的未来。长夜终尽！曙光将至！救世主已经降临！让我们跪服于其神威，高呼祂的姓名！——[WGPU](https://github.com/gfx-rs/wgpu)！！！
> <p align="right">——佚名</p>

<!---->
> __FNMDP__
> <p align="right">——switefaster</p>

蚌埠住了。

---

以上的话请务必不要当真。实话实说，Rust中称得上尽人意的**安全**图形库确实不多，其中的先锋gfx早已停止维护，转而进行[gfx-hal](https://github.com/gfx-rs/gfx)的开发<mask>然而后者现在也进入维护模式了，因为WGPU</mask>，vulkano的语法(主要是那些宏)及线程安全多少有些麻烦<mask>蛋疼</mask>，WGPU 称得上是为数不多安全，甚至线程安全，而又简便易用的图形库了。<small>你问我glium？<mask>~~GL，狗都不用~~</mask></small>

WGPU 可能不是这些库中性能最高的，却绝对称得上是最易用的，也是笔者(下文中统称"我")最喜欢的图形库。

在这篇教程中，我会尽量用通俗易懂的语言向读者(下文中或许会简称"你")介绍WGPU的基本使用。鉴于图形库的性质，我会相对详细地介绍其中涉及的数学知识以及图形库工作的方式，并且会提及到其他图形库中的写法。图形学初学者和其他图形库用户过来打酱油都可以放心食用。
