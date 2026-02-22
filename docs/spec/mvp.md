# MVP 规格（Phase 1）

> 默认 MVP：`Button -> 打开 Popover -> 选择/关闭`。如果你有更真实的流程，替换本文件即可。

## Goal

交付一套可编译验证、可演示的最小 “Rust版 React Aria/Stately” 基础设施（FocusVisible + Press + Button + Overlay v1），并验证其在 Leptos/WASM 下可用。

## Non-Goals（本阶段不做）

- Android 原生桥接（haptics、系统 API）
- Spectrum 全量 tokens/样式与全量组件
- 复杂动画/手势系统
- 多窗口/多 overlay 容器/iframe
- 完整 A11y 合规（仅保证基本键盘可操作与关键 aria）

## MVP 用户故事

- 作为用户，我可以用鼠标点击 Button 打开一个 Popover，并在点击外部或按 Esc 后关闭它。
- 作为键盘用户，我可以用 Tab 聚焦 Button，并用 Enter/Space 触发打开/关闭；焦点环只在键盘交互时显示。

## DoD Checklist（必须全通过）

### 工程与门禁

- [ ] Workspace 至少包含核心五层 crate：`ui-state-primitives/ui-headless/ui-theme/ui-motion/ui`
- [ ] 存在可提交 demo：`apps/web-demo`
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] `./scripts/check-rust-hygiene.sh`（非测试代码禁 `unwrap/expect`、禁 `let _ =`、字符串克隆热点收敛至 `Cow<'static, str>`）

### WASM 编译验证（不要求跑浏览器）

- [ ] `cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web`
- [ ] `cargo check -p ui --target wasm32-unknown-unknown`
- [ ] `cargo check -p web-demo --target wasm32-unknown-unknown`

### 行为验收（由 web-demo 体现）

- [ ] Button：`disabled` 时不触发 press；`aria-disabled/disabled` 语义正确
- [ ] Press：支持 pointer（鼠标）与 keyboard（Enter/Space）两条路径；不会重复触发
- [ ] FocusVisible：键盘交互后显示 focus ring；pointer 交互后不显示
- [ ] Overlay v1：
  - [ ] Esc 关闭
  - [ ] 点击外部关闭
  - [ ] 打开时聚焦在 overlay 内（focus trap v0 可接受：只保证 Tab 不逃逸）
