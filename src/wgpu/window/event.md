# 事件发生！

在上一节中，我们初始化了一个窗口。如果有读者将`todo!()`删去并运行了上节的例程，就会发现运行后弹出了一个空白的窗口，但是不会对你的操作做出任何响应（除了移动和缩放），甚至点右上角的$\times$都没用。这是因为我们没有处理winit发来的事件，所有响应都是空的，这一节，我们将会学习处理一些基本的事件。

我们先来看一个例子

```rust,no_run
event_loop.run(move |event, _, control_flow| match event {
        Event::DeviceEvent { event, .. } => {
            unimplemented!()
        }
        Event::WindowEvent {
            event,
            window_id,
        } if window_id == window.id() =>
            match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    resize(physical_size);
                }
                WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                    resize(*new_inner_size);
                }
                _ => {}
            }
        Event::RedrawRequested(_) => {
            render();
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        _ => {}
    });
```

这是一个渲染程序最基础的循环，现在我们来逐个分析这个回调中干了什么

首先，我们用`match event {...}`来匹配事件，其中处理了四种事件：

- `Event::DeviceEvent` 是 `Event` 的子事件，负责传递设备相关的信息，例如 **鼠标移动** **鼠标点击** **键盘按键** 等
- `Event::WindowEvent` 是 `Event` 的子事件，负责传递窗口相关的信息，例如 **窗口缩放** **窗口移动** 等等，我们顺便在这把对窗口的关闭请求处理了（注意`control_flow`的用法），$\times$终于有了用武之地
- `Event::RedrawRequested` 事件会在窗口被要求重新绘制窗口内容时被调用，这也将会是渲染部分被调用的位置
- `Event::MainEventsCleared` 事件会在事件队列中的事件清空时被调用，我们在此进行一次重绘要求，这样程序就会尽可能快地进行渲染
- 剩下的事件暂时管不着，我们直接`_`

> 如果你翻阅了`Event`的文档，那么你或许发现了`WindowEvent`和`DeviceEvent`里面都有键盘、鼠标相关的事件。值得注意的是，`DeviceEvent`获取到的事件是直接来源于系统外设的信号的，因而并不受窗口聚焦等的限制。在游戏中，我们更倾向于使用后者，而在应用中我们更倾向于使用前者。

这便是这个循环的主体，简单明了，却会是整个程序的主干，不容忽视。

现在我们的程序已经能进行基本的循环了。下一小节我们会介绍如何维持定时长的更新，这并非必要，却是游戏中常见的需求，读者可以自行选择跳过与否。
