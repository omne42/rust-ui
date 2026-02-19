# Motion 规格（v0）

目标：在不破坏 `ui-state-primitives / ui-headless / ui-components / ui-theme` 分层的前提下，原生提供“高级 motion”（非 CSS transition/animation）。

## 分层约束（必须遵守）

- `ui-state-primitives`：纯状态/状态机；禁止 motion（无 DOM）。
- `ui-headless`：交互与 A11y；**不做视觉表现**，也不做动画编排（只输出状态/handlers/attrs）。
- `ui-theme`：tokens（可包含 motion tokens：duration/easing/spring 参数），但不绑定具体实现。
- `ui-components`：组件实现与“视觉表达”；组件内部可定义 motion contract（例如 `ButtonMotion`），但不要把 motion 逻辑塞进 `ui-headless`。
- `ui-motion`：motion 引擎/后端（Web/未来其它平台），负责把 contract 变成真正的动画执行。

## v0 现状（已落地）

- 新增 crate：`crates/ui-motion`
  - Web backend（`wasm32`）：
    - **WAAPI**：`ui_motion::web::animate(...)`（keyframes/options → `element.animate(...)`）。
    - **Spring runtime**：`ui_motion::spring::SpringAnimator`（rAF 驱动；stiffness/damping/mass/precision）。
    - **Spring presets**：`ui_motion::presets::*`（对齐 bb 的 motion token 风格：fast/soft/slide/flip3d）。
  - `prefers-reduced-motion`：reduce 时跳过/降级到直接 set 目标值。
- `ui-components`（已迁移的组件）
  - `Button` / `Checkbox` / `Switch`：hover/tap 等交互反馈默认走 spring（按 bb 的手感参数）。
  - `Overlay` / `Popover`：enter/exit 通过 spring 驱动 `opacity/scale/translate`；通过 `on_exit_complete` 与上层 presence 解耦。
  - `ListBox` / `Menu` / `Select`：active highlight 使用 spring 驱动（类似 HeroUI/Framer 的“跟手高亮”）。

## API 形状（v0→v1 演进方向）

- **Component Motion Contracts（组件层）**
  - 每个组件在自己的 `motion.rs` 定义：
    - `XxxMotion`：对外可选配置（默认值合理）
    - `attach_motion(...)`：把 hook 状态与 DOM ref 连接到 motion 引擎（web 下启用，SSR/no-op）
  - 列表类组件可复用统一的 highlight contract：
    - `ui_visual_primitive::active_highlight::{ActiveHighlightMotion, attach_active_highlight_motion}`
- **Motion Engine（ui-motion）**
  - v0：keyframes + options → WAAPI（web）
  - v0：补齐 spring runtime（rAF）与 presets（tokens）
  - v1：补齐 sequence/timeline/layout(FLIP)/gesture 等更高层能力（仍保持 contract 在组件层）

## TODO（下一步）

- 继续把剩余组件迁移到 `logic/styles/motion/view`（按组件逐个推进，保持对外 API 稳定）。
- 在 `ui-theme` 增加 motion tokens（durations/easings/springs/presets），让“动效参数”可主题化。
- `ui-motion` 增加更高层能力：
  - presence（统一 enter/exit 管理）
  - sequence/timeline
  - layout motion（FLIP / shared layout）
  - gesture/drag（pointer → physics）
