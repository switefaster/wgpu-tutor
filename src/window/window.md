# 窗口初始化

现在我们将介绍如何用winit初始化窗口。

首先我们需要将winit加入到依赖中

```toml
# Cargo.toml
# ...
[dependencies]
winit = "0.25.0"
```

然后创建窗口

```rust,no_run
#![windows_subsystem = "window"]

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let _window = winit::window::WindowBuilder::new()
        .with_title("标题")
        .build(&event_loop)
        .unwrap();
    event_loop.run(move |_event, _, _control_flow| {
        todo!()
    });
}
```

完事，走人。

没错，只有短短三行，我们就完成了窗口初始化，念及此我不禁感叹winit的伟大。

还是要解释一下上面的代码的，winit中的类型`winit::event_loop::EventLoop`是winit中用来传递窗口事件的类型，本身除了`EventLoop::run`以外没有什么特别值得关注的函数。值得注意的是`EventLoop`可以传入用户自定义的事件，详情请阅读[EventLoop的文档](https://docs.rs/winit/0.25.0/winit/event_loop/struct.EventLoop.html)。

`winit::window::Window`便是窗口的主体了，主要负责保管窗口句柄，窗口大小，光标位置等的数据，具体可以设置以及获取的数据请参见[Window的文档](https://docs.rs/winit/0.25.0/winit/window/struct.Window.html)和[WindowBuilder的文档](https://docs.rs/winit/0.25.0/winit/window/struct.WindowBuilder.html)。值得一提的是`Window`实现了`HasRawWindowHandle`，所以可以直接将`&Window`作为各类图形库的窗口句柄输入。

`EventLoop::run`接受`F: 'static + FnMut(Event<'_, T>, &EventLoopWindowTarget<T>, &mut ControlFlow)`作为回调函数。第一个参数显然是窗口发送的事件，第二个参数是可以`Deref`出`&EventLoop`的一个包装，主要方便用户在事件处理过程中创建新的窗口，而第三个参数负责控制窗口状态，用法例如`*control_flow = ControlFlow::Exit`。在一次回调结束后，winit会根据ControlFlow的值改变窗口的状态，比如关闭窗口等。<mask>一切都是borrow checker的选择</mask>

> __提示__：开头的`#![windows_subsystem = "window"]`标签将会在Windows平台编译时将入口点改为`WINMAIN`，因而使得程序启动时不会弹出一个CMD。不需要的读者可以自行删去这一行。其他平台不会受到该标签的影响。

下一节，我们将会介绍如何处理窗口发送的事件，并且确定程序循环的主体。
