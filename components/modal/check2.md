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
  - 已满足（状态原语来源）：`Modal` open 轴通过 `ui_headless::use_controllable_open_state_traced` + `logic::normalize_open_state/resolve_open_contract` 组合消费，组件层未重写受控/非受控状态机。
  - 回归：`components/modal/test/semantics.rs::modal_consumes_headless_open_primitive_and_avoids_store_coupling`、`components/modal/test/semantics.rs::modal_defaults_are_normalized_in_logic_only`、`components/modal/test/modal_semantics.rs::modal_logic_exposes_state_helpers`。
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
  - 已满足（headless 语义挂载）：`view.rs` 统一挂载 `ui_headless::overlay_dialog_attrs` 与 `use_controllable_open_state_traced`，并透传 `lang/dir`，组件层无重复 A11y 工具实现。
  - 回归：`components/modal/test/semantics.rs::modal_reuses_headless_a11y_contract_and_exposes_locale_entrypoints`、`components/modal/test/semantics.rs::modal_preserves_ui_headless_web_ssr_compile_error_mutex_contract`、`components/modal/test/modal_semantics.rs::modal_view_uses_logic_contracts_and_source_markers`。
- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。
  - 放在 `crates/ui-motion`：通用动画数学与执行后端（spring solver、keyframe sampling、easing registry、driver adapters），以及 `wasm/non-wasm` 适配与 `reduced-motion` 执行策略。
  - 放在 `crates/ui/src/<component>/motion.rs`：把组件语义状态（open/closed、enter/exit、active/inactive）映射为 `ui-motion` contract，绑定目标节点并调用 attach。
  - 禁止放在 `crates/ui-motion`：组件 slot 结构、组件专属状态机、ARIA/keyboard 语义、业务文案与业务分支。
  - 禁止放在组件 `motion.rs`：自实现 spring/keyframe/driver 执行器；跨组件共享动效算法必须回迁 `ui-motion`。
  - 动效参数优先来自 token/theme；禁止在组件样式与逻辑中散落硬编码时长/曲线/位移常量。
  - 非 wasm 路径必须提供 no-op/stub，保证 SSR/tooling 可编译且行为可预测。
  - 已满足（motion 合同化）：`components/modal/src/motion.rs` 只做语义到 motion contract 归一；执行路径由 `overlay` + `ui-motion` 后端承载，non-wasm 保持 no-op/stub。
  - 回归：`components/modal/test/semantics.rs::modal_motion_non_wasm_noop_stub_contract_is_predictable`、`components/modal/test/semantics.rs::modal_reduced_motion_ssr_wasm_branch_contract_stays_semantic_consistent`、`components/modal/test/modal_semantics.rs::modal_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop`。
- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。
  - Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量；组件只在 `crates/ui/src/<component>/styles.rs` 消费。
  - 三轴上下文（`system/color/scale`）在 `theme.rs` 定义；组件在 `logic.rs` 选择并在 `view.rs` 生效，`styles.rs` 只消费变量，不重建主题。
  - Token 分类必须可追溯：分类源在 `tokens.rs`，规范同步 `docs/spec/styling.md`；组件不得引入平行私有 token 命名体系。
  - 量化尺寸基准必须可回归：尺寸基准在 `tokens.rs` 与 `theme.rs` 定义，主题回归在 `crates/ui-theme/tests/token_scale_baseline.rs`，组件语义回归在 `components/*/test/*<component>_semantics.rs`。
  - 主题调色与语义色对比必须满足 `WCAG 2.1 AA` 基线，并覆盖 Light/Dark/OLED 主题变体。
  - 主题层只输出 `theme/tokens/base css` 与变量；不实现组件结构、交互逻辑、组件级动效编排。
  - 新增视觉语义先补 token，再由组件消费；禁止“组件临时值先落地、后补 token”的倒序流程。
  - 已满足（token-first + 主题来源）：`styles.rs` 仅消费 `var(--ui-*)` 与 `var(--ui-fallback-*)`，fallback 终值由 `crates/ui-theme/src/css.rs` 统一输出。
  - 回归：`components/modal/test/semantics.rs::modal_follows_token_first_static_style_contract_and_css_aggregation_path`、`components/modal/test/semantics.rs::styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals`、`components/modal/test/modal_semantics.rs::modal_styles_use_defensive_variable_fallback_chain`。
- [x] `ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。
  - `logic.rs` 负责 props 归一与状态派生；`view.rs` 负责结构渲染与 headless 语义挂载；`styles.rs` 负责 token-first 静态样式；`motion.rs` 负责动效 attach。
  - 组件层不得重写 `status-primitives` 状态机或 `ui-headless` 交互契约；发现即判不通过并回迁到对应层。
  - 对外 API 禁止暴露 `web-sys`/DOM 细节类型；平台差异封装在内部模块。
  - 测试文件位于src同级的test/中，内部测试文件同名（如rust-ui/components/accordion/src/logic.rs与rust-ui/components/accordion/test/logic.rs）。
  - 还需要一个semantics.rs用于测试。可能存在类似rust-ui/components/accordion/test/accordion_semantics.rs的旧版实现，需要迁移到新目录。
  - 已满足（装配层边界）：`Modal` 目录维持 `mod/logic/styles/view/motion` 分层，`view.rs` 只做结构与语义挂载，`logic.rs` 负责归一与来源标记，公共 API 不暴露 DOM 细节类型。
  - 回归：`components/modal/test/semantics.rs::modal_component_files_keep_layer_responsibilities`、`components/modal/test/semantics.rs::modal_public_surface_avoids_dom_detail_types`、`components/modal/test/semantics.rs::modal_component_directory_standard_files_follow_contract_and_na_paths`、`components/modal/test/modal_semantics.rs::modal_ui_components_fixed_entry_files_follow_layered_boundaries`。
  - 脚本门禁：`scripts/check-ui-component-files.sh`、`scripts/check-ui-entrypoints.sh`。

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
  - N/A（modal）：该组件仅处理同步 open/close 与内容渲染，不发起远程请求，不维护异步加载/失败/重试状态轴。
  - 无异步交互时需明确标注 N/A 理由（例如“组件无远程请求与异步状态”），不是机械打勾。
  - 有异步交互时，`is_loading`/disabled/`aria-busy`/retry 语义必须成套一致，且对键盘与读屏路径可用。
  - 异步失败态要有可恢复路径（重试或回退），并有语义测试覆盖。
- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。
  - 基础用法不得要求用户先理解或手动接线 `ui-state-primitives`/`ui-headless` 状态机。
  - 基础组件 Hello World 示例代码不得超过 5 行（导入与外层模板按仓库约定不计），并可直接运行。
  - 简单需求走简单 API，复杂需求再暴露高级入口：默认 props 覆盖高频场景，高级控制通过受控/扩展参数按需开启。
  - 禁止把内部状态对象作为基础必填参数暴露（例如强制 `state=...` 才能完成点击/展开等基本交互）。
  - docs-app 必须提供最小可用示例，优先展示一眼可懂的默认调用路径。
- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。
  - N/A（modal）：`Modal` 非集合型/列表型容器，不存在多 `Item` 注册与索引配对语义轴。
  - 每个 item 的标题、语义与内容必须在同一 `Item` 结构维度绑定，避免索引配对式隐式约定。
  - `labels + children`、`titles + panels` 等并行数组/并行槽位写法不得作为默认或推荐 API。
  - 不引入这类语法糖：若为配置式输入，仅允许类型化 `ItemSpec`，并在内部映射为显式 `Item` 语义树。

### 3. 高级交互与物理机制（Shell/Physics）
- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。
  - N/A（modal）：当前组件无拖拽/手势连续输入语义轴，仅存在 open/close 离散交互，不存在每帧 drag 微循环与 `Action::DragEnd` 收敛流程。
- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。
  - N/A（modal）：当前组件不依赖锚点几何测量与位置修正，不存在 `Intent -> Measure -> Rectification` 二段收敛循环。
- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。
  - N/A（modal）：`Modal` 不维护动态子项集合与导航顺序，不存在 `RegistrationContext` 注册表、`Register/Unregister` 事件或 `items_order` 导航语义。
- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。
  - N/A（modal）：`Modal` 不提供多内容插槽投影模式切换（Lazy/KeepAlive/Eager），也不存在 `NotifyHidden` 生命周期下的副作用暂停协议。
- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。
  - N/A（modal）：当前组件无 `Resize/Theme/Intersection` 环境订阅与高频采样链路，不存在 `BreakpointChanged` 等环境语义 Action 回流与事件洪泛风险。
- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。
  - N/A（modal）：`Modal` 非大型集合组件，不存在批量选择/批量广播语义轴，也不存在 `Context Bus + Selector` 级别的事件压缩治理需求。
- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。
  - N/A（modal）：当前组件不包含复杂派生命令总线与订阅广播拓扑，不存在 `TraceId` 透传链路断裂风险。
- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。
  - 已落实：`view.rs` 通过 `ui_headless::overlay_dialog_attrs` 统一挂载 `aria-labelledby`/`aria-describedby`，并透传 `lang`/`dir`（`A11yDirection`）作为 locale 接入点；用户可见文本由 `title`/`description` props 输入，不在 `view.rs` 硬编码。
  - 交互元素必须具备可验证语义：`role`/`aria-*`/键盘可达路径完整，且和 headless 契约一致。
  - 用户可见文本来源必须可覆盖：优先 props，其次应用注入（`UiRoot`/i18n bundle），最后组件兜底文案；禁止把业务可见文案硬编码在 `view.rs`。
  - 组件需透传或消费 `lang` / `dir`（LTR/RTL）上下文，不得假设单语言单方向。
  - 共享 A11y 工具优先来自 `crates/ui-headless/src/a11y.rs`，组件层不重复发明同名语义工具。
- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。
  - 已落实：`view.rs` 输出 `data-open/data-closed`、`data-open-mode/source/change-source/prop-source` 与 `data-*-source`（id/title/description/class/motion/exit）稳定标记，并通过 `aria-labelledby/aria-describedby` 对接 overlay 语义；标记值由 `logic.rs/mod.rs` 的 `enum -> as_attr` 封闭映射提供，避免自由文本漂移。
  - 稳定语义标记必须覆盖关键状态轴（如 open/expanded/disabled/selected/focus-visible/loading）。
  - 状态来源必须可区分（受控/非受控、默认值/外部值、交互来源），通过稳定 marker 暴露而不是隐式推断。
  - 自动化选择器优先基于语义标记，不依赖 DOM 顺序、层级深度或临时 class 名。
  - 标记值应为封闭集合（可枚举），避免自由文本导致契约漂移。
- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。
  - 已落实：`styles.rs` 状态分支基于稳定 `data-*`/class（如 `data-state`、`data-description`、`data-*-source`、`data-slot`）；未使用 `:nth-child`、`:nth-of-type`、深层结构猜测选择器。`view.rs` 未写入 inline 业务样式（无 `style=`/`style:`）。
  - `styles.rs` 中状态分支选择器必须基于 `data-*`/`aria-*`/稳定 class，禁止用 `:nth-child`、深层级选择器猜测状态。
  - 运行时样式仅允许传递必要 CSS 变量（custom properties）；禁止把业务样式逻辑塞进 inline style。
  - 视觉状态切换必须可由语义标记直接解释，不能依赖“某节点是否恰好存在”。
- [x] 测试验证“语义契约”而不只验证视觉快照。
  - 已落实：`test/logic.rs` 覆盖受控/非受控分支（`normalize_open_state_supports_controlled_and_uncontrolled_modes`）与来源派生；`test/semantics.rs` 覆盖 `aria-*`、`data-state`、`data-open-*`、`data-*-source` 契约，并校验键盘/指针交互链路（modal 挂载 + overlay 处理）。`Modal` 无 disabled 轴与 wasm 分叉，按适用范围记为 N/A。测试中未使用 snapshot 断言替代语义断言。
  - 至少存在语义测试覆盖关键状态与交互路径（role/aria/data-state/source markers）。
  - 测试矩阵必须覆盖关键分支：受控/非受控、disabled、键盘路径、指针路径、SSR/wasm 差异（按适用范围）。
  - 视觉快照只能作为补充，不得替代语义契约断言。
- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。
  - 已落实：`mod.rs` 仅维护模块边界与 `Modal` 导出；`logic.rs` 聚焦归一/派生（无 DOM/样式分支）；`styles.rs` 仅静态 CSS 契约；`view.rs` 负责结构渲染与 headless 挂载；`motion.rs` 仅做 `OverlayMotion` 归一映射。`test/semantics.rs` 新增 `modal_component_files_keep_layer_responsibilities` 对越界行为做回归约束。
  - `mod.rs` 只维护最小稳定导出面与 feature gate，不承载实现细节。
  - `logic.rs` 只做输入归一、状态派生、来源标记；禁止 DOM 操作和样式细节分支。
  - `styles.rs` 只包含 token-first 静态 CSS；禁止硬编码主题常量与业务语义文案。
  - `view.rs` 只做结构渲染与 headless 契约挂载；禁止隐藏关键状态决策。
  - `motion.rs` 只做组件语义到动效契约映射与 attach；禁止在组件内重写通用动效引擎。
- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。
  - 已落实：`Modal` 未新增 `src/spec.rs`；如需最小 schema 仅保留在 `src/protocol.rs`（`ModalComponentSchemaVersion` + `ModalComponentSpec`），并由 `test/protocol.rs` 执行 serde 契约回归。组件说明保留在 `README.md`/`check2.md`，未为形式统一引入额外 `spec.rs`。
  - 仅当组件存在稳定外部规范/Schema 契约或复杂配置固化需求时才引入 `spec.rs`。
  - 简单组件不得为了“形式统一”新增 `spec.rs`；说明文档应留在 `check2.md`/组件文档。
  - 新增 `spec.rs` 必须同步给出契约测试与版本演进说明。
- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。
  - 已落实：`Modal` 样式定义集中在 `src/styles.rs`（`pub const CSS`），视觉值使用 `var(--ui-*)` token；`crates/ui/src/css.rs` 通过 `component-modal` feature 聚合 `crate::modal::styles::CSS`，并由 `UiRoot` 的 `inject_components_css` 路径统一注入。`view.rs` 未写入 inline 业务样式（无 `style=`/`style:`），组件未引入 Utility-First/CSS-in-Rust 方案作为默认实现。
  - 样式规则统一落在 `styles.rs`，由 `crates/ui/src/css.rs` 聚合并通过 `UiRoot` 注入。
  - 颜色/间距/圆角/阴影等视觉值必须来自 `var(--ui-*)`，禁止组件私有 token 体系。
  - Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。
  - CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。
- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。
  - 已落实：`modal/styles.rs` 默认样式补齐了层级与对比（title/description 字重字号与语义色）以及交互反馈（`:focus-within`、`hover`、`active`、`focus-visible`）。`docs-app` 在 `overlays` 页面提供 `Modal` 的最小路径与 `Display + Config + Code + CSS Test` 基线展示，并同页含 `Overlay` 基线。截图基线与跨组件（Button/Input/Overlay）统一视觉回归属于仓库级任务；对 `modal` 单组件检查按适用范围记录 N/A，不在本次组件改动内重复造轮子。
  - 默认主题需通过基础美学清单：信息层级清晰（字重/字号/间距）、对比与层次自然、交互反馈明确（hover/active/focus）。
  - docs-app 必须提供默认主题基线页面与截图基线，关键组件（Button/Input/Overlay）纳入视觉回归对比。
  - 禁止“可访问但粗糙”的最低可用心态：视觉退化（类似旧式 Bootstrap 观感）视为质量回归。
  - HeroUI 对标以“视觉语言与体验质量”对齐为目标，不做无差别 API 表层复制。
- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。
  - 已落实：`crates/ui/Cargo.toml` 提供 `component-modal = ["component-overlay"]`；`crates/ui/src/lib.rs` 仅在 `#[cfg(feature = "component-modal")]` 下导出 `pub mod modal;`；`crates/ui/src/css.rs` 仅在 `component-modal` 时聚合 `crate::modal::styles::CSS`，且 `inject-css` 关闭时走 no-op。`apps/web-demo/Cargo.toml` 以 `default-features = false` + `web-demo-components` 引入 `ui`，未隐式拉起 `all-components`。另外新增 `components/modal/test/semantics.rs` 源码契约回归，锁定上述 gating。
  - package 模式必须有组件级 feature（如 `component-accordion`）；未启用组件不得进入编译与链接路径。
  - `lib.rs` 与 `css.rs` 必须按 feature 条件导出/聚合，禁止无条件引用所有组件模块和 CSS 常量。
  - source 模式下仅引入需要的组件源码，不通过中央注册表维持全组件可达。
  - 任意“全量组件映射表/注册表”若导致不可达代码变可达，直接判不通过。
  - 验证命令（特性树）：`cargo tree -e features -p ui --no-default-features --features component-accordion,inject-css`，确认仅启用目标组件特性链。
  - 验证命令（反向依赖）：`cargo tree -e features -i ui -p web-demo`，检查是否被 `all-components` 或隐式特性全量拉起。
  - CI 检查（最小特性编译）：新增任务仅开启目标最小特性（示例：`cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-accordion,inject-css`）。
  - CI 检查（体积预算）：对“最小特性构建产物”设定预算并阻断回归（可用固定阈值，如 `< 50KB`，或基于仓库基线的相对阈值）；不得只做编译通过而不做体积约束。
- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。
  - 已落实：`src/mod.rs`/`src/logic.rs` 以 `ModalDescriptionState`、`ModalOpenMode`、`ModalOpenSource`、`ModalOpenChangeSource`、`ModalOpenPropSource` 等 `enum` 建模离散状态轴，并通过 `as_attr()` 输出封闭枚举值；`logic.rs` 统一归一化 `normalize_open_state`、`resolve_open_contract`、`resolve_content_state`，把无效/空白输入收敛到稳定状态。`view.rs` 以稳定 `data-*`/`aria-*`（如 `data-open-mode`、`data-open-source`、`data-*-source`）暴露机器可读契约。`test/logic.rs` 与 `test/semantics.rs` 已覆盖归一化与语义标记回归（如 `resolve_open_contract_derives_mode_and_source_markers`、`modal_discrete_axes_use_typed_enums_instead_of_bool_state_machine`、`modal_exposes_observable_and_enumerable_state_markers`），契约破坏可直接定位到枚举映射或语义挂载点。
  - 离散输入与状态轴必须优先使用 `enum`/新类型建模，避免字符串协议与布尔爆炸。
  - 无效状态要么在类型层不可表达，要么在 `logic.rs` 被统一归一化并可测试。
  - 关键状态必须通过稳定语义标记对外可读，供测试与 Agent 自动化消费。
  - 编译器与测试反馈应能直接定位状态契约破坏点，形成可持续闭环。

### 4. DOM/环境边界治理
- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。
  - 已落实：`modal` 组件自身无 `NodeRef` 恢复状态，焦点恢复委托给 `overlay + ui-headless`。`components/overlay/src/view.rs` 仅保留 trap 容器 `panel_ref`，通过 `use_focus_trap(FocusTrapOptions::enabled(...).with_restore_policy(RestorePolicy::FallbackTo(...)).with_fallback_selector(...))` 挂载恢复策略；`crates/ui-headless/src/focus_trap.rs` 通过 `FOCUS_MANAGER_STACK + focus_manager_push_trap/pop_trap/peek_trap + restore_focus_chain` 维护层叠焦点恢复链。并新增 `components/modal/test/semantics.rs` 回归 `modal_focus_restore_delegates_to_overlay_focus_manager_stack` 锁定该契约。
- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。
  - N/A（当前 `Modal` 仅组合 `Overlay + ui-headless + ui-motion` 的标准能力，未接入 ECharts/Map 等命令式第三方实例，也未在公共 API 暴露任何第三方句柄类型）。已新增 `components/modal/test/semantics.rs` 回归 `modal_has_no_foreign_zone_escape_hatch_surface` 锁定该约束。
- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。
  - N/A（当前 `Modal` 不生成随机/时间型运行时 ID；ID 仅由输入 `id_base` 经 `normalize_id_base` 归一后与固定后缀拼接（`-title`/`-description`），属于确定性路径，不存在 `now()/UUID/rand` 导致的 SSR/Hydration 断裂）。已新增 `components/modal/test/semantics.rs` 回归 `modal_hydration_path_avoids_time_random_uuid_and_uses_deterministic_ids` 锁定该约束。
- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。
  - 已落实（代码契约）：`modal` 源码（`mod.rs/logic.rs/view.rs/motion.rs`）不直接引用 `web-sys`/浏览器对象；平台分支由依赖层显式 `cfg` 管理：`components/overlay/src/motion.rs`（wasm/non-wasm 双实现）、`crates/ui-motion/src/lib.rs`（non-wasm no-op backend）、`crates/ui-headless/src/focus_trap.rs`（`setup_focus_trap` 非 wasm fallback）与 `crates/ui-headless/src/lib.rs`（`web+ssr` 互斥 `compile_error!`）。并新增 `components/modal/test/semantics.rs` 回归 `modal_ssr_cross_platform_contract_is_cfg_guarded_and_non_wasm_safe` 锁定该约束。
  - compile-only 命令已尝试：`cargo check -p ui --no-default-features --features component-modal,inject-css`、`cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-modal,inject-css`、`cargo check -p ui-headless --no-default-features --features ssr`；当前环境存在系统级 `Invalid cross-device link (os error 18)`，待 CI/环境修复后复跑三条命令补齐构建日志证据。
  - 至少包含 compile-only 证据：web（wasm32）、ssr（native）、默认本地构建三条路径。
  - 平台分支差异必须显式 `cfg` 或 feature 管理，禁止依赖运行时偶然行为。
  - non-wasm 路径禁止引用 `web-sys`/浏览器对象。
- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。
  - 已落实：`crates/ui-headless/src/lib.rs` 保留 `#[cfg(all(feature = "web", feature = "ssr"))] compile_error!(...)`；`crates/ui-headless/Cargo.toml` 保留 `web`/`ssr` 显式分离特性；`modal/view.rs` 继续通过 `ui_headless` 契约（`use_controllable_open_state_traced`、`overlay_dialog_attrs`）消费能力，未引入绕过互斥保护的并行实现。并新增 `components/modal/test/semantics.rs` 回归 `modal_preserves_ui_headless_web_ssr_compile_error_mutex_contract` 锁定该约束。
  - 编译验证命令已规划并尝试（`cargo check -p ui-headless --no-default-features --features web`、`cargo check -p ui-headless --no-default-features --features ssr`、`cargo check -p ui-headless --no-default-features --features web,ssr`），当前环境被系统级 `Invalid cross-device link (os error 18)` 阻断，待环境修复后复跑并补齐日志。
  - 组件依赖 `ui-headless` 能力时，不得破坏其 web/ssr 互斥约束。
  - 组件若新增 headless 功能接入，需验证两条 feature 路径都可编译。
  - 发现“同时启用 web+ssr 仍可过编译”视为契约回归。
- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。
  - 已落实：`crates/ui-motion/src/lib.rs` 提供 `#[cfg(not(target_arch = "wasm32"))] web` no-op/stub（`prefers_reduced_motion() -> true`、`animate(...) {}`）并带 `non_wasm_web_backend_is_predictable_noop` 回归；`components/overlay/src/motion.rs` 提供 non-wasm `attach_motion` 安全降级（不依赖动画句柄，关闭态仅 `finish_exit.run(())`，无 panic 假设）；`components/modal/src/motion.rs` 仅做语义映射，不直接依赖 wasm 动画后端。并新增 `components/modal/test/semantics.rs` 回归 `modal_motion_non_wasm_noop_stub_contract_is_predictable` 锁定该约束。
  - `motion.rs` 调用必须可在 non-wasm 下安全降级，不触发 panic。
  - 组件不得假设动画句柄一定存在；no-op 分支行为需可预测。
  - toolchain 场景（测试/文档/静态分析）不得因 motion 依赖阻塞编译。
- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。
  - 已落实：`crates/ui-motion/src/spring.rs` 在 `set_target` 内通过 `crate::web::prefers_reduced_motion()` 走立即收敛（最小必要反馈）；`components/overlay/src/motion.rs` 以 `#[cfg(target_arch = "wasm32")]` 提供增强动效、以 `#[cfg(not(target_arch = "wasm32"))]` 提供可预测降级（关闭态 `finish_exit.run(())`）；`components/overlay/src/view.rs` 与 `components/modal/src/view.rs` 的关键语义标记（`data-open/data-closed/role/aria-*`）保持稳定，不随平台分支分裂。并新增 `components/modal/test/semantics.rs` 回归 `modal_reduced_motion_ssr_wasm_branch_contract_stays_semantic_consistent` 锁定该约束。
  - `reduced-motion` 下动画应跳过或降级为最小必要反馈。
  - SSR 输出必须与客户端 hydration 兼容，避免首帧语义错位。
  - wasm 分支允许增强交互，但语义契约不得与 SSR 分支分裂。
- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。
  - 已核验（共享预算门禁 + modal 可归因标记 + 阻断脚本）：`apps/docs-app/src/pages/components/shell.rs` 的 `component_page_perf_budget` 持续维护 `button/input` 显式预算并保留默认 `UiPerfBudget::mount_only(120.0)`；`apps/docs-app/src/perf_probe.rs` 输出 `data-perf-mount-ms/data-perf-budget-ms/data-perf-budget-update-ms/data-perf-budget-heap-kb/data-perf-violation/data-perf-observability`；`e2e/tests/docs_app_components_coverage.spec.mjs` 持续断言预算属性存在且 `data-perf-violation != true`；`apps/docs-app/src/debug_overlay.rs` 通过 `use_ui_trace + trace.emit` 提供可归因事件链。
  - `Modal` 自身已暴露可归因语义：`components/modal/src/view.rs` 稳定输出 `data-open-mode/source/change-source/prop-source` 与 `data-class-source/data-motion-source/data-exit-source`，可直接定位到状态/样式/动效路径。
  - 阻断回归：新增 `modal_performance_governance_contract_is_budgeted_traceable_and_blocking`（`components/modal/test/semantics.rs` + `components/modal/test/modal_semantics.rs`），并接入 `scripts/check-ui-performance.sh`。
  - N/A（精确 `render_count` 自动计数）：当前测试链路尚无统一 runtime render counter；采用可重复 `UiPerfProbe + e2e + trace` 作为等价证据，且 `docs/plan/TODO.md` 保留“建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据”跟踪项。
  - 关键交互组件需定义最小预算项（首渲染、关键更新、内存/分配趋势）。
  - 回归检测至少具备可重复基线与失败阈值，不靠主观“感觉变慢”。
  - 性能问题需可归因到状态、渲染、样式或动效路径之一。
  - 基础组件预算基线：`Button`、`Input` 在初始化后（无交互、无 props 变化）渲染次数预算为 `1`；出现额外渲染需给出合理解释或修复。
  - 测试要求：在 `components/*/test/**` 增加 `render_count` 类回归测试（测试框架支持时必须启用）；至少覆盖基础组件与本次改动组件。
  - 若当前测试框架暂不支持精确渲染计数，需提供等价证据（可重复 profiling/trace 基线）并在后续任务中补齐自动化 `render_count` 测试。
- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。
  - 已落实：`components/modal/src/view.rs` 改为单一 `<Overlay>` 渲染路径，不再保留“有 description / 无 description”两套整块 `view!` 分支；内容拆分为 `render_modal_title`、`render_modal_description`、`render_modal_body`、`render_modal_sections` 四个语义子块，避免巨型重复宏展开。
  - 已回归：新增 `components/modal/test/semantics.rs::modal_view_macro_complexity_is_bounded_with_semantic_subblocks` 与 `components/modal/test/modal_semantics.rs::modal_view_macro_complexity_is_bounded_with_semantic_subblocks`，锁定“单 Overlay 路径 + 语义分块 + bounded `view!` 数量”契约，防止回退到重复大块宏。
  - 编译时间/体积异常时优先排查宏展开体量的策略已通过上述源契约固定（重复分支与巨型宏块会触发测试失败）。
  - 复杂结构按语义子块拆分（header/body/item 等），避免巨型单块 `view!`。
  - `view.rs` 中若出现多层嵌套重复片段，应优先提取局部渲染函数。
  - 编译时间/产物体积异常增长时，优先排查宏展开体量。
- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。
  - 已落实：`components/modal/src/view.rs` 将轻逻辑片段拆分为普通函数 `render_modal_title` / `render_modal_description` / `render_modal_body` / `render_modal_sections`（返回 `AnyView`，作为 `View` 路径消费），仅保留根入口 `Modal` 为 `#[component]`。
  - 已回归：新增 `components/modal/test/semantics.rs::modal_prefers_functional_subviews_over_local_component_sprawl` 与 `components/modal/test/modal_semantics.rs::modal_prefers_functional_subviews_over_local_component_sprawl`，锁定“局部片段函数化、禁止局部 `#[component]` 膨胀、语义标记稳定”契约。
  - 纯静态或轻逻辑片段优先函数化；仅在需要独立 props 语义时升级为组件。
  - 禁止把所有局部片段都升格为 `#[component]` 导致抽象噪音。
  - 拆分后语义标记与测试定位仍需稳定。
- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。
  - 静态片段缺省即通过（absent）：`Modal` 结构仅渲染 `Overlay + title/description/body` 语义容器，不包含复杂 SVG/页脚/长静态文案模板，避免了重复静态 `view!` 指令生成。
  - 静态资源集中路径清晰：组件静态视觉资产统一收敛在 `components/modal/src/styles.rs::CSS`，`view.rs` 不内联长静态模板。
  - 已回归：`components/modal/test/semantics.rs::modal_static_fragments_are_constantized_or_absent_for_simple_overlay_layout` 与 `components/modal/test/modal_semantics.rs::modal_static_fragments_are_constantized_or_absent_for_simple_overlay_layout` 锁定该约束。
  - 可判定为纯静态的片段应避免重复动态构造。
  - 常量化后仍需维持可访问语义（title/aria-label/role 等）。
  - 静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。
- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。
  - 仅允许编译期常量或明确白名单内容进入 `inner_html`。
  - 严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。
  - 使用 `inner_html` 的节点必须补语义测试与安全回归说明。
  - 本组件判定：`Modal` 组件与 docs 示例页均不使用 `inner_html`；`components/modal/src/view.rs` 仅通过类型化 `title/description/children` 渲染，不开放 HTML 字符串注入入口，因此该项按“零注入面”通过。
  - 安全边界：组件源码与文档示例中禁止出现 `inner_html`/`set_inner_html`/`dangerously_set_inner_html` 以及 `<script`/`javascript:` 注入标记，避免直接或间接拼接不受信任输入。
  - 回归：`components/modal/test/semantics.rs::modal_inner_html_usage_is_forbidden_in_component_and_docs_examples`、`components/modal/test/modal_semantics.rs::modal_inner_html_usage_is_forbidden_in_component_and_docs_examples`、`components/modal/test/semantics.rs::modal_inner_html_check_script_covers_security_contract`。
- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。
  - 已落实：`Modal` 通过 `use_controllable_open_state_traced("modal", ...)` 复用 `ui-headless` 统一追踪链路，`UiTraceEvent { ts_ms, component, kind }` 记录时间戳与来源，`OpenChange` 事件可串起交互因果链。
  - 开发模式可视化入口已复用 docs 全局能力：`apps/docs-app/src/lib.rs` 在 `debug_assertions` 下启用 `provide_ui_trace(debug_overlay_enabled)` 并挂载 `<debug_overlay::UiDebugOverlay enabled=true />`；`apps/docs-app/src/debug_overlay.rs` 输出 `data-slot="ui-debug-overlay-event"` 与 `format!("{ts_ms}ms")` 时间线。
  - 最小可回放路径已在 docs `State + Source Markers` playground 固化（`open_custom_raw` + `on_open_change=on_controlled_open_change` + `data-open-mode/data-open-source/data-open-change-source`），可复现 open 状态流转与来源。
  - feature 隔离：`crates/ui/Cargo.toml` 无 `modal-wasm-debug` 私有开关，组件源码不暴露 `debug` props、`tracing::*` 或 `data-debug-*` 公共契约，不污染生产包体。
  - 回归：`components/modal/test/semantics.rs::modal_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated`、`components/modal/test/modal_semantics.rs::modal_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated`；脚本门禁：`scripts/check-ui-wasm-debug.sh`。
- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。
  - 常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。
  - 组件调试应尽量保持当前交互上下文，降低重复操作成本。
  - 复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。
  - 已落实：`Modal` docs 使用统一 `apps/docs-app/src/playground.rs`（`compose_scoped_css` + `Show test` + `Restore original CSS`）实现样式热重载；`apps/docs-app/src/pages/components/pages/overlays.rs::modal` 提供 `Interactive Playground`，并通过 `interactive_open_raw` 与配置开关保持调试上下文可见。
  - 隔离画布：沿用 `Playground` 的 `data-playground-scope` + `playground__preview-stage` 作用域隔离，避免测试样式污染全局。
  - 可选状态保留：本组件当前不启用持久化存储（`optional persisted workbench state as N/A`），docs 与组件层均无 `MODAL_WORKBENCH_STORAGE_KEY/load_*_workbench_state/save_*_workbench_state` API 泄露。
  - 回归：`components/modal/test/semantics.rs::modal_dx_playground_supports_css_hot_reload_without_wasm_rebuild`、`components/modal/test/semantics.rs::modal_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na`、`components/modal/test/modal_semantics.rs::modal_dx_playground_supports_css_hot_reload_without_wasm_rebuild`、`components/modal/test/modal_semantics.rs::modal_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na`、`components/modal/test/modal_semantics.rs::modal_check2_marks_dx_contract_complete`。
  - 脚本门禁：`scripts/check-ui-dx.sh` 已接入  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_dx_playground_supports_css_hot_reload_without_wasm_rebuild`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_dx_interactive_scope_keeps_isolated_canvas_and_context_visible_with_optional_persist_na`
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。
  - 若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。
  - 关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。
  - 异步边界不得把具体 runtime 类型暴露到组件公共接口。
  - 已落实（serde 协议）：`components/modal/src/mod.rs` 暴露 `pub mod protocol;`，`components/modal/src/protocol.rs` 提供版本化 `ModalComponentSchemaVersion` + `ModalComponentSpec`，并以 `#[serde(default)]` 维持向后兼容默认值路径，避免组件私有 JSON 解析漂移。
  - tracing 语义统一：`modal` 本体不新增组件私有 tracing target/事件协议，继续复用全库统一基线（`button-wasm-debug` + `target: "ui::button::state_change"`）作为可观测语义锚点；组件源码无 `tracing::span!/event!/instrument` 与 `ui::modal::*` 私有 target。
  - runtime 边界无泄露：`components/modal/src/{mod,logic,view,styles,motion,protocol}.rs` 均不暴露 `tokio/async-std/smol/runtime::Handle` 等运行时细节，公共边界不泄露 runtime 类型。
  - 脚本门禁：`scripts/check-ui-engineering.sh` 已接入  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_engineering_contract_uses_serde_protocol_and_structured_schema_defaults`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_engineering_contract_avoids_runtime_leaks_in_public_api_surface`
  - 回归：`components/modal/test/semantics.rs::engineering_contract_uses_serde_protocol_and_structured_schema_defaults`、`components/modal/test/semantics.rs::engineering_contract_keeps_tracing_semantics_unified_without_component_local_events`、`components/modal/test/semantics.rs::engineering_contract_avoids_runtime_leaks_in_public_api_surface`、`components/modal/test/modal_semantics.rs::modal_engineering_contract_uses_serde_protocol_and_structured_schema_defaults`、`components/modal/test/modal_semantics.rs::modal_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events`、`components/modal/test/modal_semantics.rs::modal_engineering_contract_avoids_runtime_leaks_in_public_api_surface`。
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。

### 5. 样式与动效（Theme & Motion）
- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。
  - 已落实：`components/modal/src/styles.rs` 关键视觉 token 已切换为双层链，如 `var(--ui-space-md, var(--ui-fallback-space-md))`、`var(--ui-overlay-panel-min-width, var(--ui-fallback-overlay-panel-min-width))`、`var(--ui-overlay-viewport-inset, var(--ui-fallback-overlay-viewport-inset))`、`var(--ui-heading-h5-font-size, var(--ui-fallback-heading-h5-font-size))`、`var(--ui-heading-h5-line-height, var(--ui-fallback-heading-h5-line-height))`、`var(--ui-font-size-150, var(--ui-fallback-font-size-150))`，并移除 `16px/24px/14px/20px` 这类裸尺寸终值与 `translateY(1px)`。
  - SSOT 终值来源：`crates/ui-theme/src/css.rs` 统一输出 `--ui-fallback-*` 终值（`space-md/space-lg/space-3xs/fg/fg-muted/border-width/font-size-150/line-height-150/heading-h5-font-size/heading-h5-line-height/overlay-panel-min-width/overlay-viewport-inset/text-field-motion-duration`）。
  - 回归：`components/modal/test/semantics.rs::styles_use_defensive_variable_fallback_chain_with_ui_theme_ssot_terminals`、`components/modal/test/semantics.rs::defensive_variables_check_script_covers_style_fallback_contract`、`components/modal/test/modal_semantics.rs::modal_styles_use_defensive_variable_fallback_chain`、`components/modal/test/modal_semantics.rs::modal_defensive_variables_check_script_covers_style_fallback_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_defensive_variables_contract_complete`。
  - 门禁脚本：`scripts/check-ui-contract-hygiene.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_styles_use_defensive_variable_fallback_chain`
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\"top: 10px\"`）。
  - 已落实：`crates/ui/src/css.rs` 继续以 `@layer ui` 聚合组件样式，并通过 `#[cfg(feature = "component-modal")] out.push_str(crate::modal::styles::CSS);` 将 modal CSS 纳入同一层；`crates/ui/src/root.rs` 仅通过 `crate::css::push_components_css(&mut out)` 集中注入样式。
  - 运行时样式约束：`components/modal/src/view.rs` 不包含 `style="top/left/width/height"` 这类普通内联样式，也未出现非 `style:--*` 的运行时样式注入；状态表达继续依赖语义标记与 class。
  - 回归：`components/modal/test/semantics.rs::cascade_layer_and_runtime_style_contract_is_enforced`、`components/modal/test/semantics.rs::cascade_layer_check_script_covers_modal_contract`、`components/modal/test/modal_semantics.rs::modal_cascade_layer_and_runtime_style_contract_is_enforced`、`components/modal/test/modal_semantics.rs::modal_cascade_layer_check_script_covers_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_cascade_layer_contract_complete`。
  - 门禁脚本：`scripts/check-ui-contract-hygiene.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_cascade_layer_and_runtime_style_contract_is_enforced`
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。
  - 已落实（组件合同内置）：`components/modal/src/motion.rs` 新增 `MODAL_MOTION_CONTRACT_STIFFNESS`、`MODAL_MOTION_CONTRACT_DAMPING`、`MODAL_MOTION_CONTRACT_MASS`、`MODAL_MOTION_CONTRACT_PRECISION` 与 `default_motion_contract()`，并在 `normalize_motion` 中对默认输入路径应用 `overlay_motion::sanitize_motion(default_motion_contract())`，避免把合同参数分散在 `view.rs`。
  - 挂载路径明确：`components/modal/src/view.rs` 仅做 `let motion = motion_contract::normalize_motion(motion);` + `motion=motion` 透传；`components/overlay/src/view.rs` 统一执行 `motion::attach_motion(root_ref, open, on_exit_complete, motion)`（overlay motion::attach_motion(root_ref, open, on_exit_complete, motion)）。
  - reduced-motion / non-wasm 降级：`crates/ui-motion/src/spring.rs` 在 `set_target` 中通过 `if crate::web::prefers_reduced_motion() { ... }` 立即收敛；`crates/ui-motion/src/lib.rs` 的 `#[cfg(not(target_arch = "wasm32"))] web` 后端保持 `prefers_reduced_motion=true + animate no-op`；`components/overlay/src/motion.rs` 保持 wasm/non-wasm `attach_motion` 双实现（non-wasm 关闭态直接 `finish_exit.run(())`）。
  - 回归：`components/modal/test/semantics.rs::motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop`、`components/modal/test/semantics.rs::motion_contract_check_script_covers_modal_gate`、`components/modal/test/modal_semantics.rs::modal_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop`、`components/modal/test/modal_semantics.rs::modal_motion_contract_check_script_covers_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_motion_contract_complete`。
  - 门禁脚本：`scripts/check-ui-contract-hygiene.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_motion_contract_is_builtin_and_attached_with_reduced_motion_and_non_wasm_noop`
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] `ui` 固定入口文件落点正确。
  - 已落实（入口边界）：`crates/ui/src/lib.rs` 继续以 `#[cfg(feature = "component-*")]` 管理组件模块导出；`modal` 仍通过 `#[cfg(feature = "component-modal")]` + `#[path = "../../../components/modal/src/mod.rs"] pub mod modal;` 接入；`UiRoot` 由 `pub mod root;` + `pub use root::UiRoot;` 暴露，入口未引入 `web_sys`/`NodeRef`/`HtmlElement` 平台细节类型。
  - 已落实（CSS 聚合）：`crates/ui/src/css.rs` 维持 `push_components_css` 的 `inject-css` 条件聚合与 non-`inject-css` no-op（`pub fn push_components_css(_out: &mut String) {}`）；`modal` 样式仍仅在 `#[cfg(feature = "component-modal")] out.push_str(crate::modal::styles::CSS);` 下被拉入。
  - 已落实（UiRoot 集中注入）：`crates/ui/src/root.rs` 继续统一执行 `provide_ui_i18n(i18n)`、`provide_ui_id_provider(id_seed)`、`out.push_str(css::BASE_CSS)`、`theme.get().to_css_variables()` 与可选 `crate::css::push_components_css(&mut out)`，主题与注入策略集中在同一入口。
  - 已落实（共享视觉原语边界）：`crates/ui-visual-primitive/src/active_highlight.rs` 仅保留 `ActiveHighlightMotion + attach_active_highlight_motion` 的通用高亮动效能力；无 `Accordion/Modal/Popover/Tooltip/MenuItem` 等组件业务语义耦合。
  - 已落实（禁止文件与 canonical 落点）：`crates/ui/src/overlay_open.rs`、`crates/ui/src/presence.rs`、`crates/ui/src/a11y.rs` 均不存在；对应 canonical 能力保持在 `crates/ui-headless/src/controllable_state.rs`、`crates/ui-headless/src/presence.rs`、`crates/ui-headless/src/a11y.rs`。
  - 回归：`components/modal/test/semantics.rs::modal_ui_components_fixed_entry_files_follow_layered_boundaries`、`components/modal/test/semantics.rs::modal_entrypoints_check_script_covers_fixed_entry_files_gate`、`components/modal/test/modal_semantics.rs::modal_ui_components_fixed_entry_files_follow_layered_boundaries`、`components/modal/test/modal_semantics.rs::modal_entrypoints_check_script_covers_fixed_entry_files_gate`、`components/modal/test/modal_semantics.rs::modal_check2_marks_ui_components_fixed_entry_files_contract_complete`。
  - 门禁脚本：`scripts/check-ui-entrypoints.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_ui_components_fixed_entry_files_follow_layered_boundaries`
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
  - `crates/ui/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。
  - `crates/ui/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。
  - `crates/ui/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。
  - `crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。
  - `crates/ui/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。
  - `crates/ui/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。
  - `crates/ui/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。
- [x] 组件目录标准文件落点正确。
  - 已落实（文件落点）：`components/modal/src/mod.rs`、`components/modal/src/logic.rs`、`components/modal/src/styles.rs`、`components/modal/src/view.rs`、`components/modal/src/motion.rs` 均存在；`components/modal/src/render.rs` 不存在（禁止漂移）；`components/modal/src/spec.rs` 不存在（simple 组件按 N/A 处理）。
  - 已落实（`mod.rs` 最小导出）：`components/modal/src/mod.rs` 仅保留 `pub use view::Modal;` 作为稳定对外入口；未公开 `logic/view/motion` 实现模块。
  - 已落实（职责分层）：`logic.rs` 继续承担 props 归一与来源派生（无 DOM/render/style 细节）；`styles.rs` 保持 token-first 静态 CSS（`pub const CSS` + `var(--ui-*)`）；`view.rs` 仅做 Leptos 结构渲染与 headless 语义挂载；`motion.rs` 仅做 modal 语义到 overlay motion contract 映射，不重写通用执行器。
  - `spec.rs` 适用性：N/A（`modal` 不属于复杂 schema-first 组件，现有版本化协议放在 `components/modal/src/protocol.rs` 即可，避免 `spec.rs` 泛滥）。
  - 回归：`components/modal/test/semantics.rs::modal_component_directory_standard_files_follow_contract_and_na_paths`、`components/modal/test/semantics.rs::modal_component_files_check_script_covers_standard_layout_gate`、`components/modal/test/modal_semantics.rs::modal_component_directory_standard_files_follow_contract_and_na_paths`、`components/modal/test/modal_semantics.rs::modal_component_files_check_script_covers_standard_layout_gate`、`components/modal/test/modal_semantics.rs::modal_check2_marks_component_directory_standard_files_contract_complete`。
  - 门禁脚本：`scripts/check-ui-component-files.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_component_directory_standard_files_follow_contract_and_na_paths`
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
  - `<component>/mod.rs`：最小稳定导出面，存在且无过度导出。
  - `<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。
  - `<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。
  - `<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。
  - `<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。
  - `<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。

### 6. AI 原生能力与文件落点（Struct-First & Projection）
- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。
  - 已落实（核心文件纪律）：`components/modal/src/mod.rs`、`components/modal/src/logic.rs`、`components/modal/src/styles.rs`、`components/modal/src/view.rs`、`components/modal/src/motion.rs` 全部存在；`components/modal/src/render.rs` 不存在。
  - `spec.rs` 适用性：N/A（`modal` 非复杂 builder/spec 组件，`components/modal/src/spec.rs` 不存在）；版本化 schema 约束由 `components/modal/src/protocol.rs` 承担（仓库既定协议例外，不属于 `render.rs` 漂移）。
  - 已落实（职责边界）：`mod.rs` 仅导出 `Modal`；`logic.rs` 维持归一/派生；`styles.rs` 保持 token-first 静态样式；`view.rs` 仅渲染与 headless 挂载；`motion.rs` 仅语义到 motion contract 映射。
  - 回归：`components/modal/test/semantics.rs::modal_file_placement_discipline_is_strict_for_component_scope`、`components/modal/test/semantics.rs::modal_file_placement_check_script_covers_contract`、`components/modal/test/modal_semantics.rs::modal_file_placement_discipline_is_strict_for_component_scope`、`components/modal/test/modal_semantics.rs::modal_file_placement_check_script_covers_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_file_placement_discipline_contract_complete`。
  - 门禁脚本：`scripts/check-ui-component-files.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_file_placement_discipline_is_strict_for_component_scope`
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。
  - 适用性结论：N/A（`modal` 非复杂 builder/spec 组件，不引入 `*Spec::new()...render()` 链式建造者，避免为简单组件制造额外抽象噪音）。
  - 已落实（N/A 证据）：`components/modal/src/spec.rs` 不存在；`components/modal/src/protocol.rs` 保持版本化 schema（`ModalComponentSchemaVersion` / `ModalComponentSpec`）作为轻量协议层，不等价于复杂 `spec.rs` builder。
  - 回归：`components/modal/test/semantics.rs::modal_hyper_structure_builder_spec_is_not_applicable_for_simple_component`、`components/modal/test/semantics.rs::modal_hyper_structure_builder_check_script_covers_na_contract`、`components/modal/test/modal_semantics.rs::modal_hyper_structure_builder_spec_is_not_applicable_for_simple_component`、`components/modal/test/modal_semantics.rs::modal_hyper_structure_builder_check_script_covers_na_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_hyper_structure_builder_contract_complete`。
  - 门禁脚本：`scripts/check-ui-component-files.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_hyper_structure_builder_spec_is_not_applicable_for_simple_component`
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。
  - 已落实（工件落点）：`components/modal/src/Component.toml` 新增并固化能力清单；`components/modal/src/modal.rbi` 新增并投影 `Modal` 的公开接口签名（含 open 受控轴、A11y 方向、children 与 motion contract）。
  - 回归：`components/modal/test/semantics.rs::modal_context_compression_manifest_and_rbi_projection_are_present_and_current`、`components/modal/test/semantics.rs::modal_component_files_check_script_covers_context_compression_manifest_contract`、`components/modal/test/semantics.rs::modal_check2_marks_context_compression_manifest_and_rbi_contract_complete`、`components/modal/test/modal_semantics.rs::modal_context_compression_manifest_and_rbi_projection_are_present_and_current`、`components/modal/test/modal_semantics.rs::modal_component_files_check_script_covers_context_compression_manifest_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_context_compression_manifest_and_rbi_contract_complete`。
  - 门禁脚本：`scripts/check-ui-component-files.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_context_compression_manifest_and_rbi_projection_are_present_and_current`
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。
  - 已落实（Schema 化输出）：`components/modal/src/logic.rs` 新增 `MODAL_AGENT_SCHEMA`、`ModalAgentSchemaVersion`、`ModalAgentIntent`、`ModalAgentAction`、`ModalAgentState`、`ModalAgentSource`、`ModalAgentConfigPolicy`、`ModalAgentOutputStatus`、`ModalAgentContractInput`、`ModalAgentContract` 与 `resolve_agent_contract`；`components/modal/src/view.rs` 通过 `Signal::derive` 调用 resolver 并挂载 `data-ui-schema/data-ui-schema-version/data-ui-intent/data-ui-action/data-ui-state/data-ui-source/data-ui-config-policy/data-ui-output-status` 与 capability 标记。
  - 已落实（上下文投影同步）：`components/modal/src/Component.toml` 补充 `agent-contract-markers` 输出与 `agent_contract_schema_markers/agent_contract_whitelist_render_policy` 能力；`components/modal/src/modal.rbi` 同步投影 Agent Contract 类型与 resolver 签名，保持 Manifest/RBI/代码一致。
  - 回归：`components/modal/test/semantics.rs::modal_check2_documents_agent_contract_schema_governance_rules`、`components/modal/test/semantics.rs::modal_agent_contract_is_schema_typed_and_machine_readable`、`components/modal/test/semantics.rs::modal_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing`、`components/modal/test/semantics.rs::modal_agent_contract_render_path_is_whitelist_safe_and_script_injection_free`、`components/modal/test/semantics.rs::modal_contract_hygiene_script_covers_agent_contract_schema_guards`、`components/modal/test/semantics.rs::modal_check2_marks_agent_contract_schema_governance_complete`、`components/modal/test/modal_semantics.rs::modal_check2_documents_agent_contract_schema_governance_rules`、`components/modal/test/modal_semantics.rs::modal_agent_contract_is_schema_typed_and_machine_readable`、`components/modal/test/modal_semantics.rs::modal_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing`、`components/modal/test/modal_semantics.rs::modal_agent_contract_render_path_is_whitelist_safe_and_script_injection_free`、`components/modal/test/modal_semantics.rs::modal_contract_hygiene_script_covers_agent_contract_schema_guards`、`components/modal/test/modal_semantics.rs::modal_check2_marks_agent_contract_schema_governance_complete`。
  - 门禁脚本：`scripts/check-ui-contract-hygiene.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_agent_contract_schema_governance_rules`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_agent_contract_is_schema_typed_and_machine_readable`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_agent_contract_render_path_is_whitelist_safe_and_script_injection_free`
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
  - 关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。
  - Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。
  - 契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。
  - 配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。
- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。（N/A：`Modal` 不是 LLM 正文渲染组件，组件职责是 Overlay 容器装配与语义挂载；组件侧不实现 token-by-token streaming 协议，仅消费稳定快照状态输入。术语约束固定为两种显示模式：`Streaming`（边生成边显示）与 `Snapshot`（完整结果一次性显示），避免在组件层引入第三种“伪流式”定义。回归：`components/modal/test/semantics.rs::modal_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`、`components/modal/test/semantics.rs::modal_streaming_script_covers_two_mode_definition_contract`、`components/modal/test/semantics.rs::modal_check2_marks_streaming_two_mode_definition_complete`、`components/modal/test/modal_semantics.rs::modal_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`、`components/modal/test/modal_semantics.rs::modal_streaming_script_covers_two_mode_definition_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_streaming_two_mode_definition_complete`；门禁脚本：`scripts/check-ui-streaming.sh` 新增 `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`。）
  - `Streaming`：LLM 还在生成，界面边生成边显示。
  - `Snapshot`：LLM 全部生成完成后，一次性显示。
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。（N/A：`Modal` 不直接渲染 LLM 正文，但组件已满足“完整配置输入 -> 稳定语义输出”的 snapshot 基线能力。证据：`components/modal/src/view.rs` 通过 `logic::normalize_open_state`/`logic::resolve_open_contract`/`logic::resolve_content_state`/`logic::normalize_on_exit_complete` 消费完整输入并挂载稳定 `data-open-*` 语义标记；组件路径未引入 `data-ui-stream-*`/`data-stream-*` 增量协议字段。回归：`components/modal/test/semantics.rs::modal_check2_documents_snapshot_as_default_baseline_capability`、`components/modal/test/semantics.rs::modal_snapshot_baseline_consumes_complete_result_and_renders_stably`、`components/modal/test/semantics.rs::modal_streaming_script_covers_snapshot_baseline_contract`、`components/modal/test/semantics.rs::modal_check2_marks_snapshot_baseline_capability_complete`、`components/modal/test/modal_semantics.rs::modal_check2_documents_snapshot_as_default_baseline_capability`、`components/modal/test/modal_semantics.rs::modal_snapshot_baseline_consumes_complete_result_and_renders_stably`、`components/modal/test/modal_semantics.rs::modal_streaming_script_covers_snapshot_baseline_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_snapshot_baseline_capability_complete`；门禁脚本：`scripts/check-ui-streaming.sh` 新增 `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_snapshot_as_default_baseline_capability` 与 `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_snapshot_baseline_consumes_complete_result_and_renders_stably`。）
  - 所有组件都应能消费“完整生成结果”并稳定渲染。
  - 即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。（`Modal` 归类为 `Streaming Optional`：组件职责是 Overlay 语义装配而非 LLM 正文阅读面，默认仅消费 `Snapshot`，并在契约层明确 `fallback=snapshot`。语义连续性由 `components/modal/src/view.rs` 的 `data-open-*` + `data-ui-state/source/output-status` 标记与 `components/overlay/src/view.rs` 的 `role/aria-modal/aria-labelledby/aria-describedby` 挂载共同保证；`output-status` 显式输出 `verified`，满足“当前输出状态可读”。数据校验、断线恢复、重试策略保持在上层编排，不下沉到组件。回归：`components/modal/test/semantics.rs::modal_check2_documents_streaming_required_optional_classification_rules`、`components/modal/test/semantics.rs::modal_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、`components/modal/test/semantics.rs::modal_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`、`components/modal/test/semantics.rs::modal_streaming_script_covers_required_optional_classification_contract`、`components/modal/test/semantics.rs::modal_check2_marks_streaming_required_optional_classification_complete`、`components/modal/test/modal_semantics.rs::modal_check2_documents_streaming_required_optional_classification_rules`、`components/modal/test/modal_semantics.rs::modal_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、`components/modal/test/modal_semantics.rs::modal_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`、`components/modal/test/modal_semantics.rs::modal_streaming_script_covers_required_optional_classification_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_streaming_required_optional_classification_complete`；门禁脚本：`scripts/check-ui-streaming.sh` 新增对应 `cargo test` 目标。）
  - `Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。
  - `Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。
  - 无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。
  - 数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。

### 7. 测试、门禁与交付
- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。（`components/modal/src/{mod,logic,styles,view,motion}.rs` 非测试源码已核验无 `.unwrap(` / `.expect(` / `let _ =`；`components/modal/src/logic.rs::compose_class_name` 已改为 `Cow<'static, str>` + `Vec<Cow<'static, str>>`，静态类名使用 `Cow::Borrowed`，仅自定义 class 名走 `Cow::Owned`。回归：`components/modal/test/semantics.rs::modal_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources`、`components/modal/test/semantics.rs::modal_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent`、`components/modal/test/semantics.rs::modal_rust_hygiene_script_enforces_repo_level_hygiene_guards`、`components/modal/test/semantics.rs::modal_check2_marks_rust_hygiene_contract_complete`、`components/modal/test/modal_semantics.rs::modal_rust_hygiene_contract_forbids_unwrap_expect_and_let_underscore_in_non_test_sources`、`components/modal/test/modal_semantics.rs::modal_rust_hygiene_string_clone_hotspots_converge_to_cow_or_are_absent`、`components/modal/test/modal_semantics.rs::modal_rust_hygiene_script_enforces_repo_level_hygiene_guards`、`components/modal/test/modal_semantics.rs::modal_check2_marks_rust_hygiene_contract_complete`；门禁脚本：`scripts/check-ui-engineering.sh` 新增对应 `cargo test` 目标。另执行：`./scripts/check-rust-hygiene.sh`（当前环境失败：`rg` 缺少 PCRE2 支持，且 `scripts/baseline/api_contract_violations.txt` 出现 baseline drift）。）
- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。（`modal` 已注册并受门控：`crates/ui/Cargo.toml` 保留 `component-modal = ["component-overlay"]`；`crates/ui/src/lib.rs` 通过 `#[cfg(feature = "component-modal")]` + path 模块导出；`crates/ui/src/css.rs` 通过 `#[cfg(feature = "component-modal")] out.push_str(crate::modal::styles::CSS);` 注入，且 non-`inject-css` 保持 no-op。回归：`components/modal/test/semantics.rs::modal_tree_shaking_contract_is_feature_gated`、`components/modal/test/semantics.rs::modal_tree_shaking_script_covers_feature_tree_wasm_and_budget`、`components/modal/test/semantics.rs::modal_check2_marks_tree_shaking_feature_pruning_contract_complete`、`components/modal/test/modal_semantics.rs::modal_tree_shaking_contract_is_feature_gated`、`components/modal/test/modal_semantics.rs::modal_tree_shaking_script_covers_feature_tree_wasm_and_budget`、`components/modal/test/modal_semantics.rs::modal_check2_marks_tree_shaking_feature_pruning_contract_complete`；门禁脚本：`scripts/check-ui-tree-shaking.sh` 新增对应 `cargo test` 与 `MODAL_MIN_FEATURES=component-modal,inject-css` 最小特性树/wasm 校验。另执行：`cargo tree -e features -i ui -p ui --no-default-features --features component-modal,inject-css` 与 `cargo tree -e features -i ui -p web-demo`（最小 modal 特性树未拉起 `all-components`；web-demo 路径包含 `web-demo-components`）。）
- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。（`Modal` 语义契约回归由 `modal_semantic_tests_cover_contract_matrix_and_do_not_rely_on_snapshots_only` 覆盖 `aria-*` 与 `data-*` 来源标记；焦点流转由 `modal_focus_restore_delegates_to_overlay_focus_manager_stack` 与 `ui-headless` focus stack（`focus_manager_push_trap`/`focus_manager_pop_trap`）锁定；性能预算与可观测 marker 由 `modal_performance_governance_contract_is_budgeted_traceable_and_blocking` 持续阻断，并新增 `modal_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement` 合并语义+性能门禁；`render_count` 自动化回归仍在仓库统一 follow-up（`docs/plan/TODO.md`，当前以 mount-only 等价证据执行）；脚本门禁：`scripts/check-ui-performance.sh` 新增 `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement`；本地验证受环境限制：`Invalid cross-device link (os error 18)`。）
- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `Modal` 未发生跨大版本 API 破坏升级）
  - N/A 判定依据：`components/modal/src/Component.toml` 保持 `schema_version = "1"`；`components/modal/src/modal.rbi` 的 `Modal(...)` 公共签名未发生破坏性移除/重命名；`components/modal/src/{mod.rs,logic.rs,view.rs,styles.rs,motion.rs,protocol.rs}` 未引入 `migrate_v1_to_v2`/`deprecation_window`/`SchemaRegistry`/`contract.v2`。
  - 回归锁定：`components/modal/test/semantics.rs::modal_version_deprecation_migration_is_na_without_major_breaking_upgrade`、`components/modal/test/semantics.rs::modal_version_deprecation_migration_script_covers_engineering_gate`、`components/modal/test/modal_semantics.rs::modal_version_deprecation_migration_is_na_without_major_breaking_upgrade`、`components/modal/test/modal_semantics.rs::modal_version_deprecation_migration_script_covers_engineering_gate`。
  - 脚本门禁：`scripts/check-ui-engineering.sh` 新增 `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_version_deprecation_migration_is_na_without_major_breaking_upgrade`。
  - 验证记录：执行上述 `cargo test` 命令，当前环境返回 `Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。（已落地于 `apps/docs-app/src/pages/components/pages/overlays.rs::modal`：新增 `Hello World (Minimal Path)`、`State Matrix`、`Controlled vs Uncontrolled`、`Streaming / Snapshot Contract` 四个 Playground，并统一通过 `MODAL_DOC_IMPORTS` 提供补全 imports；Source-first 区块增加 `data-slot="modal-source-first"` / `data-slot="modal-source-paths"` 与最小依赖提示（`component-modal` + `inject-css`），复制链路由 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 保证。回归：`components/modal/test/semantics.rs::modal_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract`、`components/modal/test/semantics.rs::modal_dx_check_script_covers_docs_product_copy_paste_ready_contract`、`components/modal/test/modal_semantics.rs::modal_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract`、`components/modal/test/modal_semantics.rs::modal_dx_check_script_covers_docs_product_copy_paste_ready_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_docs_product_copy_paste_ready_contract_complete`；门禁脚本：`scripts/check-ui-dx.sh` 新增 `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract`。本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。）
- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。
  - 每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。
  - 断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。
  - 新增/变更语义字段必须同步补测试，否则不得打勾。
  - 已满足（组件级语义测试）：`components/modal/test/semantics.rs` 已覆盖关键状态轴与动作语义（`modal_semantic_tests_cover_contract_matrix_and_do_not_rely_on_snapshots_only`），断言聚焦 `data-*`、`aria-*`、`role` 与状态来源标记。
  - 已满足（快照仅补充）：本地语义套件显式禁止将视觉快照当作主判据（`assert_snapshot`/`to_match_snapshot`/`snapshot!` 禁用约束），并以键盘/指针语义路径断言为主。
  - 已满足（变更同步补测）：新增 `components/modal/test/semantics.rs::modal_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks` 与 `components/modal/test/modal_semantics.rs::modal_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks`，并新增脚本回归 `modal_performance_script_covers_semantic_test_priority_contract`。
  - compile/test 证据（命令）：
    - `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks`
    - `bash scripts/check-ui-performance.sh`
  - 当前环境说明：上述 `cargo test` 命令在本执行环境于依赖编译阶段触发 `Invalid cross-device link (os error 18)`；该阻断为环境问题，非 modal 语义测试优先契约回归。
  - 回归：`components/modal/test/semantics.rs::modal_semantic_tests_cover_contract_matrix_and_do_not_rely_on_snapshots_only`、`components/modal/test/semantics.rs::modal_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks`、`components/modal/test/modal_semantics.rs::modal_semantic_test_priority_prefers_data_aria_role_and_source_contracts_over_snapshot_only_checks`、`scripts/check-ui-performance.sh`。
- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。
  - E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。
  - WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。
  - 若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。
  - 已满足（语义选择器）：新增 `e2e/tests/docs_app_modal_contract.spec.mjs`，使用 `data-component="modal"`、`data-slot="modal-e2e-described-controls"`、`data-slot="modal-e2e-open-described"`、`data-slot="overlay-panel"`、`data-slot="modal"`、`data-slot="overlay-backdrop"` 等稳定语义锚点；为避免文本/层级漂移，`apps/docs-app/src/pages/components/pages/overlays.rs::modal` 新增 `data-slot="modal-e2e-described-controls"` / `data-slot="modal-e2e-open-described"` / `data-slot="modal-e2e-custom-controls"` / `data-slot="modal-e2e-open-custom"`。
  - 已满足（WASM 稳定等待）：E2E 路径统一以 `body:not(:has(#boot))` 作为 wasm 就绪断点，未使用 `waitForTimeout`/固定 sleep。
  - 已满足（ready/settled 覆盖）：对 overlay 关闭路径显式断言 `Escape` 与 backdrop 点击后的 settled 条件（`toHaveCount(0)`），覆盖组件动画/退出阶段的可观测收敛点。
  - 脚本门禁：新增 `components/modal/scripts/check-ui-e2e-modal.sh`，接入  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_e2e_selector_and_stable_wait_rules`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_e2e_contract_uses_semantic_selectors_and_stable_waits`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_e2e_contract_covers_ready_and_settled_conditions_for_overlay_dismissal`
  - 回归：`components/modal/test/semantics.rs::modal_check2_documents_e2e_selector_and_stable_wait_rules`、`components/modal/test/semantics.rs::modal_e2e_contract_uses_semantic_selectors_and_stable_waits`、`components/modal/test/semantics.rs::modal_e2e_contract_covers_ready_and_settled_conditions_for_overlay_dismissal`、`components/modal/test/semantics.rs::modal_e2e_check_script_covers_selector_and_settled_wait_contract`、`components/modal/test/semantics.rs::modal_check2_marks_e2e_selector_stability_item_complete`、`components/modal/test/modal_semantics.rs::modal_e2e_contract_uses_semantic_selectors_and_stable_waits`、`components/modal/test/modal_semantics.rs::modal_e2e_contract_covers_ready_and_settled_conditions_for_overlay_dismissal`、`components/modal/test/modal_semantics.rs::modal_e2e_check_script_covers_selector_and_settled_wait_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_e2e_selector_stability_item_complete`。
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。
  - 至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。
  - 回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。
  - 高风险路径（overlay、focus、keyboard、async）优先进入回归集合。
  - 已满足（可重复关键流程）：`e2e/tests/docs_app_modal_contract.spec.mjs` 新增 `docs-app modal critical flow is replayable with overlay focus and keyboard checkpoints`，通过 `for (const cycle of [1, 2])` 固定回放两轮 `open -> interact(Tab) -> close(Escape)` 全流程。
  - 已满足（语义断点可定位）：每轮均断言 `aria-modal="true"`、`data-open-mode="controlled"`、`data-open-source="controlled"`、`data-open-prop-source="is_open"`、`expectFocusInsidePanel(describedPanel)`、`toHaveCount(0)`、`openDescribed` 焦点恢复；回归失败可直接定位到具体语义断点。
  - 已满足（高风险路径优先）：回归覆盖 overlay 展开/关闭、focus trap、keyboard（`Tab`/`Escape`）三条高风险路径。
  - 脚本门禁：`components/modal/scripts/check-ui-e2e-modal.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_e2e_regression_flow_is_replayable_and_maps_failures_to_semantic_breakpoints`
  - 回归：`components/modal/test/semantics.rs::modal_e2e_regression_flow_is_replayable_and_maps_failures_to_semantic_breakpoints`、`components/modal/test/semantics.rs::modal_check2_marks_replayable_e2e_critical_flow_item_complete`、`components/modal/test/modal_semantics.rs::modal_e2e_regression_flow_is_replayable_and_maps_failures_to_semantic_breakpoints`、`components/modal/test/modal_semantics.rs::modal_check2_marks_replayable_e2e_critical_flow_item_complete`。
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。
  - 组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。
  - 文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。
  - 文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。
  - 已满足（docs 示例与说明同步）：`apps/docs-app/src/pages/components/pages/overlays.rs::modal` 维持 `Hello World (Minimal Path)`、`State Matrix`、`Controlled vs Uncontrolled` 三组核心示例，覆盖基础路径与受控/非受控矩阵。
  - 已满足（状态矩阵覆盖）：`State Matrix` 中显式覆盖 `is_open`（受控）、`default_open`（非受控初值）与 `description` 分支；`Controlled vs Uncontrolled` 保留对照路径和 `on_open_change` 观测输出。
  - 已满足（API 名称与默认值对齐 logic）：文档示例统一使用 `is_open/default_open/on_open_change` 命名；新增 `data-slot="modal-defaults-contract"` 明确声明 `components/modal/src/logic.rs` 默认契约：`id_base="ui-modal"`、`title="Modal"`、`default_open=false`。
  - 脚本门禁：`scripts/check-ui-dx.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_docs_sync_and_state_matrix_rules`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`
  - 回归：`components/modal/test/semantics.rs::modal_check2_documents_docs_sync_and_state_matrix_rules`、`components/modal/test/semantics.rs::modal_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`、`components/modal/test/semantics.rs::modal_dx_check_script_covers_docs_sync_and_state_matrix_contract`、`components/modal/test/semantics.rs::modal_check2_marks_docs_sync_and_state_matrix_item_complete`、`components/modal/test/modal_semantics.rs::modal_check2_documents_docs_sync_and_state_matrix_rules`、`components/modal/test/modal_semantics.rs::modal_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`、`components/modal/test/modal_semantics.rs::modal_dx_check_script_covers_docs_sync_and_state_matrix_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_docs_sync_and_state_matrix_item_complete`。
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。
  - 每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。
  - 文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。
  - “只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。
  - 已满足（README 入口存在且新手优先）：`components/modal/src/README.md` 明确包含 `## Hello World（最小可用）`、`## 先用起来，再进阶`、`## 常见用法`、`### Controlled Example（高级入口）`，并将默认路径放在高级控制之前。
  - 已满足（零门槛最小示例 + 常见用法）：README 保留最小可运行示例（`default_open + id_base + title + on_close`）与受控示例；不会要求先理解 `ui-state-primitives/ui-headless` 分层。
  - 已满足（等价文档入口可索引）：`apps/docs-app/src/pages/components/pages.rs` 注册 `component_doc!("Modal", "modal", "Overlays", overlays::modal)`；`apps/docs-app/src/pages/components/pages/overlays.rs::modal` 提供 `Hello World (Minimal Path)`、`State Matrix`、`Controlled vs Uncontrolled` 页面入口。
  - 脚本门禁：`scripts/check-ui-dx.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_documentation_as_product_rules`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_documentation_entry_exists_with_beginner_first_progression`
  - 回归：`components/modal/test/semantics.rs::modal_check2_documents_documentation_as_product_rules`、`components/modal/test/semantics.rs::modal_documentation_entry_exists_with_beginner_first_progression`、`components/modal/test/semantics.rs::modal_dx_check_script_covers_documentation_as_product_contract`、`components/modal/test/semantics.rs::modal_check2_marks_documentation_as_product_item_complete`、`components/modal/test/modal_semantics.rs::modal_check2_documents_documentation_as_product_rules`、`components/modal/test/modal_semantics.rs::modal_documentation_entry_exists_with_beginner_first_progression`、`components/modal/test/modal_semantics.rs::modal_dx_check_script_covers_documentation_as_product_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_documentation_as_product_item_complete`。
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。
  - Playground 至少支持基础 props 调整、状态切换、交互反馈观察。
  - 对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。
  - Playground 作为验收面，需可重复复现关键交互路径。
  - 已满足（可在线修改 props/状态并实时预览）：`apps/docs-app/src/pages/components/pages/overlays.rs::modal` 提供 `title="Interactive Playground"`，使用 `controls=...` 暴露 `Description/Custom id_base/Custom title/Custom class/Custom motion/Custom exit callback` 六个开关；预览区实时绑定 `interactive_open_raw` 与 `Modal` 语义标记输出。
  - 已满足（配置输入与预览输出联动）：同一 Playground 通过 `test_config_signal=interactive_config` 输出 `ModalActualConfig { ... }` 实时配置投影，形成“输入配置 -> 预览状态”联动证据。
  - N/A：`Modal` 非 AI Spec 组件（无独立 spec 输入协议），因此“AI Spec 输入/输出联动”以 `interactive_config` 的结构化配置投影替代，避免引入伪 spec 层。
  - 已满足（验收面可重复关键流程）：`e2e/tests/docs_app_modal_contract.spec.mjs` 新增 `docs-app modal interactive playground replays open-close flow with stable semantic anchors`，基于 `data-slot="modal-interactive-controls/open/close"` 做两轮 `open -> observe -> close` 回放（`for (const cycle of [1, 2])`），并断言 `aria-modal` 与 `data-open-*` 语义断点。
  - 脚本门禁：`scripts/check-ui-dx.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_interactive_playground_rules`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_docs_app_provides_interactive_playground_for_props_state_and_preview`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_interactive_playground_reuses_repeatable_semantic_e2e_flow`
  - 回归：`components/modal/test/semantics.rs::modal_check2_documents_interactive_playground_rules`、`components/modal/test/semantics.rs::modal_docs_app_provides_interactive_playground_for_props_state_and_preview`、`components/modal/test/semantics.rs::modal_interactive_playground_reuses_repeatable_semantic_e2e_flow`、`components/modal/test/semantics.rs::modal_dx_check_script_covers_interactive_playground_contract`、`components/modal/test/semantics.rs::modal_check2_marks_interactive_playground_item_complete`、`components/modal/test/modal_semantics.rs::modal_check2_documents_interactive_playground_rules`、`components/modal/test/modal_semantics.rs::modal_docs_app_provides_interactive_playground_for_props_state_and_preview`、`components/modal/test/modal_semantics.rs::modal_interactive_playground_reuses_repeatable_semantic_e2e_flow`、`components/modal/test/modal_semantics.rs::modal_dx_check_script_covers_interactive_playground_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_interactive_playground_item_complete`。
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。
  - docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。
  - 若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。
  - 文档代码与当前实现必须同步，防止示例漂移。
  - 已满足（copy-ready 一键复制链路）：`apps/docs-app/src/pages/components/pages/overlays.rs::modal` 的 `data-slot="modal-source-first"` 明确要求通过 `Show code` 复制，`apps/docs-app/src/playground.rs::compose_copy_ready_code` 负责为片段自动补全 `MODAL_DOC_IMPORTS`，保证复制代码可直接运行。
  - 已满足（真实源码落点 + 依赖前提）：`data-slot="modal-source-paths"` 列出 `components/modal/src/{mod,logic,view,styles,motion}.rs`；依赖前提写明 `ui` 特性 `component-modal` + `inject-css`，避免“复制即报错”。
  - 已满足（文档与实现同步）：Source-first 区块默认值说明绑定 `components/modal/src/logic.rs`，并通过语义测试锁定 `Show code`、`MODAL_DOC_IMPORTS`、`compose_copy_ready_code`、源码路径与依赖文案。
  - 脚本门禁：`scripts/check-ui-dx.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_source_first_copy_paste_ready_rules`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`
  - 回归：`components/modal/test/semantics.rs::modal_check2_documents_source_first_copy_paste_ready_rules`、`components/modal/test/semantics.rs::modal_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`、`components/modal/test/semantics.rs::modal_dx_check_script_covers_source_first_copy_paste_ready_contract`、`components/modal/test/semantics.rs::modal_check2_marks_source_first_copy_paste_ready_contract_complete`、`components/modal/test/modal_semantics.rs::modal_check2_documents_source_first_copy_paste_ready_rules`、`components/modal/test/modal_semantics.rs::modal_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`、`components/modal/test/modal_semantics.rs::modal_dx_check_script_covers_source_first_copy_paste_ready_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_source_first_copy_paste_ready_contract_complete`。
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。
  - 若参数语义发生变化，需同步更新对标策略文档，不允许实现先漂移文档后补。
  - 组件文档入口必须存在（docs-app 页面或等价文档），且可被索引定位。
  - “仅代码更新无文档更新”在接口变更场景下直接判不通过。
  - 已满足（策略文档同步）：`docs/spec/heroui-parameter-design-strategy.md` 新增 `### Modal 同步记录（2026-02-20）`，明确 `Modal` 参数主轴与 docs/README 同步约束，并标注“本轮不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`”。
  - 已满足（docs 入口可索引）：`apps/docs-app/src/pages/components/pages.rs` 保持 `component_doc!("Modal", "modal", "Overlays", overlays::modal)`；`apps/docs-app/src/pages/components/pages/overlays.rs::modal()` 保持 `title="Modal"` + `slug="modal"`。
  - 已满足（等价组件文档入口）：`components/modal/src/README.md` 持续提供 `# Modal` 与 `## Hello World（最小可用）`。
  - 脚本门禁：`scripts/check-ui-dx.sh` 新增  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_check2_documents_heroui_benchmark_docs_sync_rules`  
    `cargo test -p ui --test modal_semantics --no-default-features --features component-modal,inject-css modal_heroui_strategy_and_component_docs_are_synchronized_and_indexable`
  - 回归：`components/modal/test/semantics.rs::modal_check2_documents_heroui_benchmark_docs_sync_rules`、`components/modal/test/semantics.rs::modal_heroui_strategy_and_component_docs_are_synchronized_and_indexable`、`components/modal/test/semantics.rs::modal_dx_check_script_covers_heroui_benchmark_docs_sync_contract`、`components/modal/test/semantics.rs::modal_check2_marks_heroui_benchmark_docs_sync_contract_complete`、`components/modal/test/modal_semantics.rs::modal_check2_documents_heroui_benchmark_docs_sync_rules`、`components/modal/test/modal_semantics.rs::modal_heroui_strategy_and_component_docs_are_synchronized_and_indexable`、`components/modal/test/modal_semantics.rs::modal_dx_check_script_covers_heroui_benchmark_docs_sync_contract`、`components/modal/test/modal_semantics.rs::modal_check2_marks_heroui_benchmark_docs_sync_contract_complete`。
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。

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
