# 实施计划（Implementation Plan）

> 状态：v1（可执行、可扩展）。主线：先 MVP 验证交互闭环，再扩展 Overlay/Tauri/Android。

关联文档：

- 目标与愿景：`docs/起点_也即是目的.md`
- 上游调研定位：`docs/research/README.md`
- MVP 规格（冻结后 t00 完成）：`docs/spec/mvp.md`
- 执行 TODO（骨架→血肉）：`docs/plan/TODO.md`
- 机器可读 Task DAG：`docs/plan/task_dag.json`
- Subagent 任务卡模板：`docs/plan/SUBAGENT_TASK_TEMPLATE.md`

## 0. 输入与约束（必须冻结）

### 0.1 必要输入（未确认前，按默认值推进）

- MVP 核心用户流程（默认）：`Button -> 打开 Popover -> 选择/关闭`（覆盖 Press/FocusVisible/Overlay v1）
- Demo 位置（默认）：可提交 demo 放 `apps/`；`examples/` 仅放 research mirror（已被 `.gitignore` 忽略）

### 0.1.1 先决条件（开发者环境）

- Rust stable（建议配合 rustup）
- `wasm32-unknown-unknown` target（用于编译验证）
  - `rustup target add wasm32-unknown-unknown`

### 0.2 非目标（Phase 1 不做）

- Android 原生能力桥接（haptics 等）
- Spectrum 全量组件与全量样式
- 复杂动画系统/手势动画
- 多窗口/多 overlay 容器/iframe 追踪（仅单 window）
- 完整的 A11y 合规（先满足“可聚焦 + 键盘可操作 + 基本 aria”）

### 0.3 Definition of Done（DoD）

满足以下全部条件才算 Phase 1 完成：

- Workspace 中存在核心五层 `ui-state-primitives/ui-headless/ui-theme/ui-motion/ui`，并且边界清晰
- 存在可提交 demo（`apps/web-demo`），能展示：
  - Button 的 pressed/disabled/focus-visible 状态
  - Overlay v1（Popover 或 Modal）可被打开/关闭（Esc + 点击外部）
- `cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo test --workspace` 全绿
- `ui-headless/ui/web-demo/docs-app` 至少能 `wasm32-unknown-unknown` 编译通过（不要求跑浏览器）

### 0.4 Stop Gates（每个里程碑必须过）

- Gate A（随时可跑）：`cargo fmt --all -- --check`
- Gate B（随时可跑）：`cargo clippy --workspace --all-targets -- -D warnings`
- Gate C（随时可跑）：`cargo test --workspace`
- Gate D（WASM 编译验证）：
  - `cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web`
  - `cargo check -p ui --target wasm32-unknown-unknown`
  - `cargo check -p web-demo --target wasm32-unknown-unknown`
  - `cargo check -p docs-app --target wasm32-unknown-unknown`
- Gate E（SSR 编译验证）：
  - `cargo check -p ui-headless --no-default-features --features ssr`
- Gate F（混合分发兼容验证）：
  - 在底层 package 版本矩阵下，抽样 source 组件可编译（至少 `Button` / `Select` / `Overlay`）
  - 若版本不兼容，CI 必须输出明确失败原因与迁移指引链接
- Gate G（组件级裁剪验证）：
  - `ui` 在最小特性集下可编译（示例：`component-button,component-input`）
  - 组件 CSS 聚合结果只包含已启用特性的样式（无 `select/modal/chart` 泄漏）

## 1. 仓库结构（目标态）

```text
.
├── Cargo.toml
├── crates
│   ├── ui-state-primitives
│   ├── ui-headless
│   ├── ui-theme
│   ├── ui-motion
│   ├── ui
├── apps
│   ├── web-demo
│   ├── docs-app
│   └── tauri-demo            # Phase 2+
├── docs
│   ├── plan
│   ├── research
│   └── spec
└── scripts
```

约定：

- `ui-state-primitives`：纯状态（禁止 `web-sys`、禁止 DOM 假设）
- `ui-headless`：交互/A11y（允许 `web-sys`，但要有 feature gate）
- `ui-theme`：tokens + CSS variables（不依赖 `ui`）
- `ui-motion`：动效执行后端（web/ssr 分支都可编译）
- `ui`：Leptos 组件（不直接使用 `web-sys`，通过 `ui-headless` 间接接触 DOM）

## 2. Feature 策略（先简单、可演进）

- `ui-headless`
  - `default-features = ["web"]`
  - `feature = "web"`：启用 `web-sys`/事件绑定（WASM/CSR）
  - `feature = "ssr"`：编译可过（必要时用空实现/降级），避免在 SSR 链路爆炸

> 目标：让 `web` 与 `ssr` 两种构建模式都能稳定编译（分别 check），避免 `--all-features` 混合模式导致 Leptos csr/ssr 冲突。

## 2.1 Public API 稳定性规则（v0）

- `ui-state-primitives`：只导出“纯状态模型/状态机”，不导出任何 DOM/渲染相关类型。
- `ui-headless`：只导出“交互/可访问性模型 + Leptos 可挂载的 handlers/attrs”，不导出具体 UI 样式与 class 名。
- `ui`：只导出组件与其 props；组件对外只暴露稳定 props，不透传 headless 的内部结构体（避免锁死后续重构）。
- `ui-theme`：只导出 tokens 与生成的 CSS（变量名先冻结为 v0，后续增量扩展）。

## 2.2 分发模型（默认）

采用混合分发（Hybrid Distribution）：

- Package 分发：`ui-state-primitives` / `ui-headless` / `ui-theme` / `ui-motion`
- Source 分发：`ui`（按需拉取组件源码，shadcn-like）

约束：

- source 组件必须声明并遵守底层 package 的支持版本区间。
- 底层 package 升级必须提供迁移说明或自动迁移工具。
- 不采用“全层源码拷贝”作为默认路径，避免用户依赖爆炸。

## 2.3 组件级特性切分与裁剪（目标）

`ui` 采用“组件级 feature + 条件 CSS 聚合”策略。

要求：

- 组件或组件族按 feature 切分（如 `component-button`、`component-input`、`component-overlay`）。
- 提供 `all-components` 便利特性给 docs/demo，全量场景不破坏开发效率。
- `default-features = false` 时支持最小特性集编译（按需点菜）。
- `inject-css` 只控制注入行为，不得等价于“全量组件样式注入”。
- 禁止引入全组件中央注册表，避免破坏 DCE/LTO。

## 3. Concept Alignment Inputs (research-only)

已整理于 `docs/research/README.md`，重点链路：

- Press：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/interactions/src/usePress.ts`
- FocusVisible：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/interactions/src/useFocusVisible.ts`
- Button：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/button/src/useButton.ts`
- Overlay：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/overlays/src/useOverlay.ts`
- Leptos Portal：`examples/_upstream/leptos/leptos/src/portal.rs`
- leptos-use click-outside：`examples/_upstream/leptos-use/src/on_click_outside.rs`

## 4. Task DAG（可验证任务拆分）

> 每个任务都必须有明确输出 + 验证命令；不要在一个任务里引入多个不相关的不确定性。

### 并行组（建议）

- G1（串行）：t00 → t01 → t02
- G2（可并行）：t10 与 t11
- G3（串行）：t20-pre → t20 → t21 → t22
- G4（可并行）：t30 与 t31

### t00 - 冻结 MVP 规格（spec）

- 目标：写清楚 Phase 1 的做/不做与验收
- 输出：`docs/spec/mvp.md`
- 依赖：无
- 验收：
  - 1 页以内（强制简洁）
  - 至少 10 条可勾选 DoD checklist
- 验证命令：无（文档任务）

### t01 - Workspace 与 crate 壳

- 目标：建立 Rust workspace + 核心分层 crate（含 `ui-motion`）+ `apps/web-demo` 壳
- 输出：
  - `Cargo.toml`（workspace）
  - `crates/*` 目录与最小 `lib.rs`
  - `apps/web-demo` 最小 crate
- 依赖：t00
- 验收：`cargo check --workspace`
- 验证命令：
  - `cargo check --workspace`

### t02 - 门禁（fmt/clippy/test）

- 目标：把 “质量门禁” 固化为标准命令（必要时提供脚本/justfile）
- 输出：`scripts/check.sh`（或 `justfile`，二选一）
- 依赖：t01
- 验收：
  - `cargo fmt --all -- --check` 通过
  - `cargo clippy --workspace --all-targets -- -D warnings` 通过
  - `cargo test --workspace` 通过
- 验证命令：同上

### t03 - Task DAG 固化（机器可读）

- 目标：把 DAG 结构固化成 JSON，方便后续生成 subagent 任务卡/CI 矩阵
- 输出：`docs/plan/task_dag.json`
- 依赖：t00
- 验收：字段齐全（id/depends_on/verify 等），并与本文保持一致
- 验证命令：无（文档任务）

### t10 - ui-state-primitives：最小状态原语（Stately v0）

- 目标：提供 1-2 个纯状态 hook（优先 toggle），为组件层提供稳定 API
- 输出：`ui-state-primitives::toggle::{use_toggle_state, ToggleState}`
- 依赖：t01
- 验收：`cargo test -p ui-state-primitives`
- 验证命令：
  - `cargo test -p ui-state-primitives`

### t11 - ui-theme：tokens + CSS variables（v0）

- 目标：定义最小 tokens（color/radius/space），导出一份基础 CSS 变量
- 输出：`ui-theme`（tokens 模型 + 生成/拼接 CSS 的最小机制）
- 依赖：t01
- 验收：`cargo test -p ui-theme`
- 验证命令：
  - `cargo test -p ui-theme`

### t20-pre - ui-headless：feature gating（web/ssr）

- 目标：先把 `ui-headless` 的平台边界固定下来（web/ssr），避免后续实现时被迫大改
- 输出：`ui-headless` 的 Cargo features（默认 `web`；提供 `ssr` 降级）
- 依赖：t01
- 验收：
  - `cargo check -p ui-headless`（默认 web）
  - `cargo check -p ui-headless --no-default-features --features ssr`
  - `cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web`
- 验证命令：同上

### t20 - ui-headless：Interaction Modality / FocusVisible（v0）

- 目标：全局监听输入事件，推断 modality（keyboard/pointer/virtual），暴露 `is_focus_visible`
- 输出：`ui-headless::focus_visible` 模块 + `provide_*`/`use_*` API
- 依赖：t20-pre（先定边界再写实现）
- 验收：
  - `cargo check -p ui-headless`（默认 web）
  - `cargo check -p ui-headless --no-default-features --features ssr`
  - `cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web`
- 验证命令：同上

### t21 - ui-headless：Press（v0）

- 目标：统一 pointer/mouse/keyboard 的 press 语义，输出 `is_pressed` + handlers
- 输出：`ui-headless::press` 模块 + `use_press`
- 依赖：t20（同一交互体系，先定 modality 再定 press 的策略）
- 验收：同 t20
- 验证命令：同 t20

### t22 - ui-headless：use_button（v0）

- 目标：组合 `use_press + focusable + aria-*`，并处理非 button 元素的键盘触发
- 输出：`ui-headless::button::use_button`
- 依赖：t21
- 验收：同 t20
- 验证命令：同 t20

### t30 - ui：Button v0

- 目标：实现 `<Button>` 组件，消费 `ui-headless::use_button` 与 `ui-theme` tokens
- 输出：`ui::Button`
- 依赖：t22 + t11
- 验收：
  - `cargo check -p ui`
  - `cargo check -p ui --target wasm32-unknown-unknown`
- 验证命令：同上

### t31 - apps/web-demo：演示页 v0

- 目标：能看见 Button 的 states（disabled/pressed/focus-visible），并能触发 1 个 overlay
- 输出：`apps/web-demo`
- 依赖：t30
- 验收：
  - `cargo check -p web-demo --target wasm32-unknown-unknown`
- 验证命令：同上

### t32 - ui：组件级 feature 裁剪（v0）

- 目标：让 package 模式具备可验证的 Tree Shaking 能力（按现有特性命名与测试现状）
- 输出：
  - `ui` 的组件级 features（至少 `component-button`、`component-input`、`component-overlay/component-select` 样例）
  - 条件化 `lib.rs` 模块导出与 re-export
  - 条件化 CSS 聚合（仅拼接启用组件样式）
- 依赖：t30（先有稳定 Button 再抽 feature 边界）
- 验收：
  - `cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-button,component-input,inject-css`
  - `cargo test -p ui --test css`（当前仓库可执行的 CSS 聚合回归）
  - `cargo check -p docs-app --target wasm32-unknown-unknown`（`all-components` 场景不回归）
- 验证命令：同上

### t40 - Overlay v1（Popover 或 Modal）

- 目标：最小 overlay：Portal + topmost + Esc + click-outside + focus trap(v0)
- 输出：`ui::Overlay`（或 Popover/Modal）
- 依赖：t22（Esc/交互策略）+ t31（有 demo 驱动）
- 验收：
  - `cargo check -p ui --target wasm32-unknown-unknown`
  - demo 增加一个 popover/modal 示例（编译验证）
- 验证命令：同上

### t50 - Tauri 桌面 demo（Phase 2）

- 目标：引入 `apps/tauri-demo` 壳，复用 `web-demo` 的 wasm 资源
- 依赖：t31（先有 web demo 再谈 tauri）
- 验收：`cargo check -p tauri-demo`

### t60 - Android spike（Phase 3）

- 目标：验证 WebView 行为差异（safe-area / 输入法 / pointer events）
- 输出：`docs/research/android-spike.md`（结论 + 阻塞点 + 决策）
- 依赖：t50

## 5. 风险清单（预防性约束）

- FocusVisible 判定不稳定（不同平台 focus 事件差异）
  - 缓解：先实现 “键盘输入 → 显示 focus ring；pointerdown → 隐藏” 的最小策略，再补 virtual modality
- Press 事件重复触发（touch/pointer/click 合并问题）
  - 缓解：MVP 先仅支持 pointer events（优先），其他事件路径只做兜底
- Overlay 复杂度爆炸（z-index/portal/focus trap）
  - 缓解：Overlay v1 只支持单实例 + topmost；第二个场景再抽象
