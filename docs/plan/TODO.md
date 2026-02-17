# TODO（骨架 → 血肉，低耦合高内聚）

> 这是“人类可执行”的 TODO 列表（优先级/依赖/验收/验证都在这里）。  
> 机器可读 DAG：`docs/plan/task_dag.json`。规格冻结：`docs/spec/mvp.md`。调研入口：`docs/research/README.md`。

## A. 全局规则（必须遵守，违反即返工）

- [ ] 分层不破：`ui-state-primitives`（纯状态）→ `ui-headless`（交互/A11y）→ `ui-components`（组件）→ `apps/*`（应用）；`ui-theme/ui-motion` 作为组件横向服务层
- [ ] 依赖单向：`ui-state-primitives/ui-theme/ui-motion` 不依赖上层；`ui-headless` 不依赖 `ui-components/ui-theme`
- [ ] `ui-state-primitives` 禁止 `web-sys` / DOM / 平台能力（保持可移植、可单测）
- [ ] `ui-components` 不直接碰 `web-sys`（一律通过 `ui-headless` 注入行为）
- [ ] `ui-headless` 的 DOM 交互必须 feature-gated（至少 `web`/`ssr`），且能 `wasm32-unknown-unknown` 编译
- [ ] `ui-components` 必须支持组件级 feature 切分（最小特性集可编译），禁止全组件中央注册表
- [ ] 对外 API “小而稳”：v0 先冻结公开 API；上层不透传下层内部结构体（避免耦合）
- [ ] 每个 TODO 都必须有 Stop Gate（可运行命令）；没过门禁不允许继续加功能

## B. Stop Gates（随时可跑）

- [ ] Gate A：`cargo fmt --all -- --check`
- [ ] Gate B：`cargo clippy --workspace --all-targets -- -D warnings`
- [ ] Gate C：`cargo test --workspace`
- [ ] Gate D（WASM 编译）：`cargo check -p ui-headless --target wasm32-unknown-unknown --no-default-features --features web`
- [ ] Gate E（WASM 编译）：`cargo check -p ui-components --target wasm32-unknown-unknown`
- [ ] Gate F（WASM 编译）：`cargo check -p web-demo --target wasm32-unknown-unknown`
- [ ] Gate F2（WASM 编译）：`cargo check -p docs-app --target wasm32-unknown-unknown`
- [ ] Gate G（SSR 编译）：`cargo check -p ui-headless --no-default-features --features ssr`
- [ ] Gate H（组件级裁剪）：`cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-button,component-input,inject-css`
- [ ] Gate I（CSS 聚合回归，当前实现）：`cargo test -p ui-components --test css`
- [ ] 说明：WASM Gates 需要安装 `wasm32-unknown-unknown` target（推荐 `rustup target add wasm32-unknown-unknown`）；临时可用 `SKIP_WASM=1 ./scripts/check.sh`

## 0) 冻结输入（不做这个会反复返工）

- [x] `t00` 复核并更新 `docs/spec/mvp.md`（把默认 Button→Popover 换成你的真实流程也行）
- [x] 确认 demo 目录策略：
  - [x] 默认：可提交 demo 放 `apps/`；research mirror放 `examples/_upstream/` 且不提交
  - [ ] 若坚持 `examples/` 也要提交 demo：调整 `.gitignore` 仅忽略 `examples/_upstream/`
- [x] `t03` 确保 `docs/plan/task_dag.json` 与 `docs/plan/TODO.md` 同步（计划变化必须同步）

## 1) 工程骨架（Workspace → crates → apps）

### 1.1 Workspace 与目录结构

- [x] `t01` 创建 workspace（根 `Cargo.toml`）
- [x] `t01` 创建目录：
  - [x] `crates/ui-state-primitives`
  - [x] `crates/ui-headless`
  - [x] `crates/ui-theme`
  - [x] `crates/ui-motion`
  - [x] `crates/ui-components`
  - [x] `crates/ui-compat`
  - [x] `apps/web-demo`
  - [x] `apps/docs-app`
  - [x] `apps/tauri-demo`（Phase 2，可先占位）
- [x] `t01` 为每个 crate 建立最小 `src/lib.rs`（只导出占位模块，先不实现逻辑）
- [x] `t01` 统一 crate metadata（edition、license、repository、publish = false（先不开源发布））

**Stop Gate**
- [ ] `cargo check --workspace`

### 1.2 依赖矩阵（低耦合护栏）

- [ ] 固化依赖关系（写入各 `Cargo.toml`，禁止循环）：
  - [ ] `ui-state-primitives`：无内部依赖
  - [ ] `ui-theme`：无内部依赖
  - [ ] `ui-headless`：可依赖 `ui-state-primitives`（可选），不可依赖 `ui-components/ui-theme`
  - [ ] `ui-motion`：无内部依赖
  - [ ] `ui-components`：仅依赖 `ui-headless` + `ui-theme`（必要时再依赖 `ui-state-primitives`，但优先不依赖）
  - [ ] `ui-compat`：可依赖 `ui-headless/ui-components`，但核心分层不得反向依赖
  - [ ] `apps/*`：依赖 `ui-components`（可间接使用 headless/theme）

**Stop Gate**
- [ ] `cargo check --workspace`

## 2) 门禁与开发体验（把质量变成默认）

- [x] `t02` 新增 `scripts/check.sh`：顺序跑 Gate A→B→C→D→E→F（失败即退出）
- [ ] （可选）新增 `rust-toolchain.toml` 固定 toolchain（降低环境差异）
- [ ] （可选）新增 `scripts/ci.sh`（CI 用；本地同样可跑；当前仓库尚未提供该脚本）
- [ ] （可选）添加 `.gitignore` 条目：`target/`（等 workspace 落地后再加）

**Stop Gate**
- [ ] Gate A
- [ ] Gate B
- [ ] Gate C

## 3) v0 公共 API 冻结（先写“接口”，再长“实现”）

> 目标：先把跨 crate 的接口边界写清楚，避免实现时互相“反向渗透”。

### 3.1 ui-state-primitives v0 API（纯状态）

- [ ] 冻结模块与导出（示例）：
  - [ ] `ui_state_primitives::toggle::{ToggleState, ToggleStateOptions, use_toggle_state}`
  - [ ] `ui_state_primitives::controlled::{use_controlled_state}`（如需要）
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

## 4) ui-state-primitives（Stately v0：先有血液循环）

目标：把“状态”做成可复用、可测试、无平台依赖的最小集。

### 4.1 ToggleState

- [x] `t10` 实现 `use_toggle_state`：
  - [ ] 支持受控/非受控（controlled/uncontrolled）
  - [ ] 支持 `is_read_only`
  - [ ] 提供 `toggle()` / `set_selected(bool)`
  - [ ] 明确默认值行为（对齐 React Stately 的 `defaultSelected` 思路）
- [x] `t10` 单测覆盖：只读不变更、受控回调被调用、非受控内部更新

**Stop Gate**
- [ ] `cargo test -p ui-state-primitives`

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

### 7.0 组件内部结构（ARCHITECTURE_ZH 风格：logic/styles/motion/view）

- [x] 新增 `ui-motion` crate（Web: WAAPI；SSR/no-op），作为“高级 motion（非 CSS）”的执行后端
- [x] `Button`：拆分为 `logic.rs` / `styles.rs` / `motion.rs` / `view.rs`
- [x] `<UiRoot>`：统一注入组件 CSS（先从 Button 开始）
- [x] 其余组件迁移到相同结构（Checkbox/Switch/Overlay/Popover/Modal/Menu/ListBox/Select/MenuTrigger）
- [x] 清理 `ui-components` 的 inline style（改为 `styles.rs` + `data-*` + CSS variables）

### 7.1 Button v0

- [x] `t30` 实现 `<Button>`：
  - [x] props：`disabled`, `variant`（先少量）, `on_press`
  - [x] 内部消费 `ui_headless::use_button` 并显式挂载 handlers/attrs
  - [x] focus-visible 状态驱动 class（不要在 headless 里写样式）
  - [x] 禁止把 `ui-headless` 的内部结构体透传给 app

**Stop Gate**
- [ ] `cargo check -p ui-components --target wasm32-unknown-unknown`

### 7.1.1 组件级裁剪（Tree Shaking）v0

- [ ] 新增 `ui-components` 组件级 features（至少 `component-button`、`component-input`、`component-select`、`component-overlay` 样例 + `all-components`）
- [ ] `lib.rs` 模块与 re-export 按 feature 条件编译
- [ ] `css.rs` 聚合按 feature 条件拼接（只注入启用组件 CSS）
- [ ] 反模式清理：禁止全组件中央注册表导致所有组件可达
- [ ] 新增最小特性 CSS slicing 测试（当前仅有 `cargo test -p ui-components --test css` 的全量聚合回归）

**Stop Gate**
- [ ] Gate H
- [ ] Gate I

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

### 12.1 ui-state-primitives（集合/选择/受控工具）

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
- [x] typeahead（Menu/ListBox：500ms buffer + prefix match + loop；通过 `item_text`）
- [x] per-item disabled（Menu/ListBox：跳过 focus/activation/typeahead；通过 `is_item_disabled`）
- [x] `use_menu_item`（Action/Checkbox/Radio：role + aria-checked + handlers）
- [x] Overlay v2（v0）：`use_modal`（scroll lock + aria-hidden；排除 overlay portal）
- [ ] Overlay v2+：aria-hidden 进一步收敛到 topmost modal、inert/scroll lock 补全、嵌套 overlay 细节、Android Back 集成（History/Tauri 双通道）
- [ ] Overlay v2+：打开 topmost overlay 时 `pushState` 占位，避免 Back 直接触发路由回退/应用退出
- [ ] Overlay v2+：`popstate` 与 Tauri back 事件统一走 headless 关闭通道（先关闭 topmost overlay，再决定是否放行路由回退）
- [ ] Overlay v2+：关闭 overlay 后正确回收历史占位，避免污染正常浏览历史栈

### 12.3 ui-components（从原子到复合）

- [x] `ListBox`（v0：消费 `use_listbox`）
- [x] `Checkbox` / `Switch`（复用 toggle + press + focus）
- [ ] `IconButton`（Button 变体）
- [x] `Popover`（v0：基于 anchor rect 的定位；箭头/flip/scroll lock 后续）
- [x] `Dialog` / `Modal`（v0：`Modal` 组合组件 + aria-labelledby/aria-describedby）
- [x] `Menu` / `MenuTrigger`（v0：基于 Popover v0；typeahead + disabled items + checkbox/radio items；`close_on_action` 可选）
- [x] `Select`（v0：Button + Popover + ListBox，选择后自动关闭）
- [ ] `ComboBox`（Input + ListBox + Overlay）
- [ ] `Tooltip`（hover/focus + overlay）

### 12.4 ui-theme（Spectrum tokens/多主题）

- [ ] 从 `adobe-spectrum-css` 提取 tokens → 生成 CSS variables（自动化脚本）
- [ ] dark theme（对齐 Spectrum dark）
- [ ] density/scale（pointer: coarse vs fine 的 spacing/size 策略）

### 12.5 平台覆盖（Tauri/Android）

- [ ] Tauri：窗口/标题栏/系统菜单等差异适配策略文档化
- [ ] Android：safe-area + 输入法遮挡 + back 手势/物理键 行为规范化（与 Overlay history/back 契约打通）

## 13) Philosophy v1.0 全特性对齐矩阵（逐条提及，不得漏项）

> 对齐基线：`docs/philosophy.md`。  
> 执行规则：每次里程碑评审必须逐条过本节；新增能力必须先映射到本节条目再开工。

### 13.1 核心哲学特性（5.1-5.15）

- [ ] 5.1 分层不可破：`core -> headless -> components -> apps`，`theme/motion` 只横向服务
- [ ] 5.2 类型优先：关键状态由 enum/struct 建模，减少字符串语义与隐式约定
- [ ] 5.3 A11y 默认开启：键盘/焦点/ARIA 语义为默认能力，不做可选补丁
- [ ] 5.4 语义优先样式：稳定 `data-*`/`aria-*` 标记 + 语义测试优先于视觉快照
- [ ] 5.5 Motion 合同化：contract 在组件，执行在 `ui-motion`，遵守 reduced-motion 与 SSR/no-wasm 降级
- [ ] 5.6 发布可用与开发效率并重：保证可发布质量的同时保持可迭代速度
- [ ] 5.7 质量可执行：完成定义绑定门禁命令，不接受口头完成
- [ ] 5.8 文档即产品：规格、计划、示例、docs app 与实现同步
- [ ] 5.9 i18n/l10n 默认纳入：文案可覆盖，格式化可策略化，注入点稳定可测
- [ ] 5.10 数据可视化兼容：可扩展到 `ui-charts`，并保留语义/A11y 降级路径
- [ ] 5.11 混合分发：`core/headless/theme/motion` package-first，`components` source-first
- [ ] 5.12 生态位目标：Leptos-first，同时保持 `ui-state-primitives/ui-headless` 可迁移潜力
- [ ] 5.13 贡献者可成长：低门槛上手路径、局部任务切入、样例与清单齐备
- [ ] 5.14 样式哲学：token-first + `styles.rs` 静态契约，应用层可用 utility 但不反向污染组件契约
- [ ] 5.15 可裁剪交付：组件级 feature + CSS 同步裁剪，禁止破坏 DCE 的全局可达反模式

### 13.2 AI 原生特性（6.1-6.12）

- [ ] 6.1 战略假设落地：以 Rust 类型系统 + 编译反馈降低 AI 幻觉成本
- [ ] 6.2 类型化落地：`logic.rs` 归一化、enum 限定、source markers、语义测试
- [ ] 6.3 人/Agent 共用契约：统一可解析语义，不依赖脆弱 DOM 猜测
- [ ] 6.4 编译反馈闭环：最小变更 -> 编译/门禁 -> 诊断修复 -> 循环
- [ ] 6.5 流式基建：结构流/状态流/结果流，支持中途纠偏与断流恢复
- [ ] 6.6 Config -> Component 双通道：运行时直出 + 编译时固化并可追溯
- [ ] 6.7 Agent Contract 版本化：`schema_version`、兼容窗口、迁移说明
- [ ] 6.8 诊断适配器：`cargo/rustc` 输出结构化，供 Agent 直接消费
- [ ] 6.9 受限行为生成：Action 白名单执行器，不默认执行任意脚本
- [ ] 6.10 能力协商：版本区间 + capability negotiation，失败可降级可诊断
- [ ] 6.11 双层校验：预检（轻量）+ 重检（门禁）分层
- [ ] 6.12 策略配置：Fetch 白名单与 SetState 作用域约束，默认拒绝高风险能力

### 13.3 实施方法特性（7.1-7.10）

- [ ] 7.1 默认交付法：先目标/非目标、先薄切片、先门禁再扩面
- [ ] 7.2 组件模板：`logic/view/styles/motion/mod` 模板化并自动化脚手架
- [ ] 7.3 API 演进：命名统一、受控/非受控成对、默认值单源
- [ ] 7.4 AI 流水线：指令 -> 流式 Spec -> schema 校验 -> 预览 -> 语义检查 -> 固化 -> 门禁
- [ ] 7.5 DX 最后一公里：样式热重载、组件热开发、状态保持、workbench
- [ ] 7.6 生态胶水层：`serde`、`tracing`、async 解耦边界清晰
- [ ] 7.7 全局状态扩展：组件状态与应用状态分层，桥接层显式化
- [ ] 7.8 测试金字塔：单测/集成/E2E 分层，WASM 场景可重复回归
- [ ] 7.9 贡献工具链：脚手架、决策树、PR 自检模板
- [ ] 7.10 异步契约：统一 loading/error/aria-busy 语义，数据层通过适配层协作

### 13.4 质量执行特性（8.1-8.5）

- [ ] 8.1 “好变更”定义固化：架构/行为/A11y/测试/维护/可解释六维同达标
- [ ] 8.2 关键改动清单固化：层级、命名、无效状态、语义、分支覆盖、文档同步、门禁通过
- [ ] 8.3 反模式阻断：跨层污染、view 隐藏决策、API 泄漏、临时补丁破坏一致性
- [ ] 8.4 可观测性：关键状态/事件可追踪可回放，开发开关与生产隔离
- [ ] 8.5 性能与内存剖析：关键组件预算、profiling workbench、回归可阻断

### 13.5 治理与生态特性（12.1-12.4）

- [ ] 12.1 社区治理：贡献路径、提案模板、决策机制、争议收敛流程
- [ ] 12.2 受控逃生舱口：显式命名、显式风险、显式作用域、默认关闭、可审计
- [ ] 12.3 ADR 判例化：关键架构决策可追溯（含被否决方案）
- [ ] 12.4 商业/社区平衡：核心开源透明，商业能力不反向锁死核心演进

### 13.6 路线对比约束（10.1-10.8）

- [ ] 10.1 避免“单体组件库”退化路径
- [ ] 10.2 避免“样式先行、语义后补”的不可维护路线
- [ ] 10.3 避免“运行时灵活优先”导致的语义漂移与晚期错误暴露
- [ ] 10.4 不做 Spectrum API 表层 1:1 盲目克隆
- [ ] 10.5 抑制 HeroUI 式参数爆炸，保持语义底盘稳定
- [ ] 10.6 不走“纯聊天生成”单路径，必须结构化 + 可校验 + 可固化
- [ ] 10.7 不走“只有刚性规则无治理通道”的路线
- [ ] 10.8 不走“仅包分发”或“仅源码分发”极端路线，维持混合分发策略

### 13.7 成功指标（13）

- [ ] 门禁通过率稳定（fmt/clippy/test/ssr/wasm）
- [ ] 语义/A11y 回归率下降
- [ ] 组件 API 命名与状态契约一致性提高
- [ ] 文档覆盖与 demo 覆盖持续提升
- [ ] 新组件接入不破坏分层秩序
- [ ] 外部贡献者首个 PR 通过率与平均合并时长改善
- [ ] 关键组件性能预算达标率稳定
- [ ] AI 生成链路首轮修复成功率提升

### 13.8 当前务实重点（14，逐条落地）

- [ ] 强化状态与来源语义契约
- [ ] 推进代表组件参数模型统一
- [ ] 组件优先级按 P0/P1 场景驱动
- [ ] 渐进增强 Agent 可消费语义（Schema 化）
- [ ] 固化混合分发默认路径（底层 package + 组件 source）
- [ ] 建立 `UiSpec` 最小可用 schema 与白名单解释渲染链路
- [ ] 建立流式生成阶段状态协议（生成中/已校验/可提交）
- [ ] 建立 `UiSpec -> Rust` 最小固化通路（先覆盖 Button/Select/Overlay）
- [ ] 落地 AI 行为沙箱最小动作集（`Validate/Fetch/Navigate/SetState`）
- [ ] 落地沙箱策略配置（Fetch 白名单 + SetState 作用域）
- [ ] 设计 Agent 能力协商协议与握手流程
- [ ] 建立“预检 + 重检”校验链路原型
- [ ] 明确 i18n/l10n 注入契约（文本/数字/日期）
- [ ] 固化样式默认范式（token-first + `styles.rs`）并定义例外边界
- [ ] 建立组件开发 workbench（props 矩阵 + 状态矩阵 + 热重载）
- [ ] 提供组件脚手架与贡献者决策树
- [ ] 建立异步 action 原语与数据层适配边界
- [ ] 设计诊断适配器原型（`cargo` 输出 -> 结构化 JSON）
- [ ] 定义 `tracing` 语义与核心 span/event 规范
- [ ] 明确 async runtime 解耦边界与 adapter
- [ ] 定义测试金字塔落地清单与 E2E 选择器规范
- [ ] 启动性能/内存 profiling workbench 与预算基线
- [ ] 建立 `render_count` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据
- [ ] 定义 Agent Contract 版本策略与迁移流程
- [ ] 起草贡献与治理文档（Contributing + RFC 模板）
- [ ] 建立 ADR 模板与目录规范
- [ ] 设计 escape hatches 白名单与风险标注规范
- [ ] 在不破边界前提下持续扩展组件能力
