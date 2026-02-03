# TODO（骨架 → 血肉，低耦合高内聚）

> 这是“人类可执行”的 TODO 列表（优先级/依赖/验收/验证都在这里）。  
> 机器可读 DAG：`docs/plan/task_dag.json`。规格冻结：`docs/spec/mvp.md`。调研入口：`docs/research/README.md`。

## A. 全局规则（必须遵守，违反即返工）

- [ ] 分层不破：`ui-core`（纯状态）→ `ui-headless`（交互/A11y）→ `ui-components`（组件）→ `apps/*`（应用）
- [ ] 依赖单向：`ui-core` 不依赖任何其他 crate；`ui-theme` 不依赖 `ui-components`
- [ ] `ui-core` 禁止 `web-sys` / DOM / 平台能力（保持可移植、可单测）
- [ ] `ui-components` 不直接碰 `web-sys`（一律通过 `ui-headless` 注入行为）
- [ ] `ui-headless` 的 DOM 交互必须 feature-gated（至少 `web`/`ssr`），且能 `wasm32-unknown-unknown` 编译
- [ ] 对外 API “小而稳”：v0 先冻结公开 API；上层不透传下层内部结构体（避免耦合）
- [ ] 每个 TODO 都必须有 Stop Gate（可运行命令）；没过门禁不允许继续加功能

## B. Stop Gates（随时可跑）

- [ ] Gate A：`cargo fmt --all -- --check`
- [ ] Gate B：`cargo clippy --workspace --all-targets -- -D warnings`
- [ ] Gate C：`cargo test --workspace`
- [ ] Gate D（WASM 编译）：`cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web`
- [ ] Gate E（WASM 编译）：`cargo check -p ui-components --target wasm32-unknown-unknown`
- [ ] Gate F（WASM 编译）：`cargo check -p web-demo --target wasm32-unknown-unknown`
- [ ] Gate G（SSR 编译）：`cargo check -p ui-headless --no-default-features --features ssr`
- [ ] 说明：WASM Gates 需要安装 `wasm32-unknown-unknown` target（推荐 `rustup target add wasm32-unknown-unknown`）；临时可用 `SKIP_WASM=1 ./scripts/check.sh`

## 0) 冻结输入（不做这个会反复返工）

- [x] `t00` 复核并更新 `docs/spec/mvp.md`（把默认 Button→Popover 换成你的真实流程也行）
- [x] 确认 demo 目录策略：
  - [x] 默认：可提交 demo 放 `apps/`；upstream 参考放 `examples/_upstream/` 且不提交
  - [ ] 若坚持 `examples/` 也要提交 demo：调整 `.gitignore` 仅忽略 `examples/_upstream/`
- [x] `t03` 确保 `docs/plan/task_dag.json` 与 `docs/plan/TODO.md` 同步（计划变化必须同步）

## 1) 工程骨架（Workspace → crates → apps）

### 1.1 Workspace 与目录结构

- [x] `t01` 创建 workspace（根 `Cargo.toml`）
- [x] `t01` 创建目录：
  - [x] `crates/ui-core`
  - [x] `crates/ui-headless`
  - [x] `crates/ui-theme`
  - [x] `crates/ui-components`
  - [x] `apps/web-demo`
  - [x] `apps/tauri-demo`（Phase 2，可先占位）
- [x] `t01` 为每个 crate 建立最小 `src/lib.rs`（只导出占位模块，先不实现逻辑）
- [x] `t01` 统一 crate metadata（edition、license、repository、publish = false（先不开源发布））

**Stop Gate**
- [ ] `cargo check --workspace`

### 1.2 依赖矩阵（低耦合护栏）

- [ ] 固化依赖关系（写入各 `Cargo.toml`，禁止循环）：
  - [ ] `ui-core`：无内部依赖
  - [ ] `ui-theme`：无内部依赖
  - [ ] `ui-headless`：可依赖 `ui-core`（可选），不可依赖 `ui-components/ui-theme`
  - [ ] `ui-components`：仅依赖 `ui-headless` + `ui-theme`（必要时再依赖 `ui-core`，但优先不依赖）
  - [ ] `apps/*`：依赖 `ui-components`（可间接使用 headless/theme）

**Stop Gate**
- [ ] `cargo check --workspace`

## 2) 门禁与开发体验（把质量变成默认）

- [x] `t02` 新增 `scripts/check.sh`：顺序跑 Gate A→B→C→D→E→F（失败即退出）
- [ ] （可选）新增 `rust-toolchain.toml` 固定 toolchain（降低环境差异）
- [ ] （可选）新增 `scripts/ci.sh`（CI 用；本地同样可跑）
- [ ] （可选）添加 `.gitignore` 条目：`target/`（等 workspace 落地后再加）

**Stop Gate**
- [ ] Gate A
- [ ] Gate B
- [ ] Gate C

## 3) v0 公共 API 冻结（先写“接口”，再长“实现”）

> 目标：先把跨 crate 的接口边界写清楚，避免实现时互相“反向渗透”。

### 3.1 ui-core v0 API（纯状态）

- [ ] 冻结模块与导出（示例）：
  - [ ] `ui_core::toggle::{ToggleState, ToggleStateOptions, use_toggle_state}`
  - [ ] `ui_core::controlled::{use_controlled_state}`（如需要）
- [ ] 明确所有权与更新策略：状态由 core 持有；回调由调用者提供；不触碰 DOM

### 3.2 ui-headless v0 API（交互/A11y）

- [ ] 冻结模块与导出（示例）：
  - [ ] `ui_headless::modality::{Modality, provide_interaction_modality, use_interaction_modality}`
  - [ ] `ui_headless::focus_visible::{FocusVisibleState, provide_focus_visible, use_focus_visible}`
  - [ ] `ui_headless::press::{PressState, PressHandlers, use_press}`
  - [ ] `ui_headless::button::{ButtonOptions, ButtonAria, use_button}`
- [ ] 明确“返回物”的形状：返回 **handlers/attrs 结构体**，由组件层显式挂载（不做隐式 prop spread）
- [ ] 明确 SSR 降级：`ssr` 下不注册 window/document 监听，但 API 仍可编译（返回默认值）

### 3.3 ui-theme v0 API（tokens/CSS）

- [ ] 冻结模块与导出（示例）：
  - [ ] `ui_theme::tokens::{ColorTokens, RadiusTokens, SpaceTokens, ShadowTokens}`
  - [ ] `ui_theme::Theme`（包含 tokens + `to_css_variables()`）
  - [ ] `ui_theme::css::{BASE_CSS, SAFE_AREA_CSS(optional)}`

### 3.4 ui-components v0 API（组件）

- [ ] 冻结模块与导出（示例）：
  - [ ] `ui_components::Button`（props：`disabled`, `variant`, `on_press`）
  - [ ] `ui_components::Overlay`（或 `Popover`/`Modal` 二选一）
- [ ] 组件 props 只暴露稳定字段；不把 `ui-headless` 的内部 structs 直接暴露给 app

**Stop Gate**
- [ ] `cargo check --workspace`

## 4) ui-core（Stately v0：先有血液循环）

目标：把“状态”做成可复用、可测试、无平台依赖的最小集。

### 4.1 ToggleState

- [x] `t10` 实现 `use_toggle_state`：
  - [ ] 支持受控/非受控（controlled/uncontrolled）
  - [ ] 支持 `is_read_only`
  - [ ] 提供 `toggle()` / `set_selected(bool)`
  - [ ] 明确默认值行为（对齐 React Stately 的 `defaultSelected` 思路）
- [x] `t10` 单测覆盖：只读不变更、受控回调被调用、非受控内部更新

**Stop Gate**
- [ ] `cargo test -p ui-core`

## 5) ui-headless（React Aria v0：交互内核）

目标：先建立“交互内核”，不要先做 UI。

### 5.1 Feature 与边界（必须先完成）

- [x] `t20-pre` 定义 feature：
  - [x] `default-features = ["web"]`
  - [x] `web` 才能依赖 `web-sys`（或 leptos DOM 相关 API）
  - [x] `ssr` 下提供降级实现（保证能编译）

**Stop Gate**
- [ ] `cargo check -p ui-headless`（默认 web）
- [ ] `cargo check -p ui-headless --no-default-features --features ssr`
- [ ] `cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web`

### 5.2 Interaction Modality / FocusVisible（第一优先）

- [x] `t20` 实现全局 `Modality`（keyboard/pointer/virtual）：
  - [ ] 事件来源（MVP）：`keydown`, `pointerdown`, `click`, `focus`, `blur`
  - [ ] 最小策略：keydown → keyboard；pointerdown → pointer；无前置事件的 focus → virtual（可先简化）
  - [ ] 用 Leptos Context 提供/读取（`provide_*` / `use_*`）
- [x] `t20` `is_focus_visible`：由 modality 推导（keyboard/virtual → true；pointer → false）
- [x] `t20` 清理策略：组件卸载时移除全局监听（避免重复注册）

**Stop Gate**
- [ ] `cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web`

### 5.3 Press（第二优先）

- [x] `t21` 实现 `use_press`（MVP 先覆盖 keyboard + pointer）：
  - [x] 输出 `is_pressed`
  - [x] pointer 路径：按下→pressed=true；抬起→pressed=false；触发 `on_press`
  - [x] keyboard 路径：Enter/Space 触发（并避免页面滚动/默认行为）
  - [x] 防重复：避免 pointer/click 双触发（以 pointer 为主，click 只兜底）
  - [x] 支持 `disabled`（直接短路）
  - [x] `prevent_focus_on_press`（v0 可先占位，但要定义语义）

**Stop Gate**
- [ ] `cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web`

### 5.4 use_button（组合层）

- [x] `t22` 实现 `use_button`（组合 press + focusable + aria）：
  - [x] `<button>`：使用原生 `disabled/type`
  - [x] 自定义元素：提供 `role="button"`、`tabindex`、`aria-disabled`
  - [x] 键盘：Enter/Space 可触发 press（并满足基本 A11y）

**Stop Gate**
- [ ] `cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web`

## 6) ui-theme（tokens + CSS variables v0）

目标：把“样式数据”与“组件实现”解耦。

- [x] `t11` 定义 tokens（最小集）：`color/radius/space/shadow`
- [x] `t11` 提供 1 套 light theme（变量命名 v0 冻结）
- [x] `t11` 输出 base CSS variables（字符串常量/生成函数二选一）
- [x] （可选）safe-area CSS：作为可选片段导出，不强制所有组件启用

**Stop Gate**
- [ ] `cargo test -p ui-theme`

## 7) ui-components（Spectrum v0：组件实现）

目标：验证 headless + theme 的可组合性（先正确、再好看）。

### 7.1 Button v0

- [x] `t30` 实现 `<Button>`：
  - [x] props：`disabled`, `variant`（先少量）, `on_press`
  - [x] 内部消费 `ui_headless::use_button` 并显式挂载 handlers/attrs
  - [x] focus-visible 状态驱动 class（不要在 headless 里写样式）
  - [x] 禁止把 `ui-headless` 的内部结构体透传给 app

**Stop Gate**
- [ ] `cargo check -p ui-components --target wasm32-unknown-unknown`

### 7.2 Overlay v1（Popover/Modal 任选其一，先做一个）

- [x] `t40` 实现 Overlay v1（最小闭环）：
  - [x] Portal 渲染（Leptos `Portal`）
  - [x] topmost：只允许顶层 overlay 响应 Esc（维护最小 stack）
  - [x] Esc 关闭
  - [x] click-outside 关闭（可用 `leptos-use`）
  - [x] focus trap v0：Tab 不逃逸；关闭后把焦点还给触发元素（如可获取）

**Stop Gate**
- [ ] `cargo check -p ui-components --target wasm32-unknown-unknown`

## 8) apps/web-demo（可见的验证入口）

- [x] `t31` 选择构建方式（先选一个，避免反复切换）：
  - [x] 方案 A：trunk（CSR）
  - [ ] 方案 B：cargo-leptos
- [x] `t31` Demo 页面：
  - [x] 展示 Button states：disabled/pressed/focus-visible
  - [x] 展示 Overlay v1：打开/关闭（click/Esc/click-outside）
  - [x] 页面内写明“验收步骤”（按键操作说明）
  - [x] （新增）ListBox v0：Arrow keys + Enter/Space 选择（aria-activedescendant）

**Stop Gate**
- [ ] `cargo check -p web-demo --target wasm32-unknown-unknown`

## 9) Phase 2：Tauri desktop demo（可选，但建议尽早验证）

- [x] `t50` `apps/tauri-demo` 壳：
  - [x] 能编译通过（优先）
  - [x] 复用 web-demo 资源（后续再优化打包）

**Stop Gate**
- [ ] `cargo check -p tauri-demo`

## 10) Phase 3：Android Spike（可选）

- [x] `t60` 写 `docs/research/android-spike.md`：
  - [x] safe-area / 输入法 / pointer events 的差异结论
  - [x] go/no-go 决策与阻塞点（明确下一步要不要投入正式适配）

## 11) Phase 4：工程化（发布前再做，先占坑）

- [ ] 统一文档：每个 crate 有 1 个最小 README（目标/非目标/公共 API）
- [ ] 增量补齐：Overlay 的键盘可达性（更完整 focus trap、aria-hidden 等）
- [ ] 引入更严格的兼容测试（需要时才上 wasm-bindgen-test）

## 12) Backlog：向 React Spectrum 对齐（Phase 5+，按“先 core/headless 后 components”推进）

> 这些任务不属于 MVP/Phase 1，但用于保证路线图“完整”。每个条目在落地时都应拆成 tXX 级任务并加入 `task_dag.json`。

### 12.1 ui-core（集合/选择/受控工具）

- [x] `use_controlled_state` 完整化（支持 value/defaultValue/onChange 的通用模式）
- [x] `use_list_state`（items + selection，v0）
- [x] `use_single_selection_state` / `use_multiple_selection_state`
- [x] `use_overlay_trigger_state`（open/close/toggle + 受控/非受控）

### 12.2 ui-headless（交互/A11y 能力扩展）

- [x] `use_focus_ring`（组件级：focus/blur + 全局 focus-visible）
- [x] `use_focus_within`（容器级 focus 管理）
- [x] `use_hover`（pointer fine/coarse 的差异策略；移动端降级）
- [x] roving tabindex（Menu/ListBox/Toolbar 等键盘导航基础设施）
- [x] `use_listbox`（role/listbox/option/aria-activedescendant，v0）
- [x] `use_menu`（v0：aria-activedescendant + Arrow/Home/End 导航 + Enter/Space 激活）
- [ ] `use_menu_item`（含 typeahead、per-item disabled、checkbox/radio menu item）
- [ ] Overlay v2：aria-hidden 管理、scroll lock、嵌套 overlay、返回焦点策略完善

### 12.3 ui-components（从原子到复合）

- [x] `ListBox`（v0：消费 `use_listbox`）
- [x] `Checkbox` / `Switch`（复用 toggle + press + focus）
- [ ] `IconButton`（Button 变体）
- [x] `Popover`（v0：基于 anchor rect 的定位；箭头/flip/scroll lock 后续）
- [ ] `Dialog` / `Modal`（Overlay + focus trap 完整版）
- [x] `Menu` / `MenuTrigger`（v0：基于 Popover v0；typeahead/disabled items 后续）
- [ ] `Select`（Button + Popover + ListBox）
- [ ] `ComboBox`（Input + ListBox + Overlay）
- [ ] `Tooltip`（hover/focus + overlay）

### 12.4 ui-theme（Spectrum tokens/多主题）

- [ ] 从 `adobe-spectrum-css` 提取 tokens → 生成 CSS variables（自动化脚本）
- [ ] dark theme（对齐 Spectrum dark）
- [ ] density/scale（pointer: coarse vs fine 的 spacing/size 策略）

### 12.5 平台覆盖（Tauri/Android）

- [ ] Tauri：窗口/标题栏/系统菜单等差异适配策略文档化
- [ ] Android：safe-area + 输入法遮挡 + back 手势/物理键 行为规范化
