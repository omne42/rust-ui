# 单组件 Check List（完整版，Spectrum 对齐）

> 用途：每次新增或改动一个组件时，按本清单逐项检查。  
> 执行顺序：`大骨架 -> 小骨架 -> 实现细节 -> 测试与文档 -> 合并门禁`。  
> 术语统一：`status-primitives` 指状态原语层（当前 crate 名为 `ui-state-primitives`）。

### 0. 适用范围与顺序纪律
- [ ] 本清单仅评估“一个组件”的改动结果，不替代仓库级治理。
- [ ] 先过第 1-2 节（骨架）再进入第 3-6 节（实现细节）。
- [ ] 组件目标、非目标、风险边界已写清楚；发现跨组件/跨层系统性问题时升级为仓库级任务。

### 1. 大骨架（架构边界与层职责）
- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。
  **组件检查硬规则（必须执行）**：
  - 所有状态原语必须从 `status-primitives`（`ui-state-primitives`）获取，组件层只能消费，不得自造。
  - 下沉判定依据是“稳定状态不变量”；凡属于状态机、归一化、状态派生能力，默认先进入 `ui-state-primitives`。
  - 组件中可保留的仅是装配逻辑：props 归一、样式来源标记、slot 组织、对 `ui-state-primitives` 输出的映射。
  - 组件内若出现状态原语实现（受控/非受控状态机、single/multiple 展开规则、索引归一化、跨事件状态派生），该项直接判不通过。
  - 处理方式固定：先下沉到 `ui-state-primitives/src/<capability>.rs`（如 `expansion.rs`），在 `ui-state-primitives/src/lib.rs` 导出，再回到组件改调用。
  - 下沉后的原语必须有 `ui-state-primitives` 单元测试；组件侧只保留调用与语义挂载测试。
- [ ] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。
  **`ui-headless` 落位硬规则（必须执行）**：
  - 输入边界：消费 `status-primitives` 状态 + 用户输入事件（keyboard/pointer/focus）+ 环境能力（web/ssr）。
  - 输出边界：只输出语义契约（attrs/handlers/state）；组件层只负责挂载与组合，不得把语义判断塞回 `view.rs`。
  - 下沉判定依据是“交互/A11y 语义契约”；凡属于键盘/焦点/指针归一、ARIA 映射、交互状态语义能力，默认先进入 `ui-headless`。
  - 必须下沉：键盘模型、焦点模型、跨设备输入归一、ARIA 状态映射、overlay/presence 等交互语义。
  - A11y 契约与共享工具落点固定在 `crates/ui-headless/src/a11y.rs`；组件只在 `view.rs` 挂载，不在组件层重写。
  - 语义契约必须提供 `lang` / `dir`（LTR/RTL）接入能力；headless 不硬编码用户可见文本，文案由 i18n/l10n 层提供。
  - 语义契约正确性必须有回归：`crates/ui-components/tests/*` 断言语义标记，`e2e/tests/*` 覆盖关键交互流程。
  - 禁止放在 `ui-headless`：视觉 class 选择、CSS 规则、组件 slot 布局、组件专属动效编排、业务文案。
  - 允许留在组件层：纯视觉一次性交互且不形成可复用语义契约（例如单组件局部微交互）。
- [ ] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。
  **组件检查硬规则（必须执行）**：
  - 放在 `crates/ui-motion`：通用动画数学与执行后端（spring solver、keyframe sampling、easing registry、driver adapters），以及 `wasm/non-wasm` 适配与 `reduced-motion` 执行策略。
  - 放在 `crates/ui-components/src/<component>/motion.rs`：把组件语义状态（open/closed、enter/exit、active/inactive）映射为 `ui-motion` contract，绑定目标节点并调用 attach。
  - 禁止放在 `crates/ui-motion`：组件 slot 结构、组件专属状态机、ARIA/keyboard 语义、业务文案与业务分支。
  - 禁止放在组件 `motion.rs`：自实现 spring/keyframe/driver 执行器；跨组件共享动效算法必须回迁 `ui-motion`。
  - 动效参数优先来自 token/theme；禁止在组件样式与逻辑中散落硬编码时长/曲线/位移常量。
  - 非 wasm 路径必须提供 no-op/stub，保证 SSR/tooling 可编译且行为可预测。
- [ ] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。
  **组件检查硬规则（必须执行）**：
  - Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量；组件只在 `crates/ui-components/src/<component>/styles.rs` 消费。
  - 三轴上下文（`system/color/scale`）在 `theme.rs` 定义；组件在 `logic.rs` 选择并在 `view.rs` 生效，`styles.rs` 只消费变量，不重建主题。
  - Token 分类必须可追溯：分类源在 `tokens.rs`，规范同步 `docs/spec/styling.md`；组件不得引入平行私有 token 命名体系。
  - 量化尺寸基准必须可回归：尺寸基准在 `tokens.rs` 与 `theme.rs` 定义，主题回归在 `crates/ui-theme/tests/token_scale_baseline.rs`，组件语义回归在 `crates/ui-components/tests/<component>_semantics.rs`。
  - 主题调色与语义色对比必须满足 `WCAG 2.1 AA` 基线，并覆盖 Light/Dark/OLED 主题变体。
  - 主题层只输出 `theme/tokens/base css` 与变量；不实现组件结构、交互逻辑、组件级动效编排。
  - 新增视觉语义先补 token，再由组件消费；禁止“组件临时值先落地、后补 token”的倒序流程。
- [ ] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。
  **组件检查硬规则（必须执行）**：
  - `logic.rs` 负责 props 归一与状态派生；`view.rs` 负责结构渲染与 headless 语义挂载；`styles.rs` 负责 token-first 静态样式；`motion.rs` 负责动效 attach。
  - 组件层不得重写 `status-primitives` 状态机或 `ui-headless` 交互契约；发现即判不通过并回迁到对应层。
  - 对外 API 禁止暴露 `web-sys`/DOM 细节类型；平台差异封装在内部模块。

### 2. 小骨架（API 设计检查 + 状态管理检查）
- [ ] API 命名统一：`is_*`、`on_*`、`default_*`。
- [ ] 受控/非受控成对设计：`value + on_value_change` 对应 `default_value`，`open + on_open_change` 对应 `default_open`。
- [ ] 默认值来源单一：统一在 `logic.rs` 归一，不在 `view.rs` 分散决策。
- [ ] 状态管理语义清晰：用类型化状态输入与 `logic.rs` 归一化替代分散 view 逻辑。
- [ ] 离散状态用 `enum`（variant/size/mode 等）约束输入空间，避免多个 `Option<bool>` 拼状态机。
- [ ] `status-primitives`（`ui-state-primitives`）提供可组合状态原语，不强制单一全局状态框架；应用层全局状态接入需通过桥接层，不让组件直接绑定业务 store。
- [ ] 组件库异步交互模式统一：`is_loading`、error/retry、`aria-busy`、禁用态语义一致；鼓励可复用抽象（如 `use_async_action` 方向），避免每组件一套协议。

### 3. 实现细节（A11y / i18n-l10n / 可观测 / 样式与动效）
- [ ] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。
- [ ] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。
- [ ] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。
- [ ] 测试验证“语义契约”而不只验证视觉快照。
- [ ] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。
- [ ] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。
- [ ] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。
- [ ] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。
- [ ] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。

### 4. SSR / 跨平台 / WASM / 性能 / 工程能力
- [ ] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。
- [ ] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。
- [ ] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。
- [ ] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。
- [ ] 性能治理进入常规门禁：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。
- [ ] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。
- [ ] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。
- [ ] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。

### 5. 文件落点检查（必须提及）
- [ ] `ui-components` 固定入口文件落点正确。
  **组件检查硬规则（必须执行）**：
  - `crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。
  - `crates/ui-components/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。
  - `crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。
  - `crates/ui-components/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。
  - `crates/ui-components/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-state-primitives/src/controlled.rs`（以及 `overlay_trigger.rs` 等封装原语），组件通过 status-primitives API 消费；`ui-headless` 只负责交互/A11y 语义契约。
  - `crates/ui-components/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。
  - `crates/ui-components/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。
- [ ] 组件目录标准文件落点正确。
  **组件检查硬规则（必须执行）**：
  - `<component>/mod.rs`：最小稳定导出面，存在且无过度导出。
  - `<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。
  - `<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。
  - `<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。
  - `<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。
  - `<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。

### 6. AI 原生能力（Agent Contract + 流式）
- [ ] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。
- [ ] 流式能力作为 AI 原生特征而非可选增强：结构流（Spec/Config）、状态流（规划/校验/可预览/可提交）、结果流（增量挂载）。
- [ ] 流式输出可恢复（断流重连不破坏会话状态）、可验证（分片不绕过契约校验）、可标识（草稿/已验证/可提交）。

### 7. 测试与文档（验证闭环）
- [ ] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。
- [ ] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。
- [ ] 关键流程纳入可重复回归集合（Playwright/Cypress）。
- [ ] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。
- [ ] example 中 heroui 文档需要对标，组件文档必须存在。

### 8. 明确禁止的反模式
- [ ] 在 `status-primitives`（`ui-state-primitives`）写 DOM/样式逻辑。
- [ ] 在 `ui-headless` 写视觉和动画编排。
- [ ] 在 `view` 层隐藏关键状态决策。
- [ ] 新增参数但不纳入统一命名与契约。
- [ ] 公共 API 泄露底层实现细节类型。
- [ ] 用临时补丁破坏跨组件一致性。
- [ ] 明明是跨组件可复用状态原语，却长期留在某个组件 `logic.rs` 不下沉。

### 9. 合并门禁（最终裁决）
- [ ] 架构正确（边界不破）。
- [ ] 行为正确（状态与交互语义成立）。
- [ ] 可访问性达标（默认可用）。
- [ ] 可测试（契约可断言）。
- [ ] 可维护（命名和模式一致）。
- [ ] 可解释（人和自动化都能读懂）。
- [ ] 改动在正确层。
- [ ] 命名与全库一致。
- [ ] 无效状态被限制或归一化。
- [ ] 暴露必要语义标记。
- [ ] 覆盖 reduced-motion / SSR / wasm 分支。
- [ ] 文档与示例同步更新。
- [ ] 门禁完整通过（fmt/clippy/test/smoke 等）。
