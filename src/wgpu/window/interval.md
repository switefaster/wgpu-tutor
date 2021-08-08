# 太快了！

控制恒定的更新速度是游戏中常见的需求，这一节我们将介绍一种简单的控制更新频率的方法。

玩过 __Minecraft™__ 吗？Minecraft中的游戏逻辑循环是以20次/秒的频率进行的，称为 20 __tps(_ticks per second_)__，而渲染循环却以不同的频率（通常为60个画面（称为 __帧__）每秒，称为60 __fps(_frames per second_)__）刷新。较低的逻辑更新频率可以减少计算的负担，而更高的渲染频率则可以使游戏显得更加流畅。如果更新的速度不均匀，则可能使游戏显示的画面有违和的跳动。那么，我们应该如何控制更新的频率呢？

考虑这样一个问题，我们希望一个循环以60次每秒的速度进行更新，那么每次循环的平均用时应当为 $\tau_{0} = \frac {1} {60} sec = 0.016sec$，现在有两种情况:

- 这次循环的用时比$\tau_{0}$少，那我们自然可以等到时间为$T + \tau_{0}$，时再开始下一次循环。其中$T$是上一次循环结束的时间戳
- 如果这次循环用时比$\tau_{0}$多呢？不妨假设这一次循环所耗的时间为$(\alpha + 1)\tau_{0}$我们整整缺少了$\lfloor \alpha \rfloor$次循环！一种简单的解决方案是，当场补上这$\alpha$次循环的更新并且直接进入下一个循环，而补上的循环的更新间隔$\delta \tau = \tau_0$。这样可以一定程度上保证更新的正确性，并且如果后续更新没有再消耗大于$\tau_{0}$的时间的话也能顺利地接上

现在，我们来实现一下

```rust
use std::time::{Duration, Instant};

const TPS: f64 = 60.0;
const TIMEOUT: f32 = 3.0;

fn update(dt: Duration) {
    println!("delta tau: {:?}", dt);
}

fn main() {
    let tau = Duration::from_secs_f64(1.0 / TPS);
    let timeout = Duration::from_secs_f32(TIMEOUT);
    let begin = Instant::now();
    let mut now = Instant::now();
    let mut lag = Duration::ZERO;

    loop {
        let dt = now.elapsed();
        now = Instant::now();
        lag += dt;
        while lag > tau {
            update(tau);
            lag -= tau;
        }
        if begin.elapsed() > timeout {
            break;
        }
    }
}

```

是不是非常简单呢？唯一值得注意的一点是我们并不用`sleep`来控制间隔，而不终止循环的运行。因此我们可以有不同的频率同时运行。而主循环内部就是运行的最快的部分了。

```rust
use std::time::{Duration, Instant};

const TPS_0: f64 = 60.0;
const TPS_1: f64 = 20.0;
const TIMEOUT: f32 = 3.0;

fn update(dt: Duration) {
    println!("delta tau: {:?}", dt);
}

fn main() {
    let tau_0 = Duration::from_secs_f64(1.0 / TPS_0);
    let tau_1 = Duration::from_secs_f64(1.0 / TPS_1);
    let timeout = Duration::from_secs_f32(TIMEOUT);
    let begin = Instant::now();
    let mut now = Instant::now();
    let mut lag_0 = Duration::ZERO;
    let mut lag_1 = Duration::ZERO;

    loop {
        let dt = now.elapsed();
        now = Instant::now();
        lag_0 += dt;
        lag_1 += dt;
        while lag_0 > tau_0 {
            update(tau_0);
            lag_0 -= tau;
        }
        while lag_1 > tau_1 {
            slow_update(tau_1);
            lag_1 -= tau;
        }
        fast_update(dt);
        if begin.elapsed() > timeout {
            break;
        }
    }
}

```

放进 `EventLoop::run` 中也是同理，快去试试吧！
