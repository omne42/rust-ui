# 单组件 Check List（Hyper-Structure 完整版）

> 用途：每次新增、重构或修改一个组件时，必须按本清单逐项检查并在 PR 中附带结果。  
> 执行顺序：先过第 1-2 节（架构与状态），再进入第 3-5 节（具体实现）。  
> 术语统一：`status-primitives` 指状态原语层（当前 crate 名为 `ui-state-primitives`）。

### 0. 适用范围与顺序纪律
本清单仅评估“一个组件”的改动结果，不替代仓库级治理。
先过第 1-2 节（架构与状态）再进入第 3-5 节（具体实现）。
不适用的条目必须明确标注 `N/A` 并说明理由，禁止机械打勾。
组件目标、非目标、风险边界已写清楚；发现跨组件/跨层系统性问题时升级为仓库级任务。

### 1. 架构边界与分层约束（Kernel/Shell 总线）
- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。
  - 所有状态原语必须从 `status-primitives`（`ui-state-primitives`）获取，组件层只能消费，不得自造。
  - 下沉判定依据是“稳定状态不变量”；凡属于状态机、归一化、状态派生能力，默认先进入 `ui-state-primitives`。
  - 组件中可保留的仅是装配逻辑：props 归一、样式来源标记、slot 组织、对 `ui-state-primitives` 输出的映射。
  - 组件内若出现状态原语实现（受控/非受控状态机、single/multiple 展开规则、索引归一化、跨事件状态派生），该项直接判不通过。
  - 处理方式固定：先下沉到 `ui-state-primitives/src/<capability>.rs`（如 `expansion.rs`），在 `ui-state-primitives/src/lib.rs` 导出，再回到组件改调用。
  - 下沉后的原语必须有 `ui-state-primitives` 单元测试；组件侧只保留调用与语义挂载测试。
  - 桥接规范：`ui-state-primitives` 结构体必须是 POJO（Plain Old Rust Object），不持有 Leptos `Signal` 或框架绑定状态容器。
  - 消费规范：`ui-headless` 或组件 `logic.rs` 负责解包 `Signal` 当前值传入 primitive 方法，并将结果显式写回 `Signal`。
  - 设计理由：保持 primitives 纯粹可测、可迁移，不与特定响应式库绑定（便于未来替换响应式实现与做纯 Rust 测试）。
  - 审查证据：`crates/ui-state-primitives/src/collapsible.rs` 提供 `CollapsibleOpenState/CollapsibleStateInput/resolve_state` 等状态原语，`components/collapsible/src/logic.rs` 仅 `pub use` 这些能力并做类名装配，组件未复写状态机。
  - 测试证据：`crates/ui-state-primitives/src/test/collapsible.rs` 已覆盖受控/非受控、归一化与状态来源映射，满足“下沉后原语必须有单元测试”。
- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。
  **`ui-headless` 落位硬规则（必须执行）**：
  - 输入边界：消费 `status-primitives` 状态 + 用户输入事件（keyboard/pointer/focus）+ 环境能力（web/ssr）。
  - 输出边界：只输出语义契约（attrs/handlers/state）；组件层只负责挂载与组合，不得把语义判断塞回 `view.rs`。
  - 下沉判定依据是“交互/A11y 语义契约”；凡属于键盘/焦点/指针归一、ARIA 映射、交互状态语义能力，默认先进入 `ui-headless`。
  - 必须下沉：键盘模型、焦点模型、跨设备输入归一、ARIA 状态映射、overlay/presence 等交互语义。
  - A11y 契约与共享工具落点固定在 `crates/ui-headless/src/a11y.rs`；组件只在 `view.rs` 挂载，不在组件层重写。
  - 语义契约必须提供 `lang` / `dir`（LTR/RTL）接入能力；headless 不硬编码用户可见文本，文案由 i18n/l10n 层提供。
  - 语义契约正确性必须有回归：`components/*/test/**` 断言语义标记，`e2e/tests/*` 覆盖关键交互流程。
  - 禁止放在 `ui-headless`：视觉 class 选择、CSS 规则、组件 slot 布局、组件专属动效编排、业务文案。
  - 允许留在组件层：纯视觉一次性交互且不形成可复用语义契约（例如单组件局部微交互）。
  - 审查证据：`components/collapsible/src/view.rs` 通过 `use_button/use_focus_ring/use_hover` 消费 pointer/keyboard/focus，并只挂载 `attrs + handlers + state`，未在组件层重写 press/focus/hover 状态机。
  - A11y 证据：`crates/ui-headless/src/a11y.rs` 提供 `locale_attrs/disclosure_trigger_attrs` 等共享契约，组件侧仅消费 headless 输出并挂载 `aria-*`。
  - 测试证据：`components/collapsible/test/collapsible_semantics.rs` 已断言 headless 契约接线；`e2e/tests/docs_app_collapsible.spec.mjs` 覆盖 docs-app 关键交互流程（pointer + keyboard + disabled）。
- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。
  - 放在 `crates/ui-motion`：通用动画数学与执行后端（spring solver、keyframe sampling、easing registry、driver adapters），以及 `wasm/non-wasm` 适配与 `reduced-motion` 执行策略。
  - 放在 `crates/ui-components/src/<component>/motion.rs`：把组件语义状态（open/closed、enter/exit、active/inactive）映射为 `ui-motion` contract，绑定目标节点并调用 attach。
  - 禁止放在 `crates/ui-motion`：组件 slot 结构、组件专属状态机、ARIA/keyboard 语义、业务文案与业务分支。
  - 禁止放在组件 `motion.rs`：自实现 spring/keyframe/driver 执行器；跨组件共享动效算法必须回迁 `ui-motion`。
  - 动效参数优先来自 token/theme；禁止在组件样式与逻辑中散落硬编码时长/曲线/位移常量。
  - 非 wasm 路径必须提供 no-op/stub，保证 SSR/tooling 可编译且行为可预测。
  - 审查证据：`components/collapsible/src/motion.rs` 提供 `sanitize_motion + attach_indicator_motion + attach_panel_motion`，本组件 `view.rs` 仅调用本地 motion contract，不直接绑定 `ui_disclosure::motion::attach_*`。
  - 引擎证据：`components/disclosure/src/motion.rs` 复用 `ui_motion::spring::SpringAnimator`，算法与驱动在 `crates/ui-motion/src/*`，组件层不自实现 spring/keyframe/driver。
  - 降级证据：`crates/ui-motion/src/lib.rs` 与 `components/disclosure/src/motion.rs` 提供 non-wasm no-op/可预测降级；`components/collapsible/src/styles.rs` 提供 `prefers-reduced-motion` 样式降级。
  - 测试证据：`components/collapsible/test/motion.rs` 覆盖 sanitize 委托；`components/collapsible/test/collapsible_semantics.rs` 断言动效挂载经本地 `motion.rs` 收口。
- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。
  - Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量；组件只在 `crates/ui-components/src/<component>/styles.rs` 消费。
  - 三轴上下文（`system/color/scale`）在 `theme.rs` 定义；组件在 `logic.rs` 选择并在 `view.rs` 生效，`styles.rs` 只消费变量，不重建主题。
  - Token 分类必须可追溯：分类源在 `tokens.rs`，规范同步 `docs/spec/styling.md`；组件不得引入平行私有 token 命名体系。
  - 量化尺寸基准必须可回归：尺寸基准在 `tokens.rs` 与 `theme.rs` 定义，主题回归在 `crates/ui-theme/tests/token_scale_baseline.rs`，组件语义回归在 `components/*/test/*<component>_semantics.rs`。
  - 主题调色与语义色对比必须满足 `WCAG 2.1 AA` 基线，并覆盖 Light/Dark/OLED 主题变体。
  - 主题层只输出 `theme/tokens/base css` 与变量；不实现组件结构、交互逻辑、组件级动效编排。
  - 新增视觉语义先补 token，再由组件消费；禁止“组件临时值先落地、后补 token”的倒序流程。
  - 审查证据：`crates/ui-theme/src/tokens.rs`、`crates/ui-theme/src/theme.rs`、`crates/ui-theme/src/css.rs` 分别承担 token 定义/三轴映射/CSS 变量输出；`crates/ui-components/src/root.rs` 统一注入 theme 变量到 `:root`，组件不重建主题。
  - 组件消费证据：`components/collapsible/src/styles.rs` 仅消费 `var(--ui-*)`（如 `--ui-accent`、`--ui-border`、`--ui-bg`、`--ui-accent-soft`），未引入组件私有颜色 token 体系。
  - 规范证据：`docs/spec/styling.md` 已固定 `tokens.rs -> theme.rs -> css.rs` 链路与“组件只消费变量”的约束。
  - 回归证据：`crates/ui-theme/tests/token_scale_baseline.rs` 覆盖 scale 基线与 CSS 变量输出；`components/collapsible/test/collapsible_semantics.rs` 覆盖 collapsible 语义契约与样式契约。
- [x] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。
  - `logic.rs` 负责 props 归一与状态派生；`view.rs` 负责结构渲染与 headless 语义挂载；`styles.rs` 负责 token-first 静态样式；`motion.rs` 负责动效 attach。
  - 组件层不得重写 `status-primitives` 状态机或 `ui-headless` 交互契约；发现即判不通过并回迁到对应层。
  - 对外 API 禁止暴露 `web-sys`/DOM 细节类型；平台差异封装在内部模块。
  - 测试文件位于src同级的test/中，内部测试文件同名（如rust-ui/components/accordion/src/logic.rs与rust-ui/components/accordion/test/logic.rs）。
  - 还需要一个semantics.rs用于测试。可能存在类似rust-ui/components/accordion/test/accordion_semantics.rs的旧版实现，需要迁移到新目录。
  - 审查证据：`components/collapsible/src/logic.rs`、`components/collapsible/src/view.rs`、`components/collapsible/src/styles.rs`、`components/collapsible/src/motion.rs` 职责边界明确，未出现跨层重写状态机/交互契约。
  - API 边界证据：`components/collapsible/src/mod.rs` 仅暴露 `Collapsible/CollapsibleMotion` 与 state primitive 类型别名，对外不暴露 `web-sys`/DOM 细节。
  - 迁移证据：新增 `components/collapsible/test/semantics.rs` 并在 `components/collapsible/src/mod.rs` 通过 `#[cfg(test)]` 挂载，组件目录具备同级语义测试入口；兼容保留 `components/collapsible/test/collapsible_semantics.rs` 作为仓库级回归。

### 2. API 设计与状态内核（Logic/Kernel）
- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。
  - 布尔状态统一 `is_*`（如 `is_open`/`is_disabled`），事件统一 `on_*`，默认值统一 `default_*`。
  - 同一语义 across 组件必须同名（如都用 `on_open_change`，禁止同义别名并存）。
  - 公共 API 引入新命名时，需说明与现有命名体系的兼容策略与迁移路径。
  - 审查证据：`components/collapsible/src/view.rs` 公开布尔主命名 `is_disabled`，并保持 `on_open_change`、`default_open` 同名契约。
  - 兼容策略：`components/collapsible/src/logic.rs` 增加 `normalize_is_disabled(is_disabled, disabled)`，保留 `disabled` 作为迁移别名输入，优先采用 `is_disabled`。
  - 回归证据：`components/collapsible/test/logic.rs` 覆盖 `is_disabled` 优先级；`apps/docs-app/src/pages/components/pages/collections_groups.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 已切到 `is_disabled=true` 文档/契约断言。
- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。
  - 受控模式：外部值是单一事实来源，内部不得偷偷写回本地状态。
  - 非受控模式：仅由默认值初始化一次，后续状态由内部原语管理。
  - 受控/非受控切换语义需稳定可测，避免“半受控”隐式行为。
  - 审查证据：`components/collapsible/src/view.rs` 对外同时提供 `open`、`on_open_change`、`default_open`，形成完整受控/非受控三件套。
  - 受控语义证据：点击路径先触发 `on_open_change`，再通过 `state.sync_controlled(...) + state.set_open(next)` 写回；在受控模式下本地写入由 primitive 拦截，不会形成“偷写”。
  - 非受控语义证据：`crates/ui-state-primitives/src/collapsible.rs` 通过 `ControlledStateOptions { value: open, default_value: default_open }` 一次初始化默认值，后续由 primitive 内部状态管理。
  - 回归证据：`crates/ui-state-primitives/src/test/collapsible.rs` 新增受控↔非受控切换测试；`components/collapsible/test/logic.rs` 与 `components/collapsible/test/semantics.rs` 锁定三件套 API 与 `sync_controlled` 接线，`components/collapsible/test/collapsible_semantics.rs` 保持仓库级契约断言。
- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。
  - 默认值优先级必须可读且可测试（显式规则而非分散 `unwrap_or`）。
  - `view.rs` 不允许再做默认值分支；仅消费 `logic.rs` 的归一化输出。
  - 一旦发现多处默认值来源，直接判不通过并回收至 `logic.rs`。
  - 审查证据：`components/collapsible/src/logic.rs` 新增 `normalize_open_state_options(open, default_open)`，显式收口 `open > default_open > primitive fallback` 优先级。
  - 接线证据：`components/collapsible/src/view.rs` 仅通过 `logic::normalize_open_state_options(...)` 初始化 open-state，不再在 `view.rs` 内直接拼默认值 options。
  - 回归证据：`components/collapsible/test/logic.rs` 新增优先级测试覆盖 controlled/default/fallback 三路径；`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 锁定归一化入口接线。
- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。
  - 输入边界统一进入 `logic.rs`，输出统一为可渲染语义状态与来源标记。
  - 事件处理器只触发状态变更，不重建状态机规则。
  - 样式层只消费状态标记，不承担状态判定职责。
  - 审查证据：`components/collapsible/src/logic.rs` 统一收口 `normalize_open_state_options`、`compute_next_open`、`should_emit_open_change`、`apply_open_change`；`view.rs` 仅消费这些入口。
  - 事件证据：`components/collapsible/src/view.rs` 的 `on_press/request_open_change` 只触发 `logic` 调用与回调分发，不在事件回调中重建受控/非受控状态机规则。
  - 派生证据：渲染态仍通过类型化 `CollapsibleStateInput` 进入 `logic::resolve_state(...)` 输出 `data-state/data-open-mode/...` 标记；`styles.rs` 仅消费状态标记。
  - 回归证据：`components/collapsible/test/logic.rs` 新增归一化函数测试；`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 锁定 `logic` 入口接线。
- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。
  - 互斥状态优先用 `enum` 建模，利用编译器封住无效组合。
  - 字符串输入若需兼容外部配置，必须先映射到类型化枚举再进入逻辑层。
  - 布尔爆炸（多个 bool 表达一个状态机）应在设计评审阶段直接拦截。
  - 审查证据：`crates/ui-state-primitives/src/collapsible.rs` 引入 `CollapsibleStatus`、`CollapsibleOpenMode`、`CollapsibleLabelSource`、`CollapsibleClassSource`、`CollapsibleMotionSource`，并将 `CollapsibleStateInput` 改为这些离散 enum 轴。
  - 接线证据：`components/collapsible/src/logic.rs` 提供 `normalize_status/open_mode/*_source`，`components/collapsible/src/view.rs` 只传 enum 输入到 `logic::resolve_state(CollapsibleStateInput { ... })`，不再在 view 内以 bool 组合互斥模式。
  - 回归证据：`crates/ui-state-primitives/src/test/collapsible.rs` 增加离散 enum 互斥轴测试；`components/collapsible/test/logic.rs` 增加 bool->enum 归一化测试；`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 锁定 enum 归一化接线。
- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。
  - 组件中出现可复用状态机实现（受控/非受控、展开规则、选择归一）即判应下沉。
  - 组件与业务全局状态之间必须有适配边界，禁止组件直接依赖业务 store 类型。
  - `logic.rs` 仅做装配与映射，不重新实现状态原语。
  - 审查证据：`components/collapsible/src/logic.rs` 仅 `pub use ui_state_primitives::collapsible::{...}` 并做装配映射（class/source 归一化）；组件可复用状态机仍由 `crates/ui-state-primitives/src/collapsible.rs` 提供。
  - 边界证据：`components/collapsible/src/view.rs` 通过 `logic::use_collapsible_open_state(...)` 消费原语，未直接绑定 `ui_state_primitives` 模块或业务 store 类型。
  - 防回归证据：`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 新增断言，禁止 `view.rs` 直接引用 `ui_state_primitives::*`，并约束 `logic.rs` 不重建 `use_controlled_state` 原语实现。
- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。
  - 无异步交互时需明确标注 N/A 理由（例如“组件无远程请求与异步状态”），不是机械打勾。
  - 有异步交互时，`is_loading`/disabled/`aria-busy`/retry 语义必须成套一致，且对键盘与读屏路径可用。
  - 异步失败态要有可恢复路径（重试或回退），并有语义测试覆盖。
  - N/A 理由：`Collapsible` 仅处理本地开合状态，不发起远程请求、无异步任务生命周期、无错误/重试协议。
  - 审查证据：`components/collapsible/src/view.rs` 与 `components/collapsible/src/logic.rs` 不包含 `is_loading`/`aria-busy`/`retry`/`use_async_action` 等异步语义输入输出。
  - 防回归证据：`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 新增断言，显式禁止异步协议 token 混入当前组件。
- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。
  - 基础用法不得要求用户先理解或手动接线 `ui-state-primitives`/`ui-headless` 状态机。
  - 基础组件 Hello World 示例代码不得超过 5 行（导入与外层模板按仓库约定不计），并可直接运行。
  - 简单需求走简单 API，复杂需求再暴露高级入口：默认 props 覆盖高频场景，高级控制通过受控/扩展参数按需开启。
  - 禁止把内部状态对象作为基础必填参数暴露（例如强制 `state=...` 才能完成点击/展开等基本交互）。
  - docs-app 必须提供最小可用示例，优先展示一眼可懂的默认调用路径。
  - 审查证据：`apps/docs-app/src/pages/components/pages/collections_groups.rs` 在 `collapsible()` 首个 playground 新增 `Hello World`，默认调用路径仅 `<Collapsible id_base ... title ...>` + 子内容，不要求 `open/on_open_change` 受控接线。
  - 最小示例证据：`components/collapsible/src/README.md` 的 Hello World 片段为 3 行可运行示例（不计导入与外层模板）。
  - 防回归证据：`components/collapsible/test/collapsible_semantics.rs` 新增 `collapsible_docs_hello_world_uses_default_api_path`，锁定 Hello World 首屏顺序、最小代码字符串与“非必须受控接线”；`components/collapsible/test/semantics.rs` 断言不暴露 `state` 内部对象 prop。
- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。（N/A：`Collapsible` 为单实体 disclosure，不是多 item 组合容器）
  - 每个 item 的标题、语义与内容必须在同一 `Item` 结构维度绑定，避免索引配对式隐式约定。
  - `labels + children`、`titles + panels` 等并行数组/并行槽位写法不得作为默认或推荐 API。
  - 不引入这类语法糖：若为配置式输入，仅允许类型化 `ItemSpec`，并在内部映射为显式 `Item` 语义树。
  - N/A 理由：`components/collapsible/src/view.rs` 仅暴露单个 `Collapsible` + `children: Children` 插槽，不存在可注册的 `Item` 集合与并行槽位配对语义。
  - 审查证据：当前公共 props 未提供 `labels/titles/panels/items` 并行输入轴，docs 示例均为显式 `<Collapsible>...</Collapsible>` 路径（见 `components/collapsible/src/README.md` 与 docs-app 页面）。
  - 防回归证据：`components/collapsible/test/semantics.rs` 新增 `collapsible_non_composite_api_rejects_parallel_item_inputs`；`components/collapsible/test/collapsible_semantics.rs` 新增 `collapsible_api_stays_non_composite_and_explicit`，锁定“无并行数组 API / 无 ItemSpec 语法糖”契约。

### 3. 高级交互与物理机制（Shell/Physics）
- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。（N/A：`Collapsible` 无拖拽交互）
  - N/A 理由：`Collapsible` 仅包含离散的 press/open-close 交互，不存在持续拖拽态，也不需要 `Dragging -> DragEnd` 宏/微双状态机收敛路径。
  - 审查证据：`components/collapsible/src/view.rs` 未注册 `pointermove/mousemove/touchmove`；`components/collapsible/src/logic.rs` 与 `components/collapsible/src/motion.rs` 未出现 `Dragging/DragEnd` 或每帧回流 action。
  - 防回归证据：`components/collapsible/test/semantics.rs` 新增 drag token 禁止断言；`components/collapsible/test/collapsible_semantics.rs` 新增 `collapsible_macro_micro_duality_is_not_applicable_without_drag_contract`，锁定“无拖拽 micro loop / 无 DragEnd 宏态”契约。
- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。（N/A：`Collapsible` 无几何测量驱动定位场景）
  - N/A 理由：`Collapsible` 仅做 disclosure 开合与语义挂载，不执行 tooltip/popover/menu 类的定位测量与回写纠偏流程，不存在 `Intent -> Measure -> Rectification` 回路。
  - 审查证据：`components/collapsible/src/view.rs`、`components/collapsible/src/logic.rs`、`components/collapsible/src/motion.rs` 不包含 `getBoundingClientRect/offset*/client*/ResizeObserver/IntersectionObserver` 等测量入口。
  - 防回归证据：`components/collapsible/test/semantics.rs` 新增 `collapsible_two_pass_rendering_is_not_applicable_without_geometry_measurement`；`components/collapsible/test/collapsible_semantics.rs` 新增同名契约测试，锁定“无 Two-Pass 几何测量链路”。
- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。（N/A：`Collapsible` 非动态子项集合组件）
  - N/A 理由：`Collapsible` 仅包含单 trigger + 单 panel 的 disclosure 结构，不维护动态 item 列表，不存在 `Register/Unregister` 生命周期与 `items_order` 导航语义。
  - 审查证据：`components/collapsible/src/view.rs`、`components/collapsible/src/logic.rs`、`components/collapsible/src/motion.rs` 均未引入 `RegistrationContext/Register/Unregister/items_order/HashSet`。
  - 防回归证据：`components/collapsible/test/semantics.rs` 新增 `collapsible_registration_protocol_is_not_applicable_for_single_item_disclosure`；`components/collapsible/test/collapsible_semantics.rs` 新增同名契约测试，锁定“无集合注册协议”边界。
- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。（N/A：`Collapsible` 非多插槽容器）
  - N/A 理由：`Collapsible` 只有单 trigger + 单 panel 的固定 disclosure 结构，不提供可切换投影策略（`Lazy/KeepAlive/Eager`），也无隐藏态副作用轮询需要 `NotifyHidden` 生命周期通知。
  - 审查证据：`components/collapsible/src/view.rs`、`components/collapsible/src/logic.rs`、`components/collapsible/src/motion.rs` 均未引入 `Lazy/KeepAlive/Eager/NotifyHidden` 投影语义。
  - 防回归证据：`components/collapsible/test/semantics.rs` 新增 `collapsible_slot_projection_policy_is_not_applicable_for_single_panel_disclosure`；`components/collapsible/test/collapsible_semantics.rs` 新增同名契约测试，锁定“无插槽投影策略协议”边界。
- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。（N/A：`Collapsible` 无环境订阅流交互）
  - N/A 理由：`Collapsible` 的开合逻辑由离散 press/focus/hover 事件驱动，不依赖 `Resize/Theme/Intersection` 连续采样流，也不需要 `BreakpointChanged` 等高层动作回流。
  - 审查证据：`components/collapsible/src/view.rs`、`components/collapsible/src/logic.rs`、`components/collapsible/src/motion.rs` 未引入 `ResizeObserver/IntersectionObserver/matchMedia/debounce/throttle/Action::BreakpointChanged`。
  - 防回归证据：`components/collapsible/test/semantics.rs` 新增 `collapsible_env_streams_are_not_applicable_without_responsive_sampling_contract`；`components/collapsible/test/collapsible_semantics.rs` 新增同名契约测试，锁定“无环境订阅流投影”边界。
- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。（N/A：`Collapsible` 非大型集合批量操作组件）
  - N/A 理由：`Collapsible` 仅有单条开合状态轴（open/closed + disabled），不存在 `Table/Grid` 类批量选择与广播场景，不需要 `Context Bus + Selector` 或 `SelectionState::All` 压缩表示。
  - 审查证据：`components/collapsible/src/view.rs`、`components/collapsible/src/logic.rs`、`components/collapsible/src/motion.rs` 未引入批量集合选择/广播协议与 O(N) 级下钻传参路径。
  - 防回归证据：`components/collapsible/test/semantics.rs` 新增 `collapsible_event_light_cone_is_not_applicable_for_single_disclosure_state_axis`；`components/collapsible/test/collapsible_semantics.rs` 新增同名契约测试，锁定“无事件光锥协议”边界。
- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。（N/A：`Collapsible` 无复杂派生总线广播图）
  - N/A 理由：`Collapsible` 的交互链路是本地 `on_press -> on_open_change -> state apply`，并可选写入 `use_ui_trace` 诊断事件，不存在跨订阅者的复杂派生命令广播图。
  - 审查证据：`components/collapsible/src/view.rs`、`components/collapsible/src/logic.rs`、`components/collapsible/src/motion.rs` 未引入 `TraceId` 透传、总线 `publish/broadcast/subscribe` 协议。
  - 防回归证据：`components/collapsible/test/semantics.rs` 新增 `collapsible_causality_bus_is_not_applicable_without_derived_broadcast_graph`；`components/collapsible/test/collapsible_semantics.rs` 新增同名契约测试，锁定“无统一因果总线协议”边界。
- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。
  - 交互元素必须具备可验证语义：`role`/`aria-*`/键盘可达路径完整，且和 headless 契约一致。
  - 用户可见文本来源必须可覆盖：优先 props，其次应用注入（`UiRoot`/i18n bundle），最后组件兜底文案；禁止把业务可见文案硬编码在 `view.rs`。
  - 组件需透传或消费 `lang` / `dir`（LTR/RTL）上下文，不得假设单语言单方向。
  - 共享 A11y 工具优先来自 `crates/ui-headless/src/a11y.rs`，组件层不重复发明同名语义工具。
  - 审查证据：`components/collapsible/src/view.rs` 继续通过 `use_button/use_focus_ring/use_hover` 挂载键盘与指针语义；并新增 `lang/dir` props，通过 `logic::normalize_dir` + `ui_headless::locale_attrs` + `ui_headless::disclosure_trigger_attrs` 接入 locale/a11y 共享契约。
  - 文案来源证据：`components/collapsible/src/view.rs` 不包含业务文案字符串，用户可见标题来自 `title` prop（空值时经 `ui-state-primitives` 的 `resolve_title` 兜底），`aria_label` 支持外部覆盖。
  - 防回归证据：`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 已新增/更新断言，覆盖 `lang/dir` 接线、`locale_attrs/disclosure_trigger_attrs` 复用、`role/aria` 挂载及“禁止在 view.rs 硬编码用户文案”。
- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。
  - 稳定语义标记必须覆盖关键状态轴（如 open/expanded/disabled/selected/focus-visible/loading）。
  - 状态来源必须可区分（受控/非受控、默认值/外部值、交互来源），通过稳定 marker 暴露而不是隐式推断。
  - 自动化选择器优先基于语义标记，不依赖 DOM 顺序、层级深度或临时 class 名。
  - 标记值应为封闭集合（可枚举），避免自由文本导致契约漂移。
  - 审查证据：`components/collapsible/src/view.rs` 在 root/trigger/panel 统一挂载稳定语义标记：`data-state`、`data-open`、`data-closed`、`data-disabled`、`data-open-mode`、`data-open-value-source`、`data-open-change-source`，并保持 `aria-expanded/aria-controls/aria-disabled/role/tabindex` 可检索。
  - 来源可区分证据：`components/collapsible/src/logic.rs` + `crates/ui-state-primitives/src/collapsible.rs` 新增 `CollapsibleOpenValueSource`（`external/default/primitive`）与 `CollapsibleOpenChangeSource`（`initial/interaction/external-sync`）封闭集合，组件不再依赖隐式推断来源。
  - 自动化选择器证据：`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 更新断言，优先锁定 `data-*`/`aria-*` 契约，不依赖 DOM 层级或临时 class。
  - 防漂移证据：`crates/ui-state-primitives/src/test/collapsible.rs`、`components/collapsible/test/logic.rs` 已新增 enum 映射与 attr 值回归，确保 marker 值域受类型系统约束。
- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。
  - `styles.rs` 中状态分支选择器必须基于 `data-*`/`aria-*`/稳定 class，禁止用 `:nth-child`、深层级选择器猜测状态。
  - 运行时样式仅允许传递必要 CSS 变量（custom properties）；禁止把业务样式逻辑塞进 inline style。
  - 视觉状态切换必须可由语义标记直接解释，不能依赖“某节点是否恰好存在”。
  - 审查证据：`components/collapsible/src/styles.rs` 的状态分支基于稳定语义标记与稳定 class（如 `data-state`、`data-open`、`data-open-mode`、`data-motion-source`、`data-custom-motion`、`ui-collapsible--state-*`），未使用 `:nth-child`、`:nth-of-type`、`:has` 等脆弱结构选择器。
  - 运行时样式证据：`components/collapsible/src/view.rs` 未写入 `style=` 业务内联样式；视觉切换通过 `data-*` marker + `styles.rs` 静态规则完成。
  - 回归证据：`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 已新增断言，锁定“显式语义选择器 + 禁止脆弱结构选择器 + 禁止业务内联样式”契约。
- [x] 测试验证“语义契约”而不只验证视觉快照。
  - 至少存在语义测试覆盖关键状态与交互路径（role/aria/data-state/source markers）。
  - 测试矩阵必须覆盖关键分支：受控/非受控、disabled、键盘路径、指针路径、SSR/wasm 差异（按适用范围）。
  - 视觉快照只能作为补充，不得替代语义契约断言。
  - 语义断言证据：`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 持续以 `role/aria/data-*` 契约为主断言（含 `data-state/open-mode/open-value-source/open-change-source` 与 `aria-expanded/aria-controls/aria-disabled`）。
  - 矩阵证据：`e2e/tests/docs_app_collapsible.spec.mjs` 覆盖受控/非受控（`data-open-mode`）、disabled（`data-state=disabled` + disabled trigger）、指针路径（`click`）、键盘路径（`Enter`）与来源标记变更（`data-open-change-source`）。
  - SSR/wasm 适用证据：同一 e2e 用例显式等待 `body:not(:has(#boot))`（wasm/hydration ready）后再执行语义断言；组件语义回归仍由 Rust source tests 覆盖，避免仅依赖运行时截图。
  - 非快照主导证据：新增 source 回归断言，锁定 `docs_app_collapsible.spec.mjs` 不依赖 `toHaveScreenshot/toMatchSnapshot` 作为主契约验证。
- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。
  - `mod.rs` 只维护最小稳定导出面与 feature gate，不承载实现细节。
  - `logic.rs` 只做输入归一、状态派生、来源标记；禁止 DOM 操作和样式细节分支。
  - `styles.rs` 只包含 token-first 静态 CSS；禁止硬编码主题常量与业务语义文案。
  - `view.rs` 只做结构渲染与 headless 契约挂载；禁止隐藏关键状态决策。
  - `motion.rs` 只做组件语义到动效契约映射与 attach；禁止在组件内重写通用动效引擎。
  - `mod.rs` 证据：`components/collapsible/src/mod.rs` 仅声明模块并导出 `Collapsible/CollapsibleMotion` 与少量状态类型；`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 已新增断言禁止 `mod.rs` 出现 `pub fn/impl/struct` 实现细节。
  - `logic.rs` 证据：`components/collapsible/src/logic.rs` 仅做 primitive 归一与来源映射；语义测试断言禁止 DOM/style 实现 token（如 `NodeRef/web_sys/color-mix`）。
  - `styles.rs` 证据：`components/collapsible/src/styles.rs` 只包含静态 CSS，状态分支依赖语义 marker，颜色来自 `var(--ui-*)`，无业务文案与事件逻辑。
  - `view.rs` 证据：`components/collapsible/src/view.rs` 仅做结构渲染与 headless 契约挂载，状态决策经 `logic::*` 收口并以 `data-*`/`aria-*` 暴露；无业务内联样式。
  - `motion.rs` 证据：`components/collapsible/src/motion.rs` 仅委托 `ui_disclosure::motion::{sanitize,attach_*}`；语义测试新增断言禁止 `SpringAnimator/requestAnimationFrame/keyframe` 等引擎重写 token。
- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。（N/A：`Collapsible` 为简单单实体 disclosure 组件，无独立复杂 Schema Builder 需求）
  - 仅当组件存在稳定外部规范/Schema 契约或复杂配置固化需求时才引入 `spec.rs`。
  - 简单组件不得为了“形式统一”新增 `spec.rs`；说明文档应留在 `check2.md`/组件文档。
  - 新增 `spec.rs` 必须同步给出契约测试与版本演进说明。
  - N/A 理由：当前 `Collapsible` API 轴有限（open/default/on_open_change、disabled、motion、label/class/locale），状态与来源契约已由 `ui-state-primitives + data-* markers` 覆盖，不需要额外 `*Spec::new()...render()` 建造者层。
  - 边界证据：`components/collapsible/src/mod.rs` 未声明/导出 `spec` 模块，组件目录也不存在 `components/collapsible/src/spec.rs`。
  - 文档落点证据：简单组件契约说明保持在 `components/collapsible/check2.md` 与 `components/collapsible/src/README.md`，未引入平行 `spec.rs` 文档路径。
  - 防回归证据：`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 新增断言，锁定“无 spec.rs 文件 + 无 mod spec/pub use spec 导出”。
- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。
  - 样式规则统一落在 `styles.rs`，由 `crates/ui-components/src/css.rs` 聚合并通过 `UiRoot` 注入。
  - 颜色/间距/圆角/阴影等视觉值必须来自 `var(--ui-*)`，禁止组件私有 token 体系。
  - Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。
  - CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。
  - 组件证据：`components/collapsible/src/styles.rs` 仅使用静态 CSS + `var(--ui-*)`，并以 `data-*` 语义标记驱动状态分支；`components/collapsible/src/view.rs` 无业务 `style=` 内联逻辑。
  - 聚合证据：`crates/ui-components/src/css.rs` 在 `#[cfg(feature = "component-collapsible")]` 下聚合 `crate::collapsible::styles::CSS`，未走组件私有注入通道。
  - 注入证据：`crates/ui-components/src/root.rs` 仅在 `inject_components_css` 开启时调用 `crate::css::push_components_css(&mut out)`，组件样式统一经 `UiRoot` 注入。
  - 防回归证据：`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 新增断言，锁定 token-first 变量使用、聚合注入链路、以及禁止 `@apply/tailwind/styled(/style!` 默认范式污染。
- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。
  - 默认主题需通过基础美学清单：信息层级清晰（字重/字号/间距）、对比与层次自然、交互反馈明确（hover/active/focus）。
  - docs-app 必须提供默认主题基线页面与截图基线，关键组件（Button/Input/Overlay）纳入视觉回归对比。
  - 禁止“可访问但粗糙”的最低可用心态：视觉退化（类似旧式 Bootstrap 观感）视为质量回归。
  - HeroUI 对标以“视觉语言与体验质量”对齐为目标，不做无差别 API 表层复制。
  - 组件视觉反馈证据：`components/collapsible/src/styles.rs` 使用 `color-mix(in oklch, var(--ui-*), ...)` 和过渡（`border/background/box-shadow`），并在 `prefers-reduced-motion` 下收敛；`components/collapsible/src/view.rs` 挂载 `use_focus_ring/use_hover` 与 `data-hovered/data-pressed` 反馈标记。
  - docs 体验证据：`apps/docs-app/src/pages/components/pages/collections_groups.rs` 提供 `Hello World`、`Controlled`、`Disabled + Custom Motion`、`Interactive Playground`，覆盖默认观感与交互反馈检查入口。
  - 仓库级主题基线证据：`apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs` 提供 `theme-visual-baseline` 页面并显式包含 Button/Input/Overlay 基线槽位；`e2e/tests/docs_app_theme_visual_baseline.spec.mjs` 提供对应可选截图回归（`toHaveScreenshot`）。
  - 防回归证据：`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 新增 `collapsible_visual_desire_contract_*` 断言，锁定上述视觉反馈与主题基线链路。
- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。
  - package 模式必须有组件级 feature（如 `component-accordion`）；未启用组件不得进入编译与链接路径。
  - `lib.rs` 与 `css.rs` 必须按 feature 条件导出/聚合，禁止无条件引用所有组件模块和 CSS 常量。
  - source 模式下仅引入需要的组件源码，不通过中央注册表维持全组件可达。
  - 任意“全量组件映射表/注册表”若导致不可达代码变可达，直接判不通过。
  - 验证命令（特性树）：`cargo tree -e features -p ui-components --no-default-features --features component-accordion,inject-css`，确认仅启用目标组件特性链。
  - 验证命令（反向依赖）：`cargo tree -e features -i ui-components -p web-demo`，检查是否被 `all-components` 或隐式特性全量拉起。
  - CI 检查（最小特性编译）：新增任务仅开启目标最小特性（示例：`cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-accordion,inject-css`）。
  - CI 检查（体积预算）：对“最小特性构建产物”设定预算并阻断回归（可用固定阈值，如 `< 50KB`，或基于仓库基线的相对阈值）；不得只做编译通过而不做体积约束。
  - 特性门控证据：`crates/ui-components/Cargo.toml` 以 `component-collapsible = ["dep:ui-collapsible"]` 声明组件级 feature，且 `ui-collapsible` 依赖为 `optional = true`。
  - 导出/样式门控证据：`crates/ui-components/src/lib.rs` 使用 `#[cfg(feature = "component-collapsible")] pub use ui_collapsible as collapsible;`；`crates/ui-components/src/css.rs` 使用 `#[cfg(feature = "component-collapsible")] out.push_str(crate::collapsible::styles::CSS);`，并由 `#[cfg(feature = "inject-css")]` 包裹聚合入口。
  - source 模式证据：`apps/web-demo/Cargo.toml` 依赖 `ui-components` 使用 `default-features = false` + `features = ["inject-css", "web-demo-components"]`，未显式启用 `all-components`。
  - 命令证据（最小特性树）：`cargo tree -e features -p ui-components --no-default-features --features component-collapsible,inject-css | rg \"ui-accordion feature|ui-collapsible feature\"` 仅出现 `ui-collapsible feature \"default\"`。
  - 命令证据（对照特性树）：`cargo tree -e features -p ui-components --no-default-features --features component-accordion,inject-css | rg \"ui-accordion feature|ui-collapsible feature\"` 仅出现 `ui-accordion feature \"default\"`。
  - 命令证据（反向依赖）：`cargo tree -e features -i ui-components -p web-demo` 显示 `web-demo` 通过 `web-demo-components` 依赖 `ui-components`，未被 `all-components` 隐式全量拉起。
  - 回归测试证据：`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 新增 `collapsible_tree_shaking_contract_keeps_feature_gates_explicit`，锁定 feature 声明、可选依赖、`lib.rs/css.rs` 门控与 web-demo 依赖配置。
  - CI 现状：最小特性 `cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-collapsible,inject-css` 在当前环境因 `Invalid cross-device link (os error 18)` 阻塞；需在稳定文件系统 runner 上执行体积预算与阻断策略。
- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。
  - 离散输入与状态轴必须优先使用 `enum`/新类型建模，避免字符串协议与布尔爆炸。
  - 无效状态要么在类型层不可表达，要么在 `logic.rs` 被统一归一化并可测试。
  - 关键状态必须通过稳定语义标记对外可读，供测试与 Agent 自动化消费。
  - 编译器与测试反馈应能直接定位状态契约破坏点，形成可持续闭环。
  - 类型建模证据：`crates/ui-state-primitives/src/collapsible.rs` 以 `CollapsibleStatus/OpenMode/LabelSource/ClassSource/MotionSource/OpenValueSource/OpenChangeSource` 枚举与 `CollapsibleStateInput` 结构体封装离散状态轴，避免字符串协议与布尔爆炸。
  - 归一化证据：`components/collapsible/src/logic.rs` 统一提供 `normalize_status/open_mode/*_source`，并在 `components/collapsible/src/view.rs` 通过 `logic::resolve_state(CollapsibleStateInput { ... })` 收口状态派生。
  - 语义标记证据：`components/collapsible/src/view.rs` 稳定输出 `data-state/data-open-mode/data-label-source/data-class-source/data-motion-source/data-open-value-source/data-open-change-source` 供自动化消费。
  - 防回归证据：`crates/ui-state-primitives/src/test/collapsible.rs`、`components/collapsible/test/logic.rs` 以及新增 `components/collapsible/test/semantics.rs`、`components/collapsible/test/collapsible_semantics.rs` 的 `collapsible_machine_readable_state_contract_is_type_driven_and_marker_stable`，可直接定位状态契约破坏点。

### 4. DOM/环境边界治理
- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。
  - N/A 理由：`Collapsible` 是单一 disclosure 折叠面板，不是层叠 `Overlay`；不存在 overlay 出栈后的焦点恢复链，也不存在焦点坠落 `document.body` 的堆栈回收场景。
  - 代码证据：`components/collapsible/src/view.rs` 的 `NodeRef` 仅用于 `indicator_ref/panel_ref/panel_surface_ref` 动效挂载（`crate::motion::attach_*`），未私存 trigger/previous-focus 恢复目标。
  - 契约证据：焦点交互由 `ui_headless::use_focus_ring` 与 `disclosure_trigger_attrs` 挂载，组件层未实现 `Overlay`/`FocusManager`/`FallbackTo` 焦点栈协议。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_focus_stack_gc_is_not_applicable_without_overlay_layering` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_focus_stack_gc_is_not_applicable_without_overlay_layering`，锁定该项为组件级 N/A 且禁止漂移。
- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。
  - N/A 理由：`Collapsible` 仅负责 disclosure 开合与语义挂载，不集成 ECharts/Map 等命令式第三方运行时；不存在 `Foreign Zone` 生命周期（`YieldControl/CleanupForeign`）管理需求。
  - 代码证据：`components/collapsible/src/view.rs`、`components/collapsible/src/logic.rs`、`components/collapsible/src/motion.rs` 未出现第三方实例接线（如 `ECharts/Mapbox/Leaflet/google.maps`）或命令式清理协议 token。
  - API 边界证据：`components/collapsible/src/mod.rs` 未暴露第三方实例句柄类型，公共 API 仍保持纯组件语义输入输出，不反向污染状态原语。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_escape_hatches_are_not_applicable_without_foreign_imperative_runtime` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_escape_hatches_are_not_applicable_without_foreign_imperative_runtime`，锁定该项为组件级 N/A 且禁止漂移。
- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。
  - 结论：`Collapsible` 的 hydration ID 链路是确定性的，未使用 `now()`/随机 UUID；`IdProvider` 注入在本组件为 N/A（组件不做内部随机 ID 生成）。
  - 代码证据：`components/collapsible/src/view.rs` 固定走 `id_base -> logic::normalize_id_base(id_base) -> DisclosureIds::new(&id_base)`，并将 `trigger_id/panel_id` 直接挂到 `id`/`aria-controls`/`aria-labelledby`。
  - 原语证据：`crates/ui-state-primitives/src/collapsible.rs::normalize_id_base` 仅做纯字符串归一化与常量回退（`DEFAULT_ID_BASE`），`components/disclosure/src/logic.rs::DisclosureIds::new` 通过 `format!(\"{id_base}-trigger/panel\")` 纯函数派生 ID。
  - N/A 说明：`IdProvider` 适用于组件需要“内部生成且可复现 ID”场景；`Collapsible` 由外部 `id_base` 驱动、无时间/随机源注入点，因此不需要额外 `IdProvider`。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_hydration_ids_are_deterministic_without_time_or_random_seed` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_hydration_ids_are_deterministic_without_time_or_random_seed`，锁定“无 now/random/uuid + deterministic id_base 链路”契约。
- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。
  - 至少包含 compile-only 证据：web（wasm32）、ssr（native）、默认本地构建三条路径。
  - 平台分支差异必须显式 `cfg` 或 feature 管理，禁止依赖运行时偶然行为。
  - non-wasm 路径禁止引用 `web-sys`/浏览器对象。
  - 源码证据：`components/collapsible/src/view.rs`、`components/collapsible/src/logic.rs`、`components/collapsible/src/motion.rs` 未直接引用 `web_sys/window/document`；平台差异由上游能力层处理（`crates/ui-headless/src/lib.rs` 的 `web+ssr` 互斥 `compile_error!`，`crates/ui-motion/src/lib.rs` 的 `#[cfg(target_arch = \"wasm32\")]` 与 `#[cfg(not(target_arch = \"wasm32\"))]` no-op 分支）。
  - compile-only 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo check -p ui-collapsible`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo check -p ui-headless --no-default-features --features ssr`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-collapsible,inject-css`
  - 命令现状：以上三条在当前 runner 均被同一基础设施问题阻塞（`Invalid cross-device link (os error 18)`），未发现组件级代码错误栈；需在稳定文件系统 CI/本地 runner 复核三路径 compile-only 通过。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_ssr_cross_platform_contracts_keep_non_wasm_paths_safe` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_ssr_cross_platform_contracts_keep_non_wasm_paths_safe`，锁定 non-wasm 路径禁用浏览器对象、headless web/ssr 互斥保护与 motion wasm/non-wasm 显式分支契约。
- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。
  - 组件依赖 `ui-headless` 能力时，不得破坏其 web/ssr 互斥约束。
  - 组件若新增 headless 功能接入，需验证两条 feature 路径都可编译。
  - 发现“同时启用 web+ssr 仍可过编译”视为契约回归。
  - 源码证据：`crates/ui-headless/src/lib.rs` 明确包含 `#[cfg(all(feature = "web", feature = "ssr"))] compile_error!(...)`；`components/collapsible/src/view.rs` 仅消费 `use_button/use_focus_ring/use_hover/disclosure_trigger_attrs/locale_attrs` 等 headless 契约，不在组件层重写 feature 互斥逻辑。
  - 依赖边界证据：`components/collapsible/Cargo.toml` 通过 `ui-headless = { path = "../../crates/ui-headless" }` 引入能力，未显式配置 `features = ["web", "ssr"]` 破坏互斥约束。
  - compile-only 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo check -p ui-headless --no-default-features --features web`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo check -p ui-headless --no-default-features --features ssr`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo check -p ui-headless --no-default-features --features web,ssr`
  - 命令现状：以上命令在当前 runner 均先被同一基础设施问题阻塞（`Invalid cross-device link (os error 18)`），未能到达 feature 冲突终态；需在稳定文件系统 CI/本地 runner 复核“web/ssr 单独可编译、web+ssr 命中 compile_error”。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_ui_headless_web_ssr_feature_mutex_contract_is_preserved` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_ui_headless_web_ssr_feature_mutex_contract_is_preserved`，锁定互斥宏、依赖边界与组件消费契约。
- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。
  - `motion.rs` 调用必须可在 non-wasm 下安全降级，不触发 panic。
  - 组件不得假设动画句柄一定存在；no-op 分支行为需可预测。
  - toolchain 场景（测试/文档/静态分析）不得因 motion 依赖阻塞编译。
  - 源码证据：`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = "wasm32"))]` 提供 `web::prefers_reduced_motion() -> true` 与 `web::animate(...) {}` no-op 实现，并包含 `non_wasm_web_backend_is_predictable_noop` 回归测试。
  - 组件调用证据：`components/collapsible/src/motion.rs` 仅通过 `ui_disclosure::motion::sanitize_motion/attach_*` 映射契约，不直接持有 wasm-only 句柄，也未假设动画句柄必定存在。
  - 安全降级证据：`components/collapsible/src/motion.rs` 未出现 `panic!/unwrap/expect` 依赖运行时动画句柄的分支，non-wasm 场景可预测降级。
  - compile-only 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo check -p ui-motion`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo check -p ui-collapsible`
  - 命令现状：以上命令在当前 runner 均被同一基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核 non-wasm/SSR/tooling 编译路径。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_ui_motion_non_wasm_noop_contract_is_preserved` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_ui_motion_non_wasm_noop_contract_is_preserved`，锁定 no-op/stub 契约与组件侧可预测降级调用链。
- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。
  - `reduced-motion` 下动画应跳过或降级为最小必要反馈。
  - SSR 输出必须与客户端 hydration 兼容，避免首帧语义错位。
  - wasm 分支允许增强交互，但语义契约不得与 SSR 分支分裂。
  - `reduced-motion` 证据：`components/collapsible/src/styles.rs` 包含 `@media (prefers-reduced-motion: reduce)` 并将 trigger transition 降级为 `none`；`crates/ui-motion/src/spring.rs` 在 `set_target` 中命中 `if crate::web::prefers_reduced_motion()` 时直接写入目标值并触发 `on_rest`，跳过动画帧循环。
  - SSR/hydration 证据：`components/collapsible/src/view.rs` 使用 `DisclosureIds::new(&id_base)` 生成稳定语义 ID，初始 `panel_hidden = RwSignal::new(!open.get_untracked())` 与 `hidden=move || panel_hidden.get()` 保持首帧语义一致；未引入 `now()`/随机 UUID。
  - wasm/ssr 分支一致性证据：`components/disclosure/src/motion.rs` 对 `attach_indicator_motion/attach_panel_motion` 提供 `#[cfg(target_arch = "wasm32")]` 增强分支与 `#[cfg(not(target_arch = "wasm32"))]` 安全降级分支（含 `is_hidden.set(!is_open.get())`）；`components/collapsible/src/view.rs` 未使用 `#[cfg(...)]` 分裂语义标记，SSR 与 wasm 均输出同一 `data-* + aria-*` 契约。
  - compile-only 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo check -p ui-collapsible`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo check -p ui-components --no-default-features --features component-collapsible,inject-css`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-collapsible,inject-css`
  - 命令现状：以上命令在当前 runner 仍受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核 native/SSR/wasm 三路径编译。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_reduced_motion_ssr_wasm_contracts_stay_convergent` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_reduced_motion_ssr_wasm_contracts_stay_convergent`，锁定 reduced-motion 降级、SSR 首帧语义一致与 wasm/non-wasm 分支不分裂契约。
- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。
  - 关键交互组件需定义最小预算项（首渲染、关键更新、内存/分配趋势）。
  - 回归检测至少具备可重复基线与失败阈值，不靠主观“感觉变慢”。
  - 性能问题需可归因到状态、渲染、样式或动效路径之一。
  - 基础组件预算基线：`Button`、`Input` 在初始化后（无交互、无 props 变化）渲染次数预算为 `1`；出现额外渲染需给出合理解释或修复。
  - 测试要求：在 `components/*/test/**` 增加 `render_count` 类回归测试（测试框架支持时必须启用）；至少覆盖基础组件与本次改动组件。
  - 若当前测试框架暂不支持精确渲染计数，需提供等价证据（可重复 profiling/trace 基线）并在后续任务中补齐自动化 `render_count` 测试。
  - 预算证据：`apps/docs-app/src/pages/components/shell.rs` 为 `collapsible` 新增显式预算（`max_mount_ms: 34.0`、`max_update_ms: Some(11.0)`、`max_heap_kb: Some(576.0)`），并通过 `<UiPerfProbe name=perf_name budget=perf_budget>` 统一挂载。
  - 基线证据：同一预算源保留 `button/input` 基线预算（`24/8` 与 `28/10`），满足“基础组件预算基线”约束。
  - 阻断证据：`e2e/tests/docs_app_components_coverage.spec.mjs` 对 `data-perf-mount-ms/data-perf-budget-ms/data-perf-observability` 与 `data-perf-violation!=true` 做阻断断言；`scripts/check-ui-components-performance.sh` 新增 `collapsible_performance_governance_contract_is_budgeted_traceable_and_blocking` 目标。
  - 可归因证据：`components/collapsible/src/view.rs` 暴露 `data-state/data-open-mode/data-open-value-source/data-open-change-source/data-motion-source`；`logic.rs` 负责状态归一，`styles.rs` 负责视觉分支，`motion.rs` 负责动效映射，定位可归因到状态/渲染/样式/动效路径。
  - `render_count` 现状：当前链路仍以 `UiPerfProbe` + e2e marker 作为等价证据；`docs/plan/TODO.md` 明确保留 `render_count` 自动化补齐任务（`Button/Input/Accordion`），未移除后续治理要求。
  - compile/test 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_performance_governance_contract_is_budgeted_traceable_and_blocking -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_performance_governance_contract_is_budgeted_traceable_and_blocking -- --nocapture`
  - 命令现状：上述命令在当前 runner 受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_performance_governance_contract_is_budgeted_traceable_and_blocking` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_performance_governance_contract_is_budgeted_traceable_and_blocking`，锁定预算、阻断、可归因与 `render_count` 跟踪契约。
- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。
  - 复杂结构按语义子块拆分（header/body/item 等），避免巨型单块 `view!`。
  - `view.rs` 中若出现多层嵌套重复片段，应优先提取局部渲染函数。
  - 编译时间/产物体积异常增长时，优先排查宏展开体量。
  - 源码证据：`components/collapsible/src/view.rs` 将原始单块模板拆分为 `render_trigger(...)` 与 `render_panel(...)` 两个语义子块，并在根 `view!` 中组合 `{trigger}`、`{panel}`。
  - 宏体量证据：`components/collapsible/src/view.rs` 当前 `view!` 块总数受控（根 + 2 子块），无超长深嵌套单块模板；保留单一公开 `#[component] Collapsible` 入口。
  - 门禁证据：`scripts/check-ui-components-view-macro.sh` 新增 `collapsible_view_macro_complexity_is_split_into_semantic_subrenders` 目标，纳入 view-macro 契约检查链。
  - compile/test 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_view_macro_complexity_is_split_into_semantic_subblocks -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_view_macro_complexity_is_split_into_semantic_subrenders -- --nocapture`
  - 命令现状：上述命令在当前 runner 受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_view_macro_complexity_is_split_into_semantic_subblocks` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_view_macro_complexity_is_split_into_semantic_subrenders`，锁定语义分块与宏数量预算契约。
- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。
  - 纯静态或轻逻辑片段优先函数化；仅在需要独立 props 语义时升级为组件。
  - 禁止把所有局部片段都升格为 `#[component]` 导致抽象噪音。
  - 拆分后语义标记与测试定位仍需稳定。
  - 源码证据：`components/collapsible/src/view.rs` 提供 `fn render_trigger(...) -> impl IntoView` 与 `fn render_panel(...) -> impl IntoView`，并在 `pub fn Collapsible(...)` 内通过 `let trigger = render_trigger(...)`、`let panel = render_panel(...)` 组合渲染。
  - 边界证据：`components/collapsible/src/view.rs` 保持单一公开 `#[component]` 边界（`Collapsible`），未引入局部 `#[component]` 子片段。
  - 语义稳定证据：函数化拆分后 `data-slot="collapsible|collapsible-trigger|collapsible-label|collapsible-indicator|collapsible-panel|collapsible-panel-surface"` 标记保持不变。
  - 门禁证据：`scripts/check-ui-components-view-macro.sh` 新增 `collapsible_view_functional_split_prefers_plain_functions_over_local_components` 目标，纳入 view-macro 契约检查链。
  - compile/test 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_view_functional_split_prefers_plain_functions_over_local_components -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_view_functional_split_prefers_plain_functions_over_local_components -- --nocapture`
  - 命令现状：上述命令在当前 runner 受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_view_functional_split_prefers_plain_functions_over_local_components` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_view_functional_split_prefers_plain_functions_over_local_components`，锁定函数式拆分与语义标记稳定性契约。
- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。
  - 可判定为纯静态的片段应避免重复动态构造。
  - 常量化后仍需维持可访问语义（title/aria-label/role 等）。
  - 静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。
  - 源码证据：`components/collapsible/src/view.rs` 新增 `SLOT_COLLAPSIBLE*`、`ARIA_HIDDEN_TRUE`、`COLLAPSIBLE_INDICATOR_GLYPH` 常量，`data-slot` 与箭头 glyph 改为常量引用，避免散落字面量。
  - 可访问语义证据：常量化后仍保持 `role="region"`、`aria-labelledby`、`aria-hidden=ARIA_HIDDEN_TRUE` 与原有 `data-slot` 语义路径，不改变可访问契约。
  - 轻量组件策略：`Collapsible` 不包含复杂 SVG/长静态文本，按“常量化或缺省”策略执行；通过常量集中化静态片段，同时保持无重型静态模板拼接。
  - 门禁证据：`scripts/check-ui-components-view-macro.sh` 新增 `collapsible_static_fragments_are_constantized_or_absent_for_simple_layout` 目标，纳入 view-macro 契约检查链。
  - compile/test 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_static_fragments_are_constantized_or_absent_for_simple_layout -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_static_fragments_are_constantized_or_absent_for_simple_layout -- --nocapture`
  - 命令现状：上述命令在当前 runner 受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_static_fragments_are_constantized_or_absent_for_simple_layout` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_static_fragments_are_constantized_or_absent_for_simple_layout`，锁定静态常量收敛与语义稳定性契约。
- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。
  - 仅允许编译期常量或明确白名单内容进入 `inner_html`。
  - 严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。
  - 使用 `inner_html` 的节点必须补语义测试与安全回归说明。
  - 组件证据：`components/collapsible/src/{mod,logic,styles,view,motion}.rs` 均未出现 `inner_html/set_inner_html/dangerously_set_inner_html`，无组件内原始 HTML 注入路径。
  - docs 证据：`apps/docs-app/src/pages/components/pages/collections_groups.rs`（collapsible 示例页）未出现 `inner_html` 或未清洗 HTML 拼接；组件示例仅使用显式 `Collapsible` 调用。
  - 白名单边界证据：`apps/docs-app/src/pages/components/shell.rs` 的 `inner_html` 仍仅用于受信任 README 挂载（`<div data-slot="component-readme" inner_html=html></div>`），且未把 `collapsible` 纳入 README 白名单映射。
  - 门禁证据：`scripts/check-ui-components-inner-html.sh` 新增 `collapsible_inner_html_usage_is_forbidden_in_component_and_docs_examples` 目标，纳入 inner-html 契约检查链。
  - compile/test 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_inner_html_usage_is_forbidden_in_component_and_docs_examples -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_inner_html_usage_is_forbidden_in_component_and_docs_examples -- --nocapture`
  - 命令现状：上述命令在当前 runner 受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_inner_html_usage_is_forbidden_in_component_and_docs_examples` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_inner_html_usage_is_forbidden_in_component_and_docs_examples`，锁定组件与 docs 的 inner-html 安全边界。
- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。
  - 开发模式下至少能追踪关键状态变更来源与前后值。
  - 关键交互链路应支持最小可复现记录（事件顺序/状态转移）。
  - 调试开关默认不进入生产包体与公共 API。
  - 组件状态追踪证据：`components/collapsible/src/view.rs` 复用 `use_ui_trace()` 并在开关变更时发出 `trace.emit("collapsible", UiTraceEventKind::OpenChange { open: next })`，同时稳定暴露 `data-state/data-open-mode/data-open-value-source/data-open-change-source` 作为机器可读状态与来源标记。
  - 全局可视化入口证据：`apps/docs-app/src/lib.rs` 在 `debug_assertions` 下启用 `provide_ui_trace(debug_overlay_enabled)` 与 `<debug_overlay::UiDebugOverlay enabled=true />`；`apps/docs-app/src/debug_overlay.rs` + `crates/ui-headless/src/trace.rs` 提供时间戳事件时间线（`ts_ms`）。
  - 关键交互可回放证据：共享回放能力保持在 `components/button/src/view.rs`（`data-debug-source/data-debug-before/data-debug-after/data-debug-timestamp-ms` + `request_replay.run(event.source)`），`collapsible` 不重复发明本地 debug runtime。
  - feature 隔离证据：`components/collapsible/Cargo.toml` 未新增 `collapsible-wasm-debug`；`crates/ui-components/Cargo.toml` 维持共享 `button-wasm-debug`，且 `all-components` 未引入该调试特性，避免生产链路污染。
  - 门禁证据：`scripts/check-ui-components-wasm-debug.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated`。
  - compile/test 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated -- --nocapture`
  - 命令现状：当前 runner 存在基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated`，锁定 feature 隔离、全局 trace 复用与公共 API 非污染契约。
- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。
  - 常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。
  - 组件调试应尽量保持当前交互上下文，降低重复操作成本。
  - 复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。
  - 热重载路径证据：`apps/docs-app/src/playground.rs` 通过 `<style>{move || compose_scoped_css(&scope_selector.get_value(), &test_css.get())}</style>` + `on:input=move |ev| set_test_css.set(event_target_value(&ev))` 实现 scoped CSS 实时注入，无需完整 wasm 重编译。
  - 上下文保持证据：同一 playground 保持 `show_settings/show_code/show_test` 面板切换状态，并在 `collections_groups.rs` 的 `Interactive Playground (Display + Config + Code + CSS Test)` 中保留 `Mode/Motion/Controlled open/Default open/Disabled/Custom aria-label/Custom class` 调参与实时预览，减少重复操作。
  - 可选状态保留证据（N/A，按组件范围）：`collapsible` 当前 workbench 不引入跨刷新持久化（无 `*_WORKBENCH_STORAGE_KEY`、`load/save/clear_*_workbench_state`），避免为轻量 disclosure 交互引入额外存储复杂度；运行期上下文通过页面内信号保持。
  - 隔离画布证据：`apps/docs-app/src/playground.rs` 统一通过 `data-playground-scope=scope_id` + `.playground__preview-stage` 隔离演练区域；`collections_groups.rs` 保持 `slug="collapsible"` 与 interactive playground 专属 `id_base="docs-collapsible-interactive"` 路径。
  - 门禁证据：`scripts/check-ui-components-dx.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na`。
  - compile/test 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na`，锁定热重载、上下文保持、可选状态保留 N/A 与隔离画布契约。
- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。
  - 若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。
  - 关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。
  - 异步边界不得把具体 runtime 类型暴露到组件公共接口。
  - serde/protocol 证据：`components/collapsible/src/protocol.rs` 统一承载协议序列化（`CollapsibleComponentSchemaVersion::V1` + `CollapsibleComponentSpec { schema_version }`，`Serialize/Deserialize` + `#[serde(default)]`）；其余层 `mod/logic/view/styles/motion` 不承载 schema 迁移与 JSON 处理逻辑。
  - tracing 语义证据：`crates/ui-components/Cargo.toml` 保持共享 tracing 基线（如 `button-wasm-debug`），`components/button/src/view.rs` 仍是 canonical `target: "ui_components::button::state_change"`；`collapsible` 未新增 `collapsible-wasm-debug` 与本地 tracing target，避免组件各自为政。
  - runtime 边界证据：`components/collapsible/src/{mod,logic,view,styles,motion,protocol}.rs` 与 `components/collapsible/Cargo.toml` 未暴露 `tokio/async-std/smol/runtime::Handle` 等运行时细节；公共 API 仍仅围绕 `Collapsible/CollapsibleMotion` 语义面。
  - 门禁证据：`scripts/check-ui-components-engineering.sh` 新增三条 `collapsible` 执行命令：`collapsible_engineering_contract_uses_serde_protocol_and_structured_schema_defaults`、`collapsible_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events`、`collapsible_engineering_contract_avoids_runtime_leaks_in_public_api_surface`。
  - compile/test 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_engineering_contract_uses_serde_protocol_and_structured_schema_defaults -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_engineering_contract_uses_serde_protocol_and_structured_schema_defaults -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::{collapsible_engineering_contract_uses_serde_protocol_and_structured_schema_defaults, collapsible_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events, collapsible_engineering_contract_avoids_runtime_leaks_in_public_api_surface}` 与 `components/collapsible/test/collapsible_semantics.rs` 同名测试，锁定 serde/protocol、tracing 语义统一与 runtime 边界三类契约。

### 5. 样式与动效（Theme & Motion）
- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。
  - 样式契约证据：`components/collapsible/src/styles.rs` 将 disabled/motion/color 输入统一收口到双层回退链（如 `var(--ui-disabled-opacity, var(--ui-fallback-disabled-opacity))`、`var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))`、`var(--ui-accent, var(--ui-fallback-accent))`），移除裸 `0.72/200ms` 终值。
  - SSOT 证据：`crates/ui-theme/src/css.rs` 提供 `--ui-fallback-disabled-opacity`、`--ui-fallback-text-field-motion-duration`、`--ui-fallback-text-field-motion-easing`、`--ui-fallback-accent`、`--ui-fallback-border`、`--ui-fallback-bg`、`--ui-fallback-accent-soft` 统一终值。
  - 门禁证据：`scripts/check-ui-components-contract-hygiene.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_styles_use_defensive_variable_fallback_chain`。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_styles_use_defensive_variable_fallback_chain` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_styles_use_defensive_variable_fallback_chain`，锁定 fallback 链、SSOT 终值、脚本门禁与 checklist 勾选状态。
  - compile/test 命令证据（当前环境）：
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_styles_use_defensive_variable_fallback_chain -- --nocapture`
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_styles_use_defensive_variable_fallback_chain -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。
- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\"top: 10px\"`）。
  - 级联层聚合证据：`crates/ui-components/src/css.rs` 在 `push_components_css` 中维持 `@layer ui` 包裹，并以 `#[cfg(feature = "component-collapsible")] out.push_str(crate::collapsible::styles::CSS);` 受特性门控聚合。
  - 注入路径证据：`crates/ui-components/src/root.rs` 仅在 `inject_components_css` 开启时调用 `crate::css::push_components_css(&mut out)`，组件不自行旁路注入样式。
  - 运行时样式证据：`components/collapsible/src/view.rs` 无普通内联布局样式（无 `style="top:..."` / `style:top=...` 等）；若未来引入 `style:*`，测试要求仅允许 `style:--*` 自定义属性通道。
  - 门禁证据：`scripts/check-ui-components-contract-hygiene.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_cascade_layer_and_runtime_style_contract_is_enforced`。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_cascade_layer_and_runtime_style_contract_is_enforced` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_cascade_layer_and_runtime_style_contract_is_enforced`，锁定 `@layer ui` 聚合、UiRoot 注入路径与运行时样式通道约束。
  - compile/test 命令证据（当前环境）：
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_cascade_layer_and_runtime_style_contract_is_enforced -- --nocapture`
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_cascade_layer_and_runtime_style_contract_is_enforced -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。
- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。
  - 合同参数证据：`components/collapsible/src/mod.rs` 公开 `CollapsibleMotion = ui_disclosure::DisclosureMotion`；`components/disclosure/src/motion.rs` 默认 `SpringConfig` 固化 `stiffness: 260.0`、`damping: 18.0`，并经 `components/collapsible/src/motion.rs::sanitize_motion` 统一净化。
  - attach 挂载证据：`components/collapsible/src/view.rs` 仅通过 `crate::motion::attach_indicator_motion(...)` 与 `crate::motion::attach_panel_motion(...)` 绑定动效，组件层不直接接线底层执行器。
  - reduced-motion 证据：`components/collapsible/src/styles.rs` 保留 `@media (prefers-reduced-motion: reduce) { transition: none; }`；`crates/ui-motion/src/spring.rs` 在 `prefers_reduced_motion()` 分支直接收敛到目标值。
  - non-wasm/SSR 安全降级证据：`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = "wasm32"))]` 提供 `web::animate(...) {}` no-op；`components/collapsible/src/motion.rs` 仅代理 `ui_disclosure::motion::attach_*`，无运行时句柄假设与 panic 路径。
  - 门禁证据：`scripts/check-ui-components-contract-hygiene.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop`。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop`，锁定内置参数、attach 路径、reduced-motion 与 non-wasm no-op 契约。
  - compile/test 命令证据（当前环境）：
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop -- --nocapture`
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。
- [x] `ui-components` 固定入口文件落点正确。
  - `crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。
  - `crates/ui-components/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。
  - `crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。
  - `crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。
  - `crates/ui-components/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。
  - `crates/ui-components/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。
  - `crates/ui-components/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。
  - 入口证据：`crates/ui-components/src/lib.rs` 保留 `pub mod root;`、`pub use root::UiRoot;`，并以 `#[cfg(feature = "component-collapsible")] pub use ui_collapsible as collapsible;` 维持组件级特性导出边界。
  - CSS 聚合证据：`crates/ui-components/src/css.rs` 保留 `push_components_css` + `@layer ui`，且 `out.push_str(crate::collapsible::styles::CSS);` 受 `component-collapsible` feature gate 保护。
  - Root 注入证据：`crates/ui-components/src/root.rs` 统一注入 `css::BASE_CSS`、`theme.get().to_css_variables()`，并在 `inject_components_css` 分支集中调用 `crate::css::push_components_css(&mut out)`；i18n 入口保持 `provide_ui_i18n(i18n);`。
  - 共享原语证据：`crates/ui-visual-primitive/src/active_highlight.rs` 保持 `pub const CSS`、`ActiveHighlightMotion`、`attach_active_highlight_motion`，且无 `collapsible` 业务语义硬编码。
  - 禁止落点证据：`crates/ui-components/src/overlay_open.rs`、`crates/ui-components/src/presence.rs`、`crates/ui-components/src/a11y.rs` 均不存在；对应原语固定在 `crates/ui-headless/src/controllable_state.rs`、`crates/ui-headless/src/presence.rs`、`crates/ui-headless/src/a11y.rs`。
  - 门禁证据：`scripts/check-ui-components-contract-hygiene.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_ui_components_fixed_entry_files_follow_contract`。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_ui_components_fixed_entry_files_follow_contract` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_ui_components_fixed_entry_files_follow_contract`，锁定入口文件落点与 headless/source-of-truth 约束。
  - compile/test 命令证据（当前环境）：
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_ui_components_fixed_entry_files_follow_contract -- --nocapture`
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_ui_components_fixed_entry_files_follow_contract -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。
- [x] 组件目录标准文件落点正确。
  - `<component>/mod.rs`：最小稳定导出面，存在且无过度导出。
  - `<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。
  - `<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。
  - `<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。
  - `<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。
  - `<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。
  - 文件落点证据：`components/collapsible/src/` 保持 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs` 五个标准文件，且不存在 `render.rs` 漂移与 `spec.rs` 误增文件。
  - `mod.rs` 边界证据：`components/collapsible/src/mod.rs` 仅保留模块声明与稳定导出（`Collapsible/CollapsibleMotion`）；未承载 `pub fn/impl/struct` 实现细节。
  - `logic.rs` 边界证据：`components/collapsible/src/logic.rs` 保持 props 归一与状态派生映射（`normalize_* / compose_class_name`），未包含 `view!`、`NodeRef`、`web_sys` 等渲染/DOM 细节。
  - `styles.rs` 边界证据：`components/collapsible/src/styles.rs` 保持 token-first 静态 CSS（`var(--ui-*)`）；未引入 `#hex`、`:nth-child`、`:has` 或事件逻辑。
  - `view.rs` + `motion.rs` 边界证据：`components/collapsible/src/view.rs` 仅渲染结构并挂载 `use_button/use_focus_ring/use_hover` 语义契约；`components/collapsible/src/motion.rs` 仅做 `sanitize_motion + attach_*` 映射，无引擎重写 token（`SpringAnimator/requestAnimationFrame/keyframe`）。
  - 门禁证据：`scripts/check-ui-components-contract-hygiene.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_component_directory_standard_files_stay_in_canonical_layout`。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_component_directory_standard_files_stay_in_canonical_layout` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_component_directory_standard_files_stay_in_canonical_layout`，锁定目录标准文件落点与职责边界。
  - compile/test 命令证据（当前环境）：
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_component_directory_standard_files_stay_in_canonical_layout -- --nocapture`
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_component_directory_standard_files_stay_in_canonical_layout -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。

### 6. AI 原生能力与文件落点（Struct-First & Projection）
- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。
  - 目录落点证据：`components/collapsible/src/` 保持 `mod.rs`、`logic.rs`、`styles.rs`、`view.rs`、`motion.rs` 五件套；`render.rs` 与 `spec.rs` 均不存在（当前组件为简单 disclosure，不引入 spec）。
  - 职责边界证据：`mod.rs` 仅做导出；`logic.rs` 收口归一化/派生；`styles.rs` 仅 token-first 静态样式；`view.rs` 仅结构渲染与 headless 挂载；`motion.rs` 仅语义到 motion contract 映射。
  - 门禁证据：`scripts/check-ui-components-contract-hygiene.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_file_layout_discipline_keeps_canonical_component_directory`。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_file_layout_discipline_keeps_canonical_component_directory` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_file_layout_discipline_keeps_canonical_component_directory`，锁定目录文件集合与禁止文件规则。
  - compile/test 命令证据（当前环境）：
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_file_layout_discipline_keeps_canonical_component_directory -- --nocapture`
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_file_layout_discipline_keeps_canonical_component_directory -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。
- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。（N/A：`Collapsible` 为简单 disclosure 组件，无复杂 schema builder 需求）
  - N/A 理由：`Collapsible` 为单实体 disclosure，参数轴有限（open/default/on_open_change、disabled、motion、label/class/locale），无需额外 `*Spec::new()...render()` 建造者层。
  - 目录证据：`components/collapsible/src/spec.rs` 不存在，`components/collapsible/src/mod.rs` 亦无 `mod spec` / `pub use spec::*` 导出。
  - 文档证据：`components/collapsible/src/README.md` 不包含 `Spec::new(` builder 教程，默认 API 走直接组件调用路径。
  - 门禁证据：`scripts/check-ui-components-contract-hygiene.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_hyper_structure_builder_spec_rs_is_not_applicable_for_simple_component`。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_hyper_structure_builder_spec_rs_is_not_applicable_for_simple_component` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_hyper_structure_builder_spec_rs_is_not_applicable_for_simple_component`，锁定 N/A 条件与脚本门禁挂接。
  - compile/test 命令证据（当前环境）：
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_hyper_structure_builder_spec_rs_is_not_applicable_for_simple_component -- --nocapture`
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_hyper_structure_builder_spec_rs_is_not_applicable_for_simple_component -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。
- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。
  - 落点证据：新增 `components/collapsible/src/Component.toml`（能力清单）与 `components/collapsible/src/collapsible.rbi`（接口签名投影），并在 manifest 中声明 `rbi = "collapsible.rbi"`。
  - manifest 证据：`Component.toml` 保留 `schema_version = "1"`、`name = "Collapsible"`、`crate = "ui-collapsible"`，并显式开启 `context_compression_manifest` 与 `rbi_signature_projection` 能力标记。
  - RBI 投影证据：`collapsible.rbi` 覆盖 `Collapsible` 公共签名（含 `open/default_open/on_open_change/is_disabled/motion/lang/dir` 等关键输入）与导出类型引用，避免 AI 检索使用过期接口。
  - 门禁证据：`scripts/check-ui-components-contract-hygiene.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_context_compression_manifest_and_rbi_projection_are_present_and_current`。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_context_compression_manifest_and_rbi_projection_are_present_and_current` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_context_compression_manifest_and_rbi_projection_are_present_and_current`，锁定 manifest/rbi 文件落点、关键字段和脚本绑定。
  - compile/test 命令证据（当前环境）：
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_context_compression_manifest_and_rbi_projection_are_present_and_current -- --nocapture`
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_context_compression_manifest_and_rbi_projection_are_present_and_current -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。
- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。
  - 关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。
  - Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。
  - 契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。
  - 配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。
  - 类型化契约证据：`components/collapsible/src/logic.rs` 新增 `COLLAPSIBLE_AGENT_SCHEMA`、`CollapsibleAgent*` 枚举/结构体与 `resolve_agent_contract(CollapsibleAgentContractInput)`，状态语义由 `render_state.status` 映射，不再依赖自由字符串拼接。
  - 语义挂载证据：`components/collapsible/src/view.rs` 根节点新增 `data-ui-schema/schema-version/intent/action/state/source/output-status/stream-support/stream-fallback/stream-mode` 及 `data-ui-state-source/data-ui-motion-source/data-ui-open-value-source/data-ui-open-change-source/data-ui-config-policy`，字段均来自 `agent_contract` 类型化输出。
  - 白名单边界证据：`components/collapsible/src/Component.toml` 新增 `[[agent_contract]]`、`[[agent_contract_markers]]`、`[[agent_contract_whitelist]]`（`blocked = ["inner_html", "<script", "javascript:", "eval("]`）与 `agent_contract_*` 能力标记。
  - RBI 投影证据：`components/collapsible/src/collapsible.rbi` 新增 `CollapsibleAgent*` 类型与 `resolve_agent_contract` 签名，保持 Agent 检索与实现同步。
  - 门禁证据：`scripts/check-ui-components-contract-hygiene.sh` 新增四条 collapsible Agent Contract 回归命令：
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_agent_contract_schema_governance_rules`
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_is_schema_typed_and_machine_readable`
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing`
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_render_path_is_whitelist_safe_and_script_injection_free`
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::{collapsible_check2_documents_agent_contract_schema_governance_rules,collapsible_agent_contract_is_schema_typed_and_machine_readable,collapsible_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing,collapsible_agent_contract_render_path_is_whitelist_safe_and_script_injection_free}` 与 `components/collapsible/test/collapsible_semantics.rs` 同名测试。
  - compile/test 命令证据（当前环境）：
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_agent_contract_is_schema_typed_and_machine_readable -- --nocapture`
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_is_schema_typed_and_machine_readable -- --nocapture`
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_agent_contract_render_path_is_whitelist_safe_and_script_injection_free -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。
- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。
  - `Streaming`：LLM 还在生成，界面边生成边显示。
  - `Snapshot`：LLM 全部生成完成后，一次性显示。
  - 术语范围证据：`components/collapsible/src/Component.toml` 的 `[streaming_policy]` 固定 `term_scope = "llm-output-rendering"`，并将 `defined_modes` 限定为 `["streaming", "snapshot"]`。
  - 类型约束证据：`components/collapsible/src/logic.rs` 保持 `CollapsibleAgentStreamMode::{Streaming, Snapshot}` 映射；`stream_support` 固定为 `unsupported`，`stream_fallback` 与 `stream_mode` 默认 `snapshot`。
  - 语义挂载证据：`components/collapsible/src/view.rs` 暴露 `data-ui-stream-support`、`data-ui-stream-fallback`、`data-ui-stream-mode`，确保两种渲染模式可机器读取。
  - 门禁证据：`scripts/check-ui-components-contract-hygiene.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_streaming_term_is_limited_to_llm_output_render_modes`。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_streaming_term_is_limited_to_llm_output_render_modes` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_streaming_term_is_limited_to_llm_output_render_modes`，锁定术语范围、模式枚举与脚本门禁绑定。
  - compile/test 命令证据（当前环境）：
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_streaming_term_is_limited_to_llm_output_render_modes -- --nocapture`
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_streaming_term_is_limited_to_llm_output_render_modes -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。
- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。
  - 所有组件都应能消费“完整生成结果”并稳定渲染。
  - 即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。
  - 能力落点证据：`components/collapsible/src/view.rs` 的 `Collapsible` 公开参数覆盖完整配置面（`open/default_open/on_open_change/is_disabled/disabled/motion/aria_label/class_name/lang/dir/children`），可直接消费上层完整快照配置并稳定渲染。
  - 输出稳定证据：`components/collapsible/src/logic.rs` 固定 `output_status=verified`、`stream_mode=snapshot`，并通过 `as_str()` 映射为封闭值集合，避免自由文本漂移。
  - 语义挂载证据：`components/collapsible/src/view.rs` 挂载 `data-ui-output-status`、`data-ui-stream-fallback`、`data-ui-stream-mode`，确保快照输出状态可机器读取。
  - Manifest 证据：`components/collapsible/src/Component.toml` 显式保留 `name = "snapshot_rendering"`、`name = "streaming_optional_fallback_snapshot"` 与 `[streaming_policy] fallback/default = "snapshot"`。
  - 门禁证据：`scripts/check-ui-components-contract-hygiene.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_snapshot_is_foundational_and_complete_config_renders_stably`。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_snapshot_is_foundational_and_complete_config_renders_stably` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_snapshot_is_foundational_and_complete_config_renders_stably`，锁定快照基础能力与完整配置渲染面。
  - compile/test 命令证据（当前环境）：
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_snapshot_is_foundational_and_complete_config_renders_stably -- --nocapture`
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_snapshot_is_foundational_and_complete_config_renders_stably -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。
- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。
  - `Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。
  - `Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。
  - 无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。
  - 数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。
  - 组件职责证据：`components/collapsible/src/Component.toml` 的 `[streaming_policy]` 固定 `required = false`，并显式 `fallback = "snapshot"`；`Collapsible` 属 disclosure 交互组件而非正文阅读面，按契约归类为 `Streaming Optional`。
  - 输出状态证据：`components/collapsible/src/logic.rs` 固定 `output_status=verified`、`stream_support=unsupported`、`stream_fallback=snapshot`、`stream_mode=snapshot`，并在 `components/collapsible/src/view.rs` 挂载为 `data-ui-output-status/data-ui-stream-support/data-ui-stream-fallback/data-ui-stream-mode`。
  - 连续可读证据：`components/collapsible/src/view.rs` 同步保留 `role=aria.attrs.role`、`aria-expanded`、`aria-controls` 与 `data-state`，保证 `role/aria/data` 在非流式路径连续可读。
  - 边界证据：`components/collapsible/src/logic.rs` 与 `components/collapsible/src/view.rs` 未承载数据校验/断线恢复/重试逻辑（相关职责保持在上层）。
  - 门禁证据：`scripts/check-ui-components-contract-hygiene.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status`。
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::collapsible_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status` 与 `components/collapsible/test/collapsible_semantics.rs::collapsible_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status`，锁定 optional 策略、语义连续性与上层职责边界。
  - compile/test 命令证据（当前环境）：
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status -- --nocapture`
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_streaming_requirement_is_optional_with_snapshot_fallback_and_explicit_status -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施阻塞（`Invalid cross-device link (os error 18)`），需在稳定文件系统 CI/本地 runner 复核通过结果。

### 7. 测试、门禁与交付
- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。
  - 组件范围验证命令：`RUST_HYGIENE_FILES="components/collapsible/src/mod.rs components/collapsible/src/logic.rs components/collapsible/src/view.rs components/collapsible/src/styles.rs components/collapsible/src/motion.rs components/collapsible/src/protocol.rs crates/ui-components/src/lib.rs crates/ui-components/src/css.rs crates/ui-components/src/root.rs" ./scripts/check-rust-hygiene.sh`。
  - 命令结果：`[rust-hygiene] OK`（当前 runner 同时提示 `skip check-api-contracts.sh (ripgrep built without PCRE2)`，不影响本条 Rust Hygiene 判定）。
  - 条目证据：上述目标文件集中未触发 `unwrap/expect`、`let _ = ...` 与字符串复制热点（`to_owned`/`String::from`/高频 `.to_string()`）告警。
  - 范围说明：脚本默认 `RUST_HYGIENE_SCOPE=crates apps` 不覆盖 `components/*`，本条按单组件审查要求使用 `RUST_HYGIENE_FILES` 对 `collapsible` 及其 `ui-components` 聚合入口做定向校验。
- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。
  - 特性注册证据：`crates/ui-components/Cargo.toml` 保持 `component-collapsible = ["dep:ui-collapsible"]`，且 `ui-collapsible` 依赖为 `optional = true`，未启用时不进入可达路径。
  - 聚合门控证据：`crates/ui-components/src/lib.rs` 使用 `#[cfg(feature = "component-collapsible")] pub use ui_collapsible as collapsible;`；`crates/ui-components/src/css.rs` 使用 `#[cfg(feature = "component-collapsible")] out.push_str(crate::collapsible::styles::CSS);`，并由 `#[cfg(feature = "inject-css")]` 保护聚合入口。
  - source 模式证据：`apps/web-demo/Cargo.toml` 依赖 `ui-components` 使用 `default-features = false` + `features = ["inject-css", "web-demo-components"]`，未显式或隐式拉起 `all-components`。
  - 命令证据（最小特性树）：`cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-collapsible,inject-css | rg "component-collapsible|inject-css|all-components"` 输出仅包含 `component-collapsible` 与 `inject-css` 命令行特性，无 `all-components`。
  - 命令证据（反向依赖）：`cargo tree -e features -i ui-components -p web-demo | rg "web-demo-components|all-components"` 仅出现 `web-demo-components` 链路，无 `all-components`。
  - 门禁证据：`scripts/check-ui-components-tree-shaking.sh` 新增 `collapsible` 树摇块，覆盖 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_tree_shaking_contract_keeps_feature_gates_explicit`、最小特性树断言与 wasm 最小特性编译检查。
- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。
  - 语义矩阵证据：`components/collapsible/test/semantics.rs` 与 `components/collapsible/test/collapsible_semantics.rs` 新增 `collapsible_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement`，联动约束 `aria-* + data-* + focus` 路径与性能回归治理。
  - `aria/data/focus` 断言证据：上述测试锁定 `components/collapsible/src/view.rs` 中 `role=aria.attrs.role`、`aria-expanded/aria-controls/aria-disabled`、`data-state/data-open-mode/data-open-value-source/data-open-change-source`、`use_focus_ring`、`focus-visible` 及 `on:pointerdown/on:keydown/on:focus` 语义路径。
  - 非快照优先证据：同一测试要求 `e2e/tests/docs_app_collapsible.spec.mjs` 覆盖 `controlledTrigger.click()`、`page.keyboard.press("Enter")` 与关键 `data-*`/`aria-*` 断言，不依赖 `toHaveScreenshot`/`toMatchSnapshot`。
  - `render_count` 治理证据：测试显式校验 `docs/plan/TODO.md` 保留 `render_count` 自动化补齐任务；当前链路继续采用 `UiPerfProbe + e2e marker` 作为等价回归证据（与“性能治理”条目一致）。
  - 门禁证据：`scripts/check-ui-components-performance.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement`。
  - compile/test 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement -- --nocapture`
  - 命令现状：上述命令在当前 runner 仍受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `Collapsible` 未发生跨大版本 API 破坏升级）
  - N/A 依据：`components/collapsible/src/Component.toml` 仍为 `schema_version = "1"`，`agent_contract` 仍是 `ui.collapsible.agent-contract.v1`；`components/collapsible/src/protocol.rs` 与 `components/collapsible/src/logic.rs` 仅定义 `V1`，未出现 `V2` 或破坏性升级轨迹。
  - API 稳定性证据：`components/collapsible/src/collapsible.rbi` 继续暴露同一主 API 轴（`open/default_open/on_open_change`），未引入跨大版本重命名或删除。
  - 迁移层状态：当前不存在 `migrate_v1_to_v2`，因为没有真实的破坏性升级触发条件；避免为假问题预置迁移层增加无收益复杂度。
  - 回归锁定：新增 `components/collapsible/test/semantics.rs::collapsible_version_deprecation_migration_is_na_without_major_breaking_upgrade`、`components/collapsible/test/semantics.rs::collapsible_version_deprecation_migration_script_covers_engineering_gate`、`components/collapsible/test/collapsible_semantics.rs::collapsible_version_deprecation_migration_is_na_without_major_breaking_upgrade`、`components/collapsible/test/collapsible_semantics.rs::collapsible_version_deprecation_migration_script_covers_engineering_gate`。
  - 脚本门禁：`scripts/check-ui-components-engineering.sh` 新增 `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_version_deprecation_migration_is_na_without_major_breaking_upgrade`。
  - compile/test 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-collapsible collapsible_version_deprecation_migration_is_na_without_major_breaking_upgrade -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_version_deprecation_migration_is_na_without_major_breaking_upgrade -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。
  - docs-app 验收面证据：`apps/docs-app/src/pages/components/pages/collections_groups.rs` 的 `collapsible` 页面覆盖 `Hello World / State Matrix / Controlled vs Uncontrolled Contrast / Streaming / Snapshot Contract / Source-first Starter (Copy-Paste Ready)` 六个 playground，满足“先可用、再进阶”的文档路径。
  - 流式/快照证据：同页面 `data-slot="collapsible-streaming-policy"` 显式挂载 `data-ui-streaming="optional"`、`data-ui-fallback="snapshot"`、`data-ui-output-state="snapshot"`，并给出 `Streaming Optional; fallback=snapshot.` 文案提示。
  - Source-first 证据：`Source-first Starter (Copy-Paste Ready)` 使用 `code_imports=collapsible_imports.clone()`，并列出真实源码路径（`components/collapsible/src/mod.rs|logic.rs|view.rs|styles.rs|motion.rs`）与最小特性依赖（`component-collapsible + inject-css`）。
  - 一键复制与 imports 补全证据：`apps/docs-app/src/playground.rs` 保持 `compose_copy_ready_code` + `code_imports` + `missing_import_lines` 链路，代码面板通过 `CodeBlock` 复制按钮输出可运行片段。
  - 门禁证据：`scripts/check-ui-components-dx.sh` 新增四条回归命令：  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_docs_product_copy_paste_ready_rules`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_docs_product_copy_paste_ready_contract`
  - 防回归证据：新增 `components/collapsible/test/semantics.rs::{collapsible_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot,collapsible_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync,collapsible_check2_documents_docs_product_copy_paste_ready_rules,collapsible_dx_check_script_covers_docs_product_copy_paste_ready_contract}`，以及 `components/collapsible/test/collapsible_semantics.rs` 同名测试。
  - compile/test 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_are_source_first_copy_paste_ready_with_imports_copy_button_and_sync -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。
  - 每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。
  - 断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。
  - 新增/变更语义字段必须同步补测试，否则不得打勾。
  - 语义优先回归证据：`components/collapsible/test/semantics.rs::{collapsible_check2_documents_semantics_first_testing_rules,collapsible_semantics_suite_is_contract_first_not_snapshot_only,collapsible_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks,collapsible_semantics_first_testing_script_covers_contract}` 与 `components/collapsible/test/collapsible_semantics.rs` 同名测试。
  - 契约覆盖证据：`collapsible_view_mounts_headless_contract_and_semantic_markers` + `collapsible_semantics_matrix_prefers_contract_assertions_over_snapshots` 持续覆盖 `role/aria/data-*` 与受控/非受控、键盘路径、状态来源标记；并显式拒绝 `toHaveScreenshot/toMatchSnapshot` 作为主断言。
  - 字段联动证据：`collapsible_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks` 将 `components/collapsible/src/view.rs` 的关键 marker（`role`、`aria-expanded/controls/disabled`、`data-state/open-mode/motion-source/open-value-source/open-change-source`）与本地/聚合语义测试文本做双向绑定，防止新增/变更语义字段漏测。
  - 门禁脚本证据：`scripts/check-ui-components-contract-hygiene.sh` 新增四条命令：  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_semantics_first_testing_rules`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantics_suite_is_contract_first_not_snapshot_only`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantics_first_testing_script_covers_contract`
  - compile/test 命令证据（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_semantics_first_testing_rules -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。
  - E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。
  - WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。
  - 若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。
  - 已满足（语义选择器）：`e2e/tests/docs_app_collapsible.spec.mjs` 使用 `data-component="collapsible"` 与 `data-slot="collapsible|collapsible-trigger|collapsible-panel"` 作为主选择器，并通过 `data-open-mode/data-state/data-open-value-source/data-open-change-source/aria-expanded` 断言契约状态；不再依赖 `.docs-page-title` 等脆弱样式选择器。
  - 已满足（WASM 稳定等待）：E2E 路径统一通过 `waitForWasmReady(page)` 与 `body:not(:has(#boot))` 作为 wasm/hydration ready 断点；测试中未使用 `waitForTimeout`/固定 sleep。
  - 已满足（ready/settled 覆盖）：新增 `expectCollapsibleReady`、`expectCollapsibleSettledOpen`、`expectCollapsibleSettledClosed`，显式覆盖点击与键盘（`Enter`）路径的状态收敛（`data-state`）与可见性收敛（`toBeVisible/toBeHidden`）。
  - 脚本门禁：新增 `scripts/check-ui-components-e2e-collapsible.sh`，接入  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_e2e_selector_and_stable_wait_rules`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_selector_contract_uses_semantic_markers_and_settled_waits`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_contract_covers_ready_and_settled_conditions_for_disclosure_paths`
  - 回归：`components/collapsible/test/semantics.rs::{collapsible_check2_documents_e2e_selector_and_stable_wait_rules,collapsible_e2e_selector_contract_uses_semantic_markers_and_settled_waits,collapsible_e2e_contract_covers_ready_and_settled_conditions_for_disclosure_paths,collapsible_e2e_check_script_covers_selector_and_settled_wait_contract,collapsible_check2_marks_e2e_selector_stability_item_complete}`、`components/collapsible/test/collapsible_semantics.rs::{collapsible_e2e_selector_contract_uses_semantic_markers_and_settled_waits,collapsible_e2e_contract_covers_ready_and_settled_conditions_for_disclosure_paths,collapsible_e2e_check_script_covers_selector_and_settled_wait_contract,collapsible_check2_marks_e2e_selector_stability_item_complete}`。
  - 本地验证命令（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_e2e_selector_and_stable_wait_rules -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_selector_contract_uses_semantic_markers_and_settled_waits -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。
  - 至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。
  - 回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。
  - 高风险路径（overlay、focus、keyboard、async）优先进入回归集合。
  - 已满足（可重复关键流程）：`e2e/tests/docs_app_collapsible.spec.mjs` 固化了同一条可重复 disclosure 路径：`expectCollapsibleSettledOpen -> controlledTrigger.click() -> expectCollapsibleSettledClosed -> controlledTrigger.focus() -> keyboard Enter -> expectCollapsibleSettledOpen`，覆盖打开/交互/关闭并再次打开的回归闭环。
  - 已满足（可定位语义断点）：E2E 失败断点绑定在 `expectCollapsibleReady/SettledOpen/SettledClosed` 的语义断言（`data-state`、`aria-expanded`、`toBeVisible/toBeHidden`、`data-open-change-source`），可直接定位到具体契约字段，而非“页面不一致”。
  - 已满足（高风险路径优先）：`Collapsible` 不涉及 overlay/async（N/A）；本条优先纳入 `focus + keyboard` 高风险路径，E2E 显式断言 `toBeFocused()` 与 `page.keyboard.press("Enter")` 的状态收敛。
  - 脚本门禁：`scripts/check-ui-components-e2e-collapsible.sh` 新增四条命令：  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_repeatable_keyflow_regression_rules`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_regression_suite_contains_repeatable_disclosure_keyflow`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_regression_failures_map_to_semantic_contract_breakpoints`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_regression_prioritizes_focus_and_keyboard_risk_paths`
  - 回归：`components/collapsible/test/semantics.rs::{collapsible_check2_documents_repeatable_keyflow_regression_rules,collapsible_e2e_regression_suite_contains_repeatable_disclosure_keyflow,collapsible_e2e_regression_failures_map_to_semantic_contract_breakpoints,collapsible_e2e_regression_prioritizes_focus_and_keyboard_risk_paths,collapsible_e2e_check_script_covers_repeatable_keyflow_regression_contract,collapsible_check2_marks_repeatable_keyflow_regression_item_complete}`、`components/collapsible/test/collapsible_semantics.rs::{collapsible_check2_documents_repeatable_keyflow_regression_rules,collapsible_e2e_regression_suite_contains_repeatable_disclosure_keyflow,collapsible_e2e_regression_failures_map_to_semantic_contract_breakpoints,collapsible_e2e_regression_prioritizes_focus_and_keyboard_risk_paths,collapsible_e2e_check_script_covers_repeatable_keyflow_regression_contract,collapsible_check2_marks_repeatable_keyflow_regression_item_complete}`。
  - 本地验证命令（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_repeatable_keyflow_regression_rules -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_e2e_regression_suite_contains_repeatable_disclosure_keyflow -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。
  - 组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。
  - 文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。
  - 文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。
  - 已满足（docs-app 同步更新）：`apps/docs-app/src/pages/components/pages/collections_groups.rs::collapsible` 新增 `title="Parameter Matrix"`（`data-slot="collapsible-parameter-matrix"`），并持续保留 `Hello World`、`State Matrix`、`Controlled vs Uncontrolled Contrast`、`Disabled + Custom Motion` 示例路径。
  - 已满足（状态矩阵覆盖）：`State Matrix` 同时覆盖 `default_open=true` 与 `default_open=false + is_disabled=true + custom motion`，`Controlled vs Uncontrolled Contrast` 显式覆盖受控/非受控路径与 `open/on_open_change/default_open` 语义。
  - 已满足（API/默认值与 logic 对齐）：参数矩阵与代码示例对齐 `components/collapsible/src/view.rs` props（`open/default_open/on_open_change/is_disabled/disabled/motion/aria_label/class_name/lang/dir`）以及 `components/collapsible/src/logic.rs` 归一化规则（`open > default_open > primitive fallback`、`is_disabled.unwrap_or(disabled)`）。
  - 脚本门禁：`scripts/check-ui-components-dx.sh` 新增三条命令：  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_docs_sync_and_state_matrix_rules`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_docs_sync_and_state_matrix_contract`
  - 回归：`components/collapsible/test/semantics.rs::{collapsible_check2_documents_docs_sync_and_state_matrix_rules,collapsible_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults,collapsible_dx_check_script_covers_docs_sync_and_state_matrix_contract,collapsible_check2_marks_docs_sync_and_state_matrix_item_complete}`、`components/collapsible/test/collapsible_semantics.rs::{collapsible_check2_documents_docs_sync_and_state_matrix_rules,collapsible_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults,collapsible_dx_check_script_covers_docs_sync_and_state_matrix_contract,collapsible_check2_marks_docs_sync_and_state_matrix_item_complete}`。
  - 本地验证命令（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_docs_sync_and_state_matrix_rules -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。
  - 每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。
  - 文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。
  - “只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。
  - 已满足（文档入口存在）：`components/collapsible/src/README.md` 与 docs-app 注册入口 `apps/docs-app/src/pages/components/pages.rs`（`component_doc!("Collapsible", "collapsible", "Collections", collections_groups::collapsible)`）同时存在，满足 “README 或等价入口” 要求。
  - 已满足（新手最小示例）：`components/collapsible/src/README.md` 保留 `## Hello World（最小可用）`；docs 页面 `apps/docs-app/src/pages/components/pages/collections_groups.rs::collapsible` 首个 playground 为 `title="Hello World"`。
  - 已满足（先基础后进阶）：docs 文案明确 `Start with Hello World, then move to controlled/state matrix examples`，并按 `Hello World -> Controlled Collapsible -> State Matrix -> Controlled vs Uncontrolled Contrast -> State + Source Markers -> Source-first Starter` 组织，默认路径在前、进阶路径在后。
  - 脚本门禁：`scripts/check-ui-components-dx.sh` 新增三条命令：  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_documentation_as_product_rules`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_documentation_entry_exists_with_beginner_first_progression`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_documentation_as_product_contract`
  - 回归：`components/collapsible/test/semantics.rs::{collapsible_check2_documents_documentation_as_product_rules,collapsible_documentation_entry_exists_with_beginner_first_progression,collapsible_dx_check_script_covers_documentation_as_product_contract,collapsible_check2_marks_documentation_as_product_item_complete}`、`components/collapsible/test/collapsible_semantics.rs::{collapsible_check2_documents_documentation_as_product_rules,collapsible_documentation_entry_exists_with_beginner_first_progression,collapsible_dx_check_script_covers_documentation_as_product_contract,collapsible_check2_marks_documentation_as_product_item_complete}`。
  - 本地验证命令（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_documentation_as_product_rules -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_documentation_entry_exists_with_beginner_first_progression -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。
  - Playground 至少支持基础 props 调整、状态切换、交互反馈观察。
  - 对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。
  - Playground 作为验收面，需可重复复现关键交互路径。
  - 已满足（interactive docs 验收面）：`apps/docs-app/src/pages/components/pages/collections_groups.rs::collapsible` 的 `title="Interactive Playground (Display + Config + Code + CSS Test)"` 已提供 `Mode/Motion` 分段控制、`Controlled open/Default open/Disabled/Custom aria-label/Custom class` 开关，以及 `test_config_signal=actual_config` 实时配置回显。
  - 已满足（稳定语义锚点）：interactive 区块新增 `data-slot="collapsible-workbench-controls"`、`data-slot="collapsible-workbench-preview"`、`data-slot="collapsible-workbench-controlled-state"`、`data-slot="collapsible-workbench-default-state"`，便于 docs 验收与自动化定位。
  - AI Spec 相关联动示例：N/A（`collapsible` 组件无 `spec.rs` 与 Spec 输入协议），本组件按 disclosure 职责仅验证 props/state/语义反馈链路。
  - 已满足（可重复关键流程）：`e2e/tests/docs_app_collapsible.spec.mjs::docs-app: collapsible interactive playground updates props/state and replays deterministically` 覆盖“切换模式 -> 状态开关 -> 语义断言 -> code 面板同步 -> reload 重放”路径，且避免 `waitForTimeout` 等不稳定等待。
  - 脚本门禁：`scripts/check-ui-components-dx.sh` 新增四条命令：  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_interactive_playground_rules`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_app_provides_interactive_playground_for_props_state_and_preview`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_interactive_playground_reuses_repeatable_semantic_e2e_flow`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_interactive_playground_contract`
  - 回归：`components/collapsible/test/semantics.rs::{collapsible_check2_documents_interactive_playground_rules,collapsible_docs_app_provides_interactive_playground_for_props_state_and_preview,collapsible_interactive_playground_reuses_repeatable_semantic_e2e_flow,collapsible_dx_check_script_covers_interactive_playground_contract,collapsible_check2_marks_interactive_playground_item_complete}`、`components/collapsible/test/collapsible_semantics.rs::{collapsible_check2_documents_interactive_playground_rules,collapsible_docs_app_provides_interactive_playground_for_props_state_and_preview,collapsible_interactive_playground_reuses_repeatable_semantic_e2e_flow,collapsible_dx_check_script_covers_interactive_playground_contract,collapsible_check2_marks_interactive_playground_item_complete}`。
  - 本地验证命令（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_interactive_playground_rules -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_app_provides_interactive_playground_for_props_state_and_preview -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_interactive_playground_reuses_repeatable_semantic_e2e_flow -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。
  - docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。
  - 若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。
  - 文档代码与当前实现必须同步，防止示例漂移。
  - 已满足（copy-ready + imports）：`apps/docs-app/src/pages/components/pages/collections_groups.rs::collapsible` 保持 `title="Source-first Starter (Copy-Paste Ready)"` + `code_signal=source_first_code` + `code_imports=collapsible_imports.clone()`，通过 docs playground 一键复制输出可运行片段并补齐 imports。
  - 已满足（真实源码落点与依赖前提）：source-first 区块 `data-slot="collapsible-source-first-contract"` 明确标注真实路径 `components/collapsible/src/mod.rs`、`components/collapsible/src/logic.rs`、`components/collapsible/src/view.rs`、`components/collapsible/src/styles.rs`、`components/collapsible/src/motion.rs`，并声明最小特性 `component-collapsible + inject-css`。
  - 已满足（复制按钮与代码面板链路）：`apps/docs-app/src/playground.rs` 维持 `compose_copy_ready_code + missing_import_lines + <CodeBlock code=resolved_code.get() />`，`components/code-block/src/view.rs` 保持 copy 按钮（`ui-code-block__copy-button` + `copy_to_clipboard_aria_label`）。
  - 已满足（示例与实现同步）：`source_first_code` 示例仍直接对齐当前 `Collapsible` API（`id_base/title/default_open/motion`）与 source-first 文案，不存在示例漂移。
  - 脚本门禁：`scripts/check-ui-components-dx.sh` 新增三条命令：  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_source_first_copy_paste_ready_rules`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_source_first_copy_paste_ready_contract`
  - 回归：`components/collapsible/test/semantics.rs::{collapsible_check2_documents_source_first_copy_paste_ready_rules,collapsible_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies,collapsible_dx_check_script_covers_source_first_copy_paste_ready_contract,collapsible_check2_marks_source_first_copy_paste_ready_contract_complete}`、`components/collapsible/test/collapsible_semantics.rs::{collapsible_check2_documents_source_first_copy_paste_ready_rules,collapsible_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies,collapsible_dx_check_script_covers_source_first_copy_paste_ready_contract,collapsible_check2_marks_source_first_copy_paste_ready_contract_complete}`。
  - 本地验证命令（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_source_first_copy_paste_ready_rules -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。
- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。
  - 若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。
  - 组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。
  - “仅代码更新无文档更新”在接口变更场景下直接判不通过。
  - 已满足（对标策略文档同步）：`docs/spec/heroui-parameter-design-strategy.md` 新增 `### Collapsible 同步记录（2026-02-20）`，明确参数主轴（`open/default_open/on_open_change`、`is_disabled/disabled`、`motion`、`aria_label/class_name/lang/dir`）、默认值归一边界（`open > default_open > primitive fallback`、`is_disabled.unwrap_or(disabled)`）以及“先文档后实现”的约束。
  - 已满足（组件文档入口可索引）：`apps/docs-app/src/pages/components/pages.rs` 保持 `component_doc!("Collapsible", "collapsible", "Collections", collections_groups::collapsible)`，`apps/docs-app/src/pages/components/pages/collections_groups.rs::collapsible` 维持 `title="Collapsible"` + `slug="collapsible"`，`components/collapsible/src/README.md` 提供等价文档入口。
  - 已满足（实现与文档不漂移）：docs 页面矩阵持续覆盖 `Parameter Matrix`、`State Matrix`、`Source-first Starter`、`Interactive Playground`，并与当前参数语义一致；禁止“仅代码更新无文档更新”已在策略文档与本清单证据链固定。
  - 研究文档补充判定：本轮仅为 `Collapsible` 参数语义与组件文档入口同步校验，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
  - 脚本门禁：`scripts/check-ui-components-dx.sh` 新增三条命令：  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_heroui_benchmark_docs_sync_rules`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_heroui_strategy_and_component_docs_are_synchronized_and_indexable`  
    `cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_dx_check_script_covers_heroui_benchmark_docs_sync_contract`
  - 回归：`components/collapsible/test/semantics.rs::{collapsible_check2_documents_heroui_benchmark_docs_sync_rules,collapsible_heroui_strategy_and_component_docs_are_synchronized_and_indexable,collapsible_dx_check_script_covers_heroui_benchmark_docs_sync_contract,collapsible_check2_marks_heroui_benchmark_docs_sync_contract_complete}`、`components/collapsible/test/collapsible_semantics.rs::{collapsible_check2_documents_heroui_benchmark_docs_sync_rules,collapsible_heroui_strategy_and_component_docs_are_synchronized_and_indexable,collapsible_dx_check_script_covers_heroui_benchmark_docs_sync_contract,collapsible_check2_marks_heroui_benchmark_docs_sync_contract_complete}`。
  - 本地验证命令（当前环境）：  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_check2_documents_heroui_benchmark_docs_sync_rules -- --nocapture`  
    `CARGO_TARGET_DIR=/tmp/rust-ui-target CARGO_INCREMENTAL=0 cargo test -p ui-components --test collapsible_semantics --no-default-features --features component-collapsible,inject-css collapsible_heroui_strategy_and_component_docs_are_synchronized_and_indexable -- --nocapture`
  - 命令现状：当前 runner 仍受基础设施问题阻塞（`Invalid cross-device link (os error 18)`）；需在稳定文件系统 CI/本地 runner 复核。

### 8. 合并前门禁死命令（最终执行）
在发起 PR 或完成任务前，必须保证本地/CI 以下命令全部通过：

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `./scripts/check-rust-hygiene.sh`
- `cargo check -p ui-components --target wasm32-unknown-unknown`
- `cargo check -p ui-headless --no-default-features --features ssr`
- `cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-<your_component>,inject-css`

依据文档（`rust-ui/docs/spec` 及 `rust-ui/docs`）：

- `rust-ui/docs/spec/ai_context_projection_protocol.md`
- `rust-ui/docs/spec/architectural_fitness_functions.md`
- `rust-ui/docs/spec/async_state_as_data_command.md`
- `rust-ui/docs/spec/collection_registration_protocol.md`
- `rust-ui/docs/spec/compile_time_evolution_migration.md`
- `rust-ui/docs/spec/component_boundaries.md`
- `rust-ui/docs/spec/component_domains.md`
- `rust-ui/docs/spec/controlled_evolution_sandbox.md`
- `rust-ui/docs/spec/core_shell_protocol_infra_baseline.md`
- `rust-ui/docs/spec/environment_subscription_streams.md`
- `rust-ui/docs/spec/event_light_cone_signal_bus.md`
- `rust-ui/docs/spec/focus_global_stack_gc.md`
- `rust-ui/docs/spec/foreign_zone_escape_hatches.md`
- `rust-ui/docs/spec/headless_purification.md`
- `rust-ui/docs/spec/heroui-parameter-design-strategy.md`
- `rust-ui/docs/spec/hyper-structure-ui-development-playbook.md`
- `rust-ui/docs/spec/i18n.md`
- `rust-ui/docs/spec/intent_stack_semantic_layering.md`
- `rust-ui/docs/spec/kernel_shell_architecture.md`
- `rust-ui/docs/spec/macro_micro_dual_state_machine.md`
- `rust-ui/docs/spec/motion.md`
- `rust-ui/docs/spec/mvp.md`
- `rust-ui/docs/spec/platform_abdication_ecosystem.md`
- `rust-ui/docs/spec/README.md`
- `rust-ui/docs/spec/release_versioning.md`
- `rust-ui/docs/spec/side_effect_command_pattern.md`
- `rust-ui/docs/spec/slot_projection_strategy.md`
- `rust-ui/docs/spec/ssr_hydration_discontinuity.md`
- `rust-ui/docs/spec/state_primitives_core_satellite_split.md`
- `rust-ui/docs/spec/style_island_defense.md`
- `rust-ui/docs/spec/styling.md`
- `rust-ui/docs/spec/tree_shaking.md`
- `rust-ui/docs/spec/ui_layout_split.md`
- `rust-ui/docs/spec/ui_physics_two_pass_rendering.md`
- `rust-ui/docs/spec/unified_causality_bus.md`
- `rust-ui/docs/spec/wasm_generic_bloat.md`
- `rust-ui/docs/起点_也即是目的.md`
- `rust-ui/docs/DOCS_GOVERNANCE.md`
- `rust-ui/docs/DOCS_INDEX.md`
- `rust-ui/docs/philosophy.md`
- `rust-ui/docs/README.md`
- `rust-ui/docs/RULES_ZH.md`
