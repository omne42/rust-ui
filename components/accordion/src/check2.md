# Accordion 单组件 Check2（执行结果）

> 组件路径：`components/accordion`  
> 执行日期：2026-02-20  
> 目标：确认 Accordion 在 Hyper-Structure 清单下可合并、可回归、可追踪。

## 0. 适用范围与顺序纪律
- [x] 仅评估 `Accordion` 组件改动，不替代仓库级治理。
- [x] 已按“架构与状态 -> 实现细节”顺序核对。
- [x] 已声明风险边界：跨层问题优先回迁到 `ui-state-primitives/ui-headless/ui-motion/ui-theme`，不在 `view.rs` 打补丁。

## 1. 架构边界与分层约束（Kernel/Shell 总线）
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
  - 结论：通过。证据：`components/accordion/src/logic.rs` 使用 `ui_state_primitives::expansion::{normalize_open_indices, summarize, toggle_open_indices}`，未引入组件内状态机复刻。
- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。
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
  - 结论：通过。证据：`components/accordion/src/view.rs` 使用 `use_roving_tabindex/use_press/use_focus_ring/use_hover/disclosure_trigger_attrs`，语义挂载在 `view.rs`，样式/动效未下沉到 headless。
- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。
  - 放在 `crates/ui-motion`：通用动画数学与执行后端（spring solver、keyframe sampling、easing registry、driver adapters），以及 `wasm/non-wasm` 适配与 `reduced-motion` 执行策略。
  - 放在 `crates/ui-components/src/<component>/motion.rs`：把组件语义状态（open/closed、enter/exit、active/inactive）映射为 `ui-motion` contract，绑定目标节点并调用 attach。
  - 禁止放在 `crates/ui-motion`：组件 slot 结构、组件专属状态机、ARIA/keyboard 语义、业务文案与业务分支。
  - 禁止放在组件 `motion.rs`：自实现 spring/keyframe/driver 执行器；跨组件共享动效算法必须回迁 `ui-motion`。
  - 动效参数优先来自 token/theme；禁止在组件样式与逻辑中散落硬编码时长/曲线/位移常量。
  - 非 wasm 路径必须提供 no-op/stub，保证 SSR/tooling 可编译且行为可预测。
  - 结论：通过。证据：`components/accordion/src/motion.rs` 仅定义 `AccordionMotion`、sanitize、attach；`crates/ui-motion/src/lib.rs` non-wasm stub 编译通过。
- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。
  - Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量；组件只在 `crates/ui-components/src/<component>/styles.rs` 消费。
  - 三轴上下文（`system/color/scale`）在 `theme.rs` 定义；组件在 `logic.rs` 选择并在 `view.rs` 生效，`styles.rs` 只消费变量，不重建主题。
  - Token 分类必须可追溯：分类源在 `tokens.rs`，规范同步 `docs/spec/styling.md`；组件不得引入平行私有 token 命名体系。
  - 量化尺寸基准必须可回归：尺寸基准在 `tokens.rs` 与 `theme.rs` 定义，主题回归在 `crates/ui-theme/tests/token_scale_baseline.rs`，组件语义回归在 `crates/ui-components/tests/<component>_semantics.rs`。
  - 主题调色与语义色对比必须满足 `WCAG 2.1 AA` 基线，并覆盖 Light/Dark/OLED 主题变体。
  - 主题层只输出 `theme/tokens/base css` 与变量；不实现组件结构、交互逻辑、组件级动效编排。
  - 新增视觉语义先补 token，再由组件消费；禁止“组件临时值先落地、后补 token”的倒序流程。
  - 结论：通过。证据：`components/accordion/src/styles.rs` 仅消费 `var(--ui-*)` 变量；`accordion_semantics` 对 token-first 与硬编码禁用有回归断言。
- [x] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。
  - `logic.rs` 负责 props 归一与状态派生；`view.rs` 负责结构渲染与 headless 语义挂载；`styles.rs` 负责 token-first 静态样式；`motion.rs` 负责动效 attach。
  - 组件层不得重写 `status-primitives` 状态机或 `ui-headless` 交互契约；发现即判不通过并回迁到对应层。
  - 对外 API 禁止暴露 `web-sys`/DOM 细节类型；平台差异封装在内部模块。
  - 结论：通过。证据：`components/accordion/src/mod.rs` 维持最小导出面（`Accordion/AccordionItem/AccordionMotion` 等），内部模块私有。

## 2. API 设计与状态内核
- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。
  - 布尔状态统一 `is_*`（如 `is_open`/`is_disabled`），事件统一 `on_*`，默认值统一 `default_*`。
  - 同一语义 across 组件必须同名（如都用 `on_open_change`，禁止同义别名并存）。
  - 公共 API 引入新命名时，需说明与现有命名体系的兼容策略与迁移路径。
  - 结论：通过。证据：`components/accordion/src/view.rs` 对外轴为 `is_disabled`、`open`、`default_open`、`on_open_change`；`crates/ui-components/tests/accordion_semantics.rs` 对 legacy alias 有反向断言。
- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。
  - 受控模式：外部值是单一事实来源，内部不得偷偷写回本地状态。
  - 非受控模式：仅由默认值初始化一次，后续状态由内部原语管理。
  - 受控/非受控切换语义需稳定可测，避免“半受控”隐式行为。
  - 结论：通过。证据：item 级状态轴完整提供 `open + on_open_change + default_open`，并在 `components/accordion/src/view.rs` 通过 `mount_controlled_open_sync_effects` + `plan_open_commit` 统一提交。
- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。
  - 默认值优先级必须可读且可测试（显式规则而非分散 `unwrap_or`）。
  - `view.rs` 不允许再做默认值分支；仅消费 `logic.rs` 的归一化输出。
  - 一旦发现多处默认值来源，直接判不通过并回收至 `logic.rs`。
  - 结论：通过。证据：`components/accordion/src/view.rs` 调用 `logic::normalize_default_open_for_items`；`accordion_semantics` 明确断言 `view.rs` 不出现 `default_open.unwrap_or_default()`。
- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。
  - 输入边界统一进入 `logic.rs`，输出统一为可渲染语义状态与来源标记。
  - 事件处理器只触发状态变更，不重建状态机规则。
  - 样式层只消费状态标记，不承担状态判定职责。
  - 结论：通过。证据：根状态由 `logic::resolve_state`、开闭提交由 `logic::plan_open_commit`、来源标记由 `AccordionOpen*Source` 统一定义；`view.rs` 负责挂载。
- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。
  - 互斥状态优先用 `enum` 建模，利用编译器封住无效组合。
  - 字符串输入若需兼容外部配置，必须先映射到类型化枚举再进入逻辑层。
  - 布尔爆炸（多个 bool 表达一个状态机）应在设计评审阶段直接拦截。
  - 结论：通过。证据：`components/accordion/src/logic.rs` 使用 `AccordionSelectionMode = ExpansionMode`、`AccordionVariant`、`AccordionOpen*Source` 等 enum 轴建模。
- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。
  - 组件中出现可复用状态机实现（受控/非受控、展开规则、选择归一）即判应下沉。
  - 组件与业务全局状态之间必须有适配边界，禁止组件直接依赖业务 store 类型。
  - `logic.rs` 仅做装配与映射，不重新实现状态原语。
  - 结论：通过。证据：`components/accordion/src/logic.rs` 从 `ui_state_primitives::expansion` 直接消费归一/切换能力，无业务 store 依赖。
- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。
  - 无异步交互时需明确标注 N/A 理由（例如“组件无远程请求与异步状态”），不是机械打勾。
  - 有异步交互时，`is_loading`/disabled/`aria-busy`/retry 语义必须成套一致，且对键盘与读屏路径可用。
  - 异步失败态要有可恢复路径（重试或回退），并有语义测试覆盖。
  - 结论：N/A。理由：Accordion 不涉及远程请求或异步状态机，当前无 `is_loading/aria-busy/retry` 语义轴。
- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。
  - 基础用法不得要求用户先理解或手动接线 `ui-state-primitives`/`ui-headless` 状态机。
  - 基础组件 Hello World 示例代码不得超过 5 行（导入与外层模板按仓库约定不计），并可直接运行。
  - 简单需求走简单 API，复杂需求再暴露高级入口：默认 props 覆盖高频场景，高级控制通过受控/扩展参数按需开启。
  - 禁止把内部状态对象作为基础必填参数暴露（例如强制 `state=...` 才能完成点击/展开等基本交互）。
  - docs-app 必须提供最小可用示例，优先展示一眼可懂的默认调用路径。
  - 结论：通过。证据：`crates/ui-components/tests/accordion_semantics.rs` 断言 Hello World ≤ 5 行与默认调用路径；公开 API 不要求注入内部状态对象。
- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。
  - 每个 item 的标题、语义与内容必须在同一 `Item` 结构维度绑定，避免索引配对式隐式约定。
  - `labels + children`、`titles + panels` 等并行数组/并行槽位写法不得作为默认或推荐 API。
  - 不引入这类语法糖：若为配置式输入，仅允许类型化 `ItemSpec`，并在内部映射为显式 `Item` 语义树。
  - 结论：通过。证据：主 API 为 `<Accordion><AccordionItem .../></Accordion>`，`view.rs` 通过 `collect_accordion_items(children)` 收集显式 item 语义，不使用并行数组协议。

## 3. 高级交互与物理机制（Shell/Physics）
- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。
  - 结论：N/A。理由：Accordion 不存在拖拽态与逐帧 Dragging 回路，当前交互为点击/键盘触发展开收敛。
- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。
  - 结论：通过。证据：`components/accordion/src/motion.rs` 的 `attach_panel_motion` 先测量高度再校正动画目标；`sync_open_height` 以 `abs() < 0.5` 阈值幂等收敛，避免循环抖动。
- [ ] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。
  - 结论：不通过。现状：`components/accordion/src/view.rs` 使用一次性收集 `collect_accordion_items(children)` + `assign_item_keys`，尚未实现显式 `RegistrationContext(Register/Unregister)` 协议。
- [ ] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。
  - 结论：不通过。现状：面板当前为默认 keep-alive 风格（`hidden` 切换）但未暴露 `Lazy/KeepAlive/Eager` 策略轴，也无统一 `NotifyHidden` 生命周期通知契约。
- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。
  - 结论：N/A。理由：Accordion 无 breakpoint/theme/intersection 驱动的逻辑状态轴；仅在 `motion.rs` 内部以 `ResizeObserver` 做面板高度同步，不进入 `logic` 状态机。
- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。
  - 结论：N/A。理由：Accordion 不属于大型批量选择集合场景，无 `SelectionState::All` 级别状态压缩需求。
- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。
  - 结论：N/A。理由：Accordion 当前无跨订阅者因果总线拓扑，状态流在组件内部闭环。
- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。
  - 交互元素必须具备可验证语义：`role`/`aria-*`/键盘可达路径完整，且和 headless 契约一致。
  - 用户可见文本来源必须可覆盖：优先 props，其次应用注入（`UiRoot`/i18n bundle），最后组件兜底文案；禁止把业务可见文案硬编码在 `view.rs`。
  - 组件需透传或消费 `lang` / `dir`（LTR/RTL）上下文，不得假设单语言单方向。
  - 共享 A11y 工具优先来自 `crates/ui-headless/src/a11y.rs`，组件层不重复发明同名语义工具。
  - 结论：通过。证据：`components/accordion/src/view.rs` 使用 `disclosure_trigger_attrs`，挂载 `aria-expanded/aria-controls/role/lang/dir`；`label` 由调用方 props 提供。
- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。
  - 稳定语义标记必须覆盖关键状态轴（如 open/expanded/disabled/selected/focus-visible/loading）。
  - 状态来源必须可区分（受控/非受控、默认值/外部值、交互来源），通过稳定 marker 暴露而不是隐式推断。
  - 自动化选择器优先基于语义标记，不依赖 DOM 顺序、层级深度或临时 class 名。
  - 标记值应为封闭集合（可枚举），避免自由文本导致契约漂移。
  - 结论：通过。证据：根节点暴露 `data-open-state-source/data-open-init-source/data-open-last-change-source` 与 `data-ui-*` 契约；item/trigger/panel 暴露稳定 `data-slot/data-open/data-focus-visible`。
- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。
  - `styles.rs` 中状态分支选择器必须基于 `data-*`/`aria-*`/稳定 class，禁止用 `:nth-child`、深层级选择器猜测状态。
  - 运行时样式仅允许传递必要 CSS 变量（custom properties）；禁止把业务样式逻辑塞进 inline style。
  - 视觉状态切换必须可由语义标记直接解释，不能依赖“某节点是否恰好存在”。
  - 结论：通过。证据：`components/accordion/src/styles.rs` 仅用 `[data-motion-source]/[data-custom-motion]/[data-hovered]` 与稳定 class；`view.rs` 无业务 inline style。
- [x] 测试验证“语义契约”而不只验证视觉快照。
  - 至少存在语义测试覆盖关键状态与交互路径（role/aria/data-state/source markers）。
  - 测试矩阵必须覆盖关键分支：受控/非受控、disabled、键盘路径、指针路径、SSR/wasm 差异（按适用范围）。
  - 视觉快照只能作为补充，不得替代语义契约断言。
  - 结论：通过。证据：`crates/ui-components/tests/accordion_semantics.rs` 覆盖语义标记与契约；`e2e/tests/docs_app_accordion.spec.mjs` 使用语义选择器与键盘路径断言。
- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。
  - `mod.rs` 只维护最小稳定导出面与 feature gate，不承载实现细节。
  - `logic.rs` 只做输入归一、状态派生、来源标记；禁止 DOM 操作和样式细节分支。
  - `styles.rs` 只包含 token-first 静态 CSS；禁止硬编码主题常量与业务语义文案。
  - `view.rs` 只做结构渲染与 headless 契约挂载；禁止隐藏关键状态决策。
  - `motion.rs` 只做组件语义到动效契约映射与 attach；禁止在组件内重写通用动效引擎。
  - 结论：通过。证据：目录与职责落点满足 `components/accordion/src/{mod,logic,styles,view,motion}.rs`。
- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。
  - 仅当组件存在稳定外部规范/Schema 契约或复杂配置固化需求时才引入 `spec.rs`。
  - 简单组件不得为了“形式统一”新增 `spec.rs`；说明文档应留在 `check2.md`/组件文档。
  - 新增 `spec.rs` 必须同步给出契约测试与版本演进说明。
  - 结论：通过（N/A）。理由：Accordion 当前复杂度不需要 `spec.rs`，并由 `accordion_semantics` 明确断言未新增。
- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。
  - 样式规则统一落在 `styles.rs`，由 `crates/ui-components/src/css.rs` 聚合并通过 `UiRoot` 注入。
  - 颜色/间距/圆角/阴影等视觉值必须来自 `var(--ui-*)`，禁止组件私有 token 体系。
  - Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。
  - CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。
  - 结论：通过。证据：`styles.rs` 使用 `var(--ui-*)`，`crates/ui-components/src/css.rs` 通过 feature 聚合注入 Accordion CSS。
- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。
  - 默认主题需通过基础美学清单：信息层级清晰（字重/字号/间距）、对比与层次自然、交互反馈明确（hover/active/focus）。
  - docs-app 必须提供默认主题基线页面与截图基线，关键组件（Button/Input/Overlay）纳入视觉回归对比。
  - 禁止“可访问但粗糙”的最低可用心态：视觉退化（类似旧式 Bootstrap 观感）视为质量回归。
  - HeroUI 对标以“视觉语言与体验质量”对齐为目标，不做无差别 API 表层复制。
  - 结论：通过。证据：`accordion_semantics` 包含 `docs_app_exposes_default_theme_visual_baseline_page` 与 `docs_app_visual_baseline_has_screenshot_regression_contract` 断言。
- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。
  - package 模式必须有组件级 feature（如 `component-accordion`）；未启用组件不得进入编译与链接路径。
  - `lib.rs` 与 `css.rs` 必须按 feature 条件导出/聚合，禁止无条件引用所有组件模块和 CSS 常量。
  - source 模式下仅引入需要的组件源码，不通过中央注册表维持全组件可达。
  - 任意“全量组件映射表/注册表”若导致不可达代码变可达，直接判不通过。
  - 验证命令（特性树）：`cargo tree -e features -p ui-components --no-default-features --features component-accordion,inject-css`，确认仅启用目标组件特性链。
  - 验证命令（反向依赖）：`cargo tree -e features -i ui-components -p web-demo`，检查是否被 `all-components` 或隐式特性全量拉起。
  - CI 检查（最小特性编译）：新增任务仅开启目标最小特性（示例：`cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-accordion,inject-css`）。
  - CI 检查（体积预算）：对“最小特性构建产物”设定预算并阻断回归（可用固定阈值，如 `< 50KB`，或基于仓库基线的相对阈值）；不得只做编译通过而不做体积约束。
  - 结论：通过。证据：本次 `cargo tree` 验证显示 `component-accordion` 链可独立启用，`all-components:not-found`；最小特性 `cargo check` 已通过（见第 7 节命令记录）。
- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。
  - 离散输入与状态轴必须优先使用 `enum`/新类型建模，避免字符串协议与布尔爆炸。
  - 无效状态要么在类型层不可表达，要么在 `logic.rs` 被统一归一化并可测试。
  - 关键状态必须通过稳定语义标记对外可读，供测试与 Agent 自动化消费。
  - 编译器与测试反馈应能直接定位状态契约破坏点，形成可持续闭环。
  - 结论：通过。证据：`logic.rs` 使用 `AccordionVariant/AccordionSelectionMode/AccordionOpen*Source/AccordionAgent*` 类型化状态轴，`view.rs` 暴露封闭集合语义标记。

## 4. DOM/环境边界治理
- [x] SSR/wasm/non-wasm 分支显式 `cfg`，不在 non-wasm 直接触发浏览器对象。
- [x] `ui-headless` web/ssr 互斥 compile guard 未被破坏。
- [x] `ui-motion` non-wasm stub 契约可编译。
- [x] `reduced-motion` 分支存在并可降级。
- [ ] Hydration 确定性 ID：当前默认 `id_base` 使用本地计数器生成，后续建议统一迁移到仓库级 `IdProvider` 注入。
- [x] `view!` 复杂度受控：已拆为 `render_item_label/render_item_indicator/render_item_panel` 等语义函数。
- [x] 静态片段常量化：`ACCORDION_BASE_CLASS/ACCORDION_INDICATOR_GLYPH`。
- [x] 禁止 `inner_html`：组件路径未使用 `inner_html`。
- [x] wasm 调试能力：`accordion-wasm-debug` feature 隔离，默认不进产物。

## 5. 样式与动效
- [x] 防御性变量链与 token 变量消费已落地（含 motion CSS vars）。
- [x] 级联层契约满足：组件样式走 `styles.rs` 聚合，不在 `view.rs` 写业务 inline style。
- [x] Motion 合同化：`AccordionMotion` + `sanitize_motion` + `attach_*_motion`。

## 6. AI 原生能力
- [x] 文件落点纪律满足（无 `render.rs` 漂移）。
- [x] 上下文压缩协议已落地：`Component.toml` + `accordion.rbi`。
- [x] Agent Contract 字段已挂载（`data-ui-schema/*`）。
- [x] Streaming 判定：`Streaming Optional`，显式 `unsupported + full-snapshot fallback`。

## 7. 测试、门禁与交付证据
已执行并通过：
- [x] `cargo test -p ui-accordion`
- [x] `cargo test -p ui-components --test accordion_semantics`
- [x] `cargo clippy -p ui-accordion --all-targets -- -D warnings`
- [x] `CARGO_TARGET_DIR=/tmp/codex-accordion-check cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-accordion,inject-css`
- [x] `CARGO_TARGET_DIR=/tmp/codex-accordion-headless-ssr cargo check -p ui-headless --no-default-features --features ssr`
- [x] `cargo tree -e features -p ui-components --no-default-features --features component-accordion,inject-css`（结果：`ui-accordion feature "default"`、`ui-layout feature "inject-css"`）
- [x] `cargo tree -e features -i ui-components -p web-demo`（结果：`all-components:not-found`、`component-accordion:found`）

未在本次单组件处理中执行（仓库级全量，受当前大规模并行改动影响）：
- [ ] `cargo fmt --all -- --check`
- [ ] `cargo clippy --workspace --all-targets -- -D warnings`
- [ ] `cargo test --workspace`
- [ ] `./scripts/check-rust-hygiene.sh`
- [ ] `cargo check -p ui-components --target wasm32-unknown-unknown`

## 8. 当前结论
- 核心判断：**值得合并**（组件级契约与定向门禁已通过）。
- 主要遗留：
  - SSR Hydration 的默认 `id_base` 生成仍建议升级到仓库统一 `IdProvider`。
  - `render_count` 自动化仍是仓库级后续项，Accordion 暂无独立精确计数测试。
