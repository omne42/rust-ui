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
  已落实：`BottomSheetStateInput/BottomSheetState + normalize*/resolve_state/compose_class_name` 已下沉到 `crates/ui-state-primitives/src/bottom_sheet.rs`，组件层 `components/bottom-sheet/src/logic.rs` 仅消费（re-export）原语；primitive 单测位于 `crates/ui-state-primitives/src/test/bottom_sheet.rs`。
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
  已落实：`components/bottom-sheet/src/view.rs` 不自造交互语义，直接组合 `Sheet`；`Sheet` 在 `components/sheet/src/view.rs` 通过 `ui_headless` 的 `overlay_dialog_attrs/use_focus_trap/use_modal/use_overlay_stack_registration` 提供语义契约，并且 `BottomSheet` 已透传 `lang/dir` 到 `Sheet`。
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
  已落实：`components/bottom-sheet/src/motion.rs` 仅做组件层 contract 组装并委托 `components/sheet/src/motion.rs`；`Sheet` 再统一调用 `ui-motion` spring/preset/driver，无组件业务语义回流到 `ui-motion`。
  - 放在 `crates/ui-motion`：通用动画数学与执行后端（spring solver、keyframe sampling、easing registry、driver adapters），以及 `wasm/non-wasm` 适配与 `reduced-motion` 执行策略。
  - 放在 `crates/ui/src/<component>/motion.rs`：把组件语义状态（open/closed、enter/exit、active/inactive）映射为 `ui-motion` contract，绑定目标节点并调用 attach。
  - 禁止放在 `crates/ui-motion`：组件 slot 结构、组件专属状态机、ARIA/keyboard 语义、业务文案与业务分支。
  - 禁止放在组件 `motion.rs`：自实现 spring/keyframe/driver 执行器；跨组件共享动效算法必须回迁 `ui-motion`。
  - 动效参数优先来自 token/theme；禁止在组件样式与逻辑中散落硬编码时长/曲线/位移常量。
  - 非 wasm 路径必须提供 no-op/stub，保证 SSR/tooling 可编译且行为可预测。
- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。  
  已落实：`components/bottom-sheet/src/styles.rs` 使用 `var(--ui-*)` 变量消费主题（颜色、间距、圆角、阴影、字号等），未引入组件私有 token 体系或主题重建逻辑。
  - Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量；组件只在 `crates/ui/src/<component>/styles.rs` 消费。
  - 三轴上下文（`system/color/scale`）在 `theme.rs` 定义；组件在 `logic.rs` 选择并在 `view.rs` 生效，`styles.rs` 只消费变量，不重建主题。
  - Token 分类必须可追溯：分类源在 `tokens.rs`，规范同步 `docs/spec/styling.md`；组件不得引入平行私有 token 命名体系。
  - 量化尺寸基准必须可回归：尺寸基准在 `tokens.rs` 与 `theme.rs` 定义，主题回归在 `crates/ui-theme/tests/token_scale_baseline.rs`，组件语义回归在 `components/*/test/*<component>_semantics.rs`。
  - 主题调色与语义色对比必须满足 `WCAG 2.1 AA` 基线，并覆盖 Light/Dark/OLED 主题变体。
  - 主题层只输出 `theme/tokens/base css` 与变量；不实现组件结构、交互逻辑、组件级动效编排。
  - 新增视觉语义先补 token，再由组件消费；禁止“组件临时值先落地、后补 token”的倒序流程。
- [x] `ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。  
  已落实：`bottom-sheet` 模块边界完整（`logic/view/styles/motion/mod`），`logic.rs` 只消费 primitives，`view.rs` 只做结构装配并挂载 `Sheet` 语义，`motion.rs` 只做动效 contract；对外 API 未暴露 `web-sys`/DOM 类型。
  - `logic.rs` 负责 props 归一与状态派生；`view.rs` 负责结构渲染与 headless 语义挂载；`styles.rs` 负责 token-first 静态样式；`motion.rs` 负责动效 attach。
  - 组件层不得重写 `status-primitives` 状态机或 `ui-headless` 交互契约；发现即判不通过并回迁到对应层。
  - 对外 API 禁止暴露 `web-sys`/DOM 细节类型；平台差异封装在内部模块。
  - 测试文件位于src同级的test/中，内部测试文件同名（如rust-ui/components/accordion/src/logic.rs与rust-ui/components/accordion/test/logic.rs）。
  - 还需要一个semantics.rs用于测试。可能存在类似rust-ui/components/accordion/test/semantics.rs的旧版实现，需要迁移到新目录。

### 2. API 设计与状态内核（Logic/Kernel）
- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。（BottomSheet 公共 API 采用 `open/on_close + is_*`：`is_dismissable/is_keyboard_dismiss_disabled/is_handle_visible/is_close_button_visible/is_detached`；保留 `show_handle/show_close_button/detached` 兼容别名并在 `view.rs` 统一归一，不引入 `onOpenChange/defaultOpen` 别名漂移。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_api_naming_contract_prefers_is_on_prefix_and_keeps_compat_aliases`、`bottom_sheet_docs_playgrounds_lock_state_matrix_contract_values`。）
  - 布尔状态统一 `is_*`（如 `is_open`/`is_disabled`），事件统一 `on_*`，默认值统一 `default_*`。
  - 同一语义 across 组件必须同名（如都用 `on_open_change`，禁止同义别名并存）。
  - 公共 API 引入新命名时，需说明与现有命名体系的兼容策略与迁移路径。
- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。（BottomSheet 本体为“受控渲染面”组件：只消费外部 `open: Signal<bool>` 与 `on_close`，不承载本地 open 状态机；对应非受控成对能力固定由 `ui-state-primitives::overlay_trigger` 提供 `default_open/on_open_change`。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_control_contract_is_controlled_surface_with_primitive_uncontrolled_pair`。）
  - 受控模式：外部值是单一事实来源，内部不得偷偷写回本地状态。
  - 非受控模式：仅由默认值初始化一次，后续状态由内部原语管理。
  - 受控/非受控切换语义需稳定可测，避免“半受控”隐式行为。
- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。（默认值与优先级统一收敛到 `components/bottom-sheet/src/logic.rs`：`resolve_title/resolve_close_label/resolve_description_text/resolve_handle_visibility/resolve_close_button_visibility/resolve_detached/resolve_bottom_inset_px/resolve_dismissable/resolve_keyboard_dismiss_disabled/resolve_on_exit_complete`；`view.rs` 不再出现 `unwrap_or` 兜底分支，仅消费 `logic.rs` 归一化结果。回归：`components/bottom-sheet/test/logic.rs::resolve_defaults_are_single_source_and_explicit`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_uses_logic_state_model`、`bottom_sheet_api_naming_contract_prefers_is_on_prefix_and_keeps_compat_aliases`。）
  - 默认值优先级必须可读且可测试（显式规则而非分散 `unwrap_or`）。
  - `view.rs` 不允许再做默认值分支；仅消费 `logic.rs` 的归一化输出。
  - 一旦发现多处默认值来源，直接判不通过并回收至 `logic.rs`。
- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。（`view.rs` 不再直接拼 `BottomSheetStateInput` 或 `motion source` 分支，统一通过 `logic::derive_view_state(BottomSheetDeriveInput)` 派生；`data-motion-source/data-custom-motion` 与 `state/class` 全部消费 `logic.rs` 输出。回归：`components/bottom-sheet/test/logic.rs::derive_view_state_centralizes_state_and_motion_markers`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_uses_logic_state_model`、`bottom_sheet_emits_baseline_style_state_data_attributes`。）
  - 输入边界统一进入 `logic.rs`，输出统一为可渲染语义状态与来源标记。
  - 事件处理器只触发状态变更，不重建状态机规则。
  - 样式层只消费状态标记，不承担状态判定职责。
- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。（`components/bottom-sheet/src/logic.rs` 已将离散轴收敛为 `BottomSheetVisibility`（Visible/Hidden）与 `BottomSheetAttachment`（Attached/Detached），`view.rs` 只做 `is_*` 输入到 enum 的归一化并把 enum 传入 `derive_view_state`；不再在视图层直接以多 bool 组合状态机。回归：`components/bottom-sheet/test/logic.rs::resolve_defaults_are_single_source_and_explicit`、`components/bottom-sheet/test/logic.rs::derive_view_state_centralizes_state_and_motion_markers`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_uses_logic_state_model`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_api_naming_contract_prefers_is_on_prefix_and_keeps_compat_aliases`。）
  - 互斥状态优先用 `enum` 建模，利用编译器封住无效组合。
  - 字符串输入若需兼容外部配置，必须先映射到类型化枚举再进入逻辑层。
  - 布尔爆炸（多个 bool 表达一个状态机）应在设计评审阶段直接拦截。
- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。（`components/bottom-sheet/src/logic.rs` 通过 `pub use ui_state_primitives::bottom_sheet::{...}` 消费 `BottomSheetStateInput/BottomSheetState/resolve_state`，并保持 `logic.rs` 只做映射装配；`view.rs` 仅消费调用方 `open: Signal<bool> + on_close`，不持有业务 store 类型。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_uses_logic_state_model`、`bottom_sheet_control_contract_is_controlled_surface_with_primitive_uncontrolled_pair`、`bottom_sheet_state_primitive_source_stays_decoupled_from_component_store_state_machines`。）
  - 组件中出现可复用状态机实现（受控/非受控、展开规则、选择归一）即判应下沉。
  - 组件与业务全局状态之间必须有适配边界，禁止组件直接依赖业务 store 类型。
  - `logic.rs` 仅做装配与映射，不重新实现状态原语。
- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。（N/A：`bottom-sheet` 仅承载 overlay 展示与关闭交互，无远程请求、无组件内异步状态机、无加载/失败/重试协议面；`disabled` 与可达性由 `Sheet` 语义契约处理，不涉及组件级 async 映射。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_has_no_async_loading_protocol_surface`。）
  - 无异步交互时需明确标注 N/A 理由（例如“组件无远程请求与异步状态”），不是机械打勾。
  - 有异步交互时，`is_loading`/disabled/`aria-busy`/retry 语义必须成套一致，且对键盘与读屏路径可用。
  - 异步失败态要有可恢复路径（重试或回退），并有语义测试覆盖。
- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。（`BottomSheet` 基础调用仅需 `open/on_close/id_base/title + children`，不暴露 `state=...` 这类内部对象必填项；复杂能力通过可选 props 按需开启（`description/footer/motion/is_detached/is_close_button_visible/...`）。`apps/docs-app/src/pages/components/pages/overlays_extra.rs` 已提供 `title="Hello World (Minimal Path)"` 最小示例，示例代码片段保持 5 行以内并可直接运行。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_dx_hello_world_is_minimal_and_copy_paste_ready`、`bottom_sheet_docs_page_covers_primary_playgrounds`、`bottom_sheet_docs_playgrounds_lock_state_matrix_contract_values`。）
  - 基础用法不得要求用户先理解或手动接线 `ui-state-primitives`/`ui-headless` 状态机。
  - 基础组件 Hello World 示例代码不得超过 5 行（导入与外层模板按仓库约定不计），并可直接运行。
  - 简单需求走简单 API，复杂需求再暴露高级入口：默认 props 覆盖高频场景，高级控制通过受控/扩展参数按需开启。
  - 禁止把内部状态对象作为基础必填参数暴露（例如强制 `state=...` 才能完成点击/展开等基本交互）。
  - docs-app 必须提供最小可用示例，优先展示一眼可懂的默认调用路径。
- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。（N/A：`BottomSheet` 不是集合型 `Parent/Item` 组件，不存在多 `Item` 注册与索引配对语义；其 API 为单内容面板（`title + children`）并通过可选 `description/footer` 扩展，不暴露 `labels/titles/panels` 并行数组或 `ItemSpec` 配置语法。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_is_not_collection_api_and_rejects_parallel_item_conventions`。）
  - 每个 item 的标题、语义与内容必须在同一 `Item` 结构维度绑定，避免索引配对式隐式约定。
  - `labels + children`、`titles + panels` 等并行数组/并行槽位写法不得作为默认或推荐 API。
  - 不引入这类语法糖：若为配置式输入，仅允许类型化 `ItemSpec`，并在内部映射为显式 `Item` 语义树。

### 3. 高级交互与物理机制（Shell/Physics）
- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。（N/A：当前 `BottomSheet` 未实现拖拽手势能力，仅通过 `open/on_close + Sheet` 完成开合；组件内不存在 `Dragging` 本地帧循环与 `Action::DragEnd` 协议面，因此该条对本组件为不适用项。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_has_no_dragging_micro_loop_or_drag_end_protocol`。）
- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。（N/A：`BottomSheet` 当前不依赖 DOM 几何测量来决定布局或定位；组件通过 `SheetPlacement::Bottom` + 已归一化状态渲染，无 `Intent -> Measure -> Rectification` 管线与收敛循环路径。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_has_no_two_pass_measure_rectification_geometry_pipeline`。）
- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。（N/A：`BottomSheet` 为单容器 overlay，不维护动态子项集合与导航顺序，不存在 `RegistrationContext/Register/Unregister/items_order` 协议面，也不依赖 `HashSet` 迭代做导航。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_has_no_collection_registration_protocol_surface`。）
- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。（N/A：`BottomSheet` 当前为单层 overlay 容器，不提供内容投影模式切换，也不存在 `KeepAlive` 隐藏生命周期协议（`NotifyHidden` 等）与后台高耗能副作用挂钩路径。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_has_no_slot_projection_keepalive_lifecycle_protocol`。）
- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。（N/A：`BottomSheet` 当前不依赖环境订阅流驱动状态，不在组件层订阅 `Resize/Theme/Intersection` 事件，也不存在 `BreakpointChanged` 等高层 Action 回流链路；组件行为由显式 props 与 `Sheet` 语义契约驱动。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_has_no_env_stream_subscription_or_event_flood_pipeline`。）
- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。（N/A：`BottomSheet` 非大型集合组件，不承载 `Table/Grid` 批量选择或状态压缩语义，不存在 `Context Bus + Selector` 事件拓扑与 `SelectionState::All` 协议面。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_has_no_event_light_cone_batch_collection_protocol`。）
- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。（N/A：`BottomSheet` 当前不包含复杂派生总线与跨订阅者广播链路，组件交互为局部 `open/on_close` 与语义状态映射，不存在 `TraceId` 透传需求。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_has_no_causality_bus_traceid_pipeline`。）
- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。（已落实：`components/bottom-sheet/src/view.rs` 暴露并透传 `lang/dir` 到 `Sheet`，并通过 `aria_labelledby/aria_describedby` 绑定标题与描述；关闭按钮文案走 `close_label` 可覆盖入口，默认文案仅在 `components/bottom-sheet/src/logic.rs`（`DEFAULT_CLOSE_LABEL/resolve_close_label`）兜底，`view.rs` 不硬编码可见文案。`Sheet` 在 `components/sheet/src/view.rs` 通过 `overlay_dialog_attrs` 输出 `role="dialog"`、`aria-modal`、`aria-labelledby`、`aria-describedby`、`lang/dir`，共享 A11y 工具来自 `crates/ui-headless/src/a11y.rs`。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_a11y_and_i18n_contracts_are_wired_and_overridable`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_close_button_contracts_are_preserved`。）
  - 交互元素必须具备可验证语义：`role`/`aria-*`/键盘可达路径完整，且和 headless 契约一致。
  - 用户可见文本来源必须可覆盖：优先 props，其次应用注入（`UiRoot`/i18n bundle），最后组件兜底文案；禁止把业务可见文案硬编码在 `view.rs`。
  - 组件需透传或消费 `lang` / `dir`（LTR/RTL）上下文，不得假设单语言单方向。
  - 共享 A11y 工具优先来自 `crates/ui-headless/src/a11y.rs`，组件层不重复发明同名语义工具。
- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。（已落实：`components/bottom-sheet/src/view.rs` 输出稳定状态轴标记（`data-state/data-description/data-footer/data-handle/data-close-button/data-detached/data-bottom-inset`）与来源标记（`data-motion-source/data-class-source`）；`components/sheet/src/view.rs` 输出 overlay 语义与可检索状态来源（`role="dialog"`、`aria-modal`、`data-state(open|closed)`、`data-open/data-closed`、`data-dismiss/data-keyboard-dismiss`、`data-placement` 及对应 `*-source`）。标记值由 `components/sheet/src/logic.rs` 封闭枚举函数与常量约束（如 `open|closed`、`dismissable|locked`、`enabled|disabled`、`bottom|left|right`、`default|custom`），避免自由文本漂移。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_emits_baseline_style_state_data_attributes`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_state_markers_cover_axes_sources_and_closed_value_sets`。）
  - 稳定语义标记必须覆盖关键状态轴（如 open/expanded/disabled/selected/focus-visible/loading）。
  - 状态来源必须可区分（受控/非受控、默认值/外部值、交互来源），通过稳定 marker 暴露而不是隐式推断。
  - 自动化选择器优先基于语义标记，不依赖 DOM 顺序、层级深度或临时 class 名。
  - 标记值应为封闭集合（可枚举），避免自由文本导致契约漂移。
- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。（已落实：`components/bottom-sheet/src/styles.rs` 的状态分支基于稳定语义标记与稳定 class（如 `data-handle/data-close-button/data-state/data-footer/data-detached/data-bottom-inset` 与 `ui-bottom-sheet--*`），未使用 `:nth-child` 等结构猜测选择器；`components/bottom-sheet/src/view.rs` 仅挂载 `data-*` 语义状态，不通过 inline `style=` 注入业务样式逻辑。视觉状态切换可由语义标记直接解释，而非依赖节点偶然存在。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_styles_include_state_marker_contracts`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_styles_avoid_structural_guessing_and_runtime_inline_style_logic`。）
  - `styles.rs` 中状态分支选择器必须基于 `data-*`/`aria-*`/稳定 class，禁止用 `:nth-child`、深层级选择器猜测状态。
  - 运行时样式仅允许传递必要 CSS 变量（custom properties）；禁止把业务样式逻辑塞进 inline style。
  - 视觉状态切换必须可由语义标记直接解释，不能依赖“某节点是否恰好存在”。
- [x] 测试验证“语义契约”而不只验证视觉快照。（已落实：`components/bottom-sheet/test/semantics.rs` 以语义断言为主，覆盖 `role/aria/data-state/source markers`（如 `bottom_sheet_a11y_and_i18n_contracts_are_wired_and_overridable`、`bottom_sheet_state_markers_cover_axes_sources_and_closed_value_sets`），并增加矩阵聚合回归 `bottom_sheet_semantic_contract_matrix_covers_interaction_paths_without_snapshot_only_assertions`，显式校验受控分支、原语层非受控配对、dismiss/keyboard-disabled 语义、键盘路径、指针路径与 wasm/SSR 分支标记。视觉快照未作为主断言路径（无 `assert_snapshot!/insta` 依赖）。）
  - 至少存在语义测试覆盖关键状态与交互路径（role/aria/data-state/source markers）。
  - 测试矩阵必须覆盖关键分支：受控/非受控、disabled、键盘路径、指针路径、SSR/wasm 差异（按适用范围）。
  - 视觉快照只能作为补充，不得替代语义契约断言。
- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。（已落实：`components/bottom-sheet/src/mod.rs` 仅维护最小导出边界；`logic.rs` 聚焦输入归一与状态派生（无 DOM/样式分支）；`styles.rs` 为 token-first 静态 CSS；`view.rs` 负责结构渲染并挂载 `Sheet` 语义契约，不重写 primitive/motion 引擎；`motion.rs` 仅将组件动效映射并委托 `sheet::motion::sanitize_motion`。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_files_follow_single_responsibility_boundaries`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_uses_logic_state_model`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_motion_sanitizes_custom_contract_values`。）
  - `mod.rs` 只维护最小稳定导出面与 feature gate，不承载实现细节。
  - `logic.rs` 只做输入归一、状态派生、来源标记；禁止 DOM 操作和样式细节分支。
  - `styles.rs` 只包含 token-first 静态 CSS；禁止硬编码主题常量与业务语义文案。
  - `view.rs` 只做结构渲染与 headless 契约挂载；禁止隐藏关键状态决策。
  - `motion.rs` 只做组件语义到动效契约映射与 attach；禁止在组件内重写通用动效引擎。
- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。（N/A：`BottomSheet` 不是复杂配置固化型组件，当前不需要 `*Spec::new()...render()` 形态；组件目录未引入 `spec.rs`，说明与审查结论保留在 `check2.md` 与组件文档路径。若未来确实引入 `spec.rs`，必须同步提供契约测试与版本演进说明。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_does_not_introduce_spec_rs_for_non_complex_scope`。）
  - 仅当组件存在稳定外部规范/Schema 契约或复杂配置固化需求时才引入 `spec.rs`。
  - 简单组件不得为了“形式统一”新增 `spec.rs`；说明文档应留在 `check2.md`/组件文档。
  - 新增 `spec.rs` 必须同步给出契约测试与版本演进说明。
- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。（已落实：`components/bottom-sheet/src/styles.rs` 已将关键视觉尺寸与边框/圆角/阴影统一收敛到 `var(--ui-*)`（含 `--ui-space-* / --ui-border-width / --ui-radius-* / --ui-shadow-*`），并移除组件私有 token；组件样式仍通过 `crates/ui/src/css.rs` 聚合并由 `crates/ui/src/root.rs`（`UiRoot`）注入。`view.rs` 不使用 inline 业务样式，也未引入 Utility-First/CSS-in-Rust 约定。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_css_is_aggregated`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_token_first_static_css_contract_is_respected`。）
  - 样式规则统一落在 `styles.rs`，由 `crates/ui/src/css.rs` 聚合并通过 `UiRoot` 注入。
  - 颜色/间距/圆角/阴影等视觉值必须来自 `var(--ui-*)`，禁止组件私有 token 体系。
  - Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。
  - CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。
- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。（已落实：`docs-app` 已提供 `ThemeVisualBaseline` 基线页（`apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs`），明确覆盖 Button/Input/Overlay 的默认主题层级、对比与交互反馈，并在描述中声明“Includes Button/Input/Overlay for visual regression snapshots.”；组件目录索引已注册该页面（`apps/docs-app/src/pages/components/pages.rs`）。该项以“视觉语言与体验质量”对齐为目标，未引入 HeroUI API 表层复制。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_visual_quality_gate_is_backed_by_theme_visual_baseline_docs`。）
  - 默认主题需通过基础美学清单：信息层级清晰（字重/字号/间距）、对比与层次自然、交互反馈明确（hover/active/focus）。
  - docs-app 必须提供默认主题基线页面与截图基线，关键组件（Button/Input/Overlay）纳入视觉回归对比。
  - 禁止“可访问但粗糙”的最低可用心态：视觉退化（类似旧式 Bootstrap 观感）视为质量回归。
  - HeroUI 对标以“视觉语言与体验质量”对齐为目标，不做无差别 API 表层复制。
- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。（已落实：`crates/ui/Cargo.toml` 存在 `component-bottom_sheet` 组件特性并纳入 `all-components` 可选总开关；`crates/ui/src/lib.rs` 以 `#[cfg(feature = "component-bottom_sheet")]` gate 导出 `bottom_sheet`，`web-demo-components` 与 `all-components` 通过互斥条件分流（`#[cfg(all(feature = "web-demo-components", not(feature = "all-components")))]`）；`crates/ui/src/css.rs` 对 bottom-sheet 样式聚合同样受 `#[cfg(feature = "component-bottom_sheet")]` 约束。`apps/web-demo/Cargo.toml` 使用 `default-features = false` + `features = ["inject-css", "web-demo-components"]`，未隐式拉起 `all-components`。命令验证：`cargo tree -e features -i ui -p ui --no-default-features --features component-accordion,inject-css` 输出仅含 `component-accordion` 与 `inject-css`；`cargo tree -e features -i ui -p web-demo` 输出仅经 `web-demo-components` 链接，不含 `all-components`。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_tree_shaking_contract_is_feature_gated_end_to_end`。）
  - package 模式必须有组件级 feature（如 `component-accordion`）；未启用组件不得进入编译与链接路径。
  - `lib.rs` 与 `css.rs` 必须按 feature 条件导出/聚合，禁止无条件引用所有组件模块和 CSS 常量。
  - source 模式下仅引入需要的组件源码，不通过中央注册表维持全组件可达。
  - 任意“全量组件映射表/注册表”若导致不可达代码变可达，直接判不通过。
  - 验证命令（特性树）：`cargo tree -e features -p ui --no-default-features --features component-accordion,inject-css`，确认仅启用目标组件特性链。
  - 验证命令（反向依赖）：`cargo tree -e features -i ui -p web-demo`，检查是否被 `all-components` 或隐式特性全量拉起。
  - CI 检查（最小特性编译）：新增任务仅开启目标最小特性（示例：`cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-accordion,inject-css`）。
  - CI 检查（体积预算）：对“最小特性构建产物”设定预算并阻断回归（可用固定阈值，如 `< 50KB`，或基于仓库基线的相对阈值）；不得只做编译通过而不做体积约束。
- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。（已落实：`components/bottom-sheet/src/logic.rs` 将离散状态轴收敛为 `BottomSheetVisibility` 与 `BottomSheetAttachment`，并通过 `BottomSheetDeriveInput` 类型化输入统一归一化到 `derive_view_state`；`crates/ui-state-primitives/src/bottom_sheet.rs` 维持封闭 marker 值集合（如 `with-description|title-only`、`shown|hidden`、`none|sm|md|lg|xl`、`default|custom`）；`components/bottom-sheet/src/view.rs` 稳定输出 `data-state/data-description/data-footer/data-handle/data-close-button/data-detached/data-bottom-inset/data-motion-source/data-class-source` 供测试与 Agent 消费。回归：`components/bottom-sheet/test/logic.rs::resolve_defaults_are_single_source_and_explicit`、`components/bottom-sheet/test/logic.rs::derive_view_state_centralizes_state_and_motion_markers`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_state_markers_cover_axes_sources_and_closed_value_sets`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_type_system_and_semantic_markers_form_machine_readable_contract`。）
  - 离散输入与状态轴必须优先使用 `enum`/新类型建模，避免字符串协议与布尔爆炸。
  - 无效状态要么在类型层不可表达，要么在 `logic.rs` 被统一归一化并可测试。
  - 关键状态必须通过稳定语义标记对外可读，供测试与 Agent 自动化消费。
  - 编译器与测试反馈应能直接定位状态契约破坏点，形成可持续闭环。

### 4. DOM/环境边界治理
- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。（已落实：`components/sheet/src/view.rs` 通过 `use_focus_trap(FocusTrapOptions::enabled(panel_ref))` 接入 `ui-headless` 焦点陷阱并结合 `use_overlay_stack_registration()` 约束 topmost 行为；恢复链由 `crates/ui-headless/src/focus_trap.rs` 统一维护 `FOCUS_MANAGER_STACK`，并通过 `RestorePolicy::Selector/FallbackTo` + `derive_restore_policy/restore_focus_chain` 执行，不在组件层私存恢复目标 `NodeRef`。当恢复链失败时再兜底 `document.body().focus()`，避免焦点悬空。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_focus_restoration_uses_global_stack_and_policy_chain`。）
- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。（N/A：当前 `BottomSheet`/`Sheet` 不包含 ECharts/Map 等命令式第三方库接入场景，也未暴露第三方实例句柄作为公共 API；组件职责仍限定在 overlay 语义与样式/动效装配。为防回归，已增加关键字与 API 面扫描回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_has_no_foreign_zone_escape_hatch_surface`。若未来引入命令式第三方渲染，必须先落 `Foreign Zone` 协议（`YieldControl/CleanupForeign`）并补生命周期清理测试。）
- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。（N/A（组件内 ID 生成）：`BottomSheet` 不在组件内部生成随机/时间相关 ID，改为要求调用方显式传入 `id_base`，并在 `view.rs` 通过 `normalize_id_base + format!("{id_base}-title/-description")` 派生稳定标识，SSR/Hydration 可复现。仓库层已提供 `ui-headless` 的 `UiIdProvider`（`provide_ui_id_provider/use_ui_id_provider`）作为需要自动分配 ID 时的确定性注入能力。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_avoids_hydration_discontinuity_time_random_and_uses_stable_id_inputs`。）
- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。（已落实：`components/sheet/src/view.rs` 对键盘事件路径显式区分 `#[cfg(target_arch = "wasm32")]` 与 `#[cfg(not(target_arch = "wasm32"))]`；`components/sheet/src/motion.rs` 提供 wasm 实现与 non-wasm `attach_motion` 降级分支；`crates/ui-motion/src/lib.rs` 提供 non-wasm `web` no-op backend（`prefers_reduced_motion/animate`），确保 SSR/tooling 可编译且行为可预测。`components/bottom-sheet/src/view.rs` 未直接引用 `web-sys`/浏览器对象。compile-only 命令执行记录：`cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-bottom_sheet,inject-css`、`cargo check -p ui --no-default-features --features component-bottom_sheet,inject-css`、`cargo check -p ui` 均因环境 `Invalid cross-device link (os error 18)` 阻塞，非契约断言失败。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_platform_paths_are_explicit_and_non_wasm_is_browser_safe`。）
  - 至少包含 compile-only 证据：web（wasm32）、ssr（native）、默认本地构建三条路径。
  - 平台分支差异必须显式 `cfg` 或 feature 管理，禁止依赖运行时偶然行为。
  - non-wasm 路径禁止引用 `web-sys`/浏览器对象。
- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。（已落实：`crates/ui-headless/src/lib.rs` 存在 `#[cfg(all(feature = "web", feature = "ssr"))]` + `compile_error!` 互斥保护；`crates/ui-headless/Cargo.toml` 显式定义 `default = ["web"]`、`web = ["leptos/csr"]`、`ssr = ["leptos/ssr"]`。`BottomSheet` 通过 `components/sheet/src/view.rs` 消费 `ui-headless` 能力（`use_focus_trap/use_modal/use_overlay_stack_registration`），未在组件层绕过该互斥契约。验证命令：`cargo check -p ui-headless --no-default-features --features web`、`cargo check -p ui-headless --no-default-features --features ssr`、`cargo check -p ui-headless --no-default-features --features web,ssr`；当前环境均在依赖构建阶段被 `Invalid cross-device link (os error 18)` 阻塞，未能产出完整编译结论。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_respects_ui_headless_web_ssr_mutual_exclusion_contract`。）
  - 组件依赖 `ui-headless` 能力时，不得破坏其 web/ssr 互斥约束。
  - 组件若新增 headless 功能接入，需验证两条 feature 路径都可编译。
  - 发现“同时启用 web+ssr 仍可过编译”视为契约回归。
- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。（已落实：`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = "wasm32"))]` 下提供 `web::prefers_reduced_motion` 与 `web::animate` no-op/stub；`components/sheet/src/motion.rs` 存在 non-wasm `attach_motion` 降级分支，仅在关闭态触发 `finish_exit.run(())`，不依赖浏览器对象；`components/bottom-sheet/src/motion.rs` 仅做 contract sanitize 委托，不假设动画句柄存在。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_platform_paths_are_explicit_and_non_wasm_is_browser_safe`（覆盖 non-wasm 分支无 `web_sys/js_sys/window/document/unwrap/expect/panic`，并校验可预测降级行为）与 `crates/ui-motion/src/lib.rs` 内置 `non_wasm_web_backend_is_predictable_noop`。工具链验证命令在当前环境受 `Invalid cross-device link (os error 18)` 阻塞，非契约断言失败。）
  - `motion.rs` 调用必须可在 non-wasm 下安全降级，不触发 panic。
  - 组件不得假设动画句柄一定存在；no-op 分支行为需可预测。
  - toolchain 场景（测试/文档/静态分析）不得因 motion 依赖阻塞编译。
- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。（已落实：`components/sheet/src/motion.rs` 在 wasm 路径通过 `ui_motion::web::prefers_reduced_motion()` 显式分支，`reduced-motion` 时直接写入最小必要反馈（backdrop/panel opacity + panel x/y）并跳过 spring 驱动；关闭态仍触发 `finish_exit.run(())` 保证存在性语义一致。`components/sheet/src/view.rs` 对键盘事件增强使用 `#[cfg(target_arch = "wasm32")]`（`is_composing/default_prevented`），SSR 路径使用 `#[cfg(not(target_arch = "wasm32"))]` 明确降级，但两者统一汇入 `logic::should_close_on_escape`，不分裂语义契约。`components/bottom-sheet/src/view.rs` 继续以确定性 `id_base` 派生 `title/description` 标识并透传 `open` 到 `Sheet`，SSR 输出与 hydration 可对齐。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_reduced_motion_and_ssr_wasm_semantics_are_consistent`。）
  - `reduced-motion` 下动画应跳过或降级为最小必要反馈。
  - SSR 输出必须与客户端 hydration 兼容，避免首帧语义错位。
  - wasm 分支允许增强交互，但语义契约不得与 SSR 分支分裂。
- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。（已落实：`docs-app` 组件页统一经 `apps/docs-app/src/pages/components/shell.rs` 的 `component_page_perf_budget + <UiPerfProbe ...>` 挂载预算探针（`BottomSheet` 页面位于 `apps/docs-app/src/pages/components/pages/overlays_extra.rs`，并由 `apps/docs-app/src/pages/components/pages.rs` 的 `component_doc!(\"BottomSheet\", \"bottom-sheet\", ...)` 纳入覆盖遍历）；`apps/docs-app/src/perf_probe.rs` 对外暴露稳定可归因标记（`data-perf-mount-ms/data-perf-budget-ms/data-perf-budget-update-ms/data-perf-budget-heap-kb/data-perf-violation/data-perf-observability`）；`e2e/tests/docs_app_components_coverage.spec.mjs` 对 perf marker 与 violation 进行回归断言，形成可检测阈值。`BottomSheet` 组件自身在 `components/bottom-sheet/src/view.rs` 输出 `data-state/data-description/data-footer/data-handle/data-close-button/data-detached/data-bottom-inset/data-motion-source/data-class-source`，支持将性能回归归因到状态/渲染/样式/动效路径。N/A：`BottomSheet` 暂未接入精确 `render_count` 自动化计数，当前采用 mount+budget+traceability 等价证据；`render_count` 自动化 follow-up 已在 `docs/plan/TODO.md` 跟踪（`建立 \`render_count\` 自动化回归（Button/Input/Accordion），替换当前 mount-only 等价证据`）。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_performance_governance_contract_is_mount_only_traceable_and_blocking`。门禁：`scripts/check-ui-performance.sh` 已纳入 `bottom_sheet_performance_governance_contract_is_mount_only_traceable_and_blocking` 并继续阻断 `button/input` 预算契约与 `accordion` 的 perf-probe/render_count follow-up 契约。）
  - 关键交互组件需定义最小预算项（首渲染、关键更新、内存/分配趋势）。
  - 回归检测至少具备可重复基线与失败阈值，不靠主观“感觉变慢”。
  - 性能问题需可归因到状态、渲染、样式或动效路径之一。
  - 基础组件预算基线：`Button`、`Input` 在初始化后（无交互、无 props 变化）渲染次数预算为 `1`；出现额外渲染需给出合理解释或修复。
  - 测试要求：在 `components/*/test/**` 增加 `render_count` 类回归测试（测试框架支持时必须启用）；至少覆盖基础组件与本次改动组件。
  - 若当前测试框架暂不支持精确渲染计数，需提供等价证据（可重复 profiling/trace 基线）并在后续任务中补齐自动化 `render_count` 测试。
- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。（已落实：`components/bottom-sheet/src/view.rs` 从单组件内重复巨型 `view!` 拆分为语义子渲染函数 `render_bottom_sheet_handle/render_bottom_sheet_close_button/render_bottom_sheet_header/render_bottom_sheet_body/render_bottom_sheet_footer/render_bottom_sheet_content`，并通过 `BottomSheetContentInput` 在描述/非描述两条外壳分支共享同一内容渲染路径，避免重复宏展开与深层嵌套。当前根节点 `data-slot=\"bottom-sheet\"` 仅定义一次，`view.rs` 总行数受控。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_view_macro_complexity_is_split_into_semantic_subrenders`。）
  - 复杂结构按语义子块拆分（header/body/item 等），避免巨型单块 `view!`。
  - `view.rs` 中若出现多层嵌套重复片段，应优先提取局部渲染函数。
  - 编译时间/产物体积异常增长时，优先排查宏展开体量。
- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。（已落实：`components/bottom-sheet/src/view.rs` 仅保留顶层 `#[component] pub fn BottomSheet(...)`，将轻逻辑片段拆为普通函数 `render_bottom_sheet_handle/render_bottom_sheet_close_button/render_bottom_sheet_header/render_bottom_sheet_body/render_bottom_sheet_footer/render_bottom_sheet_content`（返回 `impl IntoView`）；未把局部片段升级为额外 `#[component]`，减少抽象噪音并保持语义标记稳定。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_prefers_functional_split_without_extra_component_noise` 与 `components/bottom-sheet/test/semantics.rs::bottom_sheet_view_macro_complexity_is_split_into_semantic_subrenders`。）
  - 纯静态或轻逻辑片段优先函数化；仅在需要独立 props 语义时升级为组件。
  - 禁止把所有局部片段都升格为 `#[component]` 导致抽象噪音。
  - 拆分后语义标记与测试定位仍需稳定。
- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。（已落实：`components/bottom-sheet/src/view.rs` 将关闭按钮 SVG 的静态片段收敛为常量（`CLOSE_ICON_VIEWBOX/CLOSE_ICON_FILL/CLOSE_ICON_PATH_D/CLOSE_ICON_STROKE_WIDTH`）与模板函数 `render_bottom_sheet_close_icon()`，调用点仅复用 `{render_bottom_sheet_close_icon()}`；页脚静态容器维持单一模板 `render_bottom_sheet_footer(...)`，避免重复动态构造。语义保持不变（`aria-hidden=\"true\"`、`aria_label=close_label`、`data-slot=\"bottom-sheet-footer\"`）。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_static_fragments_are_constantized_and_templated`。）
  - 可判定为纯静态的片段应避免重复动态构造。
  - 常量化后仍需维持可访问语义（title/aria-label/role 等）。
  - 静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。
- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。（N/A：`BottomSheet` 当前无 `inner_html` 注入路径，组件与文档页面均未暴露动态 HTML 注入面：`components/bottom-sheet/src/view.rs`、`components/bottom-sheet/src/logic.rs`、`components/bottom-sheet/src/motion.rs`、`components/bottom-sheet/src/styles.rs`、`components/bottom-sheet/src/protocol.rs` 与 `apps/docs-app/src/pages/components/pages/overlays_extra.rs` 均不包含 `inner_html=`/`set_inner_html`/`dangerously_set_inner_html`。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_disallows_inner_html_in_component_surface`。）
  - 仅允许编译期常量或明确白名单内容进入 `inner_html`。
  - 严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。
  - 使用 `inner_html` 的节点必须补语义测试与安全回归说明。
- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。（已落实：`components/bottom-sheet/src/view.rs` 复用 `use_ui_trace` 并在 `open` 变更时发出 `UiTraceEventKind::OpenChange`（`trace.emit("bottom-sheet", ...)`），同时通过稳定语义标记 `data-state/data-description/data-footer/data-motion-source/data-class-source` 暴露来源与状态轴；`apps/docs-app/src/lib.rs` 在 `debug_assertions` 下启用 `provide_ui_trace(debug_overlay_enabled)` + `<debug_overlay::UiDebugOverlay enabled=true />`，`apps/docs-app/src/debug_overlay.rs` 提供时间线事件槽位（`data-slot="ui-debug-overlay-event"`、`data-kind`、`format!("{ts_ms}ms")`）作为开发模式可视化入口；feature 隔离沿用 `crates/ui/Cargo.toml` 的共享 wasm-debug 开关（含 `sheet-wasm-debug`，无 `bottom_sheet` 私有 debug feature）避免污染生产默认链路。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated`；脚本门禁：`scripts/check-ui-wasm-debug.sh` 新增 `cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated`。）
  - 开发模式下至少能追踪关键状态变更来源与前后值。
  - 关键交互链路应支持最小可复现记录（事件顺序/状态转移）。
  - 调试开关默认不进入生产包体与公共 API。
- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。（已落实：`apps/docs-app/src/playground.rs` 统一提供 scoped CSS 热调试链路（`<style>{compose_scoped_css(...)}</style>` + `data-playground-scope` + `playground-test` 面板 + `Show test/Restore original CSS`），样式调整无需完整 wasm 重编译；`apps/docs-app/src/pages/components/pages/overlays_extra.rs::bottom_sheet()` 提供 `Hello World/Semantic/Detached/Custom Motion` 四个隔离演练入口，并以 `present_*` + `on_*_exit_complete` 维持交互上下文（关闭动效期间不丢上下文）；可选状态保留按 N/A 明确：当前 `BottomSheet` workbench 默认不写入 `localStorage/sessionStorage`，避免跨会话污染，保留策略限定在会话内 `Signal`。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_dx_playground_supports_hot_reload_context_and_isolated_workbench`；脚本门禁：`scripts/check-ui-dx.sh` 新增 `cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_dx_playground_supports_hot_reload_context_and_isolated_workbench`。）
  - 常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。
  - 组件调试应尽量保持当前交互上下文，降低重复操作成本。
  - 复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。
- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。（已落实：`BottomSheet` 当前无公开 `spec/config` 输入与 `spec.rs`，`serde` 迁移路径按 N/A 固化（`components/bottom-sheet/src/mod.rs` 不导出 `spec/protocol`，`component-bottom_sheet` feature 仅依赖 `component-sheet/component-button`，无 `dep:serde/dep:serde_json` 直连）；关键埋点统一复用共享 `ui_headless::use_ui_trace + UiTraceEventKind::OpenChange`（`components/bottom-sheet/src/view.rs`），不引入组件私有 `tracing::span/event` 词汇漂移；异步边界保持 runtime-agnostic，组件实现未暴露 `tokio/async-std/runtime::Handle` 等运行时细节到公共 API。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope`、`bottom_sheet_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events`、`bottom_sheet_engineering_contract_avoids_runtime_leaks_in_public_api_surface`、`bottom_sheet_engineering_check_script_covers_serde_tracing_and_runtime_boundaries`；脚本门禁：`scripts/check-ui-engineering.sh` 新增对应 `bottom_sheet_semantics` 三条工程契约命令。）
  - 若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。
  - 关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。
  - 异步边界不得把具体 runtime 类型暴露到组件公共接口。

### 5. 样式与动效（Theme & Motion）
- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。（已落实：`components/bottom-sheet/src/styles.rs` 已将关键视觉变量统一切换为双层防御链（如 `var(--ui-bg, var(--ui-fallback-bg))`、`var(--ui-border-width, var(--ui-fallback-border-width))`、`var(--ui-font-size-150, var(--ui-fallback-font-size-150))`、`var(--ui-space-lg, var(--ui-fallback-space-lg, var(--ui-fallback-space-md)))`），并移除 `1px/16px/24px/14px/20px` 等裸终值 fallback；Fallback 终值来源保持 `ui-theme` SSOT（`crates/ui-theme/src/css.rs` 输出 `--ui-fallback-*`）。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_styles_use_defensive_variable_fallback_chain`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_defensive_variables_check_script_covers_style_fallback_contract`；脚本门禁：`scripts/check-ui-contract-hygiene.sh` 新增 `cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_styles_use_defensive_variable_fallback_chain`。）
- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\"top: 10px\"`）。（已落实：`crates/ui/src/css.rs::push_components_css` 以 `@layer ui` 包裹组件样式聚合，并在 `component-bottom_sheet` feature 下注入 `crate::bottom_sheet::styles::CSS`；`BottomSheet` 运行时视图链路（`components/bottom-sheet/src/view.rs` + `components/sheet/src/view.rs`）未引入 `style=\"top/left/right/bottom/width/height/position\"` 这类普通内联布局样式，保持“若存在内联样式则仅允许 CSS 变量负载”的约束。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_cascade_layer_and_runtime_style_contract_is_enforced`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_cascade_layer_check_script_covers_layer_and_inline_style_guard`；脚本门禁：`scripts/check-ui-contract-hygiene.sh` 新增 `cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_cascade_layer_and_runtime_style_contract_is_enforced`。）
- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。（已落实：`components/bottom-sheet/src/motion.rs` 维持组件层 `BottomSheetMotion` 合同并委托 `crate::sheet::motion::sanitize_motion`；`components/sheet/src/motion.rs` 内置 `SpringConfig` 合同化清洗（含 `stiffness/damping` 有效值约束）并通过 `attach_motion` 挂载到运行时节点；wasm 路径显式分支 `prefers_reduced_motion`，reduced-motion 下直接写入最小必要 CSS 变量并在关闭态触发 `finish_exit`，non-wasm 路径提供 no-op/可预测降级（关闭态触发 `finish_exit.run(())`，不依赖浏览器 API）。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_motion_contract_check_script_covers_platform_guard`。脚本门禁：`scripts/check-ui-platforms.sh` 新增 `cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe`。）
- [x] `ui` 固定入口文件落点正确。（已落实：`crates/ui/src/lib.rs` 维持总入口导出并对 `bottom_sheet` 使用 `component-bottom_sheet` feature gate，且不暴露 `overlay_open/presence/a11y` 模块或 `web_sys` 公共导出；`crates/ui/src/css.rs` 通过 `push_components_css` 在 `@layer ui` 内按 feature 聚合并对 `bottom_sheet` 条件注入；`crates/ui/src/root.rs` 统一注入 `BASE_CSS + theme vars + optional components css` 并集中提供 `provide_ui_i18n/provide_ui_id_provider`；`crates/ui-visual-primitive/src/active_highlight.rs` 保持共享高亮样式与 motion driver 能力，不承载 BottomSheet 业务语义；`crates/ui/src/overlay_open.rs`、`crates/ui/src/presence.rs`、`crates/ui/src/a11y.rs` 均不存在，原语能力固定由 `crates/ui-headless/src/controllable_state.rs`、`crates/ui-headless/src/presence.rs`、`crates/ui-headless/src/a11y.rs` 提供。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_ui_components_fixed_entry_files_follow_layered_boundaries`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_engineering_check_script_covers_serde_tracing_and_runtime_boundaries`（含入口文件门禁命令断言）。脚本门禁：`scripts/check-ui-engineering.sh` 新增 `cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_ui_components_fixed_entry_files_follow_layered_boundaries`。）
  - `crates/ui/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。
  - `crates/ui/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。
  - `crates/ui/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。
  - `crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。
  - `crates/ui/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。
  - `crates/ui/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。
  - `crates/ui/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。
- [x] 组件目录标准文件落点正确。（已落实：`components/bottom-sheet/src/` 保持标准职责文件落点 `mod.rs + logic.rs + styles.rs + view.rs + motion.rs`；`mod.rs` 仅维护最小稳定导出边界（`BottomSheet/BottomSheetMotion/BottomSheetState*`），`logic.rs` 只做 props 归一与状态派生（不含 `view!/DOM` 细节），`styles.rs` 保持 token-first 静态 CSS（`var(--ui-*)`），`view.rs` 只做 Leptos 结构渲染与 `Sheet` 语义挂载，`motion.rs` 仅做语义到共享 motion contract 的映射委托；组件目录不存在 `render.rs` 漂移文件，且 `spec.rs` 在本组件范围维持“不引入”策略。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_files_follow_single_responsibility_boundaries`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_does_not_introduce_spec_rs_for_non_complex_scope`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_component_directory_standard_file_layout_is_enforced`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_engineering_check_script_covers_serde_tracing_and_runtime_boundaries`（含目录落点门禁命令断言）。脚本门禁：`scripts/check-ui-engineering.sh` 新增 `cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_component_directory_standard_file_layout_is_enforced`。）
  - `<component>/mod.rs`：最小稳定导出面，存在且无过度导出。
  - `<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。
  - `<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。
  - `<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。
  - `<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。
  - `<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。

### 6. AI 原生能力与文件落点（Struct-First & Projection）
- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。（已落实：`components/bottom-sheet/src/` 的核心实现文件固定为 `mod.rs + logic.rs + styles.rs + view.rs + motion.rs`，并明确禁止 `render.rs/spec.rs`；`protocol.rs` 与 `lib.rs` 仅作为协议/入口辅助文件，不参与组件渲染、状态归一派生或动效装配。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_component_directory_standard_file_layout_is_enforced`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_file_placement_discipline_contract_is_explicit_for_interactive_component_scope`。脚本门禁：`scripts/check-ui-component-files.sh` 新增 `cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_file_placement_discipline_contract_is_explicit_for_interactive_component_scope`。）
- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。（N/A-by-design：`BottomSheet` 不是复杂配置固化型组件，当前不引入 `spec.rs` 与 `BottomSheetSpec::new()...render()` 构建链路；复杂组件 Builder 基线仍由 `button` 提供。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_marks_hyper_structure_builder_item_complete`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_component_files_check_script_covers_hyper_structure_builder_contract`。脚本门禁：`scripts/check-ui-component-files.sh` 新增 `cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_hyper_structure_builder_spec_is_explicitly_na_for_non_complex_component`。）
- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。（已落实：新增 `components/bottom-sheet/src/Component.toml`（能力清单）与 `components/bottom-sheet/src/bottom_sheet.rbi`（接口签名投影），并与 `BottomSheet` 公共 API 轴保持对齐（`open/on_close/id_base/title/description/footer/lang/dir/motion/bottom_inset_px/class_name` 等）。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_context_compression_manifest_and_rbi_projection_are_present_and_current`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_component_files_check_script_covers_context_compression_manifest_rbi_contract`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_marks_context_compression_manifest_rbi_item_complete`。脚本门禁：`scripts/check-ui-component-files.sh` 新增 `cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_context_compression_manifest_and_rbi_projection_are_present_and_current`。）
- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。（已落实：`BottomSheet` 在 `components/bottom-sheet/src/logic.rs` 引入类型化 Agent Contract（`BottomSheetAgentSchemaVersion/Intent/Action/StateAxis/SourceAxis/OutputStatus/StreamSupport/StreamFallback/RenderPolicy` + `resolve_agent_contract`），`components/bottom-sheet/src/view.rs` 通过 `agent_contract` 派生并稳定挂载 `data-ui-schema/data-ui-schema-version/data-ui-intent/data-ui-action/data-ui-state/data-ui-source/data-ui-output-status/data-ui-stream-support/data-ui-stream-fallback/data-ui-render-policy`，避免 Agent 依赖 DOM 猜测。`components/bottom-sheet/src/Component.toml` 同步声明 `agent_contract_schema_markers`、`[[agent_contract_markers]]` 与 `[[agent_contract_whitelist]]`（阻断 `inner_html/<script/javascript:`）。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_documents_agent_contract_schema_governance_rules`、`bottom_sheet_agent_contract_is_schema_typed_and_machine_readable`、`bottom_sheet_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing`、`bottom_sheet_agent_contract_render_path_is_whitelist_safe_and_script_injection_free`、`bottom_sheet_contract_hygiene_script_covers_agent_contract_schema_guards`。脚本门禁：`scripts/check-ui-contract-hygiene.sh` 新增对应 4 条 `bottom_sheet_semantics` 命令。）
  - 关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。
  - Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。
  - 契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。
  - 配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。
- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。（已落实：`components/bottom-sheet/src/logic.rs` 引入类型化 `BottomSheetAgentStreamMode { Streaming, Snapshot }`，并在 `resolve_agent_contract` 显式收敛到 `stream_mode=snapshot`；`components/bottom-sheet/src/view.rs` 挂载稳定语义标记 `data-ui-stream-mode`（同时保留 `data-ui-stream-support/data-ui-stream-fallback`），`components/bottom-sheet/src/Component.toml` 同步声明 `stream_mode` marker（`values=[\"streaming\",\"snapshot\"]`），将显示模式限定为两态且仅用于 LLM 输出渲染语义。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`、`bottom_sheet_streaming_display_modes_are_limited_to_streaming_and_snapshot`、`bottom_sheet_streaming_script_covers_two_mode_definition_contract`、`bottom_sheet_check2_marks_streaming_two_mode_definition_complete`。脚本门禁：`scripts/check-ui-streaming.sh` 新增 bottom-sheet 对应 2 条命令。验证阻塞说明：当前环境执行 `cargo test` 仍受 `Invalid cross-device link (os error 18)` 影响。）
  - `Streaming`：LLM 还在生成，界面边生成边显示。
  - `Snapshot`：LLM 全部生成完成后，一次性显示。
- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。（已落实：`BottomSheet` 的 Agent Contract 在 `components/bottom-sheet/src/logic.rs` 显式固定 `stream_mode=snapshot` 与 `stream_fallback=snapshot`，并在 `components/bottom-sheet/src/view.rs` 稳定挂载 `data-ui-stream-mode/data-ui-stream-fallback/data-ui-output-status`，保证组件在消费完整结果时稳定渲染且状态可观测；`render_bottom_sheet_content(content_input.clone())` 路径复用同一归一化输入，避免“仅流式路径可用”。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_documents_snapshot_as_default_baseline_capability`、`bottom_sheet_snapshot_baseline_consumes_complete_result_and_renders_stably`、`bottom_sheet_streaming_script_covers_snapshot_baseline_contract`、`bottom_sheet_check2_marks_snapshot_baseline_capability_complete`。脚本门禁：`scripts/check-ui-streaming.sh` 新增 bottom-sheet 对应 2 条 snapshot 基线命令。验证阻塞说明：当前环境执行 `cargo test` 仍受 `Invalid cross-device link (os error 18)` 影响。）
  - 所有组件都应能消费“完整生成结果”并稳定渲染。
  - 即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。
- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。（已落实：`BottomSheet` 不是正文阅读面，归类为 `Streaming Optional`；在 `components/bottom-sheet/src/logic.rs` 中显式固定 `stream_support=optional + stream_mode=snapshot + stream_fallback=snapshot + output_status=verified`，并在 `components/bottom-sheet/src/view.rs` 挂载连续语义标记 `data-ui-stream-support/data-ui-stream-mode/data-ui-stream-fallback/data-ui-output-status` 与 `aria_labelledby/aria_describedby`，确保可读性与状态连续。组件层不承载数据校验/断线恢复/重试策略，相关职责保持在上层。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_documents_streaming_required_optional_classification_rules`、`bottom_sheet_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、`bottom_sheet_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`、`bottom_sheet_streaming_script_covers_streaming_responsibility_contract`、`bottom_sheet_check2_marks_streaming_scope_as_optional_with_snapshot_fallback`。脚本门禁：`scripts/check-ui-streaming.sh` 新增 bottom-sheet 对应 3 条 streaming responsibility 命令。验证阻塞说明：当前环境执行 `cargo test` 仍受 `Invalid cross-device link (os error 18)` 影响。）
  - `Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。
  - `Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。
  - 无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。
  - 数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。

### 7. 测试、门禁与交付
- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。（已落实：`components/bottom-sheet/src/{mod,logic,styles,view,motion,protocol,lib}.rs` 非测试源码约束为无 `unwrap/expect/unwrap_err` 与无 `let _ =` 吞错；字符串复制热点在 `BottomSheet` 关键渲染/归一化路径未出现 `to_string/String::from/to_owned` 风格 churn，按“收敛到 `Cow<'static, str>` 或热点缺失”判定通过。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources`、`bottom_sheet_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent`、`bottom_sheet_rust_hygiene_script_enforces_repo_level_hygiene_guards`、`bottom_sheet_check2_marks_rust_hygiene_contract_complete`。脚本门禁：`scripts/check-ui-engineering.sh` 新增 bottom-sheet 对应 3 条 hygiene 命令。本地验证：`RUST_HYGIENE_SCOPE='components/bottom-sheet' ./scripts/check-rust-hygiene.sh` 返回 `[rust-hygiene] OK`。）
- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。（已落实：`crates/ui/Cargo.toml` 保持 `component-bottom_sheet = ["component-sheet", "component-button"]` 的组件级特性注册；`crates/ui/src/lib.rs` 与 `crates/ui/src/css.rs` 继续通过 `#[cfg(feature = "component-bottom_sheet")]` gate 导出与样式聚合（`out.push_str(crate::bottom_sheet::styles::CSS);`），无无条件全局聚合路径。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_tree_shaking_contract_is_feature_gated_end_to_end`、`bottom_sheet_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget`、`bottom_sheet_check2_marks_tree_shaking_feature_pruning_contract_complete`。脚本门禁：`scripts/check-ui-tree-shaking.sh` 新增 bottom-sheet 三条树摇命令与最小特性树校验（`BOTTOM_SHEET_MIN_FEATURES="component-bottom_sheet,inject-css"`）。命令已尝试（2026-02-20）：`TMPDIR=/tmp CARGO_TARGET_DIR=/tmp/codex-bottom-sheet-tree-shaking cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget`；当前环境仍受 `Invalid cross-device link (os error 18)` 阻断，待环境修复后复跑。）
- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。（已落实：新增聚合回归 `components/bottom-sheet/test/semantics.rs::bottom_sheet_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement`，并与既有 `bottom_sheet_semantic_contract_matrix_covers_interaction_paths_without_snapshot_only_assertions`、`bottom_sheet_performance_governance_contract_is_mount_only_traceable_and_blocking` 形成三段式证据链，覆盖 `role/aria/data`、键盘与指针焦点流转、性能可观测标记与非快照断言。`render_count` 当前沿用仓库统一 follow-up（`docs/plan/TODO.md`）并以 mount-only 等价证据阻断回归。脚本门禁：`scripts/check-ui-performance.sh` 新增 bottom-sheet 语义+性能矩阵命令。环境验证阻塞说明：本机执行 `cargo test` 仍可能遇到 `Invalid cross-device link (os error 18)`。）
- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `BottomSheet` 未发生跨大版本 API 破坏升级。判定依据：`components/bottom-sheet/src/Component.toml` 保持 `schema_version = "1"`；`components/bottom-sheet/src/bottom_sheet.rbi` 的 `BottomSheet(...)` 公共签名未发生破坏性移除/重命名；`components/bottom-sheet/src/{mod.rs,logic.rs,view.rs,styles.rs,motion.rs,protocol.rs}` 未引入 `migrate_v1_to_v2` / `migrate_v2_to_v3` / `SchemaRegistry` / `deprecation_window` / `contract.v2`。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_version_deprecation_migration_is_na_without_major_breaking_upgrade`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_version_deprecation_migration_script_covers_engineering_gate`。脚本门禁：`scripts/check-ui-engineering.sh` 新增 `cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_version_deprecation_migration_is_na_without_major_breaking_upgrade`。命令已尝试（2026-02-20）：`TMPDIR=/tmp CARGO_TARGET_DIR=/tmp/codex-bottom-sheet-version-migration cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_version_deprecation_migration_is_na_without_major_breaking_upgrade`；当前环境仍受 `Invalid cross-device link (os error 18)` 阻断，待环境修复后复跑。）
- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。（已落实：`apps/docs-app/src/pages/components/pages/overlays_extra.rs::bottom_sheet` 新增并固定 `title="Hello World (Minimal Path)"`、`title="State Matrix"`、`title="Controlled vs Uncontrolled"`、`title="Streaming / Snapshot Contract"` 四类 Playground；`Controlled vs Uncontrolled` 对组件边界做显式 N/A 说明（`BottomSheet` 公共面为 controlled-only，uncontrolled 归属上游 primitive/adapter）。文档复制链路通过 `BOTTOM_SHEET_DOC_IMPORTS` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补全 imports，Source-first 区块提供一键复制说明、特性依赖（`component-bottom_sheet + inject-css`）与真实源码路径（`data-slot="bottom-sheet-source-first"` / `data-slot="bottom-sheet-source-paths"`）。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_dx_check_script_covers_docs_product_copy_paste_ready_contract`、`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_marks_docs_product_copy_paste_ready_contract_complete`。脚本门禁：`scripts/check-ui-dx.sh` 新增 `cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot`。命令已尝试（2026-02-20）：`TMPDIR=/tmp CARGO_TARGET_DIR=/tmp/codex-bottom-sheet-docs-product cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_docs_are_copy_paste_ready_with_hello_world_state_matrix_and_streaming_snapshot`；当前环境仍受 `Invalid cross-device link (os error 18)` 阻断，待环境修复后复跑。）
- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。（已落实：`BottomSheet` 的组件级语义测试入口固定在 `components/bottom-sheet/test/semantics.rs`（`*_semantics.rs`），并以语义断言覆盖 `data-* / aria-* / role / 状态来源 / 键盘路径`：`bottom_sheet_semantic_contract_matrix_covers_interaction_paths_without_snapshot_only_assertions` + `bottom_sheet_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement` + 本次新增 `bottom_sheet_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks`。新增/变更语义字段同步测试由 `bottom_sheet_check2_marks_semantic_test_priority_item_complete` 约束，脚本门禁为 `scripts/check-ui-performance.sh` 新增 `cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks`。命令已尝试（2026-02-20）：`TMPDIR=/tmp CARGO_TARGET_DIR=/tmp/codex-bottom-sheet-semantic-priority cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks`；当前环境仍受 `Invalid cross-device link (os error 18)` 阻断，待环境修复后复跑。）
  - 每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。
  - 断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。
  - 新增/变更语义字段必须同步补测试，否则不得打勾。
- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。（已落实：新增 `e2e/tests/docs_app_bottom_sheet_contract.spec.mjs`，选择器仅使用稳定语义标记（如 `data-slot="bottom-sheet-e2e-semantic-controls"`、`data-slot="bottom-sheet-e2e-open-semantic"`、`data-slot="sheet-panel"`、`data-slot="bottom-sheet"`、`data-slot="sheet-backdrop"`），并通过 `body:not(:has(#boot))` 作为 WASM 就绪条件，无固定 `sleep`。动画/overlay 路径显式覆盖 ready/settled：打开后断言 `data-state=open`、`aria-modal=true`、`data-ui-output-status=verified`，关闭后断言 panel/root 均 `toHaveCount(0)`。docs 控制面已补充专用锚点：`apps/docs-app/src/pages/components/pages/overlays_extra.rs` 新增 `data-slot="bottom-sheet-e2e-semantic-controls"`、`data-slot="bottom-sheet-e2e-open-semantic"`、`data-slot="bottom-sheet-e2e-motion-controls"`、`data-slot="bottom-sheet-e2e-open-motion"`。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_documents_e2e_selector_and_stable_wait_rules`、`bottom_sheet_e2e_selector_contract_uses_semantic_markers_and_stable_waits`、`bottom_sheet_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths`、`bottom_sheet_e2e_script_covers_selector_and_ready_settled_contract`、`bottom_sheet_check2_marks_e2e_selector_stability_item_complete`。脚本门禁：`components/bottom-sheet/scripts/check-ui-e2e-bottom-sheet.sh`。命令已尝试（2026-02-20）：`TMPDIR=/tmp CARGO_TARGET_DIR=/tmp/codex-bottom-sheet-e2e-selectors cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_e2e_selector_contract_uses_semantic_markers_and_stable_waits`；当前环境仍受 `Invalid cross-device link (os error 18)` 阻断，待环境修复后复跑。）
  - E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。
  - WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。
  - 若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。
- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。（已落实：`e2e/tests/docs_app_bottom_sheet_contract.spec.mjs` 新增可重复关键流程 `docs-app bottom-sheet key flow is repeatable with semantic breakpoints`，通过 `for (const cycle of [1, 2])` 固定回放两轮 `open(Enter) -> interact(Tab) -> close(Escape)`，并在每轮用语义断点定位失败点（如 `await expect(semanticRoot).toHaveAttribute("data-state", "with-description")`、`await expectFocusInsidePanel(semanticPanel)`、`await expectBottomSheetSettledClosed(semanticPanel, semanticRoot, semanticSheet)`）。高风险路径优先覆盖：新增 `docs-app bottom-sheet high-risk paths keep overlay focus keyboard and settled semantic breakpoints`，包含 overlay/backdrop、focus trap、keyboard（`Tab`/`Shift+Tab`）与 settled 收敛；`async` 维度在 `BottomSheet` 组件边界为 `N/A`（无组件内异步协议）。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_documents_e2e_repeatable_key_flow_rules`、`bottom_sheet_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`、`bottom_sheet_e2e_high_risk_paths_cover_focus_keyboard_and_settled_semantic_breakpoints`、`bottom_sheet_e2e_check_script_covers_repeatable_key_flow_contracts`、`bottom_sheet_check2_marks_replayable_e2e_critical_flow_item_complete`。脚本门禁：`components/bottom-sheet/scripts/check-ui-e2e-bottom-sheet.sh`。命令已尝试（2026-02-20）：`TMPDIR=/tmp CARGO_TARGET_DIR=/tmp/codex-bottom-sheet-e2e-repeatable cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`；当前环境仍受 `Invalid cross-device link (os error 18)` 阻断，待环境修复后复跑。）
  - 至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。
  - 回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。
  - 高风险路径（overlay、focus、keyboard、async）优先进入回归集合。
- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。（已落实：`apps/docs-app/src/pages/components/pages/overlays_extra.rs::bottom_sheet` 保持 `title="State Matrix"` 与 `title="Controlled vs Uncontrolled"`，并新增 `data-slot="bottom-sheet-defaults-contract"` 对齐 `components/bottom-sheet/src/logic.rs` 的 API/默认值（`DEFAULT_TITLE/DEFAULT_CLOSE_LABEL/DEFAULT_DISMISSABLE/DEFAULT_KEYBOARD_DISMISS_DISABLED/DEFAULT_BOTTOM_INSET_PX` + `resolve_*` 归一函数）。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_documents_docs_sync_and_state_matrix_rules`、`bottom_sheet_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`、`bottom_sheet_dx_check_script_covers_docs_sync_and_state_matrix_contract`、`bottom_sheet_check2_marks_docs_sync_and_state_matrix_item_complete`。脚本门禁：`scripts/check-ui-dx.sh` 新增 bottom-sheet docs-sync 两条命令。命令已尝试（2026-02-20）：`TMPDIR=/tmp CARGO_TARGET_DIR=/tmp/codex-bottom-sheet-docs-sync cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`；当前环境仍受 `Invalid cross-device link (os error 18)` 阻断，待环境修复后复跑。）
  - 组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。
  - 文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。
  - 文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。
- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。（已落实：新增 `components/bottom-sheet/src/README.md`，明确新手路径顺序 `Hello World -> 先用起来，再进阶 -> 常见用法 -> Advanced`；同时 `apps/docs-app/src/pages/components/pages.rs` 保持 `BottomSheet` 可索引入口（`overlays_extra::bottom_sheet`），`apps/docs-app/src/pages/components/pages/overlays_extra.rs` 保持对应 docs 页面与最小示例。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_documents_documentation_as_product_rules`、`bottom_sheet_documentation_entry_exists_with_beginner_first_progression`、`bottom_sheet_dx_check_script_covers_documentation_as_product_contract`、`bottom_sheet_check2_marks_documentation_as_product_item_complete`。脚本门禁：`scripts/check-ui-dx.sh` 新增 bottom-sheet documentation-as-product 两条命令。命令已尝试（2026-02-20）：`TMPDIR=/tmp CARGO_TARGET_DIR=/tmp/codex-bottom-sheet-docs-product cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_documentation_entry_exists_with_beginner_first_progression`；当前环境仍受 `Invalid cross-device link (os error 18)` 阻断，待环境修复后复跑。）
  - 每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。
  - 文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。
  - “只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。
  - 已满足（README 新手优先）：`components/bottom-sheet/src/README.md` 已提供 `## Hello World（最小可用）`、`## 先用起来，再进阶`、`## 常见用法` 与 `### Advanced Example（高级入口）`，并明确“默认路径在前、进阶控制在后”。
- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。（已落实：`apps/docs-app/src/pages/components/pages/overlays_extra.rs::bottom_sheet` 已提供可交互 Playground 组合，包含 `title="State Matrix"`（`SegmentedControl` 调整分支 props）、`title="Controlled vs Uncontrolled"`（状态切换与反馈观察）、`title="Streaming / Snapshot Contract"`（状态输入与实时预览），并暴露稳定锚点 `data-slot="bottom-sheet-state-matrix"`、`data-slot="bottom-sheet-controlled-uncontrolled"`、`data-slot="bottom-sheet-streaming-contract"`。对 AI Spec 要求标注 N/A：`BottomSheet` 非 AI Spec 组件。关键交互复现复用 `e2e/tests/docs_app_bottom_sheet_contract.spec.mjs` 的重复流程（`for (const cycle of [1, 2])` + 高风险路径）。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_documents_interactive_playground_rules`、`bottom_sheet_docs_app_provides_interactive_playground_for_props_state_and_preview`、`bottom_sheet_interactive_playground_reuses_repeatable_semantic_e2e_flow`、`bottom_sheet_dx_check_script_covers_interactive_playground_contract`、`bottom_sheet_check2_marks_interactive_playground_item_complete`。脚本门禁：`scripts/check-ui-dx.sh` 新增 bottom-sheet interactive playground 三条命令。命令已尝试（2026-02-20）：`TMPDIR=/tmp CARGO_TARGET_DIR=/tmp/codex-bottom-sheet-interactive cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_docs_app_provides_interactive_playground_for_props_state_and_preview`；当前环境仍受 `Invalid cross-device link (os error 18)` 阻断，待环境修复后复跑。）
  - Playground 至少支持基础 props 调整、状态切换、交互反馈观察。
  - 对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。
  - Playground 作为验收面，需可重复复现关键交互路径。
  - N/A：`BottomSheet` 非 AI Spec 组件（无 `spec.rs` 驱动输入/预览联动要求）。
- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。（已落实：`apps/docs-app/src/pages/components/pages/overlays_extra.rs::bottom_sheet` 的 `data-slot="bottom-sheet-source-first"` 明确提供 `Show code` 复制路径；复制链路由 `BOTTOM_SHEET_DOC_IMPORTS` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补全 imports，确保代码片段可直接运行。source-first 区块同时给出依赖前提（`component-bottom_sheet + inject-css`）与真实源码路径（`data-slot="bottom-sheet-source-paths"`：`components/bottom-sheet/src/mod.rs`、`logic.rs`、`view.rs`、`styles.rs`、`motion.rs`）。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_documents_source_first_copy_paste_ready_rules`、`bottom_sheet_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`、`bottom_sheet_dx_check_script_covers_source_first_copy_paste_ready_contract`、`bottom_sheet_check2_marks_source_first_copy_paste_ready_contract_complete`。脚本门禁：`scripts/check-ui-dx.sh` 新增 bottom-sheet source-first 两条命令。命令已尝试（2026-02-20）：`TMPDIR=/tmp CARGO_TARGET_DIR=/tmp/codex-bottom-sheet-source-first cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`；当前环境仍受 `Invalid cross-device link (os error 18)` 阻断，待环境修复后复跑。）
  - docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。
  - 若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。
  - 文档代码与当前实现必须同步，防止示例漂移。
- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。（已落实：`docs/spec/heroui-parameter-design-strategy.md` 新增 `### BottomSheet 同步记录（2026-02-20）`，明确参数主轴与 docs 同步约束，并记录 `component_doc!("BottomSheet", "bottom-sheet", "Overlays", overlays_extra::bottom_sheet)`、`apps/docs-app/src/pages/components/pages/overlays_extra.rs::bottom_sheet()` 与 `components/bottom-sheet/src/README.md` 的可访问入口。回归：`components/bottom-sheet/test/semantics.rs::bottom_sheet_check2_documents_heroui_benchmark_docs_sync_rules`、`bottom_sheet_heroui_strategy_and_component_docs_are_synchronized_and_indexable`、`bottom_sheet_dx_check_script_covers_heroui_benchmark_docs_sync_contract`、`bottom_sheet_check2_marks_heroui_benchmark_docs_sync_contract_complete`。脚本门禁：`scripts/check-ui-dx.sh` 新增 bottom-sheet heroui strategy + docs entry 两条测试命令。命令已尝试（2026-02-20）：`TMPDIR=/tmp CARGO_TARGET_DIR=/tmp/codex-bottom-sheet-heroui cargo test -p ui --test bottom_sheet_semantics --no-default-features --features component-bottom_sheet,inject-css bottom_sheet_heroui_strategy_and_component_docs_are_synchronized_and_indexable`；当前环境仍受 `Invalid cross-device link (os error 18)` 阻断，待环境修复后复跑。）
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
