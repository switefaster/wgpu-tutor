# 认识 WGPU

从本节开始，我们将正式开始接触`WGPU`这个库。这一小节只是对库的介绍，并没有过多知识。

## 什么是WGPU？

> ___Wgpu-rs is an idiomatic Rust wrapper over wgpu-core. It's designed to be suitable for general purpose graphics and computation needs of Rust community. <br>Wgpu-rs can target both the natively supported backends and WebAssembly directly.___
> <p align="right">——wgpu.rs</p>

**WGPU** 视其语境可以代指不同的对象。在Rust社区以及本文中，通常代指[wgpu-rs](https://github.com/gfx-rs/wgpu)。`wgpu-rs`是对`wgpu-core`的包装，后者是一个更加底层的包装层，负责直接与图形库底层进行交互。而`wgpu-core`又依赖于`wgpu-hal`，HAL是硬件抽象层(___Hardware Abstraction Layer___)的简称。顾名思义，HAL将各个图形库最核心的观念提取成了一个抽象层，并且后端由不同的底层库实现。这也是`wgpu-rs`得以在不改变代码的情况下改变运行底层的根本。不过由于其设计模式最接近于Vulkan，因此目前使用Vulkan底层才能得到最佳性能。

事实上，WGPU名称的来源是[WebGPU](https://www.w3.org/TR/webgpu/)标准。后者旨在为Web环境提供一个可以调用硬件GPU进行渲染和计算的标准。它的标准是为JavaScript设计的，这也解释了WGPU库中偶尔出现的一两个`async`函数，以及为何WGPU支持WebAssembly。`wgpu-rs`是`WebGPU`对Rust用户的包装，而其后的`wgpu-core`则亦成为了一些浏览器实现`WebGPU`功能的后端（如 _FireFox_）。

> wgpu-core曾经依赖的是gfx-hal。后者是一个基于Vulkan设计的硬件抽象层。后来，wgpu开发组发现gfx-hal对于wgpu而言过于冗余，于是贴合wgpu的需求开发了wgpu-hal。目前gfx-hal进入了维护模式。

---

> _Chrome_ 浏览器使用的WebGPU实现并非`wgpu-core`而是C++编写的[Dawn](https://dawn.googlesource.com/dawn)

当然，在本书中，我们只会涉及到`wgpu-rs`的部分。

## 它长什么样？

我们来看一眼[wgpu的文档](https://docs.rs/wgpu/)。可以发现，能用的大部分结构都在`wgpu::`根模块下。只有少部分实用工具在`wgpu::util::`模块下。所以你大部分时候只要输入`wgpu::`就能找到你想要的东西。

在文档的一长串结构列表中，我们可以看到熟悉的几个身影，例如

- `wgpu::Instance`
- `wgpu::Adapter`
- `wgpu::Device`
- `wgpu::Queue`
- `wgpu::CommandBuffer`
- etc.

诚然，他们对应了上一节所讲述的内容。当然，更多的还有我们暂不熟悉的结构，我们将会在之后小节的学习中逐渐认识他们。

## 可能可以用到的链接

- [wgpu的文档](https://docs.rs/wgpu/)
- [wgpu的主页](https://wgpu.rs/)
- [wgpu的CHANGELOG](https://github.com/gfx-rs/wgpu/blob/master/CHANGELOG.md)
- [sotrh的英文WGPU教程](https://sotrh.github.io/learn-wgpu/)

本小节至此结束，下一节，我们将正式开始初始化WGPU并开始我们的渲染旅途。
