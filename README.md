# Native Bevy with Tauri HUD DEMO

将 Tauri 的透明背景渲染窗口实时叠在 Bevy 的渲染窗口上，以使用原生级别性能运行游戏！

Let Tauri's transparent background rendering window be stacked on Bevy's rendering window in real time to run the game with native-level performance!

尽管 Tauri 自身可以通过 WebGL2 运行 WASM 版本的 Bevy，但可能性能不太如人意，于是就有了这个实验。

Although Tauri itself can run the WASM version of Bevy through WebGL2, the performance may not be satisfactory, so this experiment was born.

该项实验还可用于其他老游戏的 HUD 升级，通过将 Tauri 叠上动态定位的游戏窗口，以获得更好的操作体验。

This experiment can also be used to upgrade the HUD of other old games, by stacking Tauri on the dynamically positioned game window to get a better operation experience.

这只是一个实验性 DEMO，本人不对其可用性负责。

This is just an experimental DEMO, and I am not responsible for its availability.

## 开始使用 / Setup

```bash
cargo build -p _tauri
cargo run -p _bevy
```
