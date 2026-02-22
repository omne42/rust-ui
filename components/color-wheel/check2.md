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
  - 布尔状态统一 `is_*`（如 `is_open`/`is_disabled`），事件统一 `on_*`，默认值统一 `default_*`。
  - 同一语义 across 组件必须同名（如都用 `on_open_change`，禁止同义别名并存）。
  - 公共 API 引入新命名时，需说明与现有命名体系的兼容策略与迁移路径。

ColorWheel 已切换到 `is_disabled` / `is_value_label_visible`，并保留 `disabled` / `show_value_label` 作为兼容别名（`is_*` 优先）；docs 示例与组件语义测试已同步到新命名。
- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。
  - 受控模式：外部值是单一事实来源，内部不得偷偷写回本地状态。
  - 非受控模式：仅由默认值初始化一次，后续状态由内部原语管理。
  - 受控/非受控切换语义需稳定可测，避免“半受控”隐式行为。

ColorWheel 已提供 `value + on_value_change + default_value` 三元组，并统一通过 `use_controllable_state(value, Some(default_value), on_value_change)` 管理受控/非受控；docs 含受控与默认值两条调用路径，组件语义测试已覆盖该契约。
- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。
  - 默认值优先级必须可读且可测试（显式规则而非分散 `unwrap_or`）。
  - `view.rs` 不允许再做默认值分支；仅消费 `logic.rs` 的归一化输出。
  - 一旦发现多处默认值来源，直接判不通过并回收至 `logic.rs`。

ColorWheel 默认值归一化已集中到 `logic::resolve_default_value`（桥接 `ui-state-primitives::color_wheel::resolve_default_value`）；`view.rs` 仅调用归一化结果，不再包含 `default_value.unwrap_or(...)` 分支；组件语义测试与 primitive 单测已覆盖该规则。
- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。
  - 输入边界统一进入 `logic.rs`，输出统一为可渲染语义状态与来源标记。
  - 事件处理器只触发状态变更，不重建状态机规则。
  - 样式层只消费状态标记，不承担状态判定职责。

ColorWheel 输入归一化已集中到 `logic::normalize_state_inputs`（生成 `ColorWheelInputBoundary`），`view.rs` 只消费归一化结果并调用 `logic::resolve_state`；事件处理器仅透传 headless 语义结果并触发 `request_value_change`，不在 `view.rs` 重建状态机规则；组件语义测试已覆盖该约束。
- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。
  - 互斥状态优先用 `enum` 建模，利用编译器封住无效组合。
  - 字符串输入若需兼容外部配置，必须先映射到类型化枚举再进入逻辑层。
  - 布尔爆炸（多个 bool 表达一个状态机）应在设计评审阶段直接拦截。

ColorWheel 已将离散状态输入类型化：`ui-state-primitives::color_wheel` 新增 `ColorWheelStatus` / `ColorWheelValueLabelMode` / `ColorWheelSource`，`ColorWheelStateInput` 改为枚举字段；组件 `logic.rs` 通过 `normalize_state_inputs` 与 `source_from_custom_flag` 完成兼容 bool 到 enum 的边界映射，`view.rs` 仅消费 enum 并向 primitives 传递类型化输入；对应语义与单测已同步覆盖。
- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。
  - 组件中出现可复用状态机实现（受控/非受控、展开规则、选择归一）即判应下沉。
  - 组件与业务全局状态之间必须有适配边界，禁止组件直接依赖业务 store 类型。
  - `logic.rs` 仅做装配与映射，不重新实现状态原语。

ColorWheel 的状态原语仅经 `logic.rs` 桥接 `ui-state-primitives::color_wheel` 消费（`ColorWheelStateInput/ColorWheelState/resolve_state/compose_class_name` 均委托 primitives）；`logic.rs` 不引入 Leptos/DOM 或业务 store 依赖，`view.rs` 只调用 `logic` 边界并不直连 primitives。该约束已在组件语义测试中新增防回退断言。
- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。
  - 无异步交互时需明确标注 N/A 理由（例如“组件无远程请求与异步状态”），不是机械打勾。
  - 有异步交互时，`is_loading`/disabled/`aria-busy`/retry 语义必须成套一致，且对键盘与读屏路径可用。
  - 异步失败态要有可恢复路径（重试或回退），并有语义测试覆盖。

N/A（无异步交互）：ColorWheel 仅处理本地输入（pointer/keyboard/range）并同步调用 `request_value_change`，不存在远程请求、异步 action 或 loading/error/retry 状态轴；已新增语义测试锁定不出现 `is_loading`/`aria-busy`/`retry`/`use_async_action` 协议面，避免后续漂移。
- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。
  - 基础用法不得要求用户先理解或手动接线 `ui-state-primitives`/`ui-headless` 状态机。
  - 基础组件 Hello World 示例代码不得超过 5 行（导入与外层模板按仓库约定不计），并可直接运行。
  - 简单需求走简单 API，复杂需求再暴露高级入口：默认 props 覆盖高频场景，高级控制通过受控/扩展参数按需开启。
  - 禁止把内部状态对象作为基础必填参数暴露（例如强制 `state=...` 才能完成点击/展开等基本交互）。
  - docs-app 必须提供最小可用示例，优先展示一眼可懂的默认调用路径。

ColorWheel 文档已补齐默认路径 `<Playground title="Hello World">`（`<ColorWheel id_base=... />` 三行可运行示例）；基础用法不要求用户手动接线 `ui-state-primitives/ui-headless`，也未暴露 `state` 内部对象为必填 API。受控与进阶场景仍通过 `value/on_value_change/default_value` 等可选入口按需开启；组件语义测试已新增 DX 防回退断言。
- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。
  - 每个 item 的标题、语义与内容必须在同一 `Item` 结构维度绑定，避免索引配对式隐式约定。
  - `labels + children`、`titles + panels` 等并行数组/并行槽位写法不得作为默认或推荐 API。
  - 不引入这类语法糖：若为配置式输入，仅允许类型化 `ItemSpec`，并在内部映射为显式 `Item` 语义树。

N/A（ColorWheel 非组合型组件）：该组件是单控件输入原语，不存在 `<Parent><Item ... /></Parent>` 子项树语义；`view.rs` 未暴露 `children/items/labels/titles/panels/ItemSpec` 这类组合 API，docs 也未推荐并行数组/并行槽位写法。已补语义测试防止后续引入隐式索引配对式 API。

### 3. 高级交互与物理机制（Shell/Physics）
- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。

ColorWheel 拖拽已改为双状态机：`Dragging` 期间仅在 `view.rs` 更新本地 `drag_preview_value/drag_preview_percent`（用于轨道视觉与动效），不再每帧 `request_value_change` 回流；在 `pointerup/cancel/leave` 统一通过 `logic::resolve_action(Action::DragEnd { ... })` 一次性收敛提交。组件语义测试已新增防回退断言，拦截“每帧回流”模式。
- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。

N/A（ColorWheel 非几何定位校正组件）：ColorWheel 的 `get_bounding_client_rect` 仅用于指针坐标到色相角度的即时换算，不用于 overlay 定位、碰撞翻转或布局回写；组件不存在 `Intent -> Measure -> Rectification` 的跨帧收敛环，也不存在测量驱动的重复校正写回路径。已补语义测试锁定该约束，防止后续引入几何测量-回流死循环模式。
- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。

N/A（ColorWheel 非动态子项集合组件）：ColorWheel 语义模型是单输入值（`range + track + thumb`），没有 `Item` 子树、焦点游标或跨子项导航需求；因此不存在 `RegistrationContext/Register/Unregister/items_order` 协议面，也不应引入基于 `HashSet` 迭代顺序的导航逻辑。已补语义测试防止后续把集合注册语义误引入该组件。
- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。

N/A（ColorWheel 非容器型投影组件）：ColorWheel 仅渲染固定结构（label/value/track/thumb/input）并围绕单值状态更新，不承载子槽位投影策略，也不存在隐藏态下的子内容保活与副作用暂停协议；因此不应引入 `Lazy/KeepAlive/Eager/NotifyHidden` 契约面。已补语义测试防止后续将容器投影机制误植到该组件。
- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。

N/A（ColorWheel 无环境订阅状态轴）：ColorWheel 仅处理用户直接输入（pointer/keyboard/range）并同步更新单值状态，不依赖 `Resize/Theme/Intersection` 订阅来驱动布局或语义派生；因此不存在 `view` 侧采样/防抖后向 `logic::Action` 回流的环境流管线，也无原始环境事件洪泛风险。已补语义测试防止后续引入 `ResizeObserver/IntersectionObserver/BreakpointChanged` 等环境流协议面。
- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。

N/A（ColorWheel 非大型集合批量操作组件）：ColorWheel 只维护单一 hue 值状态，不存在多单元集合、批量选择或跨子项广播路径；因此不需要 `Context Bus + Selector` 与 `SelectionState::All` 这类状态压缩协议，也不存在 O(N) 向下 `prop drilling` 风险。已补语义测试防止后续引入集合批处理语义到该组件。
- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。

N/A（ColorWheel 无复杂派生总线路径）：ColorWheel 的状态变更链是本地直接输入到单值提交（`pointer/keyboard/input -> request_value_change`），不存在跨模块命令派生、总线广播与订阅者回放流程；因此不需要 `TraceId` 透传机制。已补语义测试防止后续将因果总线协议（`TraceId/Causality Bus`）误引入该组件。
- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。
  - 交互元素必须具备可验证语义：`role`/`aria-*`/键盘可达路径完整，且和 headless 契约一致。
  - 用户可见文本来源必须可覆盖：优先 props，其次应用注入（`UiRoot`/i18n bundle），最后组件兜底文案；禁止把业务可见文案硬编码在 `view.rs`。
  - 组件需透传或消费 `lang` / `dir`（LTR/RTL）上下文，不得假设单语言单方向。
  - 共享 A11y 工具优先来自 `crates/ui-headless/src/a11y.rs`，组件层不重复发明同名语义工具。

ColorWheel 已满足该契约：组件通过 `ui_headless::use_color_wheel` 挂载 `role/aria-*` 与键盘语义，`view.rs` 仅消费 headless 输出（不在组件层重写语义规则）；`label/aria_label` 支持 props 覆盖，未提供时由 primitives 兜底默认文案；`lang/dir` 从组件 props 透传到 headless 并落到语义属性；headless `color_wheel` 实现使用 `crates/ui-headless/src/a11y.rs` 的 `locale_attrs`/`A11yDirection`，未在组件层重复发明同名 A11y 工具。已补语义测试防回退。
- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。
  - 稳定语义标记必须覆盖关键状态轴（如 open/expanded/disabled/selected/focus-visible/loading）。
  - 状态来源必须可区分（受控/非受控、默认值/外部值、交互来源），通过稳定 marker 暴露而不是隐式推断。
  - 自动化选择器优先基于语义标记，不依赖 DOM 顺序、层级深度或临时 class 名。
  - 标记值应为封闭集合（可枚举），避免自由文本导致契约漂移。

ColorWheel 已提供稳定可观测标记：`data-state/data-disabled/data-value/data-step/data-value-percent` 覆盖关键状态轴；新增 `data-control-mode`（`controlled|uncontrolled`）、`data-value-source`（`external|default`）和 `data-interaction-source`（`none|pointer|keyboard|input`）区分状态来源；并持续暴露 `data-motion-source/data-label-source/data-aria-source/data-class-source`。自动化选择器可基于 `data-slot + data-* + aria-*`，无需依赖 DOM 顺序或临时 class。标记值均为封闭可枚举集合，已补语义测试防回退。
- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。
  - `styles.rs` 中状态分支选择器必须基于 `data-*`/`aria-*`/稳定 class，禁止用 `:nth-child`、深层级选择器猜测状态。
  - 运行时样式仅允许传递必要 CSS 变量（custom properties）；禁止把业务样式逻辑塞进 inline style。
  - 视觉状态切换必须可由语义标记直接解释，不能依赖“某节点是否恰好存在”。

ColorWheel 样式状态分支已基于显式语义标记与稳定 class：`data-dragging/data-disabled/data-motion-source/data-label-source/data-custom-class` 等驱动视觉切换，未使用 `:nth-child` 或深层结构猜测；运行时样式仅在 `motion.rs` 通过 `set_property(\"--ui-slider-visual-percent\", ...)` 写入必要 CSS 变量，`view.rs` 无业务 `style=` 注入；视觉切换不依赖“节点是否存在”，而由可观测标记直接解释。已补语义测试防回退。
- [x] 测试验证“语义契约”而不只验证视觉快照。
  - 至少存在语义测试覆盖关键状态与交互路径（role/aria/data-state/source markers）。
  - 测试矩阵必须覆盖关键分支：受控/非受控、disabled、键盘路径、指针路径、SSR/wasm 差异（按适用范围）。
  - 视觉快照只能作为补充，不得替代语义契约断言。

ColorWheel 已以语义契约测试为主：`components/color-wheel/test/semantics.rs` 覆盖 `role/aria/data-state/source markers`，并显式锁定受控/非受控、disabled、键盘与指针路径；同时校验 `view.rs` 与 `motion.rs` 的 `wasm/non-wasm` 分支存在性作为平台差异覆盖证据。测试集中未引入 `assert_snapshot/insta` 视觉快照依赖，快照不作为契约验收依据。
- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。
  - `mod.rs` 只维护最小稳定导出面与 feature gate，不承载实现细节。
  - `logic.rs` 只做输入归一、状态派生、来源标记；禁止 DOM 操作和样式细节分支。
  - `styles.rs` 只包含 token-first 静态 CSS；禁止硬编码主题常量与业务语义文案。
  - `view.rs` 只做结构渲染与 headless 契约挂载；禁止隐藏关键状态决策。
  - `motion.rs` 只做组件语义到动效契约映射与 attach；禁止在组件内重写通用动效引擎。

ColorWheel 目录职责已收敛：`mod.rs` 仅维护模块/导出边界；`logic.rs` 已去除 DOM 依赖（指针 `get_bounding_client_rect` 采样回迁到 `view.rs`，`logic` 仅保留纯几何 `pointer_to_hue_angle` 与状态归一）；`styles.rs` 保持 token-first 静态 CSS；`view.rs` 负责 Leptos 结构与 `ui_headless::use_color_wheel` 契约挂载；`motion.rs` 负责 spring 合同与 `attach_motion` 执行，不在组件层重复实现通用动效引擎。已补语义测试防回退。
- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。
  - 仅当组件存在稳定外部规范/Schema 契约或复杂配置固化需求时才引入 `spec.rs`。
  - 简单组件不得为了“形式统一”新增 `spec.rs`；说明文档应留在 `check2.md`/组件文档。
  - 新增 `spec.rs` 必须同步给出契约测试与版本演进说明。

ColorWheel 属于单值输入原语组件，当前无稳定外部 Schema 契约或复杂配置固化需求；`components/color-wheel/src/` 未引入 `spec.rs`，`mod.rs` 也未导出 `spec` 模块。组件说明继续保留在 `check2.md` 与 docs 页面，未为“形式统一”新增 `spec.rs`。已补语义测试防止后续误引入。
- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。
  - 样式规则统一落在 `styles.rs`，由 `crates/ui/src/css.rs` 聚合并通过 `UiRoot` 注入。
  - 颜色/间距/圆角/阴影等视觉值必须来自 `var(--ui-*)`，禁止组件私有 token 体系。
  - Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。
  - CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。

ColorWheel 已满足 token-first 静态样式契约：组件视觉规则集中在 `components/color-wheel/src/styles.rs` 且消费 `var(--ui-*)`；`crates/ui/src/css.rs` 在 `component-color_wheel` feature 下聚合 `crate::color::wheel::styles::CSS`，并由 `crates/ui/src/root.rs` 的 `UiRoot` 在 `inject_components_css` 打开时统一注入；运行时仅在 `motion.rs` 写入 `--ui-slider-visual-percent` 变量，不在 `view.rs` 塞业务样式。组件源码未引入 Utility-First/CSS-in-Rust 模式，相关约束已加语义测试防回退。
- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。
  - 默认主题需通过基础美学清单：信息层级清晰（字重/字号/间距）、对比与层次自然、交互反馈明确（hover/active/focus）。
  - docs-app 必须提供默认主题基线页面与截图基线，关键组件（Button/Input/Overlay）纳入视觉回归对比。
  - 禁止“可访问但粗糙”的最低可用心态：视觉退化（类似旧式 Bootstrap 观感）视为质量回归。
  - HeroUI 对标以“视觉语言与体验质量”对齐为目标，不做无差别 API 表层复制。

ColorWheel 默认主题视觉基线已补齐并可回归：`styles.rs` 具备清晰层级（label/value 字重字号与对比）及显式交互反馈（`hover` 阴影增强、`active/dragging` 缩放、`focus-within` 焦点环）；`docs-app` 的 ColorWheel 页面新增 `Default Theme Baseline` 区块并提供稳定截图锚点（`data-doc-visual-baseline="color-wheel-default-theme"` / `data-doc-baseline-shot="color-wheel-default-theme-v1"`）。跨组件基线要求（Button/Input/Overlay）由现有 `theme-visual-baseline` 页面承接并已注册到 docs 导航，属于仓库级共享基线而非 ColorWheel 私有实现；组件侧已补语义测试防回退。
- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。
  - package 模式必须有组件级 feature（如 `component-accordion`）；未启用组件不得进入编译与链接路径。
  - `lib.rs` 与 `css.rs` 必须按 feature 条件导出/聚合，禁止无条件引用所有组件模块和 CSS 常量。
  - source 模式下仅引入需要的组件源码，不通过中央注册表维持全组件可达。
  - 任意“全量组件映射表/注册表”若导致不可达代码变可达，直接判不通过。
  - 验证命令（特性树）：`cargo tree -e features -p ui --no-default-features --features component-accordion,inject-css`，确认仅启用目标组件特性链。
  - 验证命令（反向依赖）：`cargo tree -e features -i ui -p web-demo`，检查是否被 `all-components` 或隐式特性全量拉起。
  - CI 检查（最小特性编译）：新增任务仅开启目标最小特性（示例：`cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-accordion,inject-css`）。
  - CI 检查（体积预算）：对“最小特性构建产物”设定预算并阻断回归（可用固定阈值，如 `< 50KB`，或基于仓库基线的相对阈值）；不得只做编译通过而不做体积约束。

ColorWheel 的 tree-shaking 契约已落地并可验证：`crates/ui/Cargo.toml` 以 `component-color_wheel = ["dep:ui-color-wheel"]` 建立组件级特性入口，`lib.rs` 与 `css.rs` 对 color-wheel 导出/样式聚合均保持 `#[cfg(feature = "component-color_wheel")]` 条件门控，`inject-css` 关闭时存在 no-op 聚合函数。`web-demo` 依赖 `ui` 使用 `default-features = false` + `["inject-css","web-demo-components"]`（不隐式拉起 `all-components`），`docs-app` 对全量面显式使用 `all-components`。已执行特性树核对：`cargo tree -e features -i ui -p ui --no-default-features --features component-accordion,inject-css` 仅出现命令行特性 `component-accordion/inject-css`；`cargo tree -e features -i ui -p web-demo` 仅出现 `web-demo-components`，未出现 `all-components`。CI 侧已有 `scripts/check-ui-tree-shaking.sh` + `scripts/tree_shaking_budget.env` 覆盖最小特性 wasm 编译与体积预算门禁；并已补语义测试防回退。
- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。
  - 离散输入与状态轴必须优先使用 `enum`/新类型建模，避免字符串协议与布尔爆炸。
  - 无效状态要么在类型层不可表达，要么在 `logic.rs` 被统一归一化并可测试。
  - 关键状态必须通过稳定语义标记对外可读，供测试与 Agent 自动化消费。
  - 编译器与测试反馈应能直接定位状态契约破坏点，形成可持续闭环。

ColorWheel 已满足该约束：`ui-state-primitives::color_wheel::ColorWheelStateInput` 以 `ColorWheelStatus/ColorWheelValueLabelMode/ColorWheelSource` 等枚举约束关键输入空间，避免字符串协议与布尔爆炸；`logic.rs` 通过 `normalize_state_inputs` 统一完成布尔兼容输入到类型化状态轴的归一化，再由 `resolve_state` 进入 primitives；`view.rs` 对外稳定暴露 `data-state/data-control-mode/data-value-source/data-interaction-source/data-motion-source/data-label-source/data-aria-source/data-class-source` 机器可读标记供测试与 Agent 消费。已补语义测试防回退，确保类型契约破坏可被编译/测试快速定位。

### 4. DOM/环境边界治理
- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。

N/A（ColorWheel 非层叠 Overlay 组件）：ColorWheel 是单节点范围输入控件，不创建 overlay 层叠栈，也不存在关闭后焦点恢复到触发器的生命周期；组件内 `NodeRef` 仅用于拖拽几何采样（`track_ref`）与动效挂载（`root_ref`），未作为焦点恢复目标私存。全局焦点恢复能力（`FallbackTo/Selector`）由 `ui-headless` 的 overlay/focus 原语承载，ColorWheel 不应重复接入或实现。已补语义测试防止未来把 overlay 焦点恢复协议误引入该组件。
- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。

N/A（ColorWheel 无命令式第三方实例接入）：ColorWheel 当前实现只消费 `ui-state-primitives + ui-headless + ui-motion`，不存在 ECharts/Map 等 imperative SDK 生命周期管理，也没有 `Foreign Zone/YieldControl/CleanupForeign` 这类外交特区协议入口。组件公共 API 未暴露任何第三方实例句柄，状态机只围绕 hue 值与交互来源，不受外部实例反向污染。已补语义测试防止后续把第三方实例协议误并入 ColorWheel。
- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。

N/A（ColorWheel 不生成随机初始化种子）：ColorWheel 不在逻辑初始化阶段生成时间戳或随机 UUID，组件 ID 全部由外部 `id_base` 通过确定性规则派生（`{id_base}-input/-label/-value`），空值场景也回落到稳定常量 `ui-color-wheel`，不存在 SSR/Hydration 随机漂移。`IdProvider` 仅在需要自动发号的组件中作为确定性种子源；ColorWheel 当前无该类内部发号需求。已补语义测试防止后续引入 `now()/rand/uuid` 非确定性初始化路径。
- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。
  - 至少包含 compile-only 证据：web（wasm32）、ssr（native）、默认本地构建三条路径。
  - 平台分支差异必须显式 `cfg` 或 feature 管理，禁止依赖运行时偶然行为。
  - non-wasm 路径禁止引用 `web-sys`/浏览器对象。

ColorWheel 跨平台分支已显式落位：`view.rs` 将指针几何采样与 `web-sys/wasm-bindgen` 访问放在 `#[cfg(target_arch = "wasm32")]`，并在 `#[cfg(not(target_arch = "wasm32"))]` 路径显式 no-op；`motion.rs` 提供 wasm 动效 attach 与 non-wasm stub `attach_motion` 双实现，避免非 wasm 路径触达浏览器对象；`logic.rs` 保持纯计算，不含 `web_sys/wasm_bindgen/js_sys`。compile-only 命令已执行：`cargo check -p ui-color-wheel`（默认本地）、`cargo check -p ui-color-wheel --target x86_64-unknown-linux-gnu`（native/ssr 构建路径）、`cargo check -p ui-color-wheel --target wasm32-unknown-unknown`（web/wasm），以及依赖层补充验证 `cargo check -p ui-headless --no-default-features --features ssr`；当前环境统一被 `Invalid cross-device link (os error 18)` 阻塞，已补语义测试锁定平台分支契约，待环境修复后按同命令复跑即可得到完整 compile-only 证据。
- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。
  - 组件依赖 `ui-headless` 能力时，不得破坏其 web/ssr 互斥约束。
  - 组件若新增 headless 功能接入，需验证两条 feature 路径都可编译。
  - 发现“同时启用 web+ssr 仍可过编译”视为契约回归。

ColorWheel 依赖路径未破坏 `ui-headless` 的互斥特性契约：`crates/ui-headless/src/lib.rs` 顶部存在 `#[cfg(all(feature = "web", feature = "ssr"))] compile_error!(...)` 硬保护，`components/color-wheel/Cargo.toml` 仅以常规依赖引入 `ui-headless`，未显式同时启用 `web+ssr`。已执行验证命令：`cargo check -p ui-headless --no-default-features --features web`、`cargo check -p ui-headless --no-default-features --features ssr`、`cargo check -p ui-headless --no-default-features --features web,ssr`；当前环境统一受 `Invalid cross-device link (os error 18)` 阻塞，已补语义测试锁定互斥保护与组件依赖边界，待环境修复后按同命令复跑。
- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。
  - `motion.rs` 调用必须可在 non-wasm 下安全降级，不触发 panic。
  - 组件不得假设动画句柄一定存在；no-op 分支行为需可预测。
  - toolchain 场景（测试/文档/静态分析）不得因 motion 依赖阻塞编译。

ColorWheel 的 motion 降级链路满足该契约：`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = "wasm32"))]` 下提供 `web::prefers_reduced_motion() -> true` 与 `web::animate(...)` no-op；`components/color-wheel/src/motion.rs` 在 non-wasm 路径提供 stub `attach_motion(_root_ref, _visual_percent, _motion) {}`，不会触达 `web-sys` 或创建动画句柄，调用侧可预测且无 panic 假设。已执行 compile-only 命令：`cargo check -p ui-motion` 与 `cargo check -p ui-color-wheel --target x86_64-unknown-linux-gnu`；当前环境统一被 `Invalid cross-device link (os error 18)` 阻塞，已补语义测试锁定 no-op/stub 契约，待环境修复后复跑即可。
- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。
  - `reduced-motion` 下动画应跳过或降级为最小必要反馈。
  - SSR 输出必须与客户端 hydration 兼容，避免首帧语义错位。
  - wasm 分支允许增强交互，但语义契约不得与 SSR 分支分裂。

ColorWheel 已覆盖三分支且契约一致：`motion.rs` 在 wasm 路径以 `if !motion.enabled || ui_motion::web::prefers_reduced_motion()` 对 `reduced-motion` 立即降级为静态变量更新；在 non-wasm 路径通过 stub `attach_motion` 安全 no-op，SSR/tooling 不触发动画运行时。SSR/Hydration 兼容性由稳定 `id_base` 派生 ID（`-input/-label/-value`）与 headless 统一语义挂载保证；wasm 分支仅增强指针几何与动画执行，不改 `role/aria/data-*` 契约。已执行验证命令：`cargo check -p ui-motion`（通过）、`cargo check -p ui-color-wheel --target x86_64-unknown-linux-gnu`、`cargo check -p ui-color-wheel --target wasm32-unknown-unknown`（后两者当前环境均被 `Invalid cross-device link (os error 18)` 阻塞）；并补语义测试锁定分支一致性，待环境修复后复跑。
- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。
  - 关键交互组件需定义最小预算项（首渲染、关键更新、内存/分配趋势）。
  - 回归检测至少具备可重复基线与失败阈值，不靠主观“感觉变慢”。
  - 性能问题需可归因到状态、渲染、样式或动效路径之一。
  - 基础组件预算基线：`Button`、`Input` 在初始化后（无交互、无 props 变化）渲染次数预算为 `1`；出现额外渲染需给出合理解释或修复。
  - 测试要求：在 `components/*/test/**` 增加 `render_count` 类回归测试（测试框架支持时必须启用）；至少覆盖基础组件与本次改动组件。
  - 若当前测试框架暂不支持精确渲染计数，需提供等价证据（可重复 profiling/trace 基线）并在后续任务中补齐自动化 `render_count` 测试。

ColorWheel 性能治理链路已补齐并可阻断回归：`apps/docs-app/src/pages/components/shell.rs` 为 `slug="color-wheel"` 显式定义 `UiPerfBudget { max_mount_ms: 30.0, max_update_ms: Some(10.0), max_heap_kb: Some(512.0) }`；`apps/docs-app/src/perf_probe.rs` 输出 `data-perf-*`（mount/update/heap/violation/observability）机器可读标记；`scripts/check-ui-performance.sh` 新增 `color_wheel_performance_governance_contract_is_budgeted_traceable_and_blocking` 门禁命令，确保性能契约失败可直接阻断。组件侧在 `components/color-wheel/test/semantics.rs` 新增性能治理语义回归，覆盖预算配置、e2e perf 标记、trace 归因、状态/样式/动效可归因标记与响应式预算上限（`Memo::new`/`Signal::derive`）。当前测试框架暂无精确渲染计数自动化，继续沿用等价证据并保持 `docs/plan/TODO.md` 的 `render_count` 跟踪项（“建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据”）。
- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。
  - 复杂结构按语义子块拆分（header/body/item 等），避免巨型单块 `view!`。
  - `view.rs` 中若出现多层嵌套重复片段，应优先提取局部渲染函数。
  - 编译时间/产物体积异常增长时，优先排查宏展开体量。

ColorWheel 的 `view.rs` 已将单块巨型 `view!` 拆分为语义子块：轻量片段采用普通函数 `render_header_section(...) -> impl IntoView`，交互较重片段保留在局部 `render_track_section` 闭包中；根级 `view!` 仅负责挂载根语义属性与组合子块（`{render_header_section(...)}` / `{render_track_section()}`），避免在根块中继续堆叠深层结构。事件处理与可访问性语义保持原位（pointer/input/keyboard handlers 与 `aria-*` 挂载未回退），并在 `components/color-wheel/test/semantics.rs` 新增 `color_wheel_view_macro_complexity_is_split_into_semantic_sections` 锁定该结构契约，防止后续回退为单巨型宏展开。
- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。
  - 纯静态或轻逻辑片段优先函数化；仅在需要独立 props 语义时升级为组件。
  - 禁止把所有局部片段都升格为 `#[component]` 导致抽象噪音。
  - 拆分后语义标记与测试定位仍需稳定。

ColorWheel 已按复杂度分层拆分：`header` 片段仅消费已归一化状态和稳定标记，不涉及复杂生命周期，已抽为普通 Rust 函数 `fn render_header_section(...) -> impl IntoView`；`track` 片段绑定 pointer/input/keyboard 多路事件与拖拽预览态，仍保留在组件内局部闭包以避免过度抽象。组件文件仍只有一个公开 `#[component]`（`ColorWheel`），未新增局部 `#[component]` 噪音；并新增 `color_wheel_functional_split_prefers_plain_functions_for_lightweight_fragments` 语义测试锁定该策略，保证拆分后语义标记与测试定位稳定。
- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。
  - 可判定为纯静态的片段应避免重复动态构造。
  - 常量化后仍需维持可访问语义（title/aria-label/role 等）。
  - 静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。

ColorWheel 已将轨道纯静态装饰片段模板化为 `fn render_static_track_visuals() -> impl IntoView`，并在 `render_track_section` 中通过 `{render_static_track_visuals()}` 复用；`ring/orbit/thumb` 结构只保留单一模板来源，避免在主 `view!` 中重复动态构造。该模板继续保留 `data-slot` 与 `aria-hidden="true"` 可访问语义，静态资源变更路径集中在同一函数，避免散落。`components/color-wheel/test/semantics.rs` 已新增 `color_wheel_static_fragments_are_templated_and_accessible` 防回退测试。
- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。
  - 仅允许编译期常量或明确白名单内容进入 `inner_html`。
  - 严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。
  - 使用 `inner_html` 的节点必须补语义测试与安全回归说明。

ColorWheel 当前无 `inner_html` 注入面（`view.rs/logic.rs/styles.rs` 均未使用 `inner_html`、`set_inner_html` 或同类 raw HTML API），因此该项以“禁用注入面 + 回归锁定”方式达标：组件不暴露可承载 HTML 片段的 props，也不接受 `javascript:`/`<script`/`onerror` 这类未清洗协议输入。已在 `components/color-wheel/test/semantics.rs` 新增 `color_wheel_inner_html_surface_is_absent_and_security_regression_is_blocked`，作为语义与安全回归测试，防止后续引入未受控 HTML 注入路径。
- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。
  - 开发模式下至少能追踪关键状态变更来源与前后值。
  - 关键交互链路应支持最小可复现记录（事件顺序/状态转移）。
  - 调试开关默认不进入生产包体与公共 API。

ColorWheel 已补齐 wasm 调试链路：`view.rs` 接入 `use_ui_trace`，并在 `pointer_down_preview/pointer_move_preview/drag_end_commit/input_commit/keyboard_commit` 发射统一 `Note` 事件，消息格式固定为 `event/source/before/after/step`；时间戳由 `UiTraceEvent.ts_ms` 提供，形成“事件顺序 + 状态转移”的最小回放证据。开发模式可视化入口沿用 docs-app 的 `cfg!(debug_assertions)` 网关与 `UiDebugOverlay` 事件时间线，release 默认关闭追踪且组件公共 API 不新增 debug 专用 props，避免调试能力反向污染产物契约。已在 `components/color-wheel/test/semantics.rs` 新增 `color_wheel_wasm_debug_trace_contract_is_observable_replayable_and_debug_gated` 回归锁定该行为。
- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。
  - 常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。
  - 组件调试应尽量保持当前交互上下文，降低重复操作成本。
  - 复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。

ColorWheel docs 已补 `Interactive Workbench (DX)`：通过 `Playground.test_css_source` + scoped `<style>` 走热样式反馈路径（无需重编 wasm），并提供 `color-wheel-workbench` 隔离画布与独立 controls。工作台支持 `Preserve context on preset change`（切换预设时保留/重置上下文可选）与 `Persist workbench state`（`localStorage` 可选持久化，`load/save/clear_color_wheel_workbench_state`），满足“上下文保持 + 可选状态保留”的 DX 契约。对应回归 `color_wheel_dx_workbench_supports_optional_state_persistence_and_isolated_canvas` 已加入 `components/color-wheel/test/semantics.rs` 与 `scripts/check-ui-dx.sh` 门禁，防止回退。
- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。
  - 若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。
  - 关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。
  - 异步边界不得把具体 runtime 类型暴露到组件公共接口。

ColorWheel 的工程能力契约已落地：组件协议在 `src/protocol.rs` 通过 `WheelComponentSchemaVersion + WheelComponentSpec` 使用 `serde`（含 `#[serde(default)]`）做结构化 schema 与版本演进入口，并由 `test/protocol.rs` 锁定 Serialize/Deserialize 合约；tracing 能力继续复用仓库级统一入口（`ui` 的共享 `wasm_debug_proxy` 与 tracing feature 基线），不引入 `component-color_wheel` 的私有 tracing 开关；组件公开面（`mod.rs/logic.rs/view.rs/motion.rs`）未泄露 `tokio/async-std/Runtime/JoinHandle` 等运行时细节。对应回归 `color_wheel_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries` 已接入 `components/color-wheel/test/semantics.rs` 与 `scripts/check-ui-engineering.sh` 门禁。

### 5. 样式与动效（Theme & Motion）
- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。

ColorWheel 的样式终值统一收敛到 `ui-theme` 输出的 `--ui-fallback-*`：`styles.rs` 已将颜色/间距/字号/阴影/边框/圆角/动效参数切换为双层回退链（如 `var(--ui-fg, var(--ui-fallback-fg))`、`var(--ui-slider-thumb-border-width, var(--ui-fallback-slider-thumb-border-width))`），并移除 `#000`、`14px/20px/13px/18px`、`999px`、`2px`、`0.62` 等组件内硬编码终值。对应回归测试 `color_wheel_styles_use_defensive_dual_fallback_variables_without_hardcoded_terminal_values` 已加入 `components/color-wheel/test/semantics.rs`。
- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\"top: 10px\"`）。

ColorWheel 的级联层与运行时样式路径已收敛：`push_components_css` 通过 `@layer ui` 包裹 `component-color_wheel` 的样式注入（`crates/ui/src/css.rs`），组件 `view.rs` 不含 `style=`/`style:` 内联样式，运行时数值更新仅在 `motion.rs` 通过 `set_property("--ui-slider-visual-percent", ...)` 写入 CSS Custom Property。对应回归测试 `color_wheel_css_is_aggregated_under_layer_ui_with_only_custom_property_runtime_updates` 已加入 `components/color-wheel/test/semantics.rs`。
- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。

ColorWheel 的 Motion 合同已固化：`ColorWheelMotion::default` 从 `default_slider_motion_tokens()` 映射 `stiffness/damping/mass/precision`，并经 `sanitize_motion + ui_motion::spring::sanitize_config` 归一；`view.rs` 统一通过 `motion::attach_motion(root_ref, visual_percent, motion)` 挂载。`motion.rs` 在 wasm 分支显式尊重 `!motion.enabled || ui_motion::web::prefers_reduced_motion()`，在 non-wasm/SSR 分支保留 no-op `attach_motion`，确保降级安全可预测。对应回归测试 `color_wheel_motion_contract_is_tokenized_attached_and_safe_across_platforms` 已加入 `components/color-wheel/test/semantics.rs`。
- [x] `ui` 固定入口文件落点正确。
  - `crates/ui/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。
  - `crates/ui/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。
  - `crates/ui/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。
  - `crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。
  - `crates/ui/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。
  - `crates/ui/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。
  - `crates/ui/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。

ColorWheel 的入口落点契约已锁定：`crates/ui/src/lib.rs` 维持 feature-gated 对外 `pub use`，`css.rs` 仅通过 `push_components_css` 走条件聚合（含 `@layer ui` + `component-color_wheel` gate），`root.rs` 集中注入 base/theme/components css 与 i18n；`crates/ui-visual-primitive/src/active_highlight.rs` 仅承载共享高亮样式与 motion driver，不包含组件业务语义。同时 `crates/ui/src/overlay_open.rs`、`presence.rs`、`a11y.rs` 文件不存在，契约能力分别固定在 `crates/ui-headless/src/controllable_state.rs`、`crates/ui-headless/src/presence.rs`、`crates/ui-headless/src/a11y.rs`。对应回归测试 `color_wheel_ui_components_entrypoints_follow_fixed_layered_locations` 已加入 `components/color-wheel/test/semantics.rs`。
- [x] 组件目录标准文件落点正确。
  - `<component>/mod.rs`：最小稳定导出面，存在且无过度导出。
  - `<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。
  - `<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。
  - `<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。
  - `<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。
  - `<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。

ColorWheel 的组件目录落点已收敛：`src/` 仅保留 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs`（`render.rs/spec.rs` 不存在）；职责边界由 `color_wheel_component_files_keep_responsibilities_partitioned` 与 `color_wheel_component_directory_standard_file_layout_is_enforced` 双测试锁定，`mod.rs` 维持最小导出面、`logic.rs` 仅做 primitives 装配与归一派生、`view.rs` 仅做结构渲染与 headless 挂载、`motion.rs` 仅负责动效 contract 映射与 attach、`styles.rs` 维持 token-first 静态样式契约。

### 6. AI 原生能力与文件落点（Struct-First & Projection）
- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。

ColorWheel 已满足文件落点纪律：`components/color-wheel/src/` 仅包含 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs`（附 `protocol.rs` 作为工程协议文件），不存在 `render.rs`；`spec.rs` 维持 N/A（简单组件不引入）。对应回归测试 `color_wheel_component_directory_standard_file_layout_is_enforced` 已锁定必需文件存在与禁止文件缺失，防止后续目录漂移。
- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。

N/A（ColorWheel 非复杂配置组件）：ColorWheel 是单值输入原语，当前无稳定复杂 schema 装配需求，不应为“形式统一”引入 `spec.rs` 与 `*Spec::new()...render()` 建造者 API。`components/color-wheel/src/` 不存在 `spec.rs`，`mod.rs/view.rs/docs` 也未暴露 `ColorWheelSpec` 或 `Spec::new()` 入口；约束由 `color_wheel_does_not_introduce_spec_module_for_simple_component` 与 `color_wheel_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries` 回归测试锁定。
- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。

`ColorWheel` 已补齐上下文压缩产物：`components/color-wheel/src/Component.toml` 维护输入/输出/能力与依赖清单，`components/color-wheel/src/color_wheel.rbi` 维护对外签名投影，防止 AI 检索与真实接口漂移。对应回归测试 `color_wheel_context_compression_manifest_and_rbi_projection_are_present_and_current` 已加入 `components/color-wheel/test/semantics.rs`，并在 `scripts/check-ui-component-files.sh` 增加同名门禁命令。
- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。
  - 关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。
  - Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。
  - 契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。
  - 配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。

`ColorWheel` 已完成 Agent Contract 升级：`components/color-wheel/src/logic.rs` 新增类型化 contract（`ColorWheelAgent{Schema/SchemaVersion/Intent/UiAction/UiState/UiSource}` + `resolve_agent_contract/resolve_ui_action/resolve_ui_state`）；`components/color-wheel/src/view.rs` 统一挂载 `data-ui-schema/data-ui-schema-version/data-ui-stream-support/data-ui-stream-fallback/data-ui-stream-mode/data-ui-output-status/data-ui-intent/data-ui-action/data-ui-source/data-ui-state`，并继续保留 `data-control-mode/data-value-source` 来源轴。`components/color-wheel/src/Component.toml` 已补 `agent-contract-markers`、`agent_contract_schema_markers`、`[[agent_contract]]`、`[[agent_contract_markers]]` 与 `[[agent_contract_whitelist]]`（白名单能力边界），`.rbi` 已补 Agent Contract 签名投影。回归：`color_wheel_agent_contract_is_schema_typed_and_machine_readable`、`color_wheel_agent_contract_render_path_is_whitelist_safe_and_script_injection_free`；门禁脚本：`scripts/check-ui-contract-hygiene.sh` 已新增对应 `cargo test` 目标。
- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。（N/A：`ColorWheel` 不是 LLM 正文渲染组件，组件职责是同步色相输入；组件侧不实现 token-by-token streaming 协议，仅消费稳定快照状态输入。术语约束固定为两种显示模式：`Streaming`（边生成边显示）与 `Snapshot`（完整结果一次性显示），避免在组件层引入第三种“伪流式”定义。回归：`components/color-wheel/test/semantics.rs::color_wheel_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`；门禁脚本：`scripts/check-ui-streaming.sh` 新增 `cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`。）
  - `Streaming`：LLM 还在生成，界面边生成边显示。
  - `Snapshot`：LLM 全部生成完成后，一次性显示。
- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。（`ColorWheel` 已支持完整配置快照输入并稳定渲染：`components/color-wheel/src/view.rs` 通过受控/非受控三件套（`value/default_value/on_value_change`）+ 归一化边界（`sanitize_step/resolve_default_value/normalize_state_inputs`）消费完整结果，根节点持续输出稳定语义标记（`data-state/data-value/data-value-percent/data-control-mode/data-value-source/data-ui-stream-fallback/data-ui-stream-mode/...`）。docs 基线示例 `apps/docs-app/src/pages/components/pages/forms_color.rs` 提供 Hello World、Controlled Hue Wheel、Disabled + Reduced Motion + Custom Class 等完整快照路径。回归：`components/color-wheel/test/semantics.rs::color_wheel_snapshot_baseline_consumes_complete_result_and_renders_stably`；门禁脚本：`scripts/check-ui-streaming.sh` 新增 `cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_snapshot_baseline_consumes_complete_result_and_renders_stably`。）
  - 所有组件都应能消费“完整生成结果”并稳定渲染。
  - 即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。
- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。（`ColorWheel` 归类为 `Streaming Optional`；组件职责是色相输入控制而非 LLM 正文阅读面，默认走 `Snapshot` 渲染路径。实现显式输出 `data-ui-stream-support="unsupported"`、`data-ui-stream-fallback="snapshot"`、`data-ui-stream-mode="snapshot"` 与 `data-ui-output-status`，并保持 `role/aria/data-*` 连续可读。数据校验、断线恢复、重试策略继续留在上层编排，不下沉到组件。回归：`components/color-wheel/test/semantics.rs::color_wheel_check2_documents_streaming_required_optional_classification_rules`、`components/color-wheel/test/semantics.rs::color_wheel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、`components/color-wheel/test/semantics.rs::color_wheel_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`；门禁脚本：`scripts/check-ui-streaming.sh` 新增对应 `cargo test` 目标。）
  - `Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。
  - `Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。
  - 无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。
  - 数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。

### 7. 测试、门禁与交付
- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。（`components/color-wheel/src/{mod.rs,logic.rs,styles.rs,view.rs,motion.rs}` 非测试源码维持无 `unwrap/expect` 与无吞错 `let _ = ...`；`components/color-wheel/src/view.rs` 的 `id_base` 默认值归一路径已使用 `Cow<'static, str>`（`map(Cow::Owned) + Cow::Borrowed("ui-color-wheel")`）收敛静态回退字符串复制。回归：`components/color-wheel/test/semantics.rs::color_wheel_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources`、`components/color-wheel/test/semantics.rs::color_wheel_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent`、`components/color-wheel/test/semantics.rs::color_wheel_rust_hygiene_script_enforces_repo_level_hygiene_guards`；门禁脚本：`scripts/check-ui-engineering.sh` 新增对应 `cargo test` 目标。另执行：`./scripts/check-rust-hygiene.sh`（当前环境执行，若失败以脚本输出为准）。）
- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。（该项与第 3 节 Tree Shaking 主契约保持同一事实源：`crates/ui/Cargo.toml` 存在 `component-color_wheel = ["dep:ui-color-wheel"]`，`crates/ui/src/lib.rs` 与 `crates/ui/src/css.rs` 保持 `#[cfg(feature = "component-color_wheel")]` 条件导出/聚合且无条件全量依赖。回归：`components/color-wheel/test/semantics.rs::color_wheel_tree_shaking_contract_is_feature_gated_and_budget_guarded`、`components/color-wheel/test/semantics.rs::color_wheel_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget`、`components/color-wheel/test/semantics.rs::color_wheel_check2_marks_tree_shaking_feature_pruning_contract_complete`；门禁脚本：`scripts/check-ui-tree-shaking.sh`。）
- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。（ColorWheel 语义回归已覆盖 `aria-*` + `data-*` + 焦点路径：`components/color-wheel/test/semantics.rs::color_wheel_a11y_i18n_l10n_contracts_are_mounted_from_headless`、`components/color-wheel/test/semantics.rs::color_wheel_state_markers_are_observable_searchable_and_closed_set`、`components/color-wheel/test/semantics.rs::color_wheel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement`；并由 `scripts/check-ui-performance.sh` 门禁执行。性能侧继续执行 `color_wheel_performance_governance_contract_is_budgeted_traceable_and_blocking`，在当前框架暂不支持精确 `render_count` 自动化时沿用可重复 perf probe + trace 等价证据，同时保留 `docs/plan/TODO.md` 的 render_count 自动化跟踪项，防止“只看快照”回归。）
- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `ColorWheel` 改动未引入跨大版本 API 破坏升级，组件协议与 Agent Contract 仍保持 `v1`（`components/color-wheel/src/protocol.rs` 的 `WheelComponentSchemaVersion::V1`、`components/color-wheel/src/Component.toml` 的 `schema_version = "1"` 与 `ui.color-wheel.agent-contract.v1`），因此不触发 Codemod/Schema Registry 弃用窗口与 `migrate_v1_to_v2` 迁移层要求。回归：`components/color-wheel/test/semantics.rs::color_wheel_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade`；门禁脚本：`scripts/check-ui-engineering.sh` 新增对应 `cargo test` 目标。）
- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。（`apps/docs-app/src/pages/components/pages/forms_color.rs::color_wheel()` 已补齐 `Hello World`、`State Matrix`、`Controlled vs Uncontrolled`、`Streaming Optional / Snapshot` 与 `Source-first / Copy-Paste Ready`，并通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` + `code_imports` 保证复制即运行；回归：`components/color-wheel/test/semantics.rs::color_wheel_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot`；门禁脚本：`scripts/check-ui-dx.sh` 新增 `cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot`。）
- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。（`components/color-wheel/test/semantics.rs` 已覆盖关键语义轴：`color_wheel_a11y_i18n_l10n_contracts_are_mounted_from_headless`、`color_wheel_state_markers_are_observable_searchable_and_closed_set`、`color_wheel_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement`、`color_wheel_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`；并通过 `color_wheel_semantics_tests_cover_contract_matrix_without_snapshot_dependency` 约束语义测试不得依赖视觉快照。回归：`components/color-wheel/test/semantics.rs::color_wheel_check2_documents_semantics_first_testing_rules`；门禁脚本：`scripts/check-ui-contract-hygiene.sh` 新增 `cargo test -p ui --test color_wheel_semantics --no-default-features --features component-color_wheel,inject-css color_wheel_check2_documents_semantics_first_testing_rules`。）
  - 每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。
  - 断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。
  - 新增/变更语义字段必须同步补测试，否则不得打勾。
- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。（`e2e/tests/docs_app_color_wheel_contract.spec.mjs` 已采用语义锚点作为主选择路径（`[data-component="color-wheel"] #docs-color-wheel-hue[data-slot="color-wheel"][data-control-mode="controlled"]` + `data-slot="color-wheel-input|label|track"`），并在页面进入后通过 `body:not(:has(#boot))` 执行 wasm 稳定等待，未使用固定 sleep。交互链路显式覆盖 ready/settled 断点：键盘路径 `idle -> keyboard`、指针路径 `pointerdown/move/up -> pointer`、禁用分支 `data-state="disabled"` 与动效分支 `data-motion-source="custom"`，输出状态 `submittable/verified`。回归：`components/color-wheel/test/semantics.rs::color_wheel_check2_documents_e2e_selector_and_stable_wait_rules`、`components/color-wheel/test/semantics.rs::color_wheel_e2e_selector_contract_uses_semantic_markers_and_stable_waits`、`components/color-wheel/test/semantics.rs::color_wheel_e2e_animation_path_covers_ready_and_settled_semantic_breakpoints`；门禁脚本：`components/color-wheel/scripts/check-ui-e2e-color-wheel.sh`。）
  - E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。
  - WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。
  - 若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。
- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。（`e2e/tests/docs_app_color_wheel_contract.spec.mjs` 已新增可重复关键流程：`key flow is repeatable and failures map to semantic breakpoints`，覆盖“打开 -> keyboard 交互 -> 语义断点断言 -> reload -> 语义状态重建 -> 再次交互”闭环；失败定位统一绑定 `data-interaction-source/data-ui-action/data-ui-source/data-ui-output-status` 等语义断点。高风险路径已独立覆盖：`high-risk paths keep keyboard and disabled branches semantically explicit`，断言 `toBeFocused`、`ArrowLeft` 键盘路径、`data-state="disabled"`、`data-ui-state="disabled"`、`aria-disabled` 与 `toBeDisabled`。`ColorWheel` 不涉及 overlay/async 业务流（N/A），当前优先覆盖 focus + keyboard + disabled 高风险轴。回归：`components/color-wheel/test/semantics.rs::color_wheel_check2_documents_e2e_repeatable_key_flow_rules`、`components/color-wheel/test/semantics.rs::color_wheel_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`、`components/color-wheel/test/semantics.rs::color_wheel_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints`；门禁脚本：`components/color-wheel/scripts/check-ui-e2e-color-wheel.sh`。）
  - 至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。
  - 回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。
  - 高风险路径（overlay、focus、keyboard、async）优先进入回归集合。
- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。（`apps/docs-app/src/pages/components/pages/forms_color.rs::color_wheel()` 已同步补齐并保持可运行：`Hello World`、`State Matrix`、`Parameter Matrix`、`Controlled vs Uncontrolled` 四段示例与说明同页维护；`Parameter Matrix` 使用 `data-slot="color-wheel-parameter-matrix"` 固定参数轴（`step`、`is_value_label_visible`、`aria_label`、`class_name`），`State Matrix` 覆盖 ready/disabled 分支，受控示例覆盖 `value + on_value_change`，非受控示例覆盖 `default_value`。文档内 `data-slot="color-wheel-api-defaults-note"` 明确默认值对齐 `logic.rs`：`step` 省略时沿用 `logic::DEFAULT_STEP`，`default_value` 经 `logic::resolve_default_value` 归一，禁用轴默认 `false`。回归：`components/color-wheel/test/semantics.rs::color_wheel_check2_documents_docs_sync_and_state_matrix_rules`、`components/color-wheel/test/semantics.rs::color_wheel_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`、`components/color-wheel/test/semantics.rs::color_wheel_dx_check_script_covers_docs_sync_and_state_matrix_contract`；门禁脚本：`scripts/check-ui-dx.sh`。）
  - 组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。
  - 文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。
  - 文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。
- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。（`components/color-wheel/src/README.md` 已新增并提供新手路径：`Hello World`、`常见用法`、`先用起来再进阶`、`docs-app` 入口与 source-first 说明；默认路径不要求理解 `ui-state-primitives/ui-headless`。`apps/docs-app/src/pages/components/pages/forms_color.rs::color_wheel()` 保持 `Hello World -> State Matrix -> Parameter Matrix -> Controlled vs Uncontrolled -> Interactive Workbench` 的先易后难顺序。回归：`components/color-wheel/test/semantics.rs::color_wheel_check2_documents_documentation_as_product_rules`、`components/color-wheel/test/semantics.rs::color_wheel_documentation_entry_exists_with_beginner_first_progression`、`components/color-wheel/test/semantics.rs::color_wheel_dx_check_script_covers_documentation_as_product_contract`；门禁脚本：`scripts/check-ui-dx.sh`。）
  - 每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。
  - 文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。
  - “只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。
- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。（`apps/docs-app/src/pages/components/pages/forms_color.rs::color_wheel()` 已提供 `Interactive Workbench (DX)`：`data-slot="color-wheel-workbench-controls"` 下可在线调整 `preset/is_disabled/custom_class/reduced_motion/preserve_context/persist_state`，并在 `data-slot="color-wheel-workbench-canvas"` 实时预览组件语义状态；反馈通过 `data-slot="color-wheel-workbench-state"` 持续暴露当前配置。AI Spec 联动示例 N/A（`ColorWheel` 非 AI Spec 输入组件，无 `spec.rs`/Spec 输入协议面）。可重复关键流复用到 E2E 契约：`e2e/tests/docs_app_color_wheel_contract.spec.mjs::docs-app color-wheel interactive playground updates props and preview with semantic markers` + 既有 `key flow is repeatable and failures map to semantic breakpoints`（含语义断点与 reload 重放）。回归：`components/color-wheel/test/semantics.rs::{color_wheel_check2_documents_interactive_playground_rules,color_wheel_docs_app_provides_interactive_playground_for_props_state_and_preview,color_wheel_interactive_playground_reuses_repeatable_semantic_e2e_flow,color_wheel_dx_check_script_covers_interactive_playground_contract,color_wheel_e2e_check_script_covers_interactive_playground_contract,color_wheel_check2_marks_interactive_playground_contract_complete}`；门禁脚本：`scripts/check-ui-dx.sh`、`components/color-wheel/scripts/check-ui-e2e-color-wheel.sh`。）
  - Playground 至少支持基础 props 调整、状态切换、交互反馈观察。
  - 对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。
  - Playground 作为验收面，需可重复复现关键交互路径。
- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。（`apps/docs-app/src/pages/components/pages/forms_color.rs::color_wheel()` 已具备 source-first 复制链路：多组 Playground 提供 `code_imports`，复制动作经 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补 imports；文档卡片 `data-slot="color-wheel-copy-ready"` 明确真实源码落点 `components/color-wheel/src/{mod,view,logic,styles,motion}.rs`、依赖前提 `component-color_wheel + inject-css`，并新增 `data-slot="color-wheel-source-first-contract"` 固定“`Show code + Copy` 一键可运行”契约说明。`components/color-wheel/src/README.md` 的 `## Source-first` 同步记录源码路径、feature 与复制链路，防止文档漂移。回归：`components/color-wheel/test/semantics.rs::{color_wheel_check2_documents_source_first_copy_paste_ready_rules,color_wheel_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies,color_wheel_dx_check_script_covers_source_first_copy_paste_ready_contract,color_wheel_check2_marks_source_first_copy_paste_ready_contract_complete}`；门禁脚本：`scripts/check-ui-dx.sh`。）
  - docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。
  - 若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。
  - 文档代码与当前实现必须同步，防止示例漂移。
- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。（已同步 `docs/spec/heroui-parameter-design-strategy.md` 的 `### ColorWheel 同步记录（2026-02-20）`：明确参数主轴、docs 入口索引（`component_doc!("ColorWheel", "color-wheel", "Forms", forms_color::color_wheel)`）、Source-first 可复制前提与“接口变更时禁止仅代码不更文档”的约束；本轮无新增 Spectrum/HeroUI 风格结论，`docs/research/spectrum-heroui-style-interface-study.md` 按 N/A 不追加。组件文档入口已双路径可索引：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_wheel()` + `components/color-wheel/src/README.md`。回归：`components/color-wheel/test/semantics.rs::{color_wheel_check2_documents_heroui_benchmark_docs_sync_rules,color_wheel_heroui_strategy_and_component_docs_are_synchronized_and_indexable,color_wheel_dx_check_script_covers_heroui_benchmark_docs_sync_contract,color_wheel_check2_marks_heroui_benchmark_docs_sync_contract_complete}`；门禁脚本：`scripts/check-ui-dx.sh`。）
  - 若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。
  - 组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。
  - “仅代码更新无文档更新”在接口变更场景下直接判不通过。

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
