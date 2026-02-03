# Motion 规格（v0）

目标：在不破坏 `ui-core / ui-headless / ui-components / ui-theme` 分层的前提下，原生提供“高级 motion”（非 CSS transition/animation）。

## 分层约束（必须遵守）

- `ui-core`：纯状态/状态机；禁止 motion（无 DOM）。
- `ui-headless`：交互与 A11y；**不做视觉表现**，也不做动画编排（只输出状态/handlers/attrs）。
- `ui-theme`：tokens（可包含 motion tokens：duration/easing/spring 参数），但不绑定具体实现。
- `ui-components`：组件实现与“视觉表达”；组件内部可定义 motion contract（例如 `ButtonMotion`），但不要把 motion 逻辑塞进 `ui-headless`。
- `ui-motion`：motion 引擎/后端（Web/未来其它平台），负责把 contract 变成真正的动画执行。

## v0 现状（已落地）

- 新增 crate：`crates/ui-motion`
  - Web backend（`wasm32`）：通过调用 Web Animations API（WAAPI）执行 keyframes（绕过 web-sys unstable API 限制，直接反射调用 `element.animate(...)`）。
  - `prefers-reduced-motion`：检测到 reduce 时跳过动画。
- `ui-components::Button`
  - 模块拆分：`logic.rs` / `styles.rs` / `motion.rs` / `view.rs`
  - `styles.rs`：只产出 CSS（静态），由 `<UiRoot>` 统一注入
  - `motion.rs`：暴露 `ButtonMotion`（v0），默认 press 反馈走 WAAPI（无 CSS transitions）

## API 形状（v0→v1 演进方向）

- **Component Motion Contracts（组件层）**
  - 每个组件在自己的 `motion.rs` 定义：
    - `XxxMotion`：对外可选配置（默认值合理）
    - `attach_motion(...)`：把 hook 状态与 DOM ref 连接到 motion 引擎（web 下启用，SSR/no-op）
- **Motion Engine（ui-motion）**
  - v0：keyframes + options → WAAPI（web）
  - v1：补齐 spring/sequence/presence 等更高层能力（仍保持 contract 在组件层）

## TODO（下一步）

- 把其他组件逐个迁移到 `logic/styles/motion/view`：
  - `Popover`（open/close/presence）
  - `Overlay/Modal`（enter/exit + focus trap 的视觉表现）
  - `Select/Menu/ListBox`（highlight/selection 的 motion）
- `ui-theme` 增加 motion tokens（durations/easings/springs），让“动效参数”可主题化。
- `ui-motion` 增加 spring（物理）实现（Web：rAF 或生成 keyframes），以及可组合的 timeline/sequence API。

