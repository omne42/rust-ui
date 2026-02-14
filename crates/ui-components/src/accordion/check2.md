# 单组件 Check List（完整版，Spectrum 对齐）

> 用途：每次新增或改动一个组件时，按本清单逐项检查。  
> 执行顺序：`大骨架 -> 小骨架 -> 实现细节 -> 测试与文档 -> 合并门禁`。

### 0. 适用范围与顺序纪律
- [x] 本清单仅评估“一个组件”的改动结果，不替代仓库级治理。
- [x] 先过第 1-2 节（骨架）再进入第 3-6 节（实现细节）。
- [x] 发现跨组件/跨层系统性问题时，升级为仓库级任务，不在组件内打补丁。
- [x] 组件目标、非目标、风险边界已写清楚。

### 1. 大骨架（架构边界与层职责）
- [ ] `ui-core` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。
- [ ] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），输出 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。
- [ ] `ui-motion` 定义：动效引擎与契约执行层（spring、keyframes、WAAPI backend）。不关心业务组件语义；非 wasm 提供 no-op/stub，保证 SSR/tooling 可编译。
- [ ] `ui-theme` 定义：设计 token 与主题层（Light/Dark/OLED）+ CSS 变量生成。只输出 theme/tokens/base css，不做组件 CSS。
- [ ] `ui-components` 定义：最终 Leptos 组件层，组合 headless + motion + theme。对外 API 不暴露 `web-sys` 类型；DOM/wasm 细节放内部模块。
- [ ] 依赖方向正确：无反向依赖、无跨层偷用、无重复造轮子。
- [ ] Spectrum 三轴一致：组件明确映射 `system/color/scale`，不引入私有命名体系。

### 2. 小骨架（API 设计检查 + 状态管理检查）
- [ ] API 命名统一：`is_*`、`on_*`、`default_*`。
- [ ] 受控/非受控成对设计：`value + on_value_change` 对应 `default_value`，`open + on_open_change` 对应 `default_open`。
- [ ] 默认值来源单一：统一在 `logic.rs` 归一，不在 `view.rs` 分散决策。
- [ ] 状态管理语义清晰：用类型化状态输入与 `logic.rs` 归一化替代分散 view 逻辑。
- [ ] 离散状态用 `enum`（variant/size/mode 等）约束输入空间，避免多个 `Option<bool>` 拼状态机。
- [ ] `ui-core` 提供可组合状态原语，不强制单一全局状态框架；应用层全局状态接入需通过桥接层，不让组件直接绑定业务 store。
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
- [ ] `crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）。
- [ ] `crates/ui-components/src/css.rs`：聚合组件 CSS（`push_components_css`）。
- [ ] `crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars + components css。
- [ ] `crates/ui-components/src/active_highlight.rs`：高亮条样式与 motion driver。
- [ ] `crates/ui-components/src/overlay_open.rs`：若存在则用于 open-state 辅助；当前仓库已迁移到 `ui-headless`（`crates/ui-headless/src/controllable_state.rs`），需在组件中通过 headless API 使用。
- [ ] `crates/ui-components/src/presence.rs`：若存在则用于生命周期；当前仓库 presence 原语在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。
- [ ] `crates/ui-components/src/a11y.rs`：若存在则放共享 A11y 小工具；当前共享实现位于 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`）。
- [ ] `mod.rs`：最小稳定导出面（存在且无过度导出）。
- [ ] `logic.rs`：props 归一化、派生状态、来源标记（存在）。
- [ ] `styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`（存在）。
- [ ] `view.rs`：纯 Leptos view 结构渲染（存在，禁止 `render.rs` 漂移）。
- [ ] `motion.rs`：`XxxMotion + attach_motion`（交互组件必须有）。
- [ ] `spec.rs`：仅极少数组件专用（当前主要 button）。

### 6. AI 原生能力（Agent Contract + 流式）
- [ ] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。
- [ ] 流式能力作为 AI 原生特征而非可选增强：结构流（Spec/Config）、状态流（规划/校验/可预览/可提交）、结果流（增量挂载）。
- [ ] 流式输出可恢复（断流重连不破坏会话状态）、可验证（分片不绕过契约校验）、可标识（草稿/已验证/可提交）。

### 7. 测试与文档（验证闭环）
- [ ] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。
- [ ] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。
- [ ] 关键流程纳入可重复回归集合（Playwright/Cypress）。
- [ ] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。
- [ ] 变更说明完整：风险点、兼容性影响、验证命令与结果。

### 8. Spectrum 五项硬基线（组件级必过，含文件落点）
- [ ] 基线 1 Token 统一基线：定义 `crates/ui-theme/src/tokens.rs`，映射 `crates/ui-theme/src/theme.rs`，变量输出 `crates/ui-theme/src/css.rs`，组件消费 `crates/ui-components/src/<component>/styles.rs`。
- [ ] 基线 2 三轴上下文：`system/color/scale` 定义于 `crates/ui-theme/src/theme.rs`，组件在 `logic.rs` 选择并在 `view.rs` 生效。
- [ ] 基线 3 Token 分类可追溯：分类源 `crates/ui-theme/src/tokens.rs`，规范文档 `docs/spec/styling.md`，组件映射落点 `styles.rs`。
- [ ] 基线 4 量化尺寸可回归：尺寸基准定义在 `tokens.rs`，映射在 `theme.rs`，主题回归测试建议 `crates/ui-theme/tests/token_scale_baseline.rs`，组件语义回归 `crates/ui-components/tests/<component>_semantics.rs`。
- [ ] 基线 5 A11y + i18n/l10n：A11y 契约在 `crates/ui-headless/src/a11y.rs`，组件挂载在 `view.rs`，文案与本地化落点 `i18n.rs`（按需新增），语义测试与 E2E 分别在 `crates/ui-components/tests/*` 与 `e2e/tests/*`。

### 9. 明确禁止的反模式
- [ ] 在 `ui-core` 写 DOM/样式逻辑。
- [ ] 在 `ui-headless` 写视觉和动画编排。
- [ ] 在 `view` 层隐藏关键状态决策。
- [ ] 新增参数但不纳入统一命名与契约。
- [ ] 公共 API 泄露底层实现细节类型。
- [ ] 用临时补丁破坏跨组件一致性。

### 10. 合并门禁（最终裁决）
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
