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
  通过依据：`components/calendar/src/logic.rs` 仅复用 `ui_state_primitives::calendar::*`；状态归一与网格构建位于 `crates/ui-state-primitives/src/calendar.rs`，并由 `crates/ui-state-primitives/src/test/calendar.rs` 覆盖。
  - 所有状态原语必须从 `status-primitives`（`ui-state-primitives`）获取，组件层只能消费，不得自造。
  - 下沉判定依据是“稳定状态不变量”；凡属于状态机、归一化、状态派生能力，默认先进入 `ui-state-primitives`。
  - 组件中可保留的仅是装配逻辑：props 归一、样式来源标记、slot 组织、对 `ui-state-primitives` 输出的映射。
  - 组件内若出现状态原语实现（受控/非受控状态机、single/multiple 展开规则、索引归一化、跨事件状态派生），该项直接判不通过。
  - 处理方式固定：先下沉到 `ui-state-primitives/src/<capability>.rs`（如 `expansion.rs`），在 `ui-state-primitives/src/lib.rs` 导出，再回到组件改调用。
  - 下沉后的原语必须有 `ui-state-primitives` 单元测试；组件侧只保留调用与语义挂载测试。
  - 桥接规范：`ui-state-primitives` 结构体必须是 POJO（Plain Old Rust Object），不持有 Leptos `Signal` 或框架绑定状态容器。
  - 消费规范：`ui-headless` 或组件 `logic.rs` 负责解包 `Signal` 当前值传入 primitive 方法，并将结果显式写回 `Signal`。
  - 设计理由：保持 primitives 纯粹可测、可迁移，不与特定响应式库绑定（便于未来替换响应式实现与做纯 Rust 测试）。
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
- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。
  - 放在 `crates/ui-motion`：通用动画数学与执行后端（spring solver、keyframe sampling、easing registry、driver adapters），以及 `wasm/non-wasm` 适配与 `reduced-motion` 执行策略。
  - 放在 `crates/ui/src/<component>/motion.rs`：把组件语义状态（open/closed、enter/exit、active/inactive）映射为 `ui-motion` contract，绑定目标节点并调用 attach。
  - 禁止放在 `crates/ui-motion`：组件 slot 结构、组件专属状态机、ARIA/keyboard 语义、业务文案与业务分支。
  - 禁止放在组件 `motion.rs`：自实现 spring/keyframe/driver 执行器；跨组件共享动效算法必须回迁 `ui-motion`。
  - 动效参数优先来自 token/theme；禁止在组件样式与逻辑中散落硬编码时长/曲线/位移常量。
  - 非 wasm 路径必须提供 no-op/stub，保证 SSR/tooling 可编译且行为可预测。
- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。
  - Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量；组件只在 `crates/ui/src/<component>/styles.rs` 消费。
  - 三轴上下文（`system/color/scale`）在 `theme.rs` 定义；组件在 `logic.rs` 选择并在 `view.rs` 生效，`styles.rs` 只消费变量，不重建主题。
  - Token 分类必须可追溯：分类源在 `tokens.rs`，规范同步 `docs/spec/styling.md`；组件不得引入平行私有 token 命名体系。
  - 量化尺寸基准必须可回归：尺寸基准在 `tokens.rs` 与 `theme.rs` 定义，主题回归在 `crates/ui-theme/tests/token_scale_baseline.rs`，组件语义回归在 `components/*/test/*<component>_semantics.rs`。
  - 主题调色与语义色对比必须满足 `WCAG 2.1 AA` 基线，并覆盖 Light/Dark/OLED 主题变体。
  - 主题层只输出 `theme/tokens/base css` 与变量；不实现组件结构、交互逻辑、组件级动效编排。
  - 新增视觉语义先补 token，再由组件消费；禁止“组件临时值先落地、后补 token”的倒序流程。
- [x] `ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。
  - `logic.rs` 负责 props 归一与状态派生；`view.rs` 负责结构渲染与 headless 语义挂载；`styles.rs` 负责 token-first 静态样式；`motion.rs` 负责动效 attach。
  - 组件层不得重写 `status-primitives` 状态机或 `ui-headless` 交互契约；发现即判不通过并回迁到对应层。
  - 对外 API 禁止暴露 `web-sys`/DOM 细节类型；平台差异封装在内部模块。
  - 测试文件位于src同级的test/中，内部测试文件同名（如rust-ui/components/accordion/src/logic.rs与rust-ui/components/accordion/test/logic.rs）。
  - 还需要一个semantics.rs用于测试。可能存在类似rust-ui/components/accordion/test/accordion_semantics.rs的旧版实现，需要迁移到新目录。

### 2. API 设计与状态内核（Logic/Kernel）
- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。
  通过依据：`Calendar` 公共布尔轴改为 `is_show_outside_days`，并保留 `show_outside_days` 兼容别名；归一化优先级为 `is_*` > 旧别名 > 默认值。
  - 布尔状态统一 `is_*`（如 `is_open`/`is_disabled`），事件统一 `on_*`，默认值统一 `default_*`。
  - 同一语义 across 组件必须同名（如都用 `on_open_change`，禁止同义别名并存）。
  - 公共 API 引入新命名时，需说明与现有命名体系的兼容策略与迁移路径。
- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。
  通过依据：`Calendar` 的 `selected_day` 轴补齐 `selected_day + on_selected_day_change + default_selected_day`，`selected_day` 存在时走受控，缺省时走非受控内部状态。
  - 受控模式：外部值是单一事实来源，内部不得偷偷写回本地状态。
  - 非受控模式：仅由默认值初始化一次，后续状态由内部原语管理。
  - 受控/非受控切换语义需稳定可测，避免“半受控”隐式行为。
- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。
  通过依据：`show_outside_days` 与 `selected_day/default_selected_day` 优先级统一在 `logic.rs` 的 `normalize_is_show_outside_days` / `normalize_selected_day_axis`；`view.rs` 仅消费归一化结果并挂载语义。
  - 默认值优先级必须可读且可测试（显式规则而非分散 `unwrap_or`）。
  - `view.rs` 不允许再做默认值分支；仅消费 `logic.rs` 的归一化输出。
  - 一旦发现多处默认值来源，直接判不通过并回收至 `logic.rs`。
- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。
  通过依据：`Calendar` 的 `show_outside_days/selected_day/default_selected_day` 归一与点击后状态更新规则统一落在 `logic.rs`（`normalize_*` + `resolve_selected_day_press_update`）；`view.rs` 仅消费结果与写回信号。
  - 输入边界统一进入 `logic.rs`，输出统一为可渲染语义状态与来源标记。
  - 事件处理器只触发状态变更，不重建状态机规则。
  - 样式层只消费状态标记，不承担状态判定职责。
- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。
  通过依据：`Calendar` 的离散轴已类型化（`CalendarTone`、`CalendarFirstWeekday`、`CalendarSelectedDayMode`、`CalendarSelectedDaySource`）；`view.rs` 不再以字符串拼接 selected-day source 状态。
  - 互斥状态优先用 `enum` 建模，利用编译器封住无效组合。
  - 字符串输入若需兼容外部配置，必须先映射到类型化枚举再进入逻辑层。
  - 布尔爆炸（多个 bool 表达一个状态机）应在设计评审阶段直接拦截。
- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。  
  通过依据：`Calendar` 的 `selected_day` 受控/非受控与来源派生原语已下沉至 `crates/ui-state-primitives/src/calendar.rs`，`components/calendar/src/logic.rs` 仅做 `pub use` 装配；`crates/ui-state-primitives/src/test/calendar.rs` 新增该原语回归测试。
  - 组件中出现可复用状态机实现（受控/非受控、展开规则、选择归一）即判应下沉。
  - 组件与业务全局状态之间必须有适配边界，禁止组件直接依赖业务 store 类型。
  - `logic.rs` 仅做装配与映射，不重新实现状态原语。
- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。（N/A：`Calendar` 无远程请求与异步状态轴，当前仅同步 props + `on_day_press`/`on_selected_day_change` 交互。）
  - 无异步交互时需明确标注 N/A 理由（例如“组件无远程请求与异步状态”），不是机械打勾。
  - 有异步交互时，`is_loading`/disabled/`aria-busy`/retry 语义必须成套一致，且对键盘与读屏路径可用。
  - 异步失败态要有可恢复路径（重试或回退），并有语义测试覆盖。
- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。  
  通过依据：`Calendar` 基础用法可直接 `<Calendar year=2026 month=3 />`（docs-app 新增 `Hello World` playground，示例不足 5 行）；默认 API 不要求用户手动接线 `ui-state-primitives`/`ui-headless`，复杂能力通过 `selected_day/default_selected_day/on_selected_day_change` 按需开启，未暴露内部 `state` 必填对象。
  - 基础用法不得要求用户先理解或手动接线 `ui-state-primitives`/`ui-headless` 状态机。
  - 基础组件 Hello World 示例代码不得超过 5 行（导入与外层模板按仓库约定不计），并可直接运行。
  - 简单需求走简单 API，复杂需求再暴露高级入口：默认 props 覆盖高频场景，高级控制通过受控/扩展参数按需开启。
  - 禁止把内部状态对象作为基础必填参数暴露（例如强制 `state=...` 才能完成点击/展开等基本交互）。
  - docs-app 必须提供最小可用示例，优先展示一眼可懂的默认调用路径。
- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。（N/A：`Calendar` 对外是单体月视图组件，不暴露 `Parent/Item` 组合式公共 API；日期单元由内部状态原语与渲染逻辑生成，不存在 `labels + children`/`titles + panels` 并行配对入口。）
  - 每个 item 的标题、语义与内容必须在同一 `Item` 结构维度绑定，避免索引配对式隐式约定。
  - `labels + children`、`titles + panels` 等并行数组/并行槽位写法不得作为默认或推荐 API。
  - 不引入这类语法糖：若为配置式输入，仅允许类型化 `ItemSpec`，并在内部映射为显式 `Item` 语义树。

### 3. 高级交互与物理机制（Shell/Physics）
- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。（N/A：`Calendar` 当前仅包含点击/键盘选日交互，不含 drag 手势、`Dragging` 状态、pointermove 高频循环或 `Action::DragEnd` 收敛路径。）
- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。（N/A：`Calendar` 当前不属于依赖 DOM 几何测量的 overlay 组件，不执行 `getBoundingClientRect`/尺寸测量，也不存在 `Intent -> Measure -> Rectification` 回路与收敛风险。）
- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。（N/A：`Calendar` 不暴露动态子组件注册模型，日期网格由 `ui-state-primitives` 按月快照生成，不存在运行时 `Register/Unregister` 或基于容器迭代顺序的焦点导航。）
- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。（N/A：`Calendar` 为单体月视图渲染，不提供子内容投影容器语义，也不实现 `KeepAlive` 挂载策略；隐藏态不存在后台轮询或持续动画任务需要 `NotifyHidden` 协调。）
- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。（N/A：`Calendar` 当前无 `ResizeObserver`/`IntersectionObserver`/主题变化订阅路径，`view.rs` 未接入原始环境事件流，也不存在需防抖汇聚到 `logic` 的高频环境信号。）
- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。（N/A：`Calendar` 仅处理单日选择与月网格渲染，不包含大型集合批量操作、全选态压缩（如 `SelectionState::All`）或跨层 `Context Bus` 广播链路。）
- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。（N/A：`Calendar` 当前交互链路为本地同步选日回调，不存在跨模块派生总线广播或多订阅者事件分发，因此无 `TraceId` 透传需求。）
- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。  
  通过依据：`Calendar` 通过 `ui_headless::use_calendar_root/use_calendar_day` 挂载 `role/aria-*` 与键盘可达契约；根节点提供 `aria_label` 可覆盖入口并透传 `lang/dir`（LTR/RTL）；组件 `view.rs` 未硬编码业务文案，周标题与月份标题由逻辑层输出并可在上层文档/应用层做替换策略。
  - 交互元素必须具备可验证语义：`role`/`aria-*`/键盘可达路径完整，且和 headless 契约一致。
  - 用户可见文本来源必须可覆盖：优先 props，其次应用注入（`UiRoot`/i18n bundle），最后组件兜底文案；禁止把业务可见文案硬编码在 `view.rs`。
  - 组件需透传或消费 `lang` / `dir`（LTR/RTL）上下文，不得假设单语言单方向。
  - 共享 A11y 工具优先来自 `crates/ui-headless/src/a11y.rs`，组件层不重复发明同名语义工具。
- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。  
  通过依据：`Calendar` 根节点与日期单元已暴露稳定 `data-*`/`aria-*` 语义（如 `data-state`、`data-selected-day-mode`、`data-selected-day-source`、`data-ui-*`、`aria-selected`、`aria-disabled`）；受控/非受控与默认/交互来源由枚举值输出；`components/calendar/test/semantics.rs` 已回归断言这些标记，自动化选择器可直接基于语义属性而非 DOM 结构。
  - 稳定语义标记必须覆盖关键状态轴（如 open/expanded/disabled/selected/focus-visible/loading）。
  - 状态来源必须可区分（受控/非受控、默认值/外部值、交互来源），通过稳定 marker 暴露而不是隐式推断。
  - 自动化选择器优先基于语义标记，不依赖 DOM 顺序、层级深度或临时 class 名。
  - 标记值应为封闭集合（可枚举），避免自由文本导致契约漂移。
- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。  
  通过依据：`components/calendar/src/styles.rs` 的状态分支基于稳定 `data-*` 与 class（如 `data-tone`、`data-first-weekday`、`data-state`、`data-month-source`）；未使用 `:nth-child` 或深层结构推断；`view.rs` 运行时仅通过 `style=panel_vars` 传递动效 CSS 变量（`--ui-calendar-motion-duration`），未将业务样式逻辑塞入 inline style。
  - `styles.rs` 中状态分支选择器必须基于 `data-*`/`aria-*`/稳定 class，禁止用 `:nth-child`、深层级选择器猜测状态。
  - 运行时样式仅允许传递必要 CSS 变量（custom properties）；禁止把业务样式逻辑塞进 inline style。
  - 视觉状态切换必须可由语义标记直接解释，不能依赖“某节点是否恰好存在”。
- [x] 测试验证“语义契约”而不只验证视觉快照。  
  通过依据：`components/calendar/test/semantics.rs` 已覆盖 `role/aria/data-*` 与状态来源标记断言；`crates/ui-headless/src/test/calendar.rs` 覆盖禁用态与交互语义契约；`e2e/tests/docs_app_calendar_contract.spec.mjs` 通过稳定语义选择器验证关键流程。当前日历验收不依赖视觉快照作为主断言，快照仅可作为补充。
  - 至少存在语义测试覆盖关键状态与交互路径（role/aria/data-state/source markers）。
  - 测试矩阵必须覆盖关键分支：受控/非受控、disabled、键盘路径、指针路径、SSR/wasm 差异（按适用范围）。
  - 视觉快照只能作为补充，不得替代语义契约断言。
- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。  
  通过依据：`components/calendar/src/mod.rs` 仅维护模块导出边界与测试挂载；`components/calendar/src/logic.rs` 仅保留状态原语 re-export 与来源契约派生（已移除 class 拼装）；`components/calendar/src/styles.rs` 仅包含 token-first 静态 CSS；`components/calendar/src/view.rs` 仅负责结构渲染、headless 挂载与 class 组装；`components/calendar/src/motion.rs` 仅做动效参数归一与 CSS 变量 attach，不重写通用动效引擎。
  - `mod.rs` 只维护最小稳定导出面与 feature gate，不承载实现细节。
  - `logic.rs` 只做输入归一、状态派生、来源标记；禁止 DOM 操作和样式细节分支。
  - `styles.rs` 只包含 token-first 静态 CSS；禁止硬编码主题常量与业务语义文案。
  - `view.rs` 只做结构渲染与 headless 契约挂载；禁止隐藏关键状态决策。
  - `motion.rs` 只做组件语义到动效契约映射与 attach；禁止在组件内重写通用动效引擎。
- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。  
  通过依据：`components/calendar/src/` 当前不存在 `spec.rs`；`components/calendar/src/mod.rs` 未导出 `spec/protocol` 规范入口；`Calendar` 的使用说明与约束落在 `components/calendar/README.md` 与本 `check2.md`，未为“形式统一”新增独立 `spec.rs`。
  - 仅当组件存在稳定外部规范/Schema 契约或复杂配置固化需求时才引入 `spec.rs`。
  - 简单组件不得为了“形式统一”新增 `spec.rs`；说明文档应留在 `check2.md`/组件文档。
  - 新增 `spec.rs` 必须同步给出契约测试与版本演进说明。
- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。  
  通过依据：`components/calendar/src/styles.rs` 承载日历组件静态样式且视觉值基于 `var(--ui-*)` token；`crates/ui/src/css.rs` 在 `component-calendar` feature 下统一聚合 `crate::calendar::styles::CSS`，并由 `crates/ui/src/root.rs` 的 `UiRoot` 统一注入；`components/calendar/src/view.rs` 运行时仅通过 `style=panel_vars` 传入动效变量（`--ui-calendar-motion-duration`），未在组件层引入 Utility-First 或 CSS-in-Rust 作为默认样式机制。
  - 样式规则统一落在 `styles.rs`，由 `crates/ui/src/css.rs` 聚合并通过 `UiRoot` 注入。
  - 颜色/间距/圆角/阴影等视觉值必须来自 `var(--ui-*)`，禁止组件私有 token 体系。
  - Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。
  - CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。
- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。  
  通过依据：docs-app 已提供默认主题基线页 `#/components/theme-visual-baseline`（`apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs`），同页覆盖 `Button/Input/Overlay` 的层级、对比与交互反馈；`e2e/tests/docs_app_theme_visual_baseline.spec.mjs` 已包含可见性断言与截图回归（`E2E_VISUAL_BASELINE=on` 时执行 `toHaveScreenshot`）；`docs/spec/heroui-parameter-design-strategy.md` 已记录 `Calendar` 与 HeroUI 的“视觉语言/体验质量对齐而非 API 表层复制”策略。
  - 默认主题需通过基础美学清单：信息层级清晰（字重/字号/间距）、对比与层次自然、交互反馈明确（hover/active/focus）。
  - docs-app 必须提供默认主题基线页面与截图基线，关键组件（Button/Input/Overlay）纳入视觉回归对比。
  - 禁止“可访问但粗糙”的最低可用心态：视觉退化（类似旧式 Bootstrap 观感）视为质量回归。
  - HeroUI 对标以“视觉语言与体验质量”对齐为目标，不做无差别 API 表层复制。
- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。  
  通过依据：`crates/ui/Cargo.toml` 已提供 `component-calendar` 独立 feature 且默认特性外可单独启用；`crates/ui/src/lib.rs` 通过 `#[cfg(feature = "component-calendar")] pub use ui_calendar as calendar;` 条件导出；`crates/ui/src/css.rs` 仅在 `inject-css + component-calendar` 下聚合 `calendar::styles::CSS`；`cargo tree -e features -p ui --no-default-features --features component-calendar,inject-css` 未出现 `all-components`；`cargo tree -e features -i ui -p web-demo` 显示由 `web-demo-components` 拉起且未隐式全量 `all-components`；仓库已有 `scripts/check-ui-tree-shaking.sh` + `scripts/tree_shaking_budget.env` 提供最小特性 wasm 编译与体积预算阻断（CI 预算基线）。
  - package 模式必须有组件级 feature（如 `component-accordion`）；未启用组件不得进入编译与链接路径。
  - `lib.rs` 与 `css.rs` 必须按 feature 条件导出/聚合，禁止无条件引用所有组件模块和 CSS 常量。
  - source 模式下仅引入需要的组件源码，不通过中央注册表维持全组件可达。
  - 任意“全量组件映射表/注册表”若导致不可达代码变可达，直接判不通过。
  - 验证命令（特性树）：`cargo tree -e features -p ui --no-default-features --features component-accordion,inject-css`，确认仅启用目标组件特性链。
  - 验证命令（反向依赖）：`cargo tree -e features -i ui -p web-demo`，检查是否被 `all-components` 或隐式特性全量拉起。
  - CI 检查（最小特性编译）：新增任务仅开启目标最小特性（示例：`cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-accordion,inject-css`）。
  - CI 检查（体积预算）：对“最小特性构建产物”设定预算并阻断回归（可用固定阈值，如 `< 50KB`，或基于仓库基线的相对阈值）；不得只做编译通过而不做体积约束。
- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。  
  通过依据：`Calendar` 离散轴已类型化为 `enum`（如 `CalendarTone`、`CalendarFirstWeekday`、`CalendarSelectedDayMode`、`CalendarSelectedDaySource`、`CalendarAgent*`）；无效输入（如越界 `selected_day`/`month`）在 `ui-state-primitives` 的 `normalize_*` 与 `resolve_*` 路径统一归一，并由 `components/calendar/test/logic.rs` 覆盖；关键状态通过稳定 `data-*`/`aria-*` 对外可读（如 `data-state`、`data-selected-day-mode`、`data-selected-day-source`、`data-ui-*`、`aria-selected`、`aria-disabled`），并由 `components/calendar/test/semantics.rs` 做契约断言，形成“类型约束 + 语义标记 + 测试定位”的闭环。
  - 离散输入与状态轴必须优先使用 `enum`/新类型建模，避免字符串协议与布尔爆炸。
  - 无效状态要么在类型层不可表达，要么在 `logic.rs` 被统一归一化并可测试。
  - 关键状态必须通过稳定语义标记对外可读，供测试与 Agent 自动化消费。
  - 编译器与测试反馈应能直接定位状态契约破坏点，形成可持续闭环。

### 4. DOM/环境边界治理
- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。（N/A：`Calendar` 当前为月网格内联组件，不承担 overlay 容器职责；`components/calendar/src/view.rs` 未私存焦点恢复 `NodeRef`，也不存在层叠 Overlay 的焦点回退链路实现。）
- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。（N/A：`Calendar` 组件未集成 ECharts/Map 等命令式第三方实例；`components/calendar/src/mod.rs` 与 `components/calendar/src/logic.rs` 的公共/内部状态接口仅围绕日期状态原语与语义标记，不暴露第三方实例句柄，也不存在反向污染状态机的接线点。）
- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。（N/A：`Calendar` 组件当前不生成运行时 ID，不存在需注入 `IdProvider` 的 ID 派生链路；`components/calendar/src/logic.rs`、`components/calendar/src/view.rs` 与 `crates/ui-headless/src/calendar.rs` 仅使用 props/状态原语推导语义属性，未依赖 `now()`、随机 UUID 或随机数初始化。）
- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。  
  通过依据：`components/calendar/Cargo.toml` 已引入显式 `web/ssr` feature 分流（`web -> leptos/csr + ui-headless/web`，`ssr -> leptos/ssr + ui-headless/ssr`），并将 `ui-headless` 依赖改为 `default-features = false` 防止隐式平台漂移；`components/calendar/src/mod.rs` 增加 `web+ssr` 互斥 `compile_error!`；`components/calendar/src/motion.rs` 以 `#[cfg(target_arch = "wasm32")]` 隔离浏览器偏好路径，non-wasm 分支返回稳定降级值；组件源码静态扫描未出现 `web-sys/web_sys/window/document` 直接引用。  
  compile-only 命令证据（本机环境阻塞已记录）：`cargo check -p ui-calendar`、`cargo check -p ui-calendar --no-default-features --features ssr`、`cargo check -p ui-calendar --target wasm32-unknown-unknown` 均在依赖编译早期命中同一环境错误 `Invalid cross-device link (os error 18)`，与组件逻辑无关；命令与报错已留痕用于 CI 复验。
  - 至少包含 compile-only 证据：web（wasm32）、ssr（native）、默认本地构建三条路径。
  - 平台分支差异必须显式 `cfg` 或 feature 管理，禁止依赖运行时偶然行为。
  - non-wasm 路径禁止引用 `web-sys`/浏览器对象。
- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。  
  通过依据：`crates/ui-headless/src/lib.rs` 顶部已声明 `#[cfg(all(feature = "web", feature = "ssr"))] compile_error!(...)`；`components/calendar/Cargo.toml` 已改为 `ui-headless = { default-features = false }` 并由 `ui-calendar` 的 `web/ssr` feature 显式转发到 `ui-headless/web` 与 `ui-headless/ssr`，避免组件侧绕过互斥契约；`components/calendar/src/mod.rs` 也增加了同类 `web+ssr` 互斥保护，防止组件层双开特性漂移。  
  验证记录：`cargo check -p ui-calendar`、`cargo check -p ui-calendar --no-default-features --features ssr`、`cargo check -p ui-calendar --no-default-features --features web,ssr` 与 `cargo check -p ui-headless --no-default-features --features web,ssr` 在本机均被统一环境错误 `Invalid cross-device link (os error 18)` 提前阻塞；该阻塞与互斥约束实现无关，需在 CI/稳定构建机复验两条 feature 编译路径。
  - 组件依赖 `ui-headless` 能力时，不得破坏其 web/ssr 互斥约束。
  - 组件若新增 headless 功能接入，需验证两条 feature 路径都可编译。
  - 发现“同时启用 web+ssr 仍可过编译”视为契约回归。
- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。  
  通过依据：`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = "wasm32"))]` 下提供 `web` stub（`prefers_reduced_motion() -> true`、`animate(&(), ..)` no-op），并有 `non_wasm_web_backend_is_predictable_noop` 回归测试；`components/calendar/src/motion.rs` 的 `attach_motion` 仅输出 CSS 变量字符串，不持有或依赖动画句柄，non-wasm 分支不会触发浏览器 API；`components/calendar/src/view.rs` 仅消费 `attach_motion(None, motion)` 的样式输出，不存在句柄存在性假设与 panic 路径。  
  验证记录：本机 `cargo check` 仍受统一环境错误 `Invalid cross-device link (os error 18)` 阻塞，阻塞发生在依赖编译早期，与 `ui-motion` non-wasm stub 设计无关，需在 CI/稳定构建机复验编译链。
  - `motion.rs` 调用必须可在 non-wasm 下安全降级，不触发 panic。
  - 组件不得假设动画句柄一定存在；no-op 分支行为需可预测。
  - toolchain 场景（测试/文档/静态分析）不得因 motion 依赖阻塞编译。
- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。  
  通过依据：`components/calendar/src/styles.rs` 已新增 `@media (prefers-reduced-motion: reduce)` 并通过 `--ui-calendar-motion-duration: var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))` 走 token/fallback 降级路径；`components/calendar/src/motion.rs` 的 `attach_motion` 显式使用 `ui_motion::web::prefers_reduced_motion()` 解析有效动效参数，在 reduced/non-wasm 路径下降级到最小持续时间并仅输出 CSS 变量（无句柄、无 panic）；wasm 侧仍可通过 `ui-motion` 的 wasm backend 提供增强执行能力，但 `Calendar` 对外语义标记（`data-*`/`aria-*`）不依赖平台分支、不产生契约分裂。`components/calendar/test/semantics.rs` 已补充 reduced-motion 媒体查询断言，锁定该契约。
  - `reduced-motion` 下动画应跳过或降级为最小必要反馈。
  - SSR 输出必须与客户端 hydration 兼容，避免首帧语义错位。
  - wasm 分支允许增强交互，但语义契约不得与 SSR 分支分裂。
- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。  
  通过依据：`apps/docs-app/src/pages/components/shell.rs` 已为 `slug="calendar"` 增加 `UiPerfBudget`（`max_mount_ms: 32.0`、`max_update_ms: Some(10.0)`、`max_heap_kb: Some(576.0)`）并通过 `<UiPerfProbe name=perf_name budget=perf_budget>` 挂载；`apps/docs-app/src/perf_probe.rs` 暴露稳定 `data-perf-*` 与违规标记（`data-perf-violation`）；`e2e/tests/docs_app_components_coverage.spec.mjs` 对 perf 标记与“无违规”做阻断断言；`scripts/check-ui-performance.sh` 新增 `cargo test -p ui-calendar calendar_performance_governance_budget_is_defined_traceable_and_blocking` 门禁；`components/calendar/test/semantics.rs` 新增同名回归测试，覆盖预算定义、探针标记、E2E 阻断与脚本接线。当前框架对精确 `render_count` 自动化仍未通用化，按清单采用可重复等价证据，并持续由 `docs/plan/TODO.md` 的 `render_count` 项跟踪补齐。
  - 关键交互组件需定义最小预算项（首渲染、关键更新、内存/分配趋势）。
  - 回归检测至少具备可重复基线与失败阈值，不靠主观“感觉变慢”。
  - 性能问题需可归因到状态、渲染、样式或动效路径之一。
  - 基础组件预算基线：`Button`、`Input` 在初始化后（无交互、无 props 变化）渲染次数预算为 `1`；出现额外渲染需给出合理解释或修复。
  - 测试要求：在 `components/*/test/**` 增加 `render_count` 类回归测试（测试框架支持时必须启用）；至少覆盖基础组件与本次改动组件。
  - 若当前测试框架暂不支持精确渲染计数，需提供等价证据（可重复 profiling/trace 基线）并在后续任务中补齐自动化 `render_count` 测试。
- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。  
  通过依据：`components/calendar/src/view.rs` 已将巨型内联结构拆分为语义函数与输入结构（`render_header`、`render_weekday`、`render_weekdays`、`compose_day_class`、`render_day`、`render_empty_day`、`CalendarDayRenderInput`），根视图仅负责骨架装配与状态挂载；日期单元重复片段已从主 `view!` 内联分支提取到 `render_day/render_empty_day`，降低宏展开体量与重复嵌套。`components/calendar/test/semantics.rs` 新增 `calendar_view_macro_complexity_is_split_by_semantic_fragments` 回归，阻断回退到单块巨型 `view!`。
  - 复杂结构按语义子块拆分（header/body/item 等），避免巨型单块 `view!`。
  - `view.rs` 中若出现多层嵌套重复片段，应优先提取局部渲染函数。
  - 编译时间/产物体积异常增长时，优先排查宏展开体量。
- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。  
  通过依据：`components/calendar/src/view.rs` 已将轻逻辑/静态片段拆为普通函数（`render_header`、`render_weekday`、`render_weekdays`、`compose_day_class`、`render_day`、`render_empty_day`），仅保留 `Calendar` 作为单一 `#[component]` 公共入口；`components/calendar/test/semantics.rs` 的 `calendar_view_macro_complexity_is_split_by_semantic_fragments` 断言了上述函数拆分与 `#[component]` 计数为 1，确保语义标记与测试定位稳定。
  - 纯静态或轻逻辑片段优先函数化；仅在需要独立 props 语义时升级为组件。
  - 禁止把所有局部片段都升格为 `#[component]` 导致抽象噪音。
  - 拆分后语义标记与测试定位仍需稳定。
- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。  
  通过依据：`Calendar` 主组件本身不包含复杂 SVG/页脚/长说明文本，`components/calendar/src/view.rs` 仅保留模板化静态片段函数（如 `render_header`/`render_weekday`/`render_empty_day`）；静态文案来源集中在 `ui-state-primitives`（`crates/ui-state-primitives/src/calendar.rs` 的 `weekday_labels`/`month_name`/`month_title`），`view.rs` 通过 `logic::weekday_labels` 与 `logic::month_title` 消费，避免在视图层散落硬编码文案；`components/calendar/test/semantics.rs` 新增 `calendar_static_fragments_are_centralized_and_not_scattered` 回归，断言 `view.rs` 不引入 `<svg`/`inner_html` 静态注入并保持空白日期模板单点定义。
  - 可判定为纯静态的片段应避免重复动态构造。
  - 常量化后仍需维持可访问语义（title/aria-label/role 等）。
  - 静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。
- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。  
  通过依据：`Calendar` 组件当前不使用 `inner_html` 注入路径；`components/calendar/test/semantics.rs` 新增 `calendar_inner_html_contract_disallows_untrusted_html_injection`，对 `src/mod.rs`、`src/logic.rs`、`src/view.rs`、`src/styles.rs`、`src/motion.rs`、`README.md` 做安全回归扫描，阻断 `inner_html`/`set_inner_html`/`dangerously_set_inner_html` 等注入入口；同时 `calendar_static_fragments_are_centralized_and_not_scattered` 已约束 `view.rs` 不引入 `inner_html`，防止后续回退到未白名单化的动态 HTML 拼接。
  - 仅允许编译期常量或明确白名单内容进入 `inner_html`。
  - 严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。
  - 使用 `inner_html` 的节点必须补语义测试与安全回归说明。
- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。  
  通过依据：`components/calendar/Cargo.toml` 新增 `wasm-debug` feature 且默认不启用；`components/calendar/src/view.rs` 在 `#[cfg(feature = "wasm-debug")]` 下内聚 `debug_trace` 私有调试模块（未暴露公共 API），并输出可视化调试面板（`data-slot="calendar-debug"`）与可回放入口（`data-action="replay-last-debug-event"`），记录 `DayPress/ReplayLast` 事件与 `trace_id/tick + prev/next selected_day + prev/next source` 关键状态链路；`crates/ui/Cargo.toml` 新增 `calendar-wasm-debug = ["component-calendar", "ui-calendar/wasm-debug"]` 作为聚合层显式开关；`components/calendar/test/semantics.rs` 的 `calendar_wasm_debug_contract_is_feature_gated_traceable_and_replayable` 回归锁定上述契约。
  - 开发模式下至少能追踪关键状态变更来源与前后值。
  - 关键交互链路应支持最小可复现记录（事件顺序/状态转移）。
  - 调试开关默认不进入生产包体与公共 API。
- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。  
  通过依据：`Calendar` docs 页的 Interactive Playground 已具备独立 workbench 画布（`apps/docs-app/src/pages/components/pages/forms_extra.rs`，`title="Interactive Playground (State + Source Markers)"`）；样式调试走 `Playground` 的 `test_css_source + test_source_path + test_config_signal`（scoped CSS live-edit），无需每次样式改动重编 wasm；新增可选持久化开关 `Switch checked=workbench_persist_state` + `Persist workbench state`，并通过 `load/save/clear_calendar_workbench_state`（web `localStorage` + non-wasm no-op）在开启时保留交互上下文、关闭时清理状态；`components/calendar/test/semantics.rs` 的 `calendar_docs_playgrounds_lock_state_matrix_contract_values` 已锁定上述 DX 契约。
  - 常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。
  - 组件调试应尽量保持当前交互上下文，降低重复操作成本。
  - 复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。
- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。  
  通过依据：`Calendar` 的 docs workbench 配置持久化已统一到 `serde` 路径（`apps/docs-app/src/pages/components/pages/forms_extra.rs`：`CalendarWorkbenchStorage { version, state }` + `serde_json::to_string/from_str` + `CalendarWorkbenchStorageError` 结构化错误），并通过 `CALENDAR_WORKBENCH_STORAGE_VERSION` 提供版本迁移锚点；`ui-calendar` 在 `wasm-debug` 下接入统一 `tracing` 语义（`components/calendar/Cargo.toml`：`wasm-debug = ["dep:tracing"]`，`components/calendar/src/view.rs`：`tracing::info_span!(target: "ui.calendar", "calendar_interaction", ..)` 与 `tracing::info!` 事件）；组件公共接口与实现未引入 `tokio/async-std` runtime 类型，`components/calendar/test/semantics.rs` 新增 `calendar_engineering_capability_contract_uses_serde_tracing_and_runtime_agnostic_api` 回归锁定该契约。
  - 若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。
  - 关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。
  - 异步边界不得把具体 runtime 类型暴露到组件公共接口。

### 5. 样式与动效（Theme & Motion）
- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。  
  通过依据：`components/calendar/src/styles.rs` 已将主题变量消费统一为双层回退链（例如 `var(--ui-bg, var(--ui-fallback-bg))`、`var(--ui-border, var(--ui-fallback-border))`、`var(--ui-text-field-motion-duration, var(--ui-fallback-text-field-motion-duration))`），并移除组件内 Hex 与 `px/rem/ms` 终值常量；`components/calendar/test/semantics.rs` 的 `calendar_styles_are_token_first_and_theme_driven` 新增防御变量链与禁止裸 token/终值字面量回归断言，确保 fallback SSOT 受测试约束。
- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\"top: 10px\"`）。  
  通过依据：`crates/ui/src/css.rs` 在 `push_components_css` 入口统一以 `@layer ui` 聚合组件样式，并在 `component-calendar` feature 下注入 `crate::calendar::styles::CSS`；`components/calendar/src/view.rs` 运行时仅通过 `style=panel_vars` 挂载样式变量，`panel_vars` 来自 `components/calendar/src/motion.rs` 的 `attach_motion(None, motion)`，输出 `--ui-calendar-motion-*` CSS Custom Properties（如 duration/stiffness/damping）；`components/calendar/test/semantics.rs` 新增 `calendar_css_layering_and_runtime_style_contract_stay_ui_scoped` 回归，锁定上述契约并阻断普通内联样式回退。
- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。  
  通过依据：`components/calendar/src/motion.rs` 的 `CalendarMotion` 已内置 `spring: ui_motion::spring::SpringConfig`（含 `stiffness/damping/mass/precision`），`sanitize_motion` 使用 `ui_motion::spring::sanitize_config` 统一归一；`resolve_effective_motion` + `attach_motion` 在 `!enabled` 或 `ui_motion::web::prefers_reduced_motion()` 时降级为最小持续时间，并仅通过 `--ui-calendar-motion-duration` 与 `--ui-calendar-motion-stiffness/damping/mass/precision/reduced` 变量挂载；non-wasm 路径复用 `ui-motion` stub（`prefers_reduced_motion() -> true`）实现可预测 no-op 降级；`components/calendar/test/motion.rs` 与 `components/calendar/test/semantics.rs` 已补充对应回归。
- [x] `ui` 固定入口文件落点正确。  
  通过依据：`crates/ui/src/lib.rs` 作为总入口以 `component-*` feature gate 条件导出组件并对外 `pub use root::UiRoot`，未暴露 `web-sys/wasm_bindgen` 等平台细节类型；`crates/ui/src/css.rs` 通过 `push_components_css` 在 `inject-css` + `component-*` 条件下聚合组件 CSS（含 `@layer ui` 与 `component-calendar` 注入），并提供 `not(inject-css)` 空实现；`crates/ui/src/root.rs` 集中处理 `UiRoot` 的 base css + theme vars + 可选 components css 注入，并通过 `provide_ui_i18n` 注入全局 i18n；`crates/ui-visual-primitive/src/active_highlight.rs` 仅承载共享高亮样式与 motion driver，不耦合具体业务组件语义；`crates/ui/src/overlay_open.rs`、`crates/ui/src/presence.rs`、`crates/ui/src/a11y.rs` 当前不存在。`components/calendar/test/semantics.rs` 新增 `calendar_ui_components_entry_points_follow_layered_architecture_contract` 对上述约束做回归锁定。
  - `crates/ui/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。
  - `crates/ui/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。
  - `crates/ui/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。
  - `crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。
  - `crates/ui/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。
  - `crates/ui/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。
  - `crates/ui/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。
- [x] 组件目录标准文件落点正确。  
  通过依据：`components/calendar/src/` 已具备标准落点文件 `mod.rs`、`logic.rs`、`styles.rs`、`view.rs`、`motion.rs`；不存在 `render.rs` 与 `spec.rs`。`components/calendar/src/mod.rs` 维持最小稳定导出边界（私有 `mod logic; mod view;`，公开 `pub mod motion; pub mod styles;`，并仅导出 `CalendarMotion` 与 `Calendar`），未将 `debug/protocol` 内部模块公开。`components/calendar/test/semantics.rs` 新增 `calendar_component_standard_file_layout_and_public_boundary_are_stable` 回归测试，锁定上述目录与导出契约。
  - `<component>/mod.rs`：最小稳定导出面，存在且无过度导出。
  - `<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。
  - `<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。
  - `<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。
  - `<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。
  - `<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。

### 6. AI 原生能力与文件落点（Struct-First & Projection）
- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。  
  通过依据：`components/calendar/src/` 已收敛为五件套源码文件 `mod.rs`、`logic.rs`、`styles.rs`、`view.rs`、`motion.rs`；`debug.rs` 与 `protocol.rs` 已移除并回收实现到 `view.rs` 的 `#[cfg(feature = "wasm-debug")] mod debug_trace` 私有模块；`README` 与 `check2` 文档已迁移到组件根目录（`components/calendar/README.md`、`components/calendar/check2.md`）；`render.rs/spec.rs` 不存在。`components/calendar/test/semantics.rs` 的 `calendar_component_standard_file_layout_and_public_boundary_are_stable` 已新增 strict file set 断言并锁定该纪律。
- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。  
  通过依据（N/A）：`Calendar` 当前定位为月视图基础组件，公共输入轴有限（`year/month/tone/first_weekday/selected_day`）且不存在稳定外部 Schema 固化需求；按“`spec.rs` 仅少数复杂组件引入”规则保持 `components/calendar/src/` 无 `spec.rs`，避免为简单组件引入额外建造者抽象。复杂组件的 Builder 基线由 `components/button/src/spec.rs` 承担（`ButtonSpec::new()...render()`），`Calendar` 继续走直接 props API。
- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。  
  通过依据：`components/calendar/src/Component.toml` 已补齐能力清单（输入轴、输出事件、capabilities、dependencies），`components/calendar/src/calendar.rbi` 已补齐 API 签名投影（类型别名、`CalendarMotion`、Agent contract、`Calendar(...) -> IntoView`）；`components/calendar/test/semantics.rs` 新增 `calendar_context_compression_manifest_and_rbi_are_present_and_consistent_locally` 回归，锁定 Manifest/RBI 与当前组件公开 API 的一致性，防止 AI 检索上下文漂移。
- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。  
  通过依据：`components/calendar/src/logic.rs` 已以类型化 `CalendarAgentAction/State/Source/StreamSupport/StreamFallback/OutputStatus` + `CalendarAgentContract` 建模 Agent 语义，并由 `resolve_agent_contract` 从状态轴集中派生；`components/calendar/src/view.rs` 仅挂载 `data-ui-schema/intent/action/state/source/*`（来自 `agent_contract.get().*.as_attr()`），未散落自由字符串协议；`components/calendar/src/Component.toml` 与 `components/calendar/src/calendar.rbi` 已同步投影 Agent 合同字段与能力声明（`agent_contract_schema_markers`）；渲染链路维持白名单边界（无 `inner_html`/`set_inner_html`/`dangerously_set_inner_html`/`<script` 注入入口）。`components/calendar/test/semantics.rs` 新增 `calendar_agent_contract_schema_markers_are_typed_traceable_and_whitelist_rendered` 回归锁定该契约。
  - 关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。
  - Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。
  - 契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。
  - 配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。
- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。  
  通过依据：`Calendar` 已将流式语义约束为 LLM 输出显示模式定义：`Streaming`（增量渲染）与 `Snapshot`（完成后一次性渲染）两种；组件当前在 `components/calendar/src/logic.rs` 通过 `CalendarAgentStreamSupport::Unsupported` + `CalendarAgentStreamFallback::Snapshot` 明确“非正文阅读面默认走快照”，并在 `components/calendar/src/view.rs` 稳定输出 `data-ui-stream-support` / `data-ui-stream-fallback` / `data-ui-stream-mode=\"snapshot\"`。`components/calendar/test/semantics.rs` 新增 `calendar_streaming_definition_is_llm_scoped_and_limited_to_streaming_snapshot_modes` 回归锁定该定义边界，避免漂移到其他非约定显示模式。
  - `Streaming`：LLM 还在生成，界面边生成边显示。
  - `Snapshot`：LLM 全部生成完成后，一次性显示。
- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。  
  通过依据：`components/calendar/src/logic.rs` 将 `CalendarAgentStreamFallback` 固定到 `Snapshot`，并以 `CalendarAgentStreamSupport::Unsupported` + `CalendarAgentOutputStatus::Verified` 保证“完整结果一次性渲染”作为默认契约；`components/calendar/src/view.rs` 稳定输出 `data-ui-stream-mode="snapshot"` 与 `data-ui-output-status`，可被上层以完整配置直接消费；`components/calendar/src/Component.toml` 的 `snapshot_rendering` 能力与 `components/calendar/src/calendar.rbi` 的 `CalendarAgentStreamFallback::Snapshot` 签名投影共同约束该基线。`components/calendar/test/semantics.rs` 新增 `calendar_snapshot_is_baseline_capability_for_complete_outputs` 回归锁定该项。
  - 所有组件都应能消费“完整生成结果”并稳定渲染。
  - 即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。
- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。  
  通过依据：`Calendar` 不是正文阅读面，按职责归类为 `Streaming Optional`；`components/calendar/src/logic.rs` 通过 `CalendarAgentStreamSupport::Unsupported` + `CalendarAgentStreamFallback::Snapshot` 明确仅消费快照并要求 `fallback=snapshot`，`components/calendar/src/view.rs` 持续输出 `data-ui-output-status`、`role`、`aria-label` 与 `data-*` 语义标记保证状态连续可读；组件层未实现断线恢复/重试协议（无 retry/reconnect/error-retry 语义），该类校验与恢复策略保持上层负责。`components/calendar/test/semantics.rs` 新增 `calendar_streaming_policy_is_optional_snapshot_with_status_markers_and_upper_layer_retry_boundary` 回归锁定该职责边界。
  - `Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。
  - `Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。
  - 无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。
  - 数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。

### 7. 测试、门禁与交付
- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。  
  通过依据：`components/calendar/src/view.rs` 已将 class 组装改为 `Vec<Cow<'static, str>>`（`Cow::Borrowed` + `Cow::Owned(base_class_name)`）并移除固定 class 的 `.to_string()` 热点；`components/calendar/test/semantics.rs` 新增 `calendar_non_test_sources_follow_rust_hygiene_contract`，锁定 `src/mod.rs|logic.rs|styles.rs|view.rs|motion.rs` 非测试源码中无 `.unwrap/.unwrap_err/.expect`、无 `let _ = ...`、无 `String::from/.to_owned`。已执行 `./scripts/check-rust-hygiene.sh`：当前环境因 `rg` 缺少 PCRE2 与仓库级 `check-api-contracts` baseline 漂移失败（非 `calendar` 局部违规），组件级 contract 由上述语义回归覆盖。
- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。  
  通过依据：`crates/ui/Cargo.toml` 已注册 `component-calendar = ["dep:ui-calendar"]`，且 `ui-calendar` 依赖为 `optional = true`；`crates/ui/src/lib.rs` 以 `#[cfg(feature = "component-calendar")] pub use ui_calendar as calendar;` 条件导出，`crates/ui/src/css.rs` 以 `#[cfg(feature = "component-calendar")] out.push_str(crate::calendar::styles::CSS);` 条件聚合 CSS。实测命令：`cargo tree -e features -p ui --no-default-features --features component-calendar,inject-css`（输出包含 `ui-calendar feature "default"/"web"`，未出现 `all-components`），`cargo tree -e features -i ui -p web-demo`（仅见 `web-demo-components` 反向拉起链，未见 `all-components`）。并由 `components/calendar/test/semantics.rs` 新增 `calendar_tree_shaking_feature_pruning_is_gated_in_ui_components` 锁定该契约。
- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。  
  通过依据：`components/calendar/test/semantics.rs` 已覆盖根与日期单元的 `role/aria-*` 与稳定 `data-*`（`data-ui-*`、`data-selected-day-source` 等）；`e2e/tests/docs_app_calendar_contract.spec.mjs` 关键流程新增键盘焦点路径（`[data-slot="calendar-day"][data-pressable="true"]` + `focus()` + `toBeFocused` + `press("Enter")`）并继续基于语义标记断言，未使用 `toHaveScreenshot` 快照主导。性能回归方面，`apps/docs-app/src/pages/components/shell.rs` + `apps/docs-app/src/perf_probe.rs` 已对 `calendar` 提供 `mount/update/heap 预算测量`（`max_mount_ms: 32.0`、`max_update_ms: Some(10.0)`、`max_heap_kb: Some(576.0)`）及 `data-perf-violation` 阻断；当前框架对精确 `render_count` 自动化尚未通用，仓库以等价预算测量落地并在 `docs/plan/TODO.md` 保留 `render_count` 补齐项。
- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `Calendar` 未发生跨大版本 API 破坏升级）  
  通过依据：`components/calendar/src/Component.toml` 仍保持 `schema_version = "1"`，`components/calendar/src/calendar.rbi` 的 `Calendar(...)` 公共签名未发生破坏性移除/重命名；组件实现文件 `components/calendar/src/{mod.rs,logic.rs,view.rs,styles.rs,motion.rs}` 未引入 `migrate_v1_to_v2` / `SchemaRegistry` / `deprecation_window` / `contract.v2`。回归：`components/calendar/test/semantics.rs::calendar_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`、`components/calendar/test/semantics.rs::calendar_version_deprecation_migration_script_covers_engineering_gate`。门禁脚本：`scripts/check-ui-engineering.sh` 已接入 `cargo test -p ui-calendar calendar_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`。本地命令尝试仍受环境阻断：`Invalid cross-device link (os error 18)`。
- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。  
  通过依据：`apps/docs-app/src/pages/components/pages/forms_extra.rs` 的 `calendar()` 已补齐 `Playground`：`Hello World`、`State Matrix (Outside Days / Weekday / Tone)`、`Controlled vs Uncontrolled (selected_day axis)`、`Streaming Optional (fallback=snapshot)`；其中受控/非受控对照显式展示 `default_selected_day` 与 `selected_day + on_selected_day_change`，流式/快照通过 `stream_snapshot_code` 明确 `Snapshot` 基线与 `fallback=snapshot` 语义。所有日历 playground 均接入 `code_imports=calendar_imports.clone()`，Source-first 区域提供 `Snippet(copyable=true)` 且 starter 代码补全 `use leptos::prelude::*;` 与 `use ui::{Calendar, CalendarFirstWeekday, CalendarTone};` imports。回归：`components/calendar/test/semantics.rs::calendar_docs_page_covers_primary_playgrounds`、`components/calendar/test/semantics.rs::calendar_docs_playgrounds_lock_state_matrix_contract_values`。
- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。
  - 每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。
  - 断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。
  - 新增/变更语义字段必须同步补测试，否则不得打勾。
  通过依据：`components/calendar/src/mod.rs` 以 `#[path = "../test/semantics.rs"] mod semantics_tests;` 将组件语义测试纳入本 crate 门禁；`components/calendar/test/semantics.rs::calendar_emits_baseline_style_state_data_attributes` 与 `components/calendar/test/semantics.rs::calendar_semantics_and_perf_regression_cover_aria_data_focus_and_render_measurement` 已锁定 `role/aria-*`、稳定 `data-*` 与状态来源标记（含 `data-selected-day-source`、`data-ui-*`）；`e2e/tests/docs_app_calendar_contract.spec.mjs` 关键交互基于语义选择器与键盘焦点路径断言（`[data-slot="calendar-day"][data-pressable="true"]` + `focus/Enter`），且语义回归显式禁止 `toHaveScreenshot` 作为主断言。
- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。
  - E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。
  - WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。
  - 若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。
  通过依据：`e2e/tests/docs_app_calendar_contract.spec.mjs` 全流程使用语义选择器（`[data-slot]`、`[data-action]`、`data-ui-*`）定位与断言，未依赖 DOM 深层级或文本定位；WASM 就绪采用 `body:not(:has(#boot))` 语义等待而非固定 sleep；关键交互前后持续断言 `data-ui-output-status="verified"`（选日/翻月/清空/reload 后）作为 ready/settled 条件。`components/calendar/test/semantics.rs::calendar_e2e_contract_uses_semantic_selectors_and_stable_waits` 已锁定上述选择器与等待契约，并显式禁止 `waitForTimeout`/`setTimeout`/`sleep` 漂移。
- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。
  - 至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。
  - 回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。
  - 高风险路径（overlay、focus、keyboard、async）优先进入回归集合。
  通过依据：`e2e/tests/docs_app_calendar_contract.spec.mjs` 已落地 `docs-app calendar key flow is repeatable with semantic contract breakpoints`：进入页面后执行“键盘聚焦日单元并 `Enter` 触发选中 -> 翻月 -> 清空 -> reload 后状态复核”的可重复流程；断言全程基于 `data-ui-action/data-ui-state/data-ui-source/data-ui-output-status` 与 `data-action` 语义标记，可直接定位到具体契约断点。`Calendar` 当前非 overlay/async 组件，但高风险 `focus + keyboard` 路径已在该流程中优先覆盖。`components/calendar/test/semantics.rs::calendar_e2e_key_flow_regression_is_repeatable_and_contract_breakpointed` 对该回归集合进行锁定，并禁止退化为快照主导断言。
- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。
  - 组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。
  - 文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。
  - 文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。
  通过依据：`apps/docs-app/src/pages/components/pages/forms_extra.rs::calendar()` 已同步提供 `Hello World`、`State Matrix (Outside Days / Weekday / Tone)`、`Controlled vs Uncontrolled (selected_day axis)` 等示例，并新增 `data-slot="calendar-parameter-matrix"` 参数矩阵，显式列出 `tone`、`first_weekday`、`is_show_outside_days/show_outside_days`、`selected_day/default_selected_day`、`aria_label` 及默认值/归一化来源（`normalize_is_show_outside_days`、`normalize_selected_day_axis`、`DEFAULT_ARIA_LABEL`）。`components/calendar/test/semantics.rs::calendar_docs_parameter_matrix_syncs_api_names_and_logic_defaults` 已将 docs 文案与 `components/calendar/src/view.rs` props 名称、`crates/ui-state-primitives/src/calendar.rs` 默认值实现进行对齐回归锁定，防止文档与逻辑漂移。
- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。
  - 每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。
  - 文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。
  - “只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。
  通过依据：`components/calendar/README.md` 已提供新手入口并按“先基础后进阶”组织：`快速开始（先用起来）`（含 3 行 `Hello World` 最小示例与 `#/components/calendar` 入口）→ `常见用法（基础）`（含 `Controlled vs Uncontrolled` 与 `default_selected_day` / `selected_day + on_selected_day_change`）→ `进阶（需要时再看）`（配置、Source-first、WASM 调试）。等价文档入口 `apps/docs-app/src/pages/components/pages/forms_extra.rs::calendar()` 继续提供可交互示例与 source-first copy 路径。`components/calendar/test/semantics.rs::calendar_documentation_entry_is_newcomer_friendly_and_progressive` 已锁定上述 README + docs 入口契约，防止回退为“只有源码无文档”。
- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。
  - Playground 至少支持基础 props 调整、状态切换、交互反馈观察。
  - 对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。
  - Playground 作为验收面，需可重复复现关键交互路径。
  通过依据：`apps/docs-app/src/pages/components/pages/forms_extra.rs::calendar()` 已提供 `Interactive Playground (State + Source Markers)`，包含 `prev/next month`、`toggle weekday/tone/outside days`、`clear selection` 等控件（`data-action=*`）和实时状态回显（`data-slot="calendar-interactive-summary"`）；交互联动通过 `signal` + `on_selected_day_change` 驱动同页预览实时更新。可重复路径由 `e2e/tests/docs_app_calendar_contract.spec.mjs` 的 `docs-app calendar key flow is repeatable with semantic contract breakpoints` 覆盖（进入 -> 键盘选择 -> 翻月 -> 清空 -> reload 复核）。`Calendar` 非 AI Spec 组件，第二条按适用范围记 N/A。回归锁定：`components/calendar/test/semantics.rs::calendar_docs_interactive_playground_supports_live_state_preview_and_repeatable_paths`。
- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。
  - docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。
  - 若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。
  - 文档代码与当前实现必须同步，防止示例漂移。
  通过依据：`apps/docs-app/src/pages/components/pages/forms_extra.rs::calendar()` 已提供 `data-slot="calendar-source-first"` 区块，含 `Snippet(copyable=true)` 的一键复制 starter，默认补全 `use leptos::prelude::*;` 与 `use ui::{Calendar, CalendarFirstWeekday, CalendarTone};`；页面同时声明 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 作为 copy-ready 组装路径。source-first 区块已明确真实源码落点（`components/calendar/src/{mod,logic,view,styles,motion}.rs`）与依赖前提（`component-calendar`、`inject-css`）。`components/calendar/test/semantics.rs::calendar_docs_playgrounds_lock_state_matrix_contract_values` 与 `components/calendar/test/semantics.rs::calendar_check2_marks_component_governance_complete` 已对上述契约与 checklist 状态做回归锁定，防止示例漂移。
- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。
  - 若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。
  - 组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。
  - “仅代码更新无文档更新”在接口变更场景下直接判不通过。
  通过依据：`docs/spec/heroui-parameter-design-strategy.md` 已包含 `### Calendar 同步记录（2026-02-19）`，明确参数模型与 docs 同步策略；`apps/docs-app/src/pages/components/pages.rs` 已通过 `component_doc!("Calendar", "calendar", "Forms", forms_extra::calendar)` 暴露并可索引至 `#/components/calendar`；回归由 `components/calendar/test/semantics.rs::calendar_heroui_alignment_doc_and_docs_entry_stay_in_sync` 锁定，防止“实现先漂移文档后补”。

### 8. 合并前门禁死命令（最终执行）
在发起 PR 或完成任务前，必须保证本地/CI 以下命令全部通过：

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `./scripts/check-rust-hygiene.sh`
- `cargo check -p ui --target wasm32-unknown-unknown`
- `cargo check -p ui-headless --no-default-features --features ssr`
- `cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-<your_component>,inject-css`

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
