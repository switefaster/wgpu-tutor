# 程序，窗口和循环

在我们开始激动人心的WGPU之旅前，我们需要为其准备一个舞台，也就是窗口。写过其他图形应用程序的读者应该知道，不同平台对窗口初始化的方式千差万别，而句柄的差异又让后续的图形库初始化难以统一。在Rust中，多亏了[winit](https://github.com/rust-windowing/winit)和[RawWindowHandle](https://github.com/rust-windowing/raw-window-handle)的包装，用户得以避免为每个平台写一份代码的悲伤状况(甚至支持Android和iOS)。在本章节中，我们将介绍使用winit初始化窗口和控制循环的方式。
