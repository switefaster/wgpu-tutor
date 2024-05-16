# 窗口初始化

现在我们将介绍如何用winit初始化窗口。

> **提示：** 在`main.rs`中加入`#![windows_subsystem = "window"]`标签将会在Windows平台编译时将入口点改为`WINMAIN`，因而使得程序启动时不会弹出一个CMD。不需要的读者可以自行删去这一行。其他平台不会受到该标签的影响。

首先我们需要将winit加入到依赖中

```toml
# Cargo.toml
# ...
[dependencies]
winit = "0.30"
```

然后，神说：

```rust,no_run
#![windows_subsystem = "window"]

use std::sync::Arc;

use winit::application::ApplicationHandler;

struct Application {
    window: Arc<winit::window::Window>,
}

#[derive(Default)]
struct State {
    app: Option<Application>,
}

impl ApplicationHandler for State {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.app = Some(Application {
            window: Arc::new(
                event_loop
                    .create_window(
                        winit::window::Window::default_attributes().with_title("窗口标题"),
                    )
                    .unwrap(),
            ),
        })
    }

    fn window_event(
        &mut self,
        _event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        _event: winit::event::WindowEvent,
    ) { }
}

fn main() -> anyhow::Result<()> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    let mut state = State::default();

    event_loop.run_app(&mut state)?;

    Ok(())
}
```

于是就有了窗口。

> 如果你是从旧版的winit迁移过来的，就会发现窗口的初始化麻烦了不少。这是因为winit在 `0.30.0` 版本进行了一次重构。为了适应多平台（主要是Android），将事件处理和应用状态融合在了一起。

让我们解释一下上面的代码，winit中的类型`winit::event_loop::EventLoop`是winit中用来传递窗口事件的类型，一个winit应用可以创建多个窗口，而这多个窗口的事件都经由创建它们的`EventLoop`处理。值得注意的是`EventLoop`可以传入用户自定义的事件，详情请阅读[EventLoop的文档](https://docs.rs/winit/latest/winit/event_loop/struct.EventLoop.html)。

`EventLoop`通过`EventLoop::run_app`方法来处理事件。`run_app`会接受一个`ApplicationHandler`结构的可变引用，而这个实现了`ApplicationHandler`的结构则定义了事件具体如何被处理。

`ApplicationHandler`中有多个方法，每种方法对应了一类特定的事件。在其[文档](https://docs.rs/winit/latest/winit/application/trait.ApplicationHandler.html)中亦有记载。其中最特殊的两个，也是没有默认实现的两个，是`resumed`和`window_event`。

winit推荐我们在`resumed`中创建窗口和其他应用实例，这是因为在一些移动平台中（如Android），应用启动后还并不一定具备可以开始渲染内容的条件，而且也可能会在运行过程中丢失后重新获得渲染能力（如回到桌面屏幕再点开）。这些失去渲染能力再重新获得渲染能力的情况皆由`resumed`事件类型传递，因此我们应当在其中创建窗口和初始化渲染相关的内容。`resumed`在所有平台上都会至少在开始应用时被传递一次，因此我们在其中创建一个窗口即可。由于本教程并不打算支持移动平台，因此没有处理渲染能力丢失的情况。若有感兴趣的读者可以查看并试着处理`suspended`事件。在`resumed`中创建的`winit::window::Window`便是窗口的主体了，主要获取和操作窗口句柄，窗口大小，光标位置等，具体可以设置以及获取的数据请参见[Window的文档](https://docs.rs/winit/latest/winit/window/struct.Window.html)。值得一提的是`Window`实现了`HasRawWindowHandle`，所以可以直接将`&Window`和`Arc<Window>`等引用作为各类图形库的窗口句柄输入。为了后续在初始化 WGPU 时生命周期的处理，我们加上了一层`Arc`。

`window_event`则负责传递和窗口直接相关的各种事件，如大小改变，拖拽，失去聚焦，缩放率变化等。窗口的渲染请求也在其中处理。正如我们之前所说，一个winit应用可以拥有多个窗口，`window_event`会传入一个`WindowId`方便我们区分这些窗口。

> 如果你翻阅了事件相关的文档，那么你或许发现了`WindowEvent`和`DeviceEvent`里面都有键盘、鼠标相关的事件。值得注意的是，`DeviceEvent`获取到的事件是直接来源于系统外设的信号的，因而并不受窗口聚焦等的限制。在游戏中，我们更倾向于使用后者，而在应用中我们更倾向于使用前者。而如果你是Wayland用户，则`DeviceEvent`中根本不会收到键盘事件。

下一节，我们将会介绍如何处理窗口发送的事件，并且确定程序循环的主体。
