# Android Spike（Tauri + Android System WebView）

> 目标：在“尽量不引入原生 UI 框架”的前提下，验证 `Leptos CSR + Tauri(Android)` 作为容器是否能满足交互与 A11y 的最低要求，并列出阻塞点与 go/no-go 决策。

## 结论（当前）

- 当前代码层面：`ui-headless` 的 Press/FocusVisible/Overlay/focus-trap 都是基于标准 DOM 事件与 `env(safe-area-inset-*)` 思路，可在 Android WebView 上“理论可行”，但必须做一次真实设备验证。
- 关键风险不在渲染，而在 **输入法/视口** 与 **触摸交互差异**（尤其是 overlay + focus trap + click outside）。

## 假设与范围

- Android 端采用 **Tauri v2 mobile**，UI 跑在 **Android System WebView**（不是原生 Compose/View）。
- MVP 行为只要求：
  - Button 的 Press 语义在触摸下不重复触发；
  - FocusVisible（键盘/指针）在触摸设备上合理降级；
  - Overlay：Esc（如有硬键盘）+ 点击外部关闭 + Tab 不逃逸（硬键盘场景）。

## 必测项（按优先级）

### P0：输入事件一致性（Press）

验证点：

- `pointerdown/pointerup/click` 在 WebView 上的触发顺序与是否重复。
- 长按是否触发额外 click / context menu（部分设备/ROM 可能有差异）。
- 滚动手势下的 pointercancel 是否可靠触发（用于 pressed 状态复位）。

建议记录：

- 事件序列日志（按一次、长按、滑动取消、双击）。
- 是否出现“pointer + click 双触发”的 on_press 重复。

### P0：Overlay 关闭策略（click outside + back）

验证点：

- 点击蒙层是否可靠触发（某些 WebView 可能会把 click 合并/延迟）。
- Android 系统 Back（手势/实体键）如何与 Overlay 关闭联动（是否需要在 Tauri 层拦截并向 WebView 发事件）。

### P0：软键盘与视口（VisualViewport）

验证点：

- 聚焦输入框时，软键盘弹出是否导致 overlay panel 被遮挡。
- `position: fixed` 的 overlay 在键盘弹出时是否抖动/错位。

建议策略（后续实现）：

- 基于 `window.visualViewport` 监听 resize/scroll，给 overlay 容器设置 bottom offset。
- 对话框/Popover 的布局避免依赖 100vh，优先用 `dvh/svh`（若可用）或 JS 修正高度。

### P1：Safe Area（刘海/圆角/沉浸式）

验证点：

- Android WebView 是否支持 `env(safe-area-inset-*)`（取决于 WebView 版本与是否 `viewport-fit=cover`）。
- Tauri Android 是否默认全屏沉浸；是否需要额外 CSS 类 `.safe-area` 包裹。

建议策略：

- 基础样式提供 `.safe-area`（`ui-theme::css::SAFE_AREA_CSS` 已提供）。
- 需要时在 Tauri/Android 端注入 `<meta name="viewport" content="viewport-fit=cover">`（需验证 Tauri 2 mobile 的默认模板）。

### P1：FocusVisible / FocusTrap 在触摸设备的体验

验证点：

- 触摸点击后 focus ring 是否被错误显示（应尽量不显示）。
- 连接硬键盘时 Tab/Shift+Tab 是否可用；overlay trap 是否能把焦点留在 panel 内。

## 实施步骤（建议）

1) 安装工具链
- Android Studio + SDK/NDK
- `rustup` + `wasm32-unknown-unknown`
- `tauri-cli`（v2）

2) 运行桌面壳（基线）
- `cd apps/tauri-demo`
- `cargo tauri dev`

3) Android 初始化与运行（需要在本机完成）
- `cd apps/tauri-demo`
- `cargo tauri android init`
- `cargo tauri android dev`

4) 设备测试矩阵（最小）
- 1 台真机（Android 12+）
- 1 台模拟器（同版本）
- 1 个“系统 WebView 版本较旧”的设备/模拟器（用于压力测试）

## Go / No-Go 决策

### Go 条件

- Press 在触摸下无重复触发；pressed 状态能在 cancel 时复位。
- Overlay click outside 可靠关闭；back 键可选但建议可关闭 topmost overlay。
- 软键盘弹出时 overlay 不会严重错位（允许 v0 先做 JS 修正）。

### No-Go 阻塞

- WebView 事件序列导致 Press 无法稳定去重（必须引入复杂的 touch/pointer 兼容层仍不稳定）。
- 软键盘导致 fixed overlay 无法可靠布局（需要大量原生侧改造）。
- FocusTrap 在硬键盘场景不可用且无法修复（影响可访问性底线）。

## 后续任务（落回 TODO）

- `ui-headless`: Press 增强（pointer/touch/click 更完整的去重策略）。
- `ui-headless`: Overlay v2（back 关闭、scroll lock、嵌套 overlay、aria-hidden）。
- `ui-theme`: 默认加入 safe-area 支持（按平台开关）。

