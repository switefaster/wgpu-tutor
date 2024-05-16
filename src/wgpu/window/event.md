# 事件发生！

在上一节中，我们初始化了一个窗口。如果有读者运行了上节的例程，就会发现运行后弹出了一个空白的窗口，但是不会对你的操作做出任何响应（除了移动和缩放），甚至点右上角的$\times$都没用。这是因为我们没有处理winit发来的事件，所有响应都是空的，这一节，我们将会学习处理一些基本的事件。

我们先来看一个例子

```rust,no_run
fn window_event(
    &mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
) {
    if let Some(Application { window }) = &mut self.app {
        if window.id() != window_id {
            return;
        }
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(physical_size) => {
                resize(physical_size);
            }
            WindowEvent::RedrawRequested(_) => {
                render();
                window.request_redraw();
            }
            _ => (),
        }
    }
}=
```

这是一个渲染程序最基础的循环，现在我们来逐个分析这个回调中干了什么

首先，我们用`match event {...}`来匹配事件，其中处理了三种事件：

- `WindowEvent::CloseRequested` 会处理窗口收到的关闭请求，如点击$\times$。正是因为没处理这个事件，窗口才对$\times$根本不作响应。
- `WindowEvent::RedrawRequested` 事件会在窗口被要求重新绘制窗口内容时被调用，这也将会是渲染部分被调用的位置。我们在每次渲染结束时都调用一次`window.request_redraw()`，这得以让我们的渲染自发地持续下去。
- `WindowEvent::Resized` 事件在窗口大小改变时传入，我们通常用其调整我们的帧缓冲大小（这是什么？一切皆在 [3.1](../infra/graphics.md) 揭晓）。
- 剩下的事件暂时管不着，我们直接`_`

这便是这个循环的主体，简单明了，却会是整个程序的主干，不容忽视。

现在我们的程序已经能进行基本的循环了。下一小节我们会介绍如何维持定时长的更新，这并非必要，却是游戏中常见的需求，读者可以自行选择跳过与否。
