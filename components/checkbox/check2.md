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
  - 通过依据（checkbox）：`components/checkbox/src/logic.rs` 仅消费 `ui_state_primitives::checkbox::{CheckboxState, CheckboxStateInput, resolve_state}` 并装配受控/非受控来源标记；组件侧未重写状态原语 struct/state machine。
  - 通过依据（checkbox）：`crates/ui-state-primitives/src/checkbox.rs` 持有纯 Rust 状态定义（无 Leptos `Signal`、无 `web_sys` 依赖）；组件 `view.rs` 只解包信号值后调用 `logic::resolve_state`/`resolve_checked_control`。
  - 通过依据（checkbox）：新增回归 `components/checkbox/test/semantics.rs::checkbox_check2_documents_status_primitives_layer_rules`、`components/checkbox/test/semantics.rs::checkbox_status_primitives_layer_is_consumed_without_component_local_state_machine`、`components/checkbox/test/semantics.rs::checkbox_engineering_script_covers_status_primitives_and_two_pass_geometry_contracts`、`components/checkbox/test/semantics.rs::checkbox_check2_marks_status_primitives_and_two_pass_geometry_items_complete`，并同步镜像到 `components/checkbox/test/checkbox_semantics.rs::checkbox_check2_documents_status_primitives_layer_rules`、`components/checkbox/test/checkbox_semantics.rs::checkbox_status_primitives_layer_is_consumed_without_component_local_state_machine`、`components/checkbox/test/checkbox_semantics.rs::checkbox_engineering_script_covers_status_primitives_and_two_pass_geometry_contracts`、`components/checkbox/test/checkbox_semantics.rs::checkbox_check2_marks_status_primitives_and_two_pass_geometry_items_complete`。
  - 通过依据（checkbox）：`scripts/check-ui-components-engineering.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_status_primitives_layer_rules`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_status_primitives_layer_is_consumed_without_component_local_state_machine`
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
  - 放在 `crates/ui-components/src/<component>/motion.rs`：把组件语义状态（open/closed、enter/exit、active/inactive）映射为 `ui-motion` contract，绑定目标节点并调用 attach。
  - 禁止放在 `crates/ui-motion`：组件 slot 结构、组件专属状态机、ARIA/keyboard 语义、业务文案与业务分支。
  - 禁止放在组件 `motion.rs`：自实现 spring/keyframe/driver 执行器；跨组件共享动效算法必须回迁 `ui-motion`。
  - 动效参数优先来自 token/theme；禁止在组件样式与逻辑中散落硬编码时长/曲线/位移常量。
  - 非 wasm 路径必须提供 no-op/stub，保证 SSR/tooling 可编译且行为可预测。
- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。
  - Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量；组件只在 `crates/ui-components/src/<component>/styles.rs` 消费。
  - 三轴上下文（`system/color/scale`）在 `theme.rs` 定义；组件在 `logic.rs` 选择并在 `view.rs` 生效，`styles.rs` 只消费变量，不重建主题。
  - Token 分类必须可追溯：分类源在 `tokens.rs`，规范同步 `docs/spec/styling.md`；组件不得引入平行私有 token 命名体系。
  - 量化尺寸基准必须可回归：尺寸基准在 `tokens.rs` 与 `theme.rs` 定义，主题回归在 `crates/ui-theme/tests/token_scale_baseline.rs`，组件语义回归在 `components/*/test/*<component>_semantics.rs`。
  - 主题调色与语义色对比必须满足 `WCAG 2.1 AA` 基线，并覆盖 Light/Dark/OLED 主题变体。
  - 主题层只输出 `theme/tokens/base css` 与变量；不实现组件结构、交互逻辑、组件级动效编排。
  - 新增视觉语义先补 token，再由组件消费；禁止“组件临时值先落地、后补 token”的倒序流程。
- [x] `ui-components` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。
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
- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。
  - 受控模式：外部值是单一事实来源，内部不得偷偷写回本地状态。
  - 非受控模式：仅由默认值初始化一次，后续状态由内部原语管理。
  - 受控/非受控切换语义需稳定可测，避免“半受控”隐式行为。
- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。
  - 默认值优先级必须可读且可测试（显式规则而非分散 `unwrap_or`）。
  - `view.rs` 不允许再做默认值分支；仅消费 `logic.rs` 的归一化输出。
  - 一旦发现多处默认值来源，直接判不通过并回收至 `logic.rs`。
- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。
  - 输入边界统一进入 `logic.rs`，输出统一为可渲染语义状态与来源标记。
  - 事件处理器只触发状态变更，不重建状态机规则。
  - 样式层只消费状态标记，不承担状态判定职责。
- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。
  - 互斥状态优先用 `enum` 建模，利用编译器封住无效组合。
  - 字符串输入若需兼容外部配置，必须先映射到类型化枚举再进入逻辑层。
  - 布尔爆炸（多个 bool 表达一个状态机）应在设计评审阶段直接拦截。
- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。
  - 组件中出现可复用状态机实现（受控/非受控、展开规则、选择归一）即判应下沉。
  - 组件与业务全局状态之间必须有适配边界，禁止组件直接依赖业务 store 类型。
  - `logic.rs` 仅做装配与映射，不重新实现状态原语。
- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。
  - 无异步交互时需明确标注 N/A 理由（例如“组件无远程请求与异步状态”），不是机械打勾。
  - 有异步交互时，`is_loading`/disabled/`aria-busy`/retry 语义必须成套一致，且对键盘与读屏路径可用。
  - 异步失败态要有可恢复路径（重试或回退），并有语义测试覆盖。
  - N/A 理由（checkbox）：当前仅本地同步勾选状态切换，不发起远程请求，也不存在组件级异步加载/失败/重试状态轴。
- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。
  - 基础用法不得要求用户先理解或手动接线 `ui-state-primitives`/`ui-headless` 状态机。
  - 基础组件 Hello World 示例代码不得超过 5 行（导入与外层模板按仓库约定不计），并可直接运行。
  - 简单需求走简单 API，复杂需求再暴露高级入口：默认 props 覆盖高频场景，高级控制通过受控/扩展参数按需开启。
  - 禁止把内部状态对象作为基础必填参数暴露（例如强制 `state=...` 才能完成点击/展开等基本交互）。
  - docs-app 必须提供最小可用示例，优先展示一眼可懂的默认调用路径。
  - 通过依据（checkbox）：README 与 docs-app 已提供 `<Checkbox>"Accept terms"</Checkbox>` 最小示例，复杂场景通过可选 `is_checked/on_checked_change` 开启，无 `state` 必填输入。
- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。
  - 每个 item 的标题、语义与内容必须在同一 `Item` 结构维度绑定，避免索引配对式隐式约定。
  - `labels + children`、`titles + panels` 等并行数组/并行槽位写法不得作为默认或推荐 API。
  - 不引入这类语法糖：若为配置式输入，仅允许类型化 `ItemSpec`，并在内部映射为显式 `Item` 语义树。
  - N/A 理由（checkbox）：该组件是单体输入控件，不是组合容器，不存在 `Parent/Item`、`labels + children`、`titles + panels` 这类组合 API 面。

### 3. 高级交互与物理机制（Shell/Physics）
- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。
  - N/A 理由（checkbox）：当前仅支持点击/按键切换与 hover/press 动效，无拖拽态、无 `pointermove` 连续帧处理、无 `Action::DragEnd` 回流轴。
- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。
  - N/A 理由（checkbox）：该组件为单体勾选控件，无 tooltip/popover/menu 几何定位语义，不读取 DOM 尺寸/位置，不存在 `Intent -> Measure -> Rectification` 回流链路。
  - 通过依据（checkbox）：`components/checkbox/src/{logic,view,motion}.rs` 不包含 `getBoundingClientRect/getClientRects/clientWidth/clientHeight/measure_pass/rectification_pass` 等几何测量与二阶段纠偏实现。
  - 通过依据（checkbox）：新增回归 `components/checkbox/test/semantics.rs::checkbox_two_pass_geometry_rendering_is_na_and_measurement_free` 与 `components/checkbox/test/semantics.rs::checkbox_check2_marks_status_primitives_and_two_pass_geometry_items_complete`（同步镜像到 `components/checkbox/test/checkbox_semantics.rs`）。
  - 通过依据（checkbox）：当前环境执行新增 cargo 门禁命令时在编译产物写入阶段触发 `Invalid cross-device link (os error 18)`，阻断点不在该语义契约实现本身。
- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。
  - N/A 理由（checkbox）：该组件无动态子项集合，不存在 `RegistrationContext`、`Register/Unregister`、`items_order` 或基于迭代顺序的集合导航问题。
- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。
  - N/A 理由（checkbox）：该组件不是容器型投影组件，仅渲染单个控件与标签，不提供子树投影策略（`Lazy/KeepAlive/Eager`）或隐藏生命周期通知通道。
- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。
  - N/A 理由（checkbox）：该组件无 `Resize/Theme/Intersection` 环境订阅输入，不存在连续环境事件流与防抖采样路径，也无 `BreakpointChanged` 一类高层语义动作回流。
- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。
  - N/A 理由（checkbox）：该组件是单项输入控件，不承载大型集合批量操作，不存在 `Context Bus + Selector`、`SelectionState::All` 或 O(N) 级下钻分发路径。
- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。
  - N/A 理由（checkbox）：该组件不存在复杂派生总线与跨订阅者广播链路，交互为本地单点切换，不涉及 `TraceId` 透传场景。
- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。
  - 交互元素必须具备可验证语义：`role`/`aria-*`/键盘可达路径完整，且和 headless 契约一致。
  - 用户可见文本来源必须可覆盖：优先 props，其次应用注入（`UiRoot`/i18n bundle），最后组件兜底文案；禁止把业务可见文案硬编码在 `view.rs`。
  - 组件需透传或消费 `lang` / `dir`（LTR/RTL）上下文，不得假设单语言单方向。
  - 共享 A11y 工具优先来自 `crates/ui-headless/src/a11y.rs`，组件层不重复发明同名语义工具。
  - 通过依据（checkbox）：`view.rs` 通过 `use_checkbox` 挂载 `role/aria-*` 与键盘路径，文本由 `children`/`aria_label` 输入；`lang`/`dir` 透传到 headless，且 `ui-headless/src/checkbox.rs` 使用 `a11y::locale_attrs` 统一处理方向与语言。
- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。
  - 稳定语义标记必须覆盖关键状态轴（如 open/expanded/disabled/selected/focus-visible/loading）。
  - 状态来源必须可区分（受控/非受控、默认值/外部值、交互来源），通过稳定 marker 暴露而不是隐式推断。
  - 自动化选择器优先基于语义标记，不依赖 DOM 顺序、层级深度或临时 class 名。
  - 标记值应为封闭集合（可枚举），避免自由文本导致契约漂移。
  - 通过依据（checkbox）：`view.rs` 暴露 `data-state/data-checked/data-disabled/data-focus-visible/data-state-source/data-checked-source/data-handler-source` 与 `aria-*`；来源值来自 `ui-state-primitives` 枚举 `source_attr()`（封闭集合，避免自由文本漂移）。
- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。
  - `styles.rs` 中状态分支选择器必须基于 `data-*`/`aria-*`/稳定 class，禁止用 `:nth-child`、深层级选择器猜测状态。
  - 运行时样式仅允许传递必要 CSS 变量（custom properties）；禁止把业务样式逻辑塞进 inline style。
  - 视觉状态切换必须可由语义标记直接解释，不能依赖“某节点是否恰好存在”。
  - 通过依据（checkbox）：`styles.rs` 状态分支使用 `data-state/data-disabled/data-enabled/data-focus-visible` 与稳定 class，未使用 `:nth-child` 或 `:disabled` 猜测状态；`view.rs` 无业务 `style=` 分支，运行时仅在 `motion.rs` 写入 `--ui-checkbox-scale/--ui-checkbox-indicator` CSS 变量。
- [x] 测试验证“语义契约”而不只验证视觉快照。
  - 至少存在语义测试覆盖关键状态与交互路径（role/aria/data-state/source markers）。
  - 测试矩阵必须覆盖关键分支：受控/非受控、disabled、键盘路径、指针路径、SSR/wasm 差异（按适用范围）。
  - 视觉快照只能作为补充，不得替代语义契约断言。
  - 通过依据（checkbox）：`test/semantics.rs` 已含 `checkbox_state_markers_are_stable_observable_and_enumerated` 与 `checkbox_semantic_contract_matrix_covers_interaction_paths_without_snapshot_substitution`；前者断言 `role/aria/data-state/source markers`，后者覆盖受控/非受控/disabled（来自 `test/logic.rs`）、键盘/指针事件挂载、`wasm/non-wasm` 分支存在性，并显式禁止 `insta`/`assert_*_snapshot!` 替代语义断言。
- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。
  - `mod.rs` 只维护最小稳定导出面与 feature gate，不承载实现细节。
  - `logic.rs` 只做输入归一、状态派生、来源标记；禁止 DOM 操作和样式细节分支。
  - `styles.rs` 只包含 token-first 静态 CSS；禁止硬编码主题常量与业务语义文案。
  - `view.rs` 只做结构渲染与 headless 契约挂载；禁止隐藏关键状态决策。
  - `motion.rs` 只做组件语义到动效契约映射与 attach；禁止在组件内重写通用动效引擎。
  - 通过依据（checkbox）：`test/semantics.rs` 新增 `checkbox_component_files_respect_responsibility_boundaries`，逐文件断言 `mod` 仅导出边界、`logic` 无 DOM/样式实现、`styles` 保持 token-first 静态 CSS、`view` 不重写 primitives/motion 内核、`motion` 不承载 aria/view 语义并复用 `ui_motion::spring::SpringAnimator`。
- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。
  - 仅当组件存在稳定外部规范/Schema 契约或复杂配置固化需求时才引入 `spec.rs`。
  - 简单组件不得为了“形式统一”新增 `spec.rs`；说明文档应留在 `check2.md`/组件文档。
  - 新增 `spec.rs` 必须同步给出契约测试与版本演进说明。
  - N/A 理由（checkbox）：该组件是简单单体输入控件，不需要额外 `spec.rs` 建造者/Schema 固化；当前目录仅保留 `logic/styles/view/motion` 与最小 `protocol.rs` 版本化契约，无 `spec.rs` 文件与 `mod spec` 导出。
- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。
  - 样式规则统一落在 `styles.rs`，由 `crates/ui-components/src/css.rs` 聚合并通过 `UiRoot` 注入。
  - 颜色/间距/圆角/阴影等视觉值必须来自 `var(--ui-*)`，禁止组件私有 token 体系。
  - Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。
  - CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。
  - 通过依据（checkbox）：`test/semantics.rs` 新增 `checkbox_token_first_static_styles_are_aggregated_via_ui_root_without_utility_pollution`，断言 `styles.rs` 视觉值走 `var(--ui-*)`、`crates/ui-components/src/css.rs` 在 `component-checkbox` feature 下聚合 `crate::checkbox::styles::CSS`，并由 `crates/ui-components/src/root.rs` 的 `UiRoot`（`inject_components_css` + `push_components_css`）统一注入；同时阻断 `@apply/tailwind/tw-/styled(/css!/stylex` 等 Utility-First/CSS-in-Rust 污染。
- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。
  - 默认主题需通过基础美学清单：信息层级清晰（字重/字号/间距）、对比与层次自然、交互反馈明确（hover/active/focus）。
  - docs-app 必须提供默认主题基线页面与截图基线，关键组件（Button/Input/Overlay）纳入视觉回归对比。
  - 禁止“可访问但粗糙”的最低可用心态：视觉退化（类似旧式 Bootstrap 观感）视为质量回归。
  - HeroUI 对标以“视觉语言与体验质量”对齐为目标，不做无差别 API 表层复制。
  - 通过依据（checkbox）：复用仓库级视觉基线门禁。`apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs` 提供 `ThemeVisualBaseline`（覆盖 `Button/Input/Overlay`），`e2e/tests/docs_app_theme_visual_baseline.spec.mjs` 锁定 page/button/input/overlay 截图基线，`docs/spec/heroui-parameter-design-strategy.md` 明确“对齐视觉语言与体验质量、不做 HeroUI API 表层复制”；组件侧回归由 `test/semantics.rs::checkbox_visual_desire_gate_reuses_theme_baseline_and_heroui_alignment_contracts` 约束 docs 注册与上述基线契约。
- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。
  - package 模式必须有组件级 feature（如 `component-accordion`）；未启用组件不得进入编译与链接路径。
  - `lib.rs` 与 `css.rs` 必须按 feature 条件导出/聚合，禁止无条件引用所有组件模块和 CSS 常量。
  - source 模式下仅引入需要的组件源码，不通过中央注册表维持全组件可达。
  - 任意“全量组件映射表/注册表”若导致不可达代码变可达，直接判不通过。
  - 验证命令（特性树）：`cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-checkbox,inject-css`，确认仅启用目标组件特性链。
  - 验证命令（反向依赖）：`cargo tree -e features -i ui-components -p web-demo`，检查是否被 `all-components` 或隐式特性全量拉起。
  - CI 检查（最小特性编译）：新增任务仅开启目标最小特性（示例：`cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features component-checkbox,inject-css`）。
  - CI 检查（体积预算）：对“最小特性构建产物”设定预算并阻断回归（可用固定阈值，如 `< 50KB`，或基于仓库基线的相对阈值）；不得只做编译通过而不做体积约束。
  - 通过依据（checkbox）：`crates/ui-components/Cargo.toml` 存在 `component-checkbox = ["dep:ui-checkbox"]`，`crates/ui-components/src/lib.rs`/`crates/ui-components/src/css.rs` 对 checkbox 导出与 CSS 聚合均以 `#[cfg(feature = "component-checkbox")]` 门控；`apps/web-demo/Cargo.toml` 显式 `default-features = false + web-demo-components`（未拉起 `all-components`），`apps/docs-app/Cargo.toml` 显式 `all-components` 作为全量验收面。
  - 通过依据（checkbox）：实测 `cargo tree -e features -i ui-components -p ui-components --no-default-features --features component-checkbox,inject-css` 仅出现命令行特性 `component-checkbox` 与 `inject-css`；`cargo tree -e features -i ui-components -p web-demo` 出现 `web-demo-components` 且未出现 `all-components`。
  - 通过依据（checkbox）：CI 已接入 `scripts/check-ui-components-tree-shaking.sh`（`.github/workflows/ci.yml`），脚本同时执行最小特性 `cargo check` + wasm release 构建，并依据 `scripts/tree_shaking_budget.env` 的 `TREE_SHAKING_BASELINE_RLIB_BYTES/TREE_SHAKING_MAX_RATIO_PERCENT` 做体积预算阻断；本地 wasm 编译受环境 `Invalid cross-device link (os error 18)` 影响，编译证据以 CI 门禁为准。
- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。
  - 离散输入与状态轴必须优先使用 `enum`/新类型建模，避免字符串协议与布尔爆炸。
  - 无效状态要么在类型层不可表达，要么在 `logic.rs` 被统一归一化并可测试。
  - 关键状态必须通过稳定语义标记对外可读，供测试与 Agent 自动化消费。
  - 编译器与测试反馈应能直接定位状态契约破坏点，形成可持续闭环。
  - 通过依据（checkbox）：离散输入与关键状态已类型化，`logic.rs` 使用 `CheckboxVariant/CheckboxSize/CheckboxCheckedState`，状态来源使用 `ui-state-primitives::checkbox` 的 `CheckboxControlMode/CheckboxCheckedValueSource/CheckboxChangeHandlerSource`（封闭枚举，`source_attr()` 输出固定集合）。
  - 通过依据（checkbox）：无效组合在 `logic.rs` 统一归一化，`resolve_checked_control` 与 `derive_render_state` 将受控/非受控、禁用与焦点可见等状态收敛为单一 `CheckboxRenderState`；对应回归位于 `components/checkbox/test/logic.rs`（如 `derive_render_state_centralizes_state_derivation_and_source_marker`）。
  - 通过依据（checkbox）：关键状态通过稳定语义标记对外暴露，`view.rs` 输出 `data-state/data-checked/data-unchecked/data-disabled/data-focus-visible/data-state-source/data-checked-source/data-handler-source` 与 `aria-*`，可直接供自动化选择与 Agent 消费。
  - 通过依据（checkbox）：契约破坏点可被编译器与测试直接拦截。类型漂移会在编译期报错；语义契约由 `components/checkbox/test/semantics.rs` 中 `checkbox_discrete_states_are_enum_constrained`、`checkbox_state_markers_are_stable_observable_and_enumerated` 锁定；来源枚举闭包由 `crates/ui-state-primitives/src/test/checkbox.rs::checked_source_and_handler_source_attrs_are_closed_enumerations` 约束。

### 4. DOM/环境边界治理
- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。
  - N/A 理由（checkbox）：该组件是单体输入控件，不参与层叠 `Overlay` 打开/关闭链路，也不存在“关闭后恢复焦点目标”语义；`view.rs` 的 `NodeRef` 仅用于本地动效挂载（root/indicator），不作为焦点恢复句柄，因此不涉及 Focus Stack/FallbackTo/Selector 契约。
- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。
  - N/A 理由（checkbox）：该组件未集成 ECharts/Map 或其他命令式第三方运行时实例，不存在 `Foreign Zone` 边界需求；现有 `wasm_bindgen/web_sys` 仅用于组件内部动效样式变量写入（`motion.rs`），未将第三方实例暴露到公共 API，也未反向污染状态机。
- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。
  - N/A 理由（checkbox）：该组件无内部 ID 生成与跨端 ID 对齐需求，不创建 `for/id/aria-controls` 成对标识，因此不需要接入 `IdProvider` 种子链路。
  - 通过依据（checkbox）：`logic.rs/view.rs/motion.rs` 与 `ui-headless/src/checkbox.rs` 不包含 `now()/Date::now/SystemTime::now/Instant::now/Uuid::new_v4/rand/Math::random`；新增 `components/checkbox/test/semantics.rs::checkbox_ssr_hydration_discontinuity_contract_avoids_time_random_and_implicit_ids` 锁定该契约并阻断回归。
- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。
  - 至少包含 compile-only 证据：web（wasm32）、ssr（native）、默认本地构建三条路径。
  - 平台分支差异必须显式 `cfg` 或 feature 管理，禁止依赖运行时偶然行为。
  - non-wasm 路径禁止引用 `web-sys`/浏览器对象。
  - 通过依据（checkbox）：`motion.rs` 对 `attach_root_motion/attach_indicator_motion` 提供显式 `#[cfg(target_arch = "wasm32")]` 与 `#[cfg(not(target_arch = "wasm32"))]` 分支；`logic.rs/styles.rs/view.rs` 不引用 `web_sys/js_sys/wasm_bindgen`，浏览器对象仅在 wasm 分支使用。
  - 通过依据（checkbox）：仓库 compile-only 门禁已覆盖三路径，`scripts/check.sh` 包含 native 默认检查、`ui-headless --features ssr`（ssr native）与 `--target wasm32-unknown-unknown --features web`（web wasm）及 `ui-components` wasm 路径；`scripts/check-ui-components-platforms.sh` 补充跨平台 compile-only 与 web+ssr 互斥守卫。
  - 通过依据（checkbox）：新增 `components/checkbox/test/semantics.rs::checkbox_platform_contract_covers_web_ssr_wasm_and_non_wasm_boundaries`，锁定上述 `cfg` 分支、non-wasm API 边界与 compile-only 脚本证据，防止契约回归。
- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。
  - 组件依赖 `ui-headless` 能力时，不得破坏其 web/ssr 互斥约束。
  - 组件若新增 headless 功能接入，需验证两条 feature 路径都可编译。
  - 发现“同时启用 web+ssr 仍可过编译”视为契约回归。
  - 通过依据（checkbox）：`crates/ui-headless/src/lib.rs` 顶层存在 `#[cfg(all(feature = "web", feature = "ssr"))] compile_error!(...)`，明确禁止同时启用 `web+ssr`。
  - 通过依据（checkbox）：`components/checkbox/Cargo.toml` 仅以路径依赖 `ui-headless`，未强行注入 `features = ["web","ssr"]` 组合，未破坏 headless 互斥约束。
  - 通过依据（checkbox）：`scripts/check.sh` 保持 `ssr` 与 `web wasm` 两条 compile-only 路径；`scripts/check-ui-components-platforms.sh` 额外执行 `cargo check -p ui-headless --no-default-features --features web,ssr` 并强制校验失败日志包含 `mutually exclusive`，可阻断互斥契约回归。
  - 通过依据（checkbox）：新增 `components/checkbox/test/semantics.rs::checkbox_headless_web_ssr_mutex_contract_is_preserved`，锁定上述 `compile_error!` 守卫、组件依赖边界与脚本互斥门禁。
- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。
  - `motion.rs` 调用必须可在 non-wasm 下安全降级，不触发 panic。
  - 组件不得假设动画句柄一定存在；no-op 分支行为需可预测。
  - toolchain 场景（测试/文档/静态分析）不得因 motion 依赖阻塞编译。
  - 通过依据（checkbox）：`crates/ui-motion/src/lib.rs` 提供 `#[cfg(not(target_arch = "wasm32"))] pub mod web` no-op backend，`prefers_reduced_motion()` 固定返回 `true`，`animate(...)` 为安全空实现；`crates/ui-motion/tests/non_wasm_stub.rs` 覆盖该 stub 行为。
  - 通过依据（checkbox）：`components/checkbox/src/motion.rs` 对 `attach_root_motion/attach_indicator_motion` 提供非 wasm 分支，行为仅为 `std::hint::black_box(sanitize_motion(motion))`，不依赖动画句柄存在，不触发 `panic/unwrap/expect`。
  - 通过依据（checkbox）：新增 `components/checkbox/test/semantics.rs::checkbox_motion_non_wasm_stub_contract_is_predictable_and_safe`，锁定 `ui-motion` non-wasm stub 合约与 checkbox non-wasm 降级路径，避免 SSR/tooling 场景回归。
- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。
  - `reduced-motion` 下动画应跳过或降级为最小必要反馈。
  - SSR 输出必须与客户端 hydration 兼容，避免首帧语义错位。
  - wasm 分支允许增强交互，但语义契约不得与 SSR 分支分裂。
  - 通过依据（checkbox）：`styles.rs` 提供 `@media (prefers-reduced-motion: reduce)` 分支并禁用过渡；`ui-motion` 的 `SpringAnimator::set_target` 在 reduced-motion 下即时收敛（由 `crates/ui-motion/tests/spring.rs` 覆盖）。
  - 通过依据（checkbox）：`motion.rs` 对 root/indicator attach 都有显式 wasm 与 non-wasm 分支（wasm 增强、non-wasm 安全降级），且 non-wasm 分支不依赖动画句柄。
  - 通过依据（checkbox）：`view.rs` 未做目标平台条件分支，`role/aria-*` 与 `data-*` 语义标记对 SSR/wasm 保持同一契约；compile-only 证据来自 `scripts/check.sh` 的 `ui-headless ssr` 与 `ui-headless web wasm`/`ui-components wasm` 路径。
  - 通过依据（checkbox）：新增 `components/checkbox/test/semantics.rs::checkbox_reduced_motion_ssr_wasm_branches_keep_semantics_consistent`，锁定上述 reduced-motion、SSR、wasm 分支与语义一致性要求。
- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。
  - 关键交互组件需定义最小预算项（首渲染、关键更新、内存/分配趋势）。
  - 回归检测至少具备可重复基线与失败阈值，不靠主观“感觉变慢”。
  - 性能问题需可归因到状态、渲染、样式或动效路径之一。
  - 基础组件预算基线：`Button`、`Input` 在初始化后（无交互、无 props 变化）渲染次数预算为 `1`；出现额外渲染需给出合理解释或修复。
  - 测试要求：在 `components/*/test/**` 增加 `render_count` 类回归测试（测试框架支持时必须启用）；至少覆盖基础组件与本次改动组件。
  - 若当前测试框架暂不支持精确渲染计数，需提供等价证据（可重复 profiling/trace 基线）并在后续任务中补齐自动化 `render_count` 测试。
  - 通过依据（checkbox）：`apps/docs-app/src/pages/components/shell.rs` 在 `component_page_perf_budget` 为 `"checkbox"` 定义显式预算（`max_mount_ms: 22.0`、`max_update_ms: Some(7.0)`、`max_heap_kb: Some(320.0)`），预算不再依赖 `_ => UiPerfBudget::mount_only(...)` 默认兜底。
  - 通过依据（checkbox）：`apps/docs-app/src/perf_probe.rs` 与 `e2e/tests/docs_app_components_coverage.spec.mjs` 已提供稳定 `data-perf-*` 观测与失败断言（`data-perf-violation`），可重复检测并阻断回归。
  - 通过依据（checkbox）：`scripts/check-ui-components-performance.sh` 新增 `checkbox` 性能契约门禁：`cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_performance_governance_budget_is_defined_and_blocking_locally`，并与 `button/input` 基线及 `perf_render_count_follow_up_is_tracked_in_plan` 同步执行。
  - 通过依据（checkbox）：新增 `components/checkbox/test/semantics.rs::checkbox_performance_governance_budget_is_defined_and_blocking_locally`，锁定预算定义、观测标记、阻断脚本与 `render_count` 跟踪链路（`docs/plan/TODO.md`）。
- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。
  - 复杂结构按语义子块拆分（header/body/item 等），避免巨型单块 `view!`。
  - `view.rs` 中若出现多层嵌套重复片段，应优先提取局部渲染函数。
  - 编译时间/产物体积异常增长时，优先排查宏展开体量。
  - 通过依据（checkbox）：`components/checkbox/src/view.rs` 将 indicator 嵌套片段拆分为 `render_checkbox_indicator` 与 `render_checkbox_indicator_icon` 普通函数，主组件保持单一公共 `#[component]` 边界，避免在根 `view!` 中内联深层 SVG 结构。
  - 通过依据（checkbox）：新增 `components/checkbox/test/semantics.rs::checkbox_view_macro_complexity_is_split_into_semantic_subrenders_locally`，锁定 `view!` 块数量上界、语义分块函数存在、无局部 `#[component]` 噪音。
  - 通过依据（checkbox）：`scripts/check-ui-components-view-macro.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_view_macro_complexity_is_split_into_semantic_subrenders_locally`，纳入统一阻断链路。
- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。
  - 纯静态或轻逻辑片段优先函数化；仅在需要独立 props 语义时升级为组件。
  - 禁止把所有局部片段都升格为 `#[component]` 导致抽象噪音。
  - 拆分后语义标记与测试定位仍需稳定。
  - 通过依据（checkbox）：`components/checkbox/src/view.rs` 将轻逻辑/静态片段拆分为普通函数 `render_checkbox_indicator` 与 `render_checkbox_indicator_icon`（返回 `impl IntoView`），未新增局部 `#[component]`。
  - 通过依据（checkbox）：主组件仍保持单一公共 `#[component] pub fn Checkbox(...)` 边界，局部渲染函数仅服务结构拆分，不引入额外 props 语义层。
  - 通过依据（checkbox）：新增 `components/checkbox/test/semantics.rs::checkbox_view_functional_split_prefers_plain_functions_over_local_components_locally`，锁定普通函数拆分、禁止局部 `#[component]`、并验证语义标记稳定性。
  - 通过依据（checkbox）：`scripts/check-ui-components-view-macro.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_view_functional_split_prefers_plain_functions_over_local_components_locally`，纳入统一阻断链路。
- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。
  - 可判定为纯静态的片段应避免重复动态构造。
  - 常量化后仍需维持可访问语义（title/aria-label/role 等）。
  - 静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。
  - 通过依据（checkbox）：`components/checkbox/src/view.rs` 将静态片段统一收敛为常量（`SLOT_CHECKBOX*`、`CHECK_ICON_*`），并由 `render_checkbox_indicator_icon`/`render_checkbox_indicator` 模板化消费，避免静态字面量在 `view!` 中散落重复。
  - 通过依据（checkbox）：静态 SVG 片段保留稳定可访问语义（`aria-hidden=\"true\"`、`focusable=\"false\"`）且组件主语义仍由 `role=aria.attrs.role`、`aria-*` 契约承载，不破坏读屏路径。
  - 通过依据（checkbox）：新增 `components/checkbox/test/semantics.rs::checkbox_static_fragments_are_constantized_with_stable_semantics_locally`，锁定静态常量定义、唯一字面量来源和语义标记稳定性。
  - 通过依据（checkbox）：`scripts/check-ui-components-view-macro.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_static_fragments_are_constantized_with_stable_semantics_locally`，纳入统一阻断链路。
- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。
  - 仅允许编译期常量或明确白名单内容进入 `inner_html`。
  - 严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。
  - 使用 `inner_html` 的节点必须补语义测试与安全回归说明。
  - 通过依据（checkbox）：组件运行时路径（`mod.rs/logic.rs/styles.rs/motion.rs/view.rs`）不包含 `inner_html/set_inner_html/dangerously_set_inner_html`，不存在原始 HTML 注入面。
  - 通过依据（checkbox）：docs 示例路径 `apps/docs-app/src/pages/components/pages/forms.rs` 不包含 `inner_html` 或脚本协议注入片段；该组件不依赖 HTML 字符串拼接展示。
  - 通过依据（checkbox）：新增 `components/checkbox/test/semantics.rs::checkbox_inner_html_usage_is_forbidden_in_component_and_docs_examples_locally`，锁定组件与 docs 路径禁用原始 HTML 注入契约。
  - 通过依据（checkbox）：`scripts/check-ui-components-inner-html.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_inner_html_usage_is_forbidden_in_component_and_docs_examples_locally`，纳入统一阻断链路。
- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。
  - 开发模式下至少能追踪关键状态变更来源与前后值。
  - 关键交互链路应支持最小可复现记录（事件顺序/状态转移）。
  - 调试开关默认不进入生产包体与公共 API。
  - N/A 理由（checkbox）：当前不引入组件专属 wasm-debug/replay 运行时与公共调试 API，采用“共享全局调试入口 + 组件语义标记等价追踪”策略，避免为单控件新增调试面导致产物污染。
  - 通过依据（checkbox）：`components/checkbox/Cargo.toml` 仅保留 `default = []`，无 `wasm-debug` 或 `checkbox-wasm-debug` 特性；`crates/ui-components/Cargo.toml` 未引入 `checkbox` 专属 wasm-debug feature，保持调试能力集中在共享特性链。
  - 通过依据（checkbox）：docs 全局调试入口保持在 `apps/docs-app/src/lib.rs` + `apps/docs-app/src/debug_overlay.rs` + `crates/ui-headless/src/trace.rs`（`provide_ui_trace(debug_overlay_enabled)`、`UiDebugOverlay`、事件 `ts_ms` 追踪），checkbox 不重复造轮子。
  - 通过依据（checkbox）：`view.rs` 暴露稳定状态与来源标记（`data-state/data-state-source/data-checked-source/data-handler-source/data-motion-source`）用于等价可追踪；组件公开面未泄漏 `UiTrace/provide_ui_trace/replay` 等调试内部能力。
  - 通过依据（checkbox）：新增 `components/checkbox/test/semantics.rs::checkbox_wasm_debug_contract_is_explicitly_na_and_feature_isolated_locally`，并在 `scripts/check-ui-components-wasm-debug.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_wasm_debug_contract_is_explicitly_na_and_feature_isolated_locally`。
- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。
  - 常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。
  - 组件调试应尽量保持当前交互上下文，降低重复操作成本。
  - 复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。
  - N/A 理由（checkbox）：当前 docs 交互演练默认走共享 Playground，不新增组件级本地持久化开关；可选状态保留维持“可接入但默认关闭”策略，避免为单控件引入额外存储协议噪音。
  - 通过依据（checkbox）：共享演练底座 `apps/docs-app/src/playground.rs` 已提供 scoped CSS 热替换与隔离预览舞台（`compose_scoped_css`、`data-playground-scope`、`playground__preview-stage`），样式调试无需重编 wasm。
  - 通过依据（checkbox）：`apps/docs-app/src/pages/components/pages/forms.rs` 的 `checkbox()` 提供 `Interactive Playground`，暴露 `interactive_checked` + `on_checked_change` 的上下文保持交互演练路径，具备隔离 workbench 入口。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na_locally`，锁定热样式、隔离画布、可选持久化 N/A 三项契约。
  - 通过依据（checkbox）：`scripts/check-ui-components-dx.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_dx_playground_supports_css_hot_reload_and_isolated_canvas_with_optional_persist_na_locally`，纳入统一 DX 阻断链路。
- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。
  - 若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。
  - 关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。
  - 异步边界不得把具体 runtime 类型暴露到组件公共接口。
  - N/A 理由（checkbox）：该组件无 `spec/config` 反序列化输入和版本迁移链路，`components/checkbox/src/` 下无 `spec.rs`，当前不需要组件级 `serde` 错误结构化流程。
  - N/A 理由（checkbox）：该组件无异步 IO 状态轴，不暴露运行时句柄；公共 API 仅暴露同步状态信号与语义属性，不绑定 `tokio/async-std`。
  - 通过依据（checkbox）：`components/checkbox/test/semantics.rs` 新增 `checkbox_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope_locally`，锁定无 `spec.rs`、无 `serde/serde_json` 漂移，并验证 checklist 工程契约条目。
  - 通过依据（checkbox）：`components/checkbox/test/semantics.rs` 新增 `checkbox_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events_locally`，约束 tracing 语义沿用仓库统一基线，不引入 checkbox 专属 tracing feature/event target。
  - 通过依据（checkbox）：`components/checkbox/test/semantics.rs` 新增 `checkbox_engineering_contract_avoids_runtime_leaks_in_public_api_surface_locally`，阻断 `tokio/async-std/smol/runtime::Handle` 等 runtime 细节泄漏到组件公开面。
  - 通过依据（checkbox）：`scripts/check-ui-components-engineering.sh` 新增 checkbox 三条门禁命令（上述三个测试），并由 `checkbox_engineering_check_script_covers_serde_tracing_and_runtime_boundaries_locally` 锁定脚本覆盖。

### 5. 样式与动效（Theme & Motion）
- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。
  - 通过依据（checkbox）：`components/checkbox/src/styles.rs` 已将关键主题 token 全量切换为双层回退链，覆盖间距/颜色/边框/焦点/尺寸/字号/动效等路径（如 `var(--ui-checkbox-gap, var(--ui-fallback-checkbox-gap))`、`var(--ui-border, var(--ui-fallback-border))`、`var(--ui-checkbox-size-sm, var(--ui-fallback-checkbox-size-sm))`）。
  - 通过依据（checkbox）：组件样式中移除 `var(--ui-font-size-150, 14px)`、`var(--ui-line-height-150, 20px)` 这类裸尺寸终值；fallback 终点统一落到 `--ui-fallback-*` 变量。
  - 通过依据（checkbox）：`crates/ui-theme/src/css.rs` 增补 checkbox 相关 fallback 终值变量（`--ui-fallback-checkbox-focus-outline-*`、`--ui-fallback-checkbox-size-*`、`--ui-fallback-checkbox-radius-*`、`--ui-fallback-checkbox-indicator-size-*`），保持 theme 层 SSOT。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_styles_use_defensive_variable_fallback_chain_locally`，锁定双层回退链、theme fallback 变量存在性与禁用裸终值。
  - 通过依据（checkbox）：`scripts/check-ui-components-contract-hygiene.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_styles_use_defensive_variable_fallback_chain_locally`，纳入统一阻断链路。
- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\"top: 10px\"`）。
  - 通过依据（checkbox）：`crates/ui-components/src/css.rs` 在 `push_components_css` 中统一写入 `@layer ui`，并在 `component-checkbox` feature 下聚合 `crate::checkbox::styles::CSS`，保持组件样式在统一层级注入。
  - 通过依据（checkbox）：`crates/ui-components/src/root.rs` 的 `UiRoot` 仅通过 `inject_components_css -> crate::css::push_components_css` 注入组件层样式，不在组件 view/logic 扩散业务内联样式。
  - 通过依据（checkbox）：`components/checkbox/src/view.rs` 与 `components/checkbox/src/logic.rs` 无 `style=` 业务内联分支；运行时样式更新集中在 `components/checkbox/src/motion.rs` 且仅写入 `--ui-checkbox-scale/--ui-checkbox-indicator` CSS custom properties。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_cascade_layer_and_runtime_style_contract_is_enforced_locally`，锁定 `@layer ui` 聚合、UiRoot 注入路径与“CSS 变量-only”运行时约束。
  - 通过依据（checkbox）：`scripts/check-ui-components-contract-hygiene.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_cascade_layer_and_runtime_style_contract_is_enforced_locally`，纳入统一阻断链路。
- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。
  - 通过依据（checkbox）：`components/checkbox/src/motion.rs` 以 `CheckboxMotion` 内置 spring contract（root/indicator 双轴），默认参数明确包含 `stiffness/damping`，并通过 `sanitize_motion` 对无效参数做收敛。
  - 通过依据（checkbox）：组件语义态通过 `attach_root_motion/attach_indicator_motion` 挂载到 motion 契约（checkbox 采用拆分 attach，而非单一 `attach_motion`），`view.rs` 在 root/indicator 节点显式调用挂载。
  - 通过依据（checkbox）：`styles.rs` 提供 `@media (prefers-reduced-motion: reduce)` 降级，`ui-motion` spring 在 reduced-motion 下即时收敛（`crates/ui-motion/tests/spring.rs` 回归覆盖）。
  - 通过依据（checkbox）：`motion.rs` 提供 `#[cfg(not(target_arch = "wasm32"))]` no-op 分支（`black_box(sanitize_motion(motion))`），保证 SSR/tooling 非 wasm 路径可预测且不 panic。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe_locally`，锁定 spring 参数合同、reduced-motion/no-op 路径与门禁脚本覆盖。
  - 通过依据（checkbox）：`scripts/check-ui-components-platforms.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe_locally`，纳入平台阻断链路。
- [x] `ui-components` 固定入口文件落点正确。
  - `crates/ui-components/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。
  - `crates/ui-components/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。
  - `crates/ui-components/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。
  - `crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。
  - `crates/ui-components/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。
  - `crates/ui-components/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。
  - `crates/ui-components/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。
  - 通过依据（checkbox）：`crates/ui-components/src/lib.rs` 保持 `#[cfg(feature = "component-checkbox")] pub use ui_checkbox as checkbox;`，并通过 `pub mod root; pub use root::UiRoot;` 暴露稳定入口；未泄漏 `web_sys/NodeRef/JsValue` 等平台细节类型。
  - 通过依据（checkbox）：`crates/ui-components/src/css.rs` 在 `push_components_css` 里保持 `@layer ui` 聚合，且 `component-checkbox` 通过 `out.push_str(crate::checkbox::styles::CSS);` 按 feature 条件注入。
  - 通过依据（checkbox）：`crates/ui-components/src/root.rs` 持续集中注入 `BASE_CSS + theme vars + (optional) components css`，并提供全局 `provide_ui_i18n(i18n)` 与 `provide_ui_id_provider(id_seed)`。
  - 通过依据（checkbox）：`crates/ui-visual-primitive/src/active_highlight.rs` 仅承载共享高亮样式/motion driver（`ActiveHighlightMotion` + `attach_active_highlight_motion`），不携带 checkbox 业务语义。
  - 通过依据（checkbox）：`crates/ui-components/src/overlay_open.rs`、`crates/ui-components/src/presence.rs`、`crates/ui-components/src/a11y.rs` 均不存在；canonical 能力固定在 `crates/ui-headless/src/{controllable_state.rs,presence.rs,a11y.rs}`。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_ui_components_fixed_entry_files_follow_layered_boundaries_locally`，并在 `scripts/check-ui-components-entrypoints.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_ui_components_fixed_entry_files_follow_layered_boundaries_locally`。
- [x] 组件目录标准文件落点正确。
  - `<component>/mod.rs`：最小稳定导出面，存在且无过度导出。
  - `<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。
  - `<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。
  - `<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。
  - `<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。
  - `<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。
  - 通过依据（checkbox）：`components/checkbox/src/` 目录具备 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs` 标准文件，且 `render.rs/spec.rs` 不存在，保持“交互组件必须有 motion，简单组件无 spec”落点约束。
  - 通过依据（checkbox）：`mod.rs` 仅保留最小稳定导出面（`pub use logic::{CheckboxSize, CheckboxVariant}; pub use motion::CheckboxMotion; pub use view::Checkbox;`），无 `pub mod logic/motion/view` 过度导出。
  - 通过依据（checkbox）：`logic.rs` 聚焦 props 归一与派生（`resolve_checked_control/derive_render_state`），`styles.rs` 聚焦 token-first 静态 CSS，`view.rs` 聚焦 Leptos 结构 + headless 挂载，`motion.rs` 聚焦 `CheckboxMotion + attach_*_motion` 映射；职责边界由语义回归持续约束。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_component_directory_standard_files_follow_contract_and_na_paths_locally`，锁定目录落点、职责边界、`spec.rs` N/A 与 `render.rs` 禁止漂移。
  - 通过依据（checkbox）：`scripts/check-ui-components-component-files.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_component_directory_standard_files_follow_contract_and_na_paths_locally`，纳入组件文件职责阻断链路。

### 6. AI 原生能力与文件落点（Struct-First & Projection）
- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。
  - 通过依据（checkbox）：`components/checkbox/src/` 仅包含 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs` 这五个标准职责文件；`render.rs` 不存在，避免渲染实现漂移。
  - 通过依据（checkbox）：`mod.rs` 维持“导出边界”角色（`mod logic; mod motion; pub mod styles; mod view;` + 最小 `pub use`），`logic.rs/styles.rs/view.rs/motion.rs` 各自保留归一派生、Token 样式、渲染挂载、动效映射单一职责。
  - 通过依据（checkbox）：`spec.rs` 维持 N/A（checkbox 不是复杂 schema/builders 组件），当前目录无 `spec.rs` 文件且无 `mod spec` 漂移。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_file_placement_discipline_contract_is_explicit_for_interactive_component_scope_locally`，锁定文件存在性/缺失性、职责锚点与 check2 契约标记。
  - 通过依据（checkbox）：`scripts/check-ui-components-component-files.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_file_placement_discipline_contract_is_explicit_for_interactive_component_scope_locally`，纳入组件文件落点阻断链路。
- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。
  - N/A 理由（checkbox）：该组件是基础交互控件，不属于“复杂 schema/builders”组件范围；当前不引入 `spec.rs` 与 `*Spec::new()...render()` 链路。
  - 通过依据（checkbox）：`components/checkbox/src/` 下无 `spec.rs`，且 `mod.rs/logic.rs/view.rs/styles.rs/motion.rs` 无 `mod spec`、`pub use spec::`、`Spec::new(`、`.render()` 等 Hyper-Structure Builder 暴露痕迹。
  - 通过依据（checkbox）：`components/checkbox/src/README.md`、`components/checkbox/src/Component.toml`、`components/checkbox/src/checkbox.rbi` 均未引入 `CheckboxSpec`/schema builder 协议，保持“简单组件不强行 builder 化”。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component_locally`，锁定本项 N/A 契约与禁止 token。
  - 通过依据（checkbox）：`scripts/check-ui-components-component-files.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component_locally`，纳入组件文件契约阻断链路。
- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。
  - 通过依据（checkbox）：`components/checkbox/src/` 同步维护 `Component.toml` 与 `checkbox.rbi`，并由目录回归测试锁定文件存在性，防止“实现改了但投影未更新”。
  - 通过依据（checkbox）：`Component.toml` 维持可追踪能力清单（`schema_version`、输入轴、`[[outputs]] semantic-markers`、`[[capabilities]] context_compression_manifest/rbi_signature_projection`），确保 Agent 可读能力基线稳定。
  - 通过依据（checkbox）：`checkbox.rbi` 维持签名投影（`CheckboxVariant/CheckboxSize/CheckboxMotion` 与 `Checkbox(...) -> impl IntoView`），覆盖关键受控轴与 A11y 上下文参数（`is_checked/on_checked_change/default_checked/dir`）。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_context_compression_manifest_and_rbi_are_present_and_consistent_locally`，锁定 Manifest + RBI 的存在性与关键字段一致性。
  - 通过依据（checkbox）：`scripts/check-ui-components-component-files.sh` 新增门禁命令 `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_context_compression_manifest_and_rbi_are_present_and_consistent_locally`，纳入组件文件契约阻断链路。
- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。
  - 关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。
  - Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。
  - 契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。
  - 配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。
  - 通过依据（checkbox）：`components/checkbox/src/logic.rs` 新增类型化 Agent Contract 模型（`CHECKBOX_AGENT_SCHEMA`、`CheckboxAgentSchemaVersion/Intent/Action/State/Source`、`CheckboxAgentContract{Input}`、`resolve_agent_contract`），契约字段由类型与状态映射统一生成。
  - 通过依据（checkbox）：`components/checkbox/src/view.rs` 在根节点统一挂载 `data-ui-schema/version/intent/action/state/source` 与 `data-ui-*-source`，契约可追溯到 state/checked/handler/motion 来源，不依赖 DOM 猜测。
  - 通过依据（checkbox）：`components/checkbox/src/Component.toml` 补齐 `[[outputs]] agent-contract-markers`、`[[capabilities]] agent_contract_schema_markers`、`[[agent_contract]]`、`[[agent_contract_markers]]` 与 `[[agent_contract_whitelist]]`，明确 schema 轴与白名单能力边界。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_agent_contract_is_schema_typed_and_machine_readable_locally` 与 `components/checkbox/test/semantics.rs::checkbox_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally`，覆盖类型化字段、禁止字符串拼接与脚本注入路径。
  - 通过依据（checkbox）：`scripts/check-ui-components-contract-hygiene.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_agent_contract_is_schema_typed_and_machine_readable_locally`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_agent_contract_render_path_is_whitelist_safe_and_script_injection_free_locally`
- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。
  - `Streaming`：LLM 还在生成，界面边生成边显示。
  - `Snapshot`：LLM 全部生成完成后，一次性显示。
  - 通过依据（checkbox）：术语范围在组件清单中固定为“仅 LLM 输出渲染两种显示模式”，避免把普通交互状态误标为 streaming/snapshot 渲染协议。
  - 通过依据（checkbox）：checkbox 不是 LLM 正文阅读面，本组件保持 snapshot-only 渲染路径；仅暴露治理型 `data-ui-stream-support/fallback/output-status` 标记，不引入 token 增量传输协议。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`，同时断言 `view.rs/logic.rs/docs-app forms 页` 不出现 `data-ui-stream-mode` / `data-ui-output-state` / `use_ai_space_state` 以及 `stream_chunk/token_delta` 等增量协议标记。
  - 通过依据（checkbox）：`scripts/check-ui-components-streaming.sh` 新增门禁命令
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`。
- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。
  - 所有组件都应能消费“完整生成结果”并稳定渲染。
  - 即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。
  - 通过依据（checkbox）：`components/checkbox/src/view.rs` 持续以完整 props + children 渲染稳定结构（`Checkbox(...) -> IntoView`、`children()`、`data-state`/`data-state-source`），不依赖流式增量协议即可完成可用渲染。
  - 通过依据（checkbox）：`components/checkbox/src/logic.rs` 保持输入归一与状态派生主路径（`resolve_checked_control`、`derive_render_state`、`compose_class_name`），完整输入配置在 snapshot 路径下可稳定收敛。
  - 通过依据（checkbox）：`components/checkbox/src/Component.toml` 新增 `[[capabilities]] name = "snapshot_rendering" enabled = true`，并维持关键输入轴声明（`is_checked/default_checked/variant/size/motion`）。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_check2_documents_snapshot_as_default_baseline_capability` 与 `components/checkbox/test/semantics.rs::checkbox_snapshot_baseline_consumes_complete_result_and_renders_stably`，锁定清单约束与“完整结果稳定渲染”能力。
  - 通过依据（checkbox）：`scripts/check-ui-components-streaming.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_snapshot_as_default_baseline_capability`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_snapshot_baseline_consumes_complete_result_and_renders_stably`。
- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。
  - `Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。
  - `Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。
  - 无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。
  - 数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。
  - 通过依据（checkbox）：checkbox 不是正文阅读面，归类为 `Streaming Optional`；组件渲染以 `Snapshot` 为基线，固定 `fallback=snapshot`，不承担 token 流拼装职责。
  - 通过依据（checkbox）：`components/checkbox/src/logic.rs` 通过类型化契约输出 `CheckboxAgentStreamSupport::Optional`、`CheckboxAgentStreamFallback::Snapshot`、`CheckboxAgentOutputStatus::Verified`，避免字符串散落与语义漂移。
  - 通过依据（checkbox）：`components/checkbox/src/view.rs` 挂载 `data-ui-stream-support`、`data-ui-stream-fallback`、`data-ui-output-status`，并持续挂载 `role/aria-* / data-*`（`role`、`aria-disabled`、`aria-checked`、`data-state`）保证状态连续可读。
  - 通过依据（checkbox）：`components/checkbox/src/Component.toml` 在 `[[agent_contract_markers]]` 中封闭约束 `stream_support=optional`、`stream_fallback=snapshot`、`output_status=verified`。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_check2_marks_streaming_scope_as_optional_with_snapshot_fallback`，锁定分类、fallback、输出状态标记与 role/aria/data 连续性。
  - 通过依据（checkbox）：`scripts/check-ui-components-streaming.sh` 新增门禁命令
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_marks_streaming_scope_as_optional_with_snapshot_fallback`。

### 7. 测试、门禁与交付
- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。
  - 通过依据（checkbox）：`components/checkbox/src/mod.rs`、`components/checkbox/src/logic.rs`、`components/checkbox/src/view.rs`、`components/checkbox/src/styles.rs`、`components/checkbox/src/motion.rs` 非测试实现路径不含 `unwrap(` / `expect(` / `unwrap_err(` / `let _ =`，由语义回归持续锁定。
  - 通过依据（checkbox）：`components/checkbox/src/logic.rs` 将 class-name 组装热点收敛为 `Vec<Cow<'static, str>>`（`use std::borrow::Cow;` + `Cow::Borrowed/Cow::Owned`），避免 `trim().to_string()` 与常量字符串重复复制。
  - 通过依据（checkbox）：新增语义回归
    `components/checkbox/test/semantics.rs::checkbox_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources`
    `components/checkbox/test/semantics.rs::checkbox_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent`
    `components/checkbox/test/semantics.rs::checkbox_rust_hygiene_script_enforces_repo_level_hygiene_guards`
    `components/checkbox/test/semantics.rs::checkbox_check2_marks_rust_hygiene_contract_complete`
  - 通过依据（checkbox）：`scripts/check-ui-components-engineering.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_rust_hygiene_script_enforces_repo_level_hygiene_guards`
  - 通过依据（checkbox）：仓库级脚本 `./scripts/check-rust-hygiene.sh` 持续作为统一卫生门禁（禁止 `unwrap/expect`、`let _ =` 与字符串复制热点漂移）。
- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui-components` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。
  - 通过依据（checkbox）：`crates/ui-components/Cargo.toml` 已注册 `component-checkbox = ["dep:ui-checkbox"]`，`crates/ui-components/src/lib.rs` 与 `crates/ui-components/src/css.rs` 对 checkbox 导出与样式聚合均受 `#[cfg(feature = "component-checkbox")]` 门控，无无条件全局依赖。
  - 通过依据（checkbox）：`apps/web-demo/Cargo.toml` 以 `default-features = false` + `web-demo-components` 反向依赖 `ui-components`，未拉起 `all-components`；`apps/docs-app/Cargo.toml` 显式使用 `all-components` 作为全量验收面，边界清晰。
  - 通过依据（checkbox）：`components/checkbox/test/semantics.rs::checkbox_tree_shaking_contract_is_feature_gated_and_ci_enforced` 与 `components/checkbox/test/semantics.rs::checkbox_check2_marks_tree_shaking_feature_pruning_contract_complete` 锁定 feature 注册、`lib.rs/css.rs` 门控、CI tree-shaking 脚本与清单证据。
  - 通过依据（checkbox）：`scripts/check-ui-components-tree-shaking.sh` 新增 checkbox 门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_tree_shaking_contract_is_feature_gated_and_ci_enforced`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_marks_tree_shaking_feature_pruning_contract_complete`
  - 通过依据（checkbox）：脚本新增 checkbox 最小特性树与 wasm compile-only 校验：
    `cargo tree -e features -i ui-components -p ui-components --no-default-features --features "$CHECKBOX_MIN_FEATURES"`
    `cargo check -p ui-components --target wasm32-unknown-unknown --no-default-features --features "$CHECKBOX_MIN_FEATURES"`，并阻断 `all-components` 漏拉起。
- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。
  - 通过依据（checkbox）：`components/checkbox/test/semantics.rs::checkbox_semantic_contract_matrix_covers_interaction_paths_without_snapshot_substitution` 已覆盖 `role/aria-*`、`data-*` 与键盘/指针焦点路径，并显式阻断 `assert_snapshot!/insta` 作为主验证手段。
  - 通过依据（checkbox）：`components/checkbox/test/semantics.rs::checkbox_performance_governance_budget_is_defined_and_blocking_locally` 已覆盖性能预算、可观测标记、阻断脚本与 `render_count` 跟踪计划（`docs/plan/TODO.md`）的等价证据链。
  - 通过依据（checkbox）：新增综合回归 `components/checkbox/test/semantics.rs::checkbox_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement`，统一校验 `aria/data/focus` 标记、非快照断言约束、`render_count` 跟踪与门禁脚本挂载一致性。
  - 通过依据（checkbox）：`scripts/check-ui-components-performance.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement`。
- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。
  - N/A：本次 `Checkbox` 改动未引入跨大版本 API 破坏升级，组件 Agent Contract 仍保持 `v1`（`components/checkbox/src/logic.rs` 的 `CheckboxAgentSchemaVersion::V1`，`components/checkbox/src/Component.toml` 的 `schema_version = "1"` 与 `ui.checkbox.agent-contract.v1`）。
  - 通过依据（checkbox）：当前实现与契约未引入 `migrate_v1_to_v2`、Schema Registry 弃用窗口或 `v2` 协议标记；因此不触发 Codemod/Registry 迁移层要求。
  - 通过依据（checkbox）：新增回归 `components/checkbox/test/semantics.rs::checkbox_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade_locally`，锁定 `v1` 标记、禁止迁移层误引入与清单证据一致性。
  - 通过依据（checkbox）：`scripts/check-ui-components-engineering.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_version_deprecation_migration_registry_is_explicitly_na_without_major_breaking_upgrade_locally`。
- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。
  - 通过依据（checkbox）：`apps/docs-app/src/pages/components/pages/forms.rs::checkbox()` 已覆盖 `Hello World`、`Variant + Disabled matrix` 与 `Controlled vs Uncontrolled (Comparison)` Playground，完成最小路径、状态矩阵、受控/非受控对照。
  - 通过依据（checkbox）：docs 页面新增 `data-slot="checkbox-streaming-policy"` 与 `data-slot="checkbox-streaming-modes"`，显式标注 `Streaming Optional; fallback=snapshot.` 与 snapshot 展现语义。
  - 通过依据（checkbox）：docs 页面新增 `data-slot="checkbox-copy-ready"`、`data-slot="checkbox-source-paths"`、`data-slot="checkbox-source-prerequisites"`，并通过 `Playground` 的 `code_imports` 与 `compose_copy_ready_code` 提供 import-ready 一键复制路径。
  - 通过依据（checkbox）：新增语义回归 `components/checkbox/test/semantics.rs::checkbox_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot`，锁定 docs 结构、copy-ready imports、streaming/snapshot 文案与门禁脚本挂载。
  - 通过依据（checkbox）：新增 E2E 回归 `e2e/tests/docs_app_checkbox_contract.spec.mjs::docs-app checkbox playground source is copy-paste ready`，覆盖 code panel copy-ready 与 streaming policy 展示。
  - 通过依据（checkbox）：`scripts/check-ui-components-dx.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot`。
  - 通过依据（checkbox）：执行入口为 `bash scripts/check-ui-components-dx.sh`（当前环境若出现 `Invalid cross-device link (os error 18)`，阻断点在编译产物写入阶段而非该契约本身）。
- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。
  - 每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。
  - 断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。
  - 新增/变更语义字段必须同步补测试，否则不得打勾。
  - 通过依据（checkbox）：交互组件语义回归位于 `components/checkbox/test/semantics.rs`，并已覆盖关键状态轴、状态来源、键盘/指针路径与 A11y 语义挂载。
  - 通过依据（checkbox）：新增 `components/checkbox/test/semantics.rs::checkbox_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks`，锁定 `data-*` / `aria-*` / `role` / source markers 为主断言并禁止 snapshot-only 断言替代。
  - 通过依据（checkbox）：`scripts/check-ui-components-performance.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks`。
- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。
  - E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。
  - WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。
  - 若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。
  - 通过依据（checkbox）：`e2e/tests/docs_app_checkbox_contract.spec.mjs` 改为语义锚点选择器（如 `data-slot="checkbox-e2e-controlled-surface"`、`data-slot="checkbox-e2e-disabled-on"`、`[data-slot="checkbox"][role="checkbox"]`），并统一通过 `body:not(:has(#boot))` + `waitForWasmReady` 作为 WASM 稳定就绪等待；禁用 `hasText`/`getByText`/`locator("text=")`/`waitForTimeout`。
  - 通过依据（checkbox）：`apps/docs-app/src/pages/components/pages/forms.rs` 为 checkbox docs 场景补齐稳定锚点：`checkbox-e2e-interactive-surface`、`checkbox-e2e-controlled-surface`、`checkbox-e2e-controlled-target`、`checkbox-e2e-controlled-last-change`、`checkbox-e2e-matrix-surface`、`checkbox-e2e-disabled-on`、`checkbox-e2e-disabled-off`，避免依赖文本和脆弱层级定位。
  - 通过依据（checkbox）：新增回归 `components/checkbox/test/semantics.rs::checkbox_check2_documents_e2e_selector_and_stable_wait_rules`、`components/checkbox/test/semantics.rs::checkbox_e2e_selector_contract_uses_semantic_markers_and_settled_waits`、`components/checkbox/test/semantics.rs::checkbox_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_paths`、`components/checkbox/test/semantics.rs::checkbox_e2e_check_script_covers_selector_and_settled_wait_contract`，并同步镜像到 `components/checkbox/test/checkbox_semantics.rs::checkbox_check2_documents_e2e_selector_and_stable_wait_rules`、`components/checkbox/test/checkbox_semantics.rs::checkbox_e2e_selector_contract_uses_semantic_markers_and_settled_waits`、`components/checkbox/test/checkbox_semantics.rs::checkbox_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_paths`、`components/checkbox/test/checkbox_semantics.rs::checkbox_e2e_check_script_covers_selector_and_settled_wait_contract`。
  - 通过依据（checkbox）：新增门禁脚本 `scripts/check-ui-components-e2e-checkbox.sh`，执行：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_e2e_selector_and_stable_wait_rules`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_selector_contract_uses_semantic_markers_and_settled_waits`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_contract_covers_ready_and_settled_conditions_for_checkbox_paths`
  - 通过依据（checkbox）：当前环境执行上述 cargo 命令时在编译产物写入阶段触发 `Invalid cross-device link (os error 18)`，阻断点不在该语义契约实现本身。
- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。
  - 至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。
  - 回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。
  - 高风险路径（overlay、focus、keyboard、async）优先进入回归集合。
  - 通过依据（checkbox）：`e2e/tests/docs_app_checkbox_contract.spec.mjs` 新增可重复关键流程 `docs-app checkbox key flow is repeatable and failures map to semantic breakpoints`，固定路由后通过语义锚点执行 `focus -> Space`，断言 `data-state-source/data-checked-source/data-handler-source/data-ui-state`，随后 `page.reload()` 重放同路径并复验同一语义断点。
  - 通过依据（checkbox）：同一 E2E 文件新增高风险路径 `docs-app checkbox high-risk paths keep focus keyboard and disabled branches semantically explicit`，覆盖 focus + keyboard 主路径与 disabled 分支（`aria-disabled` + `toBeDisabled` + disabled 后状态不变）；`overlay/async` 在 checkbox 组件职责下为 N/A。
  - 通过依据（checkbox）：新增回归 `components/checkbox/test/semantics.rs::checkbox_check2_documents_repeatable_e2e_regression_collection`、`components/checkbox/test/semantics.rs::checkbox_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`、`components/checkbox/test/semantics.rs::checkbox_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints`、`components/checkbox/test/semantics.rs::checkbox_e2e_check_script_covers_selector_and_key_flow_contracts`，并同步镜像到 `components/checkbox/test/checkbox_semantics.rs::checkbox_check2_documents_repeatable_e2e_regression_collection`、`components/checkbox/test/checkbox_semantics.rs::checkbox_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`、`components/checkbox/test/checkbox_semantics.rs::checkbox_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints`、`components/checkbox/test/checkbox_semantics.rs::checkbox_e2e_check_script_covers_selector_and_key_flow_contracts`。
  - 通过依据（checkbox）：`scripts/check-ui-components-e2e-checkbox.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_repeatable_e2e_regression_collection`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints`
  - 通过依据（checkbox）：当前环境执行上述 cargo 命令时在编译产物写入阶段触发 `Invalid cross-device link (os error 18)`，阻断点不在该契约实现本身。
- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。
  - 组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。
  - 文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。
  - 文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。
  - 通过依据（checkbox）：`apps/docs-app/src/pages/components/pages/forms.rs::checkbox` 已覆盖 `Hello World`、`Variant + Disabled matrix`、`Controlled vs Uncontrolled (Comparison)`，并在同页保留 interactive controls（variant/size）与 `default_checked` 非受控示例。
  - 通过依据（checkbox）：示例 API 与默认语义对齐 `components/checkbox/src/logic.rs` + `components/checkbox/src/view.rs` + `components/checkbox/src/Component.toml` + `components/checkbox/src/checkbox.rbi`：主命名 `is_checked/on_checked_change/default_checked/is_disabled`，默认值契约 `default_checked=None`、`is_disabled=None`、`disabled=false`。
  - 通过依据（checkbox）：新增回归 `components/checkbox/test/semantics.rs::checkbox_check2_documents_docs_sync_and_state_matrix_rules`、`components/checkbox/test/semantics.rs::checkbox_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`、`components/checkbox/test/semantics.rs::checkbox_dx_check_script_covers_docs_sync_and_state_matrix_contract`、`components/checkbox/test/semantics.rs::checkbox_check2_marks_docs_sync_and_state_matrix_item_complete`，并同步镜像到 `components/checkbox/test/checkbox_semantics.rs::checkbox_check2_documents_docs_sync_and_state_matrix_rules`、`components/checkbox/test/checkbox_semantics.rs::checkbox_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`、`components/checkbox/test/checkbox_semantics.rs::checkbox_dx_check_script_covers_docs_sync_and_state_matrix_contract`、`components/checkbox/test/checkbox_semantics.rs::checkbox_check2_marks_docs_sync_and_state_matrix_item_complete`。
  - 通过依据（checkbox）：`scripts/check-ui-components-dx.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_docs_sync_and_state_matrix_rules`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`
  - 通过依据（checkbox）：当前环境执行上述 cargo 命令时在编译产物写入阶段触发 `Invalid cross-device link (os error 18)`，阻断点不在该契约实现本身。
- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。
  - 每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。
  - 文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。
  - “只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。
  - 通过依据（checkbox）：文档入口存在且可索引：`components/checkbox/src/README.md` + `apps/docs-app/src/pages/components/pages.rs` 的 `component_doc!("Checkbox", "checkbox", "Forms", forms::checkbox)` + `apps/docs-app/src/pages/components/pages/forms.rs::checkbox`。
  - 通过依据（checkbox）：README 已补齐新手路径结构 `## Hello World（最小可用）` → `## 常见用法` → `## 先用起来，再进阶` → `### Controlled（高级入口）`，默认路径先于进阶参数。
  - 通过依据（checkbox）：新增回归 `components/checkbox/test/semantics.rs::checkbox_check2_documents_documentation_as_product_rules`、`components/checkbox/test/semantics.rs::checkbox_documentation_entry_exists_with_beginner_first_progression`、`components/checkbox/test/semantics.rs::checkbox_dx_check_script_covers_documentation_as_product_contract`、`components/checkbox/test/semantics.rs::checkbox_check2_marks_documentation_as_product_item_complete`，并同步镜像到 `components/checkbox/test/checkbox_semantics.rs::checkbox_check2_documents_documentation_as_product_rules`、`components/checkbox/test/checkbox_semantics.rs::checkbox_documentation_entry_exists_with_beginner_first_progression`、`components/checkbox/test/checkbox_semantics.rs::checkbox_dx_check_script_covers_documentation_as_product_contract`、`components/checkbox/test/checkbox_semantics.rs::checkbox_check2_marks_documentation_as_product_item_complete`。
  - 通过依据（checkbox）：`scripts/check-ui-components-dx.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_documentation_as_product_rules`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_documentation_entry_exists_with_beginner_first_progression`
  - 通过依据（checkbox）：当前环境执行上述 cargo 命令时在编译产物写入阶段触发 `Invalid cross-device link (os error 18)`，阻断点不在该契约实现本身。
- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。
  - Playground 至少支持基础 props 调整、状态切换、交互反馈观察。
  - 对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。
  - Playground 作为验收面，需可重复复现关键交互路径。
  - 通过依据（checkbox）：`apps/docs-app/src/pages/components/pages/forms.rs::checkbox` 已提供 `title="Interactive Playground"`，包含 `controls`（variant/size/checked/disabled/custom class）+ 实时预览区（`is_checked/on_checked_change`、`variant/size/is_disabled`）+ `test_config_signal` 配置回显。
  - 通过依据（checkbox）：可重复验收路径复用 `e2e/tests/docs_app_checkbox_contract.spec.mjs::docs-app checkbox key flow is repeatable and failures map to semantic breakpoints`（`focus -> Space -> semantic asserts -> reload -> replay`），满足 Playground 验收面可重放要求。
  - 通过依据（checkbox）：AI Spec 条款对该组件按 N/A 处理（checkbox 非 spec 组件）：docs 与组件源码未引入 `Spec::new(...)` / `mod spec;`。
  - 通过依据（checkbox）：新增回归 `components/checkbox/test/semantics.rs::checkbox_check2_documents_interactive_playground_rules`、`components/checkbox/test/semantics.rs::checkbox_docs_app_provides_interactive_playground_for_props_state_and_preview`、`components/checkbox/test/semantics.rs::checkbox_interactive_playground_reuses_repeatable_semantic_e2e_flow`、`components/checkbox/test/semantics.rs::checkbox_dx_check_script_covers_interactive_playground_contract`、`components/checkbox/test/semantics.rs::checkbox_check2_marks_interactive_playground_item_complete`，并同步镜像到 `components/checkbox/test/checkbox_semantics.rs::checkbox_check2_documents_interactive_playground_rules`、`components/checkbox/test/checkbox_semantics.rs::checkbox_docs_app_provides_interactive_playground_for_props_state_and_preview`、`components/checkbox/test/checkbox_semantics.rs::checkbox_interactive_playground_reuses_repeatable_semantic_e2e_flow`、`components/checkbox/test/checkbox_semantics.rs::checkbox_dx_check_script_covers_interactive_playground_contract`、`components/checkbox/test/checkbox_semantics.rs::checkbox_check2_marks_interactive_playground_item_complete`。
  - 通过依据（checkbox）：`scripts/check-ui-components-dx.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_interactive_playground_rules`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_docs_app_provides_interactive_playground_for_props_state_and_preview`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_interactive_playground_reuses_repeatable_semantic_e2e_flow`
  - 通过依据（checkbox）：当前环境执行上述 cargo 命令时在编译产物写入阶段触发 `Invalid cross-device link (os error 18)`，阻断点不在该契约实现本身。
- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。
  - docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。
  - 若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。
  - 文档代码与当前实现必须同步，防止示例漂移。
  - 通过依据（checkbox）：`apps/docs-app/src/pages/components/pages/forms.rs::checkbox` 已提供 `data-slot="checkbox-source-first"`、`data-slot="checkbox-copy-ready"`、`data-slot="checkbox-source-paths"`、`data-slot="checkbox-source-prerequisites"`，并列出真实源码落点（`components/checkbox/src/view.rs`、`components/checkbox/src/logic.rs`、`components/checkbox/src/styles.rs`、`apps/docs-app/src/pages/components/pages/forms.rs`）与依赖前提（`component-checkbox`、`inject-css`）。
  - 通过依据（checkbox）：复制链路由 `apps/docs-app/src/playground.rs::compose_copy_ready_code` + `DEFAULT_PLAYGROUND_IMPORTS` 提供 import-ready 输出，E2E `e2e/tests/docs_app_checkbox_contract.spec.mjs::docs-app checkbox playground source is copy-paste ready` 断言 code panel `data-copyable` 与 `use leptos::prelude::*; use ui_components::*;`。
  - 通过依据（checkbox）：新增回归 `components/checkbox/test/semantics.rs::checkbox_check2_documents_source_first_copy_paste_ready_rules`、`components/checkbox/test/semantics.rs::checkbox_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`、`components/checkbox/test/semantics.rs::checkbox_dx_check_script_covers_source_first_copy_paste_ready_contract`、`components/checkbox/test/semantics.rs::checkbox_check2_marks_source_first_copy_paste_ready_contract_complete`，并同步镜像到 `components/checkbox/test/checkbox_semantics.rs::checkbox_check2_documents_source_first_copy_paste_ready_rules`、`components/checkbox/test/checkbox_semantics.rs::checkbox_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`、`components/checkbox/test/checkbox_semantics.rs::checkbox_dx_check_script_covers_source_first_copy_paste_ready_contract`、`components/checkbox/test/checkbox_semantics.rs::checkbox_check2_marks_source_first_copy_paste_ready_contract_complete`。
  - 通过依据（checkbox）：`scripts/check-ui-components-dx.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_source_first_copy_paste_ready_rules`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`
  - 通过依据（checkbox）：当前环境执行上述 cargo 命令时在编译产物写入阶段触发 `Invalid cross-device link (os error 18)`，阻断点不在该契约实现本身。
- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。
  - 若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。
  - 组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。
  - “仅代码更新无文档更新”在接口变更场景下直接判不通过。
  - 通过依据（checkbox）：`docs/spec/heroui-parameter-design-strategy.md` 新增 `### Checkbox 同步记录（2026-02-20）`，明确参数主轴 `is_checked/default_checked/on_checked_change` 与 `is_disabled`（兼容 `disabled`）以及 docs/README 同步约束，并标注本轮不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
  - 通过依据（checkbox）：组件文档入口可索引：`apps/docs-app/src/pages/components/pages.rs` 保持 `component_doc!("Checkbox", "checkbox", "Forms", forms::checkbox)`，`apps/docs-app/src/pages/components/pages/forms.rs::checkbox` 保持 `title="Checkbox"` 与 `slug="checkbox"`，`components/checkbox/src/README.md` 提供等价入口。
  - 通过依据（checkbox）：新增回归 `components/checkbox/test/semantics.rs::checkbox_check2_documents_heroui_benchmark_docs_sync_rules`、`components/checkbox/test/semantics.rs::checkbox_heroui_strategy_and_component_docs_are_synchronized_and_indexable`、`components/checkbox/test/semantics.rs::checkbox_dx_check_script_covers_heroui_benchmark_docs_sync_contract`、`components/checkbox/test/semantics.rs::checkbox_check2_marks_heroui_benchmark_docs_sync_contract_complete`，并同步镜像到 `components/checkbox/test/checkbox_semantics.rs::checkbox_check2_documents_heroui_benchmark_docs_sync_rules`、`components/checkbox/test/checkbox_semantics.rs::checkbox_heroui_strategy_and_component_docs_are_synchronized_and_indexable`、`components/checkbox/test/checkbox_semantics.rs::checkbox_dx_check_script_covers_heroui_benchmark_docs_sync_contract`、`components/checkbox/test/checkbox_semantics.rs::checkbox_check2_marks_heroui_benchmark_docs_sync_contract_complete`。
  - 通过依据（checkbox）：`scripts/check-ui-components-dx.sh` 新增门禁命令：
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_check2_documents_heroui_benchmark_docs_sync_rules`
    `cargo test -p ui-components --test checkbox_semantics --no-default-features --features component-checkbox,inject-css checkbox_heroui_strategy_and_component_docs_are_synchronized_and_indexable`
  - 通过依据（checkbox）：当前环境执行上述 cargo 命令时在编译产物写入阶段触发 `Invalid cross-device link (os error 18)`，阻断点不在该契约实现本身。

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
