# 窗口初始化

现在我们将介绍如何用winit初始化窗口。

首先我们需要将winit加入到依赖中

```toml
# Cargo.toml
# ...
[dependencies]
winit = { version = "0.29", features = ["rwh_05"], default-features = false }
# wgpu 0.18 的 RawWindowHandle 版本是 0.5。如果有编译失败，可以试试 cargo update 然后再试试
```

然后创建窗口

```rust,no_run
#![windows_subsystem = "window"]

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_title("标题")
        .build(&event_loop).unwrap();

    event_loop.run(move |event, target| match event {
        _ => (),
    }).unwrap();

    Ok(())
}
```

完事，走人。

没错，只有短短三行，我们就完成了窗口初始化，念及此我不禁感叹winit的伟大。

还是要解释一下上面的代码的，winit中的类型`winit::event_loop::EventLoop`是winit中用来传递窗口事件的类型，本身除了`EventLoop::run`以外没有什么特别值得关注的函数。值得注意的是`EventLoop`可以传入用户自定义的事件，详情请阅读[EventLoop的文档](https://docs.rs/winit/latest/winit/event_loop/struct.EventLoop.html)。

`winit::window::Window`便是窗口的主体了，主要负责保管窗口句柄，窗口大小，光标位置等的数据，具体可以设置以及获取的数据请参见[Window的文档](https://docs.rs/winit/latest/winit/window/struct.Window.html)和[WindowBuilder的文档](https://docs.rs/winit/latest/winit/window/struct.WindowBuilder.html)。值得一提的是`Window`实现了`HasRawWindowHandle`，所以可以直接将`&Window`作为各类图形库的窗口句柄输入。

`EventLoop::run`接受`F: FnMut(Event<T>, &EventLoopWindowTarget<T>),`作为回调函数。第一个参数显然是窗口发送的事件，第二个参数是允许控制窗口状态和进行窗口操作的对象。在其中，你可以关闭窗口，设置窗口的等待状态，甚至创建新的窗口。我们暂时没有使用它，但是在下一章中我们会用它对一些窗口事件做出响应。

> **提示：** 开头的`#![windows_subsystem = "window"]`标签将会在Windows平台编译时将入口点改为`WINMAIN`，因而使得程序启动时不会弹出一个CMD。不需要的读者可以自行删去这一行。其他平台不会受到该标签的影响。

下一节，我们将会介绍如何处理窗口发送的事件，并且确定程序循环的主体。
