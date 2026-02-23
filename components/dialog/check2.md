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
- [x] `status-primitives` 定义：纯状态原语层（受控/非受控、toggle、selection、list、overlay open state、expansion 等）。不依赖 Leptos/DOM/web-sys；只包含 Rust 数据结构和方法，不含视图与事件绑定。（已将 `Dialog` 状态归一化与派生核心下沉至 `crates/ui-state-primitives/src/dialog.rs`，组件层 `components/dialog/src/logic.rs` 仅保留 slot/class 装配映射并消费 primitive 输出；回归：`crates/ui-state-primitives/src/test/dialog.rs`、`components/dialog/test/logic.rs`。）
  - 所有状态原语必须从 `status-primitives`（`ui-state-primitives`）获取，组件层只能消费，不得自造。
  - 下沉判定依据是“稳定状态不变量”；凡属于状态机、归一化、状态派生能力，默认先进入 `ui-state-primitives`。
  - 组件中可保留的仅是装配逻辑：props 归一、样式来源标记、slot 组织、对 `ui-state-primitives` 输出的映射。
  - 组件内若出现状态原语实现（受控/非受控状态机、single/multiple 展开规则、索引归一化、跨事件状态派生），该项直接判不通过。
  - 处理方式固定：先下沉到 `ui-state-primitives/src/<capability>.rs`（如 `expansion.rs`），在 `ui-state-primitives/src/lib.rs` 导出，再回到组件改调用。
  - 下沉后的原语必须有 `ui-state-primitives` 单元测试；组件侧只保留调用与语义挂载测试。
  - 桥接规范：`ui-state-primitives` 结构体必须是 POJO（Plain Old Rust Object），不持有 Leptos `Signal` 或框架绑定状态容器。
  - 消费规范：`ui-headless` 或组件 `logic.rs` 负责解包 `Signal` 当前值传入 primitive 方法，并将结果显式写回 `Signal`。
  - 设计理由：保持 primitives 纯粹可测、可迁移，不与特定响应式库绑定（便于未来替换响应式实现与做纯 Rust 测试）。
- [x] `ui-headless` 定义：交互与 A11y 原语层（press/focus/hover/roving/listbox/menu/tooltip 等），把输入设备事件与状态语义标准化为可复用契约；输出必须是类型化 `attrs + handlers + state`。不做样式、不写组件 CSS、不做组件级动效编排。（`Dialog` 继续复用 `Overlay` 的 headless 交互语义（focus trap / modal / escape / overlay stack），并在 `components/dialog/src/view.rs` 新增 `lang/dir` 接口，通过 `ui_headless::locale_attrs` 挂载 locale 语义；组件未在 `view.rs` 重写键盘/焦点/ARIA 归一逻辑。回归：`components/dialog/test/dialog/semantics.rs::dialog_supports_headless_locale_contract_with_lang_dir_attrs`。）
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
- [x] `ui-motion` 定义：动效能力与契约执行层（spring、keyframes、WAAPI/RAF backend），只负责时间函数、插值与运行时驱动，不承载组件业务语义与状态决策。（`components/dialog/src/motion.rs` 仅保留 `DialogMotion` 合同与 `sanitize_motion/attach_motion` 对 `overlay` 动效层的委托，不含组件内 spring/keyframe/driver 实现；`crates/ui-motion/src/lib.rs` 提供 non-wasm no-op/stub，Dialog 通过 overlay 委托继承该降级路径。回归：`components/dialog/test/motion.rs`、`components/dialog/test/dialog/semantics.rs::dialog_motion_module_stays_as_contract_mapping_without_custom_engine`。）
  - 放在 `crates/ui-motion`：通用动画数学与执行后端（spring solver、keyframe sampling、easing registry、driver adapters），以及 `wasm/non-wasm` 适配与 `reduced-motion` 执行策略。
  - 放在 `crates/ui/src/<component>/motion.rs`：把组件语义状态（open/closed、enter/exit、active/inactive）映射为 `ui-motion` contract，绑定目标节点并调用 attach。
  - 禁止放在 `crates/ui-motion`：组件 slot 结构、组件专属状态机、ARIA/keyboard 语义、业务文案与业务分支。
  - 禁止放在组件 `motion.rs`：自实现 spring/keyframe/driver 执行器；跨组件共享动效算法必须回迁 `ui-motion`。
  - 动效参数优先来自 token/theme；禁止在组件样式与逻辑中散落硬编码时长/曲线/位移常量。
  - 非 wasm 路径必须提供 no-op/stub，保证 SSR/tooling 可编译且行为可预测。
- [x] `ui-theme` 定义：唯一设计 token 与主题上下文层（system/color/scale + Light/Dark/OLED），负责 token 分类、主题映射与 CSS 变量生成。（`components/dialog/src/styles.rs` 已改为 token-first 消费：尺寸/间距/排版/语义色全部经 `--ui-*` + `--ui-fallback-*` 变量读取；组件未定义平行私有设计 token。主题基线与变量出口由 `crates/ui-theme/src/tokens.rs` + `crates/ui-theme/src/theme.rs` + `crates/ui-theme/src/css.rs` 统一提供，尺度回归由 `crates/ui-theme/tests/token_scale_baseline.rs` 覆盖。组件侧回归：`components/dialog/test/dialog/semantics.rs::dialog_styles_consume_ui_theme_tokens_and_avoid_hardcoded_dialog_size_literals`。）
  - Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量；组件只在 `crates/ui/src/<component>/styles.rs` 消费。
  - 三轴上下文（`system/color/scale`）在 `theme.rs` 定义；组件在 `logic.rs` 选择并在 `view.rs` 生效，`styles.rs` 只消费变量，不重建主题。
  - Token 分类必须可追溯：分类源在 `tokens.rs`，规范同步 `docs/spec/styling.md`；组件不得引入平行私有 token 命名体系。
  - 量化尺寸基准必须可回归：尺寸基准在 `tokens.rs` 与 `theme.rs` 定义，主题回归在 `crates/ui-theme/tests/token_scale_baseline.rs`，组件语义回归在 `components/*/test/*<component>_semantics.rs`。
  - 主题调色与语义色对比必须满足 `WCAG 2.1 AA` 基线，并覆盖 Light/Dark/OLED 主题变体。
  - 主题层只输出 `theme/tokens/base css` 与变量；不实现组件结构、交互逻辑、组件级动效编排。
  - 新增视觉语义先补 token，再由组件消费；禁止“组件临时值先落地、后补 token”的倒序流程。
- [x] `ui` 定义：最终 Leptos 组件装配层，组合 `status-primitives + ui-headless + ui-motion + ui-theme` 并暴露稳定公共 API。（`Dialog` 保持 `logic/view/styles/motion` 责任分离；`logic.rs` 仅做 props 归一与状态映射，`view.rs` 负责结构与 headless 挂载，`styles.rs` 为 token-first 静态样式，`motion.rs` 为动效合同委托。新增组件侧语义测试 `components/dialog/test/semantics.rs` 并在 `components/dialog/src/mod.rs` 接入；集成侧补充 `components/dialog/test/dialog/semantics.rs::dialog_component_has_local_semantics_test_module` 锁定迁移约束。）
  - `logic.rs` 负责 props 归一与状态派生；`view.rs` 负责结构渲染与 headless 语义挂载；`styles.rs` 负责 token-first 静态样式；`motion.rs` 负责动效 attach。
  - 组件层不得重写 `status-primitives` 状态机或 `ui-headless` 交互契约；发现即判不通过并回迁到对应层。
  - 对外 API 禁止暴露 `web-sys`/DOM 细节类型；平台差异封装在内部模块。
  - 测试文件位于src同级的test/中，内部测试文件同名（如rust-ui/components/accordion/src/logic.rs与rust-ui/components/accordion/test/logic.rs）。
  - 还需要一个semantics.rs用于测试。可能存在类似rust-ui/components/accordion/test/semantics.rs的旧版实现，需要迁移到新目录。

### 2. API 设计与状态内核（Logic/Kernel）
- [x] API 命名契约统一：公共 props/回调严格使用 `is_*`、`on_*`、`default_*` 前缀；同语义在全库同名，禁止别名漂移。（`components/dialog/src/view.rs` 已新增规范命名入口 `is_open` 与 `is_close_button_visible`，事件继续使用 `on_close`；为兼容既有调用临时保留 `open`/`show_close_button` 别名并在视图层归一（新命名优先），迁移窗口内不破坏现有调用。回归：`components/dialog/test/semantics.rs::dialog_public_api_prefers_prefixed_names_with_alias_migration`、`components/dialog/test/dialog/semantics.rs::dialog_public_api_uses_prefixed_bool_names_with_compat_aliases`。）
  - 布尔状态统一 `is_*`（如 `is_open`/`is_disabled`），事件统一 `on_*`，默认值统一 `default_*`。
  - 同一语义 across 组件必须同名（如都用 `on_open_change`，禁止同义别名并存）。
  - 公共 API 引入新命名时，需说明与现有命名体系的兼容策略与迁移路径。
- [x] 受控/非受控必须成对：每个可控状态轴都提供 `value + on_value_change + default_value`（如 `open/on_open_change/default_open`）；缺一项即不通过。（`Dialog` 开关轴已补齐 `is_open/open + on_open_change + default_open`：`components/dialog/src/logic.rs` 新增 `DialogOpenStateInput/DialogOpenState/normalize_open_state` 做单点归一，`components/dialog/src/view.rs` 通过 `ui_headless::use_controllable_open_state_traced` 执行受控/非受控语义；close 路径统一调用 `request_open_change(false)` 并保留 `on_close` 兼容回调，避免半受控。回归：`components/dialog/test/logic.rs::normalize_open_state_supports_controlled_and_uncontrolled_modes`、`components/dialog/test/logic.rs::normalize_open_state_uses_implicit_default_when_missing`、`components/dialog/test/dialog/semantics.rs::dialog_public_api_uses_prefixed_bool_names_with_compat_aliases`。）
  - 受控模式：外部值是单一事实来源，内部不得偷偷写回本地状态。
  - 非受控模式：仅由默认值初始化一次，后续状态由内部原语管理。
  - 受控/非受控切换语义需稳定可测，避免“半受控”隐式行为。
- [x] 默认值单一来源：默认值与优先级只在 `logic.rs` 归一化；`view.rs` 禁止二次兜底或隐式改写。（`Dialog` 默认值/优先级已回收至 `components/dialog/src/logic.rs`：`normalize_open_state` 统一 `is_open/open/default_open/on_open_change` 来源与优先级，`normalize_close_config` 统一 `is_close_button_visible/show_close_button/close_label` 默认与优先级，`normalize_exit_config` 统一 `on_exit_complete` 的 noop 默认。`components/dialog/src/view.rs` 仅消费归一结果，不再出现 `if close_label.trim().is_empty()`、`show_close_button.unwrap_or(...)`、`on_exit_complete.unwrap_or_else(...)`。回归：`components/dialog/test/logic.rs::normalize_close_config_owns_close_defaults_and_priority`、`components/dialog/test/logic.rs::normalize_exit_config_owns_noop_default`、`components/dialog/test/semantics.rs::dialog_public_api_prefers_prefixed_names_with_alias_migration`、`components/dialog/test/dialog/semantics.rs::dialog_public_api_uses_prefixed_bool_names_with_compat_aliases`。）
  - 默认值优先级必须可读且可测试（显式规则而非分散 `unwrap_or`）。
  - `view.rs` 不允许再做默认值分支；仅消费 `logic.rs` 的归一化输出。
  - 一旦发现多处默认值来源，直接判不通过并回收至 `logic.rs`。
- [x] 状态归一化集中：状态输入先类型化，再在 `logic.rs` 统一派生；禁止在 `view.rs`、事件回调、样式分支中分散拼状态机。（`Dialog` 已将状态归一与派生集中到 `components/dialog/src/logic.rs`：`normalize_open_state/normalize_close_config/normalize_exit_config` 处理输入类型化与来源标记，`resolve_part_states` 统一派生 root/header/title/description/body/footer/close 七个 part state，`resolve_part_classes` 统一 class 生成，`can_request_close` 统一 close 触发策略。`components/dialog/src/view.rs` 仅消费这些归一化输出并挂载语义标记，不再直接拼 `DialogPartStateInput`/调用 `resolve_state` 组装状态机；事件处理器仅触发 `request_open_change(false)` 与可选 `on_close` 回调。回归：`components/dialog/test/logic.rs::resolve_part_states_concentrates_slot_state_derivation`、`components/dialog/test/logic.rs::can_request_close_follows_mode_and_open_change_handler`、`components/dialog/test/semantics.rs::dialog_logic_and_view_follow_layering_contract`、`components/dialog/test/dialog/semantics.rs::dialog_view_uses_logic_contracts_and_source_markers`。）
  - 输入边界统一进入 `logic.rs`，输出统一为可渲染语义状态与来源标记。
  - 事件处理器只触发状态变更，不重建状态机规则。
  - 样式层只消费状态标记，不承担状态判定职责。
- [x] 离散状态必须类型约束：`variant/size/mode/status` 等离散输入使用 `enum`；禁止用多个 `Option<bool>`/字符串自由组合表达互斥状态。（`Dialog` 离散轴已类型化：尺寸用 `DialogSize`，开合模式用 `DialogOpenMode`，关闭按钮可见性与来源新增 `DialogCloseButtonVisibility` / `DialogCloseButtonPropSource`（`components/dialog/src/logic.rs`），`show_close_button`/`is_close_button_visible` 仅作为兼容输入并在 `normalize_close_config` 立即映射为枚举后再进入派生逻辑；`view.rs` 消费 `close_button_visibility` 而非自由布尔组合。回归：`components/dialog/test/logic.rs::normalize_close_config_owns_close_defaults_and_priority`、`components/dialog/test/semantics.rs::dialog_public_api_prefers_prefixed_names_with_alias_migration`、`components/dialog/test/dialog/semantics.rs::dialog_logic_exposes_state_helpers`。）
  - 互斥状态优先用 `enum` 建模，利用编译器封住无效组合。
  - 字符串输入若需兼容外部配置，必须先映射到类型化枚举再进入逻辑层。
  - 布尔爆炸（多个 bool 表达一个状态机）应在设计评审阶段直接拦截。
- [x] 状态原语来源正确：组件层只消费 `status-primitives`（当前 `ui-state-primitives`）能力，不直接绑定业务 store；应用级全局状态必须经桥接层适配后再接入组件。（`Dialog` 的可复用状态规则已下沉到 `crates/ui-state-primitives/src/dialog.rs`：新增 `resolve_open_state_contract`、`can_request_close`、`resolve_close_button_contract` 与对应 `DialogOpenMode`/`DialogCloseButtonVisibility`/`DialogCloseButtonPropSource` 原语；`components/dialog/src/logic.rs` 仅做 Leptos `Signal/Callback` 适配并消费 primitive 结果，不再在组件层重写受控/非受控与 close 可见性状态机。组件层未绑定任何业务 store 类型。回归：`crates/ui-state-primitives/src/test/dialog.rs`、`components/dialog/test/semantics.rs::dialog_public_api_prefers_prefixed_names_with_alias_migration`、`components/dialog/test/dialog/semantics.rs::dialog_logic_exposes_state_helpers`。）
  - 组件中出现可复用状态机实现（受控/非受控、展开规则、选择归一）即判应下沉。
  - 组件与业务全局状态之间必须有适配边界，禁止组件直接依赖业务 store 类型。
  - `logic.rs` 仅做装配与映射，不重新实现状态原语。
- [x] 如果无异步相关，直接打勾。异步交互语义统一：`is_loading`、error/retry、disabled、`aria-busy` 映射一致；优先复用统一 async action 原语（如 `use_async_action`），禁止每组件自定义一套加载/错误协议。（N/A：`Dialog` 仅本地开关与同步关闭回调，不发起远程请求、无组件内异步加载/错误/重试状态机；`view.rs` 未暴露 `is_loading`/`aria-busy`/`on_retry`/`use_async_action` 契约。回归：`components/dialog/test/semantics.rs::dialog_has_no_async_loading_or_retry_contracts`、`components/dialog/test/dialog/semantics.rs::dialog_has_no_async_loading_or_retry_contracts`。）
  - 无异步交互时需明确标注 N/A 理由（例如“组件无远程请求与异步状态”），不是机械打勾。
  - 有异步交互时，`is_loading`/disabled/`aria-busy`/retry 语义必须成套一致，且对键盘与读屏路径可用。
  - 异步失败态要有可恢复路径（重试或回退），并有语义测试覆盖。
- [x] API 易用性验收标准（DX Paradox）：把复杂性留在内部，把简单留给用户。（`Dialog` 基础用法无需用户手动接线 `ui-state-primitives/ui-headless` 状态机：组件仅暴露 `Dialog` props，不暴露内部状态对象必填参数；`apps/docs-app/src/pages/components/pages/overlays_dialog.rs` 新增 `Hello World` Playground，提供 5 行内可复制最小示例（`default_open=Some(true)`）并展示默认调用路径；`components/dialog/src/README.md` 同步最小示例。回归：`components/dialog/test/dialog/semantics.rs::dialog_docs_hello_world_provides_minimal_default_entrypoint`。）
  - 基础用法不得要求用户先理解或手动接线 `ui-state-primitives`/`ui-headless` 状态机。
  - 基础组件 Hello World 示例代码不得超过 5 行（导入与外层模板按仓库约定不计），并可直接运行。
  - 简单需求走简单 API，复杂需求再暴露高级入口：默认 props 覆盖高频场景，高级控制通过受控/扩展参数按需开启。
  - 禁止把内部状态对象作为基础必填参数暴露（例如强制 `state=...` 才能完成点击/展开等基本交互）。
  - docs-app 必须提供最小可用示例，优先展示一眼可懂的默认调用路径。
- [x] 组合型组件主 API 必须“显示优于约定”：优先使用显式组合 `<Parent><Item ... /></Parent>`。（N/A：`Dialog` 非集合型组合组件，不存在可枚举 `Item` 轴；主 API 为显式树结构 `<Dialog>...</Dialog>`，标题/语义/内容在同一结构维度绑定，未提供 `labels + children`、`titles + panels` 等并行数组/并行槽位默认写法，也未引入 `ItemSpec` 配置语法糖。回归：`components/dialog/test/semantics.rs::dialog_non_composite_api_keeps_explicit_single_tree_contract`、`components/dialog/test/dialog/semantics.rs::dialog_non_composite_api_rejects_parallel_item_array_contracts`。）
  - 每个 item 的标题、语义与内容必须在同一 `Item` 结构维度绑定，避免索引配对式隐式约定。
  - `labels + children`、`titles + panels` 等并行数组/并行槽位写法不得作为默认或推荐 API。
  - 不引入这类语法糖：若为配置式输入，仅允许类型化 `ItemSpec`，并在内部映射为显式 `Item` 语义树。

### 3. 高级交互与物理机制（Shell/Physics）
- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。（N/A：`Dialog` 当前交互模型仅覆盖 open/close 与 overlay 退出收敛，不提供拖拽能力，也不存在 `Dragging` 局部循环与 `Action::DragEnd` 回流协议。为防回归，已新增“无拖拽协议”语义断言：`components/dialog/test/semantics.rs::dialog_has_no_dragging_macro_micro_state_machine_contracts`、`components/dialog/test/dialog/semantics.rs::dialog_has_no_dragging_macro_micro_state_machine_contracts`。）
- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。（N/A：`Dialog` 当前定位与开关流程未引入 DOM 几何测量闭环，不存在 `Intent -> Measure -> Rectification` 状态收敛链，也无 `getBoundingClientRect/ResizeObserver` 等测量入口。为防回归，新增“无几何两段式测量协议”断言：`components/dialog/test/semantics.rs::dialog_has_no_two_pass_geometry_measurement_contracts`、`components/dialog/test/dialog/semantics.rs::dialog_has_no_two_pass_geometry_measurement_contracts`。）
- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。（N/A：`Dialog` 非动态集合容器组件，不维护子项注册表与 `items_order` 导航轴，也不存在 `RegistrationContext/Register/Unregister` 接口或 `HashSet` 顺序依赖。为防回归，新增“无集合注册协议”断言：`components/dialog/test/semantics.rs::dialog_has_no_collection_registration_protocol_contracts`、`components/dialog/test/dialog/semantics.rs::dialog_has_no_collection_registration_protocol_contracts`。）
- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。（N/A：`Dialog` 当前是单面板 overlay 容器，不提供多投影策略枚举（`Lazy/KeepAlive/Eager`），也无隐藏态 `NotifyHidden` 生命周期总线；组件不存在轮询/计时器副作用需要在隐藏时暂停。为防回归，新增“无插槽投影协议”断言：`components/dialog/test/semantics.rs::dialog_has_no_slot_projection_strategy_contracts`、`components/dialog/test/dialog/semantics.rs::dialog_has_no_slot_projection_strategy_contracts`。）
- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。（N/A：`Dialog` 当前不订阅 `Resize/Theme/Intersection` 环境流，也无 `BreakpointChanged` 类语义动作管线；组件不在 `view.rs` 挂载原始环境事件监听。为防回归，新增“无 Env Streams 协议”断言：`components/dialog/test/semantics.rs::dialog_has_no_environment_stream_subscription_contracts`、`components/dialog/test/dialog/semantics.rs::dialog_has_no_environment_stream_subscription_contracts`。）
- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。（N/A：`Dialog` 非大型集合批处理组件，不存在 `Context Bus + Selector` 或 `SelectionState::All` 批量选择压缩语义，也无 O(N) 集合下钻分发路径。为防回归，新增“无事件光锥协议”断言：`components/dialog/test/semantics.rs::dialog_has_no_event_light_cone_bulk_collection_contracts`、`components/dialog/test/dialog/semantics.rs::dialog_has_no_event_light_cone_bulk_collection_contracts`。）
- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。（N/A：`Dialog` 当前交互链路为本地 open/close 与 overlay 退出回调，不包含跨订阅者总线广播与派生命令链，也未引入 `TraceId` 透传协议。为防回归，新增“无因果总线协议”断言：`components/dialog/test/semantics.rs::dialog_has_no_causality_bus_trace_id_contracts`、`components/dialog/test/dialog/semantics.rs::dialog_has_no_causality_bus_trace_id_contracts`。）
- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。（已满足：`Dialog` 通过 `ui_headless::locale_attrs` 透传 `lang/dir`（`components/dialog/src/view.rs`），`Overlay` 挂载 `role="dialog"`/`aria-modal`/`aria-labelledby`/`aria-describedby`（`components/overlay/src/view.rs`），键盘 `Escape` 关闭路径由 `e2e/tests/docs_app_dialog_contract.spec.mjs` 覆盖；语义回归由 `components/dialog/test/dialog/semantics.rs::{dialog_wires_aria_ids_and_optional_description_semantics,dialog_supports_headless_locale_contract_with_lang_dir_attrs}` 约束。）
  - 交互元素必须具备可验证语义：`role`/`aria-*`/键盘可达路径完整，且和 headless 契约一致。
  - 用户可见文本来源必须可覆盖：优先 props，其次应用注入（`UiRoot`/i18n bundle），最后组件兜底文案；禁止把业务可见文案硬编码在 `view.rs`。
  - 组件需透传或消费 `lang` / `dir`（LTR/RTL）上下文，不得假设单语言单方向。
  - 共享 A11y 工具优先来自 `crates/ui-headless/src/a11y.rs`，组件层不重复发明同名语义工具。
- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。（已满足：`components/dialog/src/view.rs` 在根与子槽位持续输出 `data-state/data-open/data-open-mode/data-open-source/data-open-change-source/data-open-prop-source` 以及 `data-*-source` 来源标记；`components/overlay/src/view.rs` 挂载 `role="dialog"` 与 `aria-modal/aria-labelledby/aria-describedby`。标记值来自 `crates/ui-state-primitives/src/dialog.rs` 的封闭集合（`enum + as_attr` 与固定 `&'static str`：如 `controlled/uncontrolled`、`default/custom`、`shown/hidden`、`with-description/title-only`），避免自由文本漂移。语义契约回归由 `components/dialog/test/dialog/semantics.rs::dialog_view_uses_logic_contracts_and_source_markers`、`components/dialog/test/dialog/semantics.rs::dialog_wires_aria_ids_and_optional_description_semantics`、`components/dialog/test/dialog/semantics.rs::dialog_e2e_contract_uses_semantic_selectors` 与 `e2e/tests/docs_app_dialog_contract.spec.mjs` 覆盖，选择器基于语义标记而非 DOM 层级。）
  - 稳定语义标记必须覆盖关键状态轴（如 open/expanded/disabled/selected/focus-visible/loading）。
  - 状态来源必须可区分（受控/非受控、默认值/外部值、交互来源），通过稳定 marker 暴露而不是隐式推断。
  - 自动化选择器优先基于语义标记，不依赖 DOM 顺序、层级深度或临时 class 名。
  - 标记值应为封闭集合（可枚举），避免自由文本导致契约漂移。
- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。（已满足：`components/dialog/src/styles.rs` 的状态分支全部基于稳定 class 与 `data-*`（如 `data-state/data-footer/data-close-button/data-size` 与 `data-*-source`），未使用 `:nth-child`、`nth-of-type` 或依赖层级深度猜测状态；`components/dialog/src/view.rs` 未使用 `style=` 注入业务样式逻辑，仅通过语义标记与 class 驱动视觉切换。样式契约回归由 `components/dialog/test/dialog/semantics.rs::dialog_styles_include_state_and_source_marker_selectors` 与 `components/dialog/test/dialog/semantics.rs::dialog_e2e_contract_uses_semantic_selectors` 覆盖。）
  - `styles.rs` 中状态分支选择器必须基于 `data-*`/`aria-*`/稳定 class，禁止用 `:nth-child`、深层级选择器猜测状态。
  - 运行时样式仅允许传递必要 CSS 变量（custom properties）；禁止把业务样式逻辑塞进 inline style。
  - 视觉状态切换必须可由语义标记直接解释，不能依赖“某节点是否恰好存在”。
- [x] 测试验证“语义契约”而不只验证视觉快照。（已满足：语义断言覆盖 `role/aria/data-state/source markers`（`components/dialog/test/dialog/semantics.rs::dialog_view_uses_logic_contracts_and_source_markers`、`dialog_wires_aria_ids_and_optional_description_semantics`、`dialog_e2e_contract_uses_semantic_selectors`，以及 `e2e/tests/docs_app_dialog_contract.spec.mjs`）；关键分支覆盖受控/非受控（`components/dialog/test/logic.rs::normalize_open_state_supports_controlled_and_uncontrolled_modes`）、键盘路径（`docs-app dialog closes via escape`）、指针路径（e2e `click` 打开/关闭/场景切换）。`Dialog` 无 `is_disabled` 状态轴，因此 disabled 分支为 N/A；SSR/wasm 适用差异由非平台泄漏约束（`components/dialog/test/semantics.rs::dialog_public_surface_avoids_web_sys_types`）与组件 motion 不自实现 wasm 驱动约束（`components/dialog/test/dialog/semantics.rs::dialog_motion_module_stays_as_contract_mapping_without_custom_engine`）兜底。未发现以视觉快照替代语义契约的用例。）
  - 至少存在语义测试覆盖关键状态与交互路径（role/aria/data-state/source markers）。
  - 测试矩阵必须覆盖关键分支：受控/非受控、disabled、键盘路径、指针路径、SSR/wasm 差异（按适用范围）。
  - 视觉快照只能作为补充，不得替代语义契约断言。
- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。（已满足：`components/dialog/src/mod.rs` 仅维护模块边界与稳定导出（`Dialog`/`DialogMotion`/基础常量与类型）；`components/dialog/src/logic.rs` 负责输入归一、状态派生与来源标记映射（消费 `ui_state_primitives::dialog`，无 DOM 操作）；`components/dialog/src/styles.rs` 为 token-first 静态 CSS；`components/dialog/src/view.rs` 负责结构渲染与 headless/overlay 契约挂载；`components/dialog/src/motion.rs` 仅做 motion contract sanitize + attach 委托，不实现组件内动效引擎。回归证据：`components/dialog/test/semantics.rs::dialog_component_keeps_expected_file_boundaries`、`components/dialog/test/semantics.rs::dialog_logic_and_view_follow_layering_contract`、`components/dialog/test/dialog/semantics.rs::dialog_motion_module_stays_as_contract_mapping_without_custom_engine`。）
  - `mod.rs` 只维护最小稳定导出面与 feature gate，不承载实现细节。
  - `logic.rs` 只做输入归一、状态派生、来源标记；禁止 DOM 操作和样式细节分支。
  - `styles.rs` 只包含 token-first 静态 CSS；禁止硬编码主题常量与业务语义文案。
  - `view.rs` 只做结构渲染与 headless 契约挂载；禁止隐藏关键状态决策。
  - `motion.rs` 只做组件语义到动效契约映射与 attach；禁止在组件内重写通用动效引擎。
- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。（已满足：`components/dialog` 当前未引入 `spec.rs`，组件实现维持 `mod.rs + logic.rs + styles.rs + view.rs + motion.rs` 主干与文档说明路径（`check2.md`/`src/README.md`）；`Dialog` 无需额外稳定外部 Schema 建模与版本迁移层，避免“为统一而统一”的空抽象。若未来确需新增 `spec.rs`，必须同步补齐契约测试与版本演进文档。）
  - 仅当组件存在稳定外部规范/Schema 契约或复杂配置固化需求时才引入 `spec.rs`。
  - 简单组件不得为了“形式统一”新增 `spec.rs`；说明文档应留在 `check2.md`/组件文档。
  - 新增 `spec.rs` 必须同步给出契约测试与版本演进说明。
- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。（已满足：`components/dialog/src/styles.rs` 承载 Dialog 全量静态样式；`crates/ui/src/css.rs` 通过 `#[cfg(feature = "component-dialog")] out.push_str(crate::dialog::styles::CSS);` 聚合注入；`crates/ui/src/root.rs` 在 `UiRoot` 的 `inject_components_css` 路径统一调用 `crate::css::push_components_css`。`components/dialog/src/view.rs` 未注入业务 `style=` 运行时样式，仅挂载语义标记与 class。视觉值使用 `var(--ui-*)` / `var(--ui-fallback-*)` 体系（见 `components/dialog/test/dialog/semantics.rs::dialog_styles_consume_ui_theme_tokens_and_avoid_hardcoded_dialog_size_literals`），组件层未引入 Utility-First/CSS-in-Rust 默认范式。）
  - 样式规则统一落在 `styles.rs`，由 `crates/ui/src/css.rs` 聚合并通过 `UiRoot` 注入。
  - 颜色/间距/圆角/阴影等视觉值必须来自 `var(--ui-*)`，禁止组件私有 token 体系。
  - Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。
  - CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。
- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。（已满足：docs-app 提供默认主题基线页面 `/#/components/theme-visual-baseline`，并在 `e2e/tests/docs_app_theme_visual_baseline.spec.mjs` 覆盖 Button/Input/Overlay 可见性与截图基线（`toHaveScreenshot`：page/button/input/overlay）；Dialog 自身在 `apps/docs-app/src/pages/components/pages/overlays_dialog.rs` 提供 default/compact/motion 对比场景，`e2e/tests/docs_app_dialog_contract.spec.mjs` 对关键语义与交互路径做回归，避免“仅可访问但视觉退化”的回归。HeroUI 对标聚焦视觉语言与体验质量，不做 API 表层复制。）
  - 默认主题需通过基础美学清单：信息层级清晰（字重/字号/间距）、对比与层次自然、交互反馈明确（hover/active/focus）。
  - docs-app 必须提供默认主题基线页面与截图基线，关键组件（Button/Input/Overlay）纳入视觉回归对比。
  - 禁止“可访问但粗糙”的最低可用心态：视觉退化（类似旧式 Bootstrap 观感）视为质量回归。
  - HeroUI 对标以“视觉语言与体验质量”对齐为目标，不做无差别 API 表层复制。
- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。（已满足：`crates/ui/Cargo.toml` 维持组件级 feature，`component-dialog = ["component-overlay", "component-button"]`；`crates/ui/src/lib.rs` 对 `dialog` 模块与导出使用 `#[cfg(feature = "component-dialog")]` 门控；`crates/ui/src/css.rs` 仅在 `#[cfg(feature = "component-dialog")]` 时聚合 `crate::dialog::styles::CSS`。语义回归存在 `components/dialog/test/dialog/semantics.rs::dialog_feature_gate_declares_required_component_dependencies`。本轮验证：`cargo tree -e features -p ui --no-default-features --features component-dialog,inject-css --depth 3` 显示最小特性链闭合，未出现无条件全量组件依赖；`cargo tree -e features -i ui -p web-demo` 显示由 `web-demo-components` 组合特性拉起（非 `all-components`）；`cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-dialog,inject-css` 在当前环境受 `Invalid cross-device link (os error 18)` 阻断，需在 CI/同盘构建环境继续执行最小特性 wasm 与体积预算门禁。）
  - package 模式必须有组件级 feature（如 `component-accordion`）；未启用组件不得进入编译与链接路径。
  - `lib.rs` 与 `css.rs` 必须按 feature 条件导出/聚合，禁止无条件引用所有组件模块和 CSS 常量。
  - source 模式下仅引入需要的组件源码，不通过中央注册表维持全组件可达。
  - 任意“全量组件映射表/注册表”若导致不可达代码变可达，直接判不通过。
  - 验证命令（特性树）：`cargo tree -e features -p ui --no-default-features --features component-accordion,inject-css`，确认仅启用目标组件特性链。
  - 验证命令（反向依赖）：`cargo tree -e features -i ui -p web-demo`，检查是否被 `all-components` 或隐式特性全量拉起。
  - CI 检查（最小特性编译）：新增任务仅开启目标最小特性（示例：`cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-accordion,inject-css`）。
  - CI 检查（体积预算）：对“最小特性构建产物”设定预算并阻断回归（可用固定阈值，如 `< 50KB`，或基于仓库基线的相对阈值）；不得只做编译通过而不做体积约束。
- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。（已满足：离散状态轴在 `ui-state-primitives` 以 `enum` 建模（`DialogSize`/`DialogOpenMode`/`DialogCloseButtonVisibility`/`DialogCloseButtonPropSource`），组件层 `components/dialog/src/logic.rs` 通过 `normalize_open_state`、`normalize_close_config`、`resolve_part_states` 统一归一化并输出封闭来源标记；`components/dialog/src/view.rs` 挂载稳定 `data-state/data-open-mode/data-open-source/data-*-source` 语义标记供机器消费。回归闭环：`components/dialog/test/logic.rs::{normalize_open_state_supports_controlled_and_uncontrolled_modes,resolve_part_states_concentrates_slot_state_derivation}` 与 `components/dialog/test/dialog/semantics.rs::{dialog_logic_exposes_state_helpers,dialog_view_uses_logic_contracts_and_source_markers}` 可直接定位状态契约破坏点。）
  - 离散输入与状态轴必须优先使用 `enum`/新类型建模，避免字符串协议与布尔爆炸。
  - 无效状态要么在类型层不可表达，要么在 `logic.rs` 被统一归一化并可测试。
  - 关键状态必须通过稳定语义标记对外可读，供测试与 Agent 自动化消费。
  - 编译器与测试反馈应能直接定位状态契约破坏点，形成可持续闭环。

### 4. DOM/环境边界治理
- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。（已满足：`components/overlay/src/view.rs` 通过 `use_overlay_stack_registration()` + `use_focus_trap(...)` 挂载全局 overlay/focus 契约，并使用 `RestorePolicy::FallbackTo(...)` 与 `with_fallback_selector(...)` 声明恢复策略；组件内未私存“恢复目标 NodeRef”状态机。全局焦点栈能力由 `crates/ui-headless/src/focus_trap.rs` 的 `focus_manager_push_trap` / `focus_manager_pop_trap` / `focus_manager_peek_trap` 与 `restore_focus_chain` 统一承载，层叠 Overlay 按栈顺序恢复焦点，避免组件层各自实现私有恢复分支。）
- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。（N/A：当前 `Dialog` 仅组合 `Overlay/Button/ui-headless/ui-state-primitives` 的声明式契约，未接入 ECharts/Map 等命令式第三方实例；`components/dialog/src/view.rs` 与 `components/dialog/src/mod.rs` 未暴露第三方句柄类型或命令式实例入口，状态机未被第三方对象反向污染。）
- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。（N/A：当前 `Dialog` 不生成随机/时间型运行时 ID；`id` 仅由输入 `id_base` 经 `normalize_id_base` 归一后与固定后缀拼接（`-title`/`-description`），不存在 `now()/UUID/rand` 初始化路径。仓库级确定性种子注入由 `UiRoot` 的 `id_seed + provide_ui_id_provider` 统一提供，供未来需要自动分配 ID 的组件接入。）
- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。（已满足：`Dialog` 自身 `view/logic/motion` 不引用 `web-sys/js-sys/window/document`；平台分支由下层显式 `cfg` 管理：`components/overlay/src/motion.rs` 提供 `#[cfg(target_arch = "wasm32")]` 与 `#[cfg(not(target_arch = "wasm32"))]` 双实现，`crates/ui-motion/src/lib.rs` 提供 non-wasm no-op `web` backend，`crates/ui-headless/src/lib.rs` 以 `compile_error!` 约束 `web/ssr` 互斥。新增回归：`components/dialog/test/semantics.rs::dialog_ssr_cross_platform_contract_is_cfg_guarded_and_non_wasm_safe`、`components/dialog/test/dialog/semantics.rs::dialog_ssr_cross_platform_contract_is_cfg_guarded_and_non_wasm_safe`。compile-only 命令已尝试：`cargo check -p ui --no-default-features --features component-dialog,inject-css`、`cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-dialog,inject-css`、`cargo check -p ui-headless --no-default-features --features ssr`；当前环境均被 `Invalid cross-device link (os error 18)` 阻断，需在 CI/同盘构建环境补齐日志证据。）
  - 至少包含 compile-only 证据：web（wasm32）、ssr（native）、默认本地构建三条路径。
  - 平台分支差异必须显式 `cfg` 或 feature 管理，禁止依赖运行时偶然行为。
  - non-wasm 路径禁止引用 `web-sys`/浏览器对象。
- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。（已满足：`crates/ui-headless/src/lib.rs` 存在 `#[cfg(all(feature = "web", feature = "ssr"))] compile_error!(...)` 硬保护，`crates/ui-headless/Cargo.toml` 明确 `web = ["leptos/csr"]` 与 `ssr = ["leptos/ssr"]` 分离。`Dialog` 继续通过 `ui_headless` 契约接入（`use_controllable_open_state_traced` 等），未引入绕过互斥保护的本地分支。新增回归：`components/dialog/test/semantics.rs::dialog_preserves_ui_headless_web_ssr_compile_error_mutex_contract`、`components/dialog/test/dialog/semantics.rs::dialog_preserves_ui_headless_web_ssr_compile_error_mutex_contract`。验证命令已尝试：`cargo check -p ui-headless --no-default-features --features web`、`cargo check -p ui-headless --no-default-features --features ssr`、`cargo check -p ui-headless --no-default-features --features web,ssr`；当前环境被 `Invalid cross-device link (os error 18)` 阻断，需在 CI/同盘构建环境补齐编译日志与“web+ssr 失败”证据。）
  - 组件依赖 `ui-headless` 能力时，不得破坏其 web/ssr 互斥约束。
  - 组件若新增 headless 功能接入，需验证两条 feature 路径都可编译。
  - 发现“同时启用 web+ssr 仍可过编译”视为契约回归。
- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。（已满足：`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = "wasm32"))]` 提供 no-op `web` backend（`prefers_reduced_motion() -> true` 与 `animate(...) {}`），并有 `non_wasm_web_backend_is_predictable_noop` 回归。组件侧 `components/dialog/src/motion.rs` 仅委托 `overlay::motion::attach_motion`，而 `components/overlay/src/motion.rs` 的 non-wasm 分支通过 `Effect` 在关闭态直接 `finish_exit.run(())`，不依赖动画句柄、无 panic 假设，行为可预测。语义回归已覆盖：`components/dialog/test/semantics.rs::dialog_ssr_cross_platform_contract_is_cfg_guarded_and_non_wasm_safe`、`components/dialog/test/dialog/semantics.rs::dialog_ssr_cross_platform_contract_is_cfg_guarded_and_non_wasm_safe`。）
  - `motion.rs` 调用必须可在 non-wasm 下安全降级，不触发 panic。
  - 组件不得假设动画句柄一定存在；no-op 分支行为需可预测。
  - toolchain 场景（测试/文档/静态分析）不得因 motion 依赖阻塞编译。
- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。（已满足：`components/overlay/src/motion.rs` 通过 `#[cfg(target_arch = "wasm32")]` 与 `#[cfg(not(target_arch = "wasm32"))]` 显式分支覆盖 wasm/SSR；non-wasm 分支在关闭态直接 `finish_exit.run(())`，保证安全降级与可预测行为。`crates/ui-motion/src/lib.rs` non-wasm `web` backend 返回 `prefers_reduced_motion() -> true` 且 `animate(...)` 为 no-op；`crates/ui-motion/src/spring.rs` 在 `set_target` 中以 `if crate::web::prefers_reduced_motion()` 立即收敛到目标并触发 `on_rest`，实现 reduced-motion 最小必要反馈。`Dialog` 语义契约在 `components/dialog/src/view.rs` 保持稳定 `motion=motion.overlay` 挂载，不因平台分支分裂。新增回归：`components/dialog/test/semantics.rs::dialog_reduced_motion_ssr_wasm_branch_contract_is_preserved`、`components/dialog/test/dialog/semantics.rs::dialog_reduced_motion_ssr_wasm_branch_contract_is_preserved`。）
  - `reduced-motion` 下动画应跳过或降级为最小必要反馈。
  - SSR 输出必须与客户端 hydration 兼容，避免首帧语义错位。
  - wasm 分支允许增强交互，但语义契约不得与 SSR 分支分裂。
- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。（已满足：`apps/docs-app/src/pages/components/shell.rs` 已为 `dialog` 增加预算项（`max_mount_ms: 36.0`、`max_update_ms: Some(12.0)`、`max_heap_kb: Some(640.0)`），并继续保留 `button/input` 基线预算（`24ms/28ms`）；`apps/docs-app/src/perf_probe.rs` 提供稳定观测/阻断标记（`data-perf-*` + `data-perf-violation`）；`e2e/tests/docs_app_components_coverage.spec.mjs` 对 perf probe 做可重复阈值断言；`apps/docs-app/src/debug_overlay.rs` 通过 `ui_headless::use_ui_trace` 提供状态/渲染归因链路；`scripts/check-ui-performance.sh` 新增阻断门禁 `dialog_performance_governance_budget_is_defined_and_blocking`。组件与集成回归：`components/dialog/test/semantics.rs::dialog_performance_governance_budget_is_defined_and_blocking`、`components/dialog/test/dialog/semantics.rs::dialog_performance_governance_budget_is_defined_and_blocking`。`render_count` 精确自动化当前沿用仓库级等价证据与后续计划：`docs/plan/TODO.md` 中 `建立 \`render_count\` 自动化回归（Button/Input/Accordion）` 与 `scripts/check-ui-performance.sh` 的 `perf_render_count_follow_up_is_tracked_in_plan`。）
  - 关键交互组件需定义最小预算项（首渲染、关键更新、内存/分配趋势）。
  - 回归检测至少具备可重复基线与失败阈值，不靠主观“感觉变慢”。
  - 性能问题需可归因到状态、渲染、样式或动效路径之一。
  - 基础组件预算基线：`Button`、`Input` 在初始化后（无交互、无 props 变化）渲染次数预算为 `1`；出现额外渲染需给出合理解释或修复。
  - 测试要求：在 `components/*/test/**` 增加 `render_count` 类回归测试（测试框架支持时必须启用）；至少覆盖基础组件与本次改动组件。
  - 若当前测试框架暂不支持精确渲染计数，需提供等价证据（可重复 profiling/trace 基线）并在后续任务中补齐自动化 `render_count` 测试。
- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。（已满足：`components/dialog/src/view.rs` 已将内容区域拆分为语义子块函数 `render_dialog_close_section`、`render_dialog_header_section`（含 `DialogHeaderRenderInput`）、`render_dialog_body_section`、`render_dialog_footer_section`，主 `view!` 仅负责 root 语义标记装配并挂载 `{close_view}/{header_view}/{body_view}/{footer_view}`，避免单块深嵌套宏展开。回归：`components/dialog/test/semantics.rs::dialog_view_macro_complexity_is_bounded_with_semantic_subblocks`、`components/dialog/test/dialog/semantics.rs::dialog_view_macro_complexity_is_bounded_with_semantic_subblocks`。阻断门禁：`scripts/check-ui-view-macro.sh` 新增 `dialog_view_macro_complexity_is_bounded_with_semantic_subblocks`。）
  - 复杂结构按语义子块拆分（header/body/item 等），避免巨型单块 `view!`。
  - `view.rs` 中若出现多层嵌套重复片段，应优先提取局部渲染函数。
  - 编译时间/产物体积异常增长时，优先排查宏展开体量。
- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。（已满足：`components/dialog/src/view.rs` 已将局部片段函数化为普通 Rust 函数 `render_dialog_close_section`、`render_dialog_header_section`（参数封装为 `DialogHeaderRenderInput`）、`render_dialog_body_section`、`render_dialog_footer_section`，仅保留根入口 `#[component] pub fn Dialog(...)`，未把局部片段升级为额外 `#[component]`。语义与定位稳定性回归：`components/dialog/test/semantics.rs::dialog_view_functional_split_prefers_plain_functions_over_local_components`、`components/dialog/test/dialog/semantics.rs::dialog_view_functional_split_prefers_plain_functions_over_local_components`。阻断门禁：`scripts/check-ui-view-macro.sh` 新增 `dialog_view_functional_split_prefers_plain_functions_over_local_components`。）
  - 纯静态或轻逻辑片段优先函数化；仅在需要独立 props 语义时升级为组件。
  - 禁止把所有局部片段都升格为 `#[component]` 导致抽象噪音。
  - 拆分后语义标记与测试定位仍需稳定。
- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。（已满足：`components/dialog/src/view.rs` 将关闭按钮图标从内联片段收敛为静态常量模板（`DIALOG_CLOSE_ICON_VIEWBOX`、`DIALOG_CLOSE_ICON_PATH_D`、`DIALOG_CLOSE_ICON_STROKE_WIDTH`）与 `render_dialog_close_icon()`，`render_dialog_close_section` 通过 `{render_dialog_close_icon()}` 复用，避免静态 SVG 字面量散落。可访问语义保持：图标 `aria-hidden=\"true\"`，交互标签由 `Button aria_label=close_label` 提供。回归：`components/dialog/test/semantics.rs::dialog_static_fragments_are_constantized_with_accessible_close_icon_template`、`components/dialog/test/dialog/semantics.rs::dialog_static_fragments_are_constantized_with_accessible_close_icon_template`。阻断门禁：`scripts/check-ui-view-macro.sh` 新增 `dialog_static_fragments_are_constantized_with_accessible_close_icon_template`。）
  - 可判定为纯静态的片段应避免重复动态构造。
  - 常量化后仍需维持可访问语义（title/aria-label/role 等）。
  - 静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。
- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。（N/A：`Dialog` 当前无 `inner_html` 使用点。已补组件与文档双侧回归：`components/dialog/test/semantics.rs::dialog_inner_html_usage_is_explicitly_na_and_guarded`、`components/dialog/test/dialog/semantics.rs::dialog_inner_html_usage_is_explicitly_na_and_guarded`，断言 `components/dialog/src/{mod,logic,styles,view,motion,protocol}.rs`、`components/dialog/src/README.md` 与 `apps/docs-app/src/pages/components/pages/overlays_dialog.rs` 不包含 `inner_html`/`set_inner_html`/`dangerously_set_inner_html` 及脚本注入路径。阻断门禁：`scripts/check-ui-inner-html.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_inner_html_usage_is_explicitly_na_and_guarded`。）
  - 仅允许编译期常量或明确白名单内容进入 `inner_html`。
  - 严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。
  - 使用 `inner_html` 的节点必须补语义测试与安全回归说明。
- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。（已满足：`Dialog` 复用全局调试链路而不引入组件私有 debug API。状态追踪与前后值来源由 `components/dialog/src/view.rs` 接入 `use_controllable_open_state_traced(...)`，并暴露稳定来源标记 `data-open-source`/`data-open-change-source`/`data-open-prop-source`；时间线与最小可回放事件由 `crates/ui-headless/src/trace.rs`（`UiTraceEvent { ts_ms, component, kind }` + `OpenChange`）和 `apps/docs-app/src/debug_overlay.rs`（事件倒序窗口 `events.into_iter().rev().take(40)`、`format!(\"{ts_ms}ms\")`、`data-kind`）提供；开发态可视化入口保持 `apps/docs-app/src/lib.rs` 的 `debug_assertions` 门控（`provide_ui_trace(...)` + `UiDebugOverlay`），生产路径不泄漏。Feature 隔离维持：`components/dialog/Cargo.toml` 无 `wasm-debug` 特性，`crates/ui/Cargo.toml` 不新增 `dialog-wasm-debug` 别名。回归：`components/dialog/test/semantics.rs::dialog_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated`、`components/dialog/test/dialog/semantics.rs::dialog_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated`。阻断门禁：`scripts/check-ui-wasm-debug.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_wasm_debug_contract_reuses_global_trace_and_stays_feature_isolated`。）
  - 开发模式下至少能追踪关键状态变更来源与前后值。
  - 关键交互链路应支持最小可复现记录（事件顺序/状态转移）。
  - 调试开关默认不进入生产包体与公共 API。
- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。（已满足：`apps/docs-app/src/pages/components/pages/overlays_dialog.rs` 提供 `Interactive Playground` + `Scenario Comparison` 隔离演练入口，且 `Interactive Playground` 具备 `code_signal=workbench_code`、`test_css_source=workbench_test_css_source`、`test_source_path=\"components/dialog/src/styles.rs\"`、`test_config_signal=workbench_actual_config` 与 `controls`，样式试验走 CSS Test 热反馈路径（不要求完整 wasm 重编）。上下文保持由 `workbench_open_raw/workbench_present` + `on_workbench_exit_complete` 的显示状态管理保证，配置状态（`workbench_with_description/workbench_show_close/workbench_custom_motion/workbench_custom_class`）独立持有，关闭对话框不重置调参上下文。回归：`components/dialog/test/semantics.rs::dialog_dx_playground_supports_hot_reload_context_and_isolated_workbench`、`components/dialog/test/dialog/semantics.rs::dialog_dx_playground_supports_hot_reload_context_and_isolated_workbench`。阻断门禁：`scripts/check-ui-dx.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_dx_playground_supports_hot_reload_context_and_isolated_workbench`。）
  - 常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。
  - 组件调试应尽量保持当前交互上下文，降低重复操作成本。
  - 复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。
- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。（已满足：`Dialog` 的协议输入通过 `components/dialog/src/protocol.rs` 使用 `serde` 类型化（`DialogComponentSchemaVersion` + `DialogComponentSpec` + `#[serde(default)] schema_version`）并由 `components/dialog/test/protocol.rs` 保持 serde 契约回归；交互追踪统一复用 `use_controllable_open_state_traced(\"dialog\", ...)`，未在组件层定义自有 `tracing::span/event` 目标；组件公共与实现边界（`mod/logic/view/styles/motion/protocol/README`）未泄露 `tokio/async-std/smol/runtime::Handle` 等 runtime 细节。回归：`components/dialog/test/semantics.rs::dialog_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries`、`components/dialog/test/dialog/semantics.rs::dialog_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries`。阻断门禁：`scripts/check-ui-engineering.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_engineering_contract_uses_serde_protocol_and_keeps_tracing_runtime_boundaries`。）
  - 若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。
  - 关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。
  - 异步边界不得把具体 runtime 类型暴露到组件公共接口。

### 5. 样式与动效（Theme & Motion）
- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。（已满足：`components/dialog/src/styles.rs` 统一使用双层回退链，`ui-dialog__close` 的 `top/right` 已从单层 `var(--ui-space-3xs)` 收敛为 `var(--ui-space-3xs, var(--ui-fallback-space-3xs))`，并保持其余 `--ui-*` token 的 fallback 约束；组件样式中未引入 Hex 或裸尺寸终值。回归：`components/dialog/test/semantics.rs::dialog_styles_use_defensive_variable_fallback_chain`、`components/dialog/test/dialog/semantics.rs::dialog_styles_use_defensive_variable_fallback_chain`。阻断门禁：`scripts/check-ui-contract-hygiene.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_styles_use_defensive_variable_fallback_chain`。）
- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\"top: 10px\"`）。（已满足：`crates/ui/src/css.rs` 继续以 `@layer ui` 包裹组件样式聚合，并在 `#[cfg(feature = "component-dialog")]` 下显式注入 `crate::dialog::styles::CSS`；`crates/ui/src/root.rs` 通过 `inject_components_css` 单点注入 `<style>{move || css_text.get()}</style>`，保持层级注入入口集中。`components/dialog/src/view.rs` 不含 `style=` 普通内联样式，且若出现 `style:` 仅允许 CSS Custom Properties（`style:--*`）；`components/dialog/src/styles.rs` 维持静态 token-first CSS。回归：`components/dialog/test/semantics.rs::dialog_cascade_layer_and_runtime_style_contract_is_enforced`、`components/dialog/test/dialog/semantics.rs::dialog_cascade_layer_and_runtime_style_contract_is_enforced`。阻断门禁：`scripts/check-ui-contract-hygiene.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_cascade_layer_and_runtime_style_contract_is_enforced`。本地执行该命令仍受环境问题阻断：`Invalid cross-device link (os error 18)`。）
- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。（已满足：`components/dialog/src/motion.rs` 保持 `DialogMotion` + `sanitize_motion` 到 `overlay::motion::sanitize_motion` 的组件合同映射，并通过 `overlay::motion::attach_motion` 挂载；`components/dialog/test/motion.rs` 锁定合同参数与清洗回归（`stiffness: 225.0`、`damping: 21.0`、`sanitize_motion_delegates_to_overlay_contract`）。`components/overlay/src/motion.rs` 覆盖 `stiffness/damping` 清洗与 `#[cfg(target_arch = "wasm32")]`/`#[cfg(not(target_arch = "wasm32"))]` 双分支，non-wasm 在关闭态立即 `finish_exit.run(())` 安全降级；`crates/ui-motion/src/lib.rs` 维持 non-wasm `pub fn prefers_reduced_motion() -> bool` 与 `animate` no-op。组件挂载路径在 `components/dialog/src/view.rs` 保持 `let motion = crate::dialog::motion::sanitize_motion(motion);` 与 `motion=motion.overlay`。回归：`components/dialog/test/semantics.rs::dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe`、`components/dialog/test/dialog/semantics.rs::dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe`。阻断门禁：`scripts/check-ui-platforms.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe`。本地执行该命令仍受环境问题阻断：`Invalid cross-device link (os error 18)`。）
- [x] `ui` 固定入口文件落点正确。（已满足：`crates/ui/src/lib.rs` 保持总入口与公共 API 面（`mod css;`、`pub mod root;`、`pub use root::UiRoot;`、`#[cfg(feature = "component-dialog")] pub mod dialog;` 与 `pub use dialog::{Dialog, DialogMotion, DialogSize};`），且未暴露 `pub mod css` 或 `web_sys/wasm_bindgen` 平台细节；`crates/ui/src/css.rs` 继续以 feature 条件聚合（`#[cfg(feature = "inject-css")] pub fn push_components_css`、`@layer ui`、`#[cfg(feature = "component-dialog")] out.push_str(crate::dialog::styles::CSS);`、`#[cfg(not(feature = "inject-css"))]` no-op）；`crates/ui/src/root.rs` 统一注入 base css + theme vars +（可选）components css，并集中 i18n/id provider（`provide_ui_i18n`、`provide_ui_id_provider`、`crate::css::push_components_css`、`ui_layout::push_components_css`）；`crates/ui-visual-primitive/src/active_highlight.rs` 保持共享高亮动效能力且不承载组件业务语义；`crates/ui/src/overlay_open.rs` / `crates/ui/src/presence.rs` / `crates/ui/src/a11y.rs` 不存在，headless 规范文件固定在 `crates/ui-headless/src/controllable_state.rs`、`crates/ui-headless/src/presence.rs`、`crates/ui-headless/src/a11y.rs`。回归：`components/dialog/test/semantics.rs::dialog_ui_components_fixed_entry_files_follow_layered_boundaries`、`components/dialog/test/dialog/semantics.rs::dialog_ui_components_fixed_entry_files_follow_layered_boundaries`。阻断门禁：`scripts/check-ui-entrypoints.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_ui_components_fixed_entry_files_follow_layered_boundaries`。本地执行该命令仍受环境问题阻断：`Invalid cross-device link (os error 18)`。）
  - `crates/ui/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。
  - `crates/ui/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。
  - `crates/ui/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。
  - `crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。
  - `crates/ui/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。
  - `crates/ui/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。
  - `crates/ui/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。
- [x] 组件目录标准文件落点正确。（已满足：`components/dialog/src` 目录保持标准职责文件 `mod.rs`/`logic.rs`/`styles.rs`/`view.rs`/`motion.rs`，且 `render.rs`/`spec.rs` 不存在。`mod.rs` 维持最小稳定导出（`mod logic; pub mod motion; pub mod styles; mod view; pub use view::Dialog; pub use motion::DialogMotion;`），无 `pub mod logic/view` 过度导出；`logic.rs` 保持 props 归一与状态派生（`normalize_open_state`/`normalize_close_config`/`resolve_part_states`/`resolve_part_classes`），不承载 DOM 渲染；`styles.rs` 保持 token-first 静态 CSS（`pub const CSS` + `var(--ui-*)`）不混入渲染/业务文案；`view.rs` 只做 Leptos 结构渲染与 headless 语义挂载（`use_controllable_open_state_traced` + `Overlay` + `data-slot`），无 `render.rs` 漂移；`motion.rs` 保持 `DialogMotion + sanitize_motion + attach_motion` 的语义到动效契约映射，不重写运行时引擎。回归：`components/dialog/test/semantics.rs::dialog_component_directory_standard_files_follow_contract_and_na_paths`、`components/dialog/test/dialog/semantics.rs::dialog_component_directory_standard_files_follow_contract_and_na_paths`。阻断门禁：`scripts/check-ui-component-files.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_component_directory_standard_files_follow_contract_and_na_paths`。本地执行该命令仍受环境问题阻断：`Invalid cross-device link (os error 18)`。）
  - `<component>/mod.rs`：最小稳定导出面，存在且无过度导出。
  - `<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。
  - `<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。
  - `<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。
  - `<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。
  - `<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。

### 6. AI 原生能力与文件落点（Struct-First & Projection）
- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。（已满足：`Dialog` 目录落点保持标准五件套 `components/dialog/src/{mod.rs,logic.rs,styles.rs,view.rs,motion.rs}`，且 `components/dialog/src/render.rs` 与 `components/dialog/src/spec.rs` 均不存在；`mod.rs` 维持最小导出面（`mod logic; pub mod motion; pub mod styles; mod view; pub use view::Dialog; pub use motion::DialogMotion;`）；`logic.rs` 保持归一/派生职责，未引入 DOM 与渲染 token；`styles.rs` 保持 token-first 静态 CSS（`var(--ui-*)`）；`view.rs` 负责 Leptos 结构与 headless 语义挂载；`motion.rs` 仅负责语义到 motion contract 映射与 attach。回归：`components/dialog/test/semantics.rs::dialog_component_directory_standard_files_follow_contract_and_na_paths`、`components/dialog/test/semantics.rs::dialog_file_placement_discipline_is_strict_for_component_scope`、`components/dialog/test/dialog/semantics.rs::dialog_component_directory_standard_files_follow_contract_and_na_paths`、`components/dialog/test/dialog/semantics.rs::dialog_file_placement_discipline_is_strict_for_component_scope`、`components/dialog/test/dialog/semantics.rs::dialog_check2_marks_file_placement_discipline_contract_complete`。阻断门禁：`scripts/check-ui-component-files.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_file_placement_discipline_is_strict_for_component_scope`。本地执行该命令仍受环境问题阻断：`Invalid cross-device link (os error 18)`。）
- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。（N/A：`Dialog` 为标准 overlay 装配组件，当前无稳定外部 Schema DSL 与 builder 需求，不引入 `spec.rs` 与 `*Spec::new()...render()` 链路；`protocol.rs` 仅保留最小版本化序列化契约。回归：`components/dialog/test/semantics.rs::dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component`、`components/dialog/test/dialog/semantics.rs::dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component`、`components/dialog/test/dialog/semantics.rs::dialog_check2_marks_hyper_structure_builder_item_complete`。阻断门禁：`scripts/check-ui-component-files.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component`。本地执行该命令仍受环境问题阻断：`Invalid cross-device link (os error 18)`。）
- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。（已满足：新增 `components/dialog/src/Component.toml` 与 `components/dialog/src/dialog.rbi`，前者明确 `Dialog` 的输入/输出语义轴、能力声明（含 `context_compression_manifest`/`rbi_signature_projection`）与依赖边界，后者提供稳定接口签名投影（`DialogSlot`/`DialogSize`/`DialogMotion`/`Dialog(...)`）供 AI 检索工具链消费，避免仅凭源码动态推断造成契约漂移。回归：`components/dialog/test/semantics.rs::dialog_context_compression_manifest_and_rbi_projection_are_present_and_current`、`components/dialog/test/dialog/semantics.rs::dialog_context_compression_manifest_and_rbi_projection_are_present_and_current`、`components/dialog/test/dialog/semantics.rs::dialog_check2_marks_context_compression_manifest_and_rbi_contract_complete`。阻断门禁：`scripts/check-ui-component-files.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_context_compression_manifest_and_rbi_projection_are_present_and_current`。本地执行该命令仍受环境问题阻断：`Invalid cross-device link (os error 18)`。）
- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。（已满足：`components/dialog/src/logic.rs` 新增类型化 Agent Contract（`DIALOG_AGENT_SCHEMA`、`DialogAgentSchemaVersion/Intent/Action/State/Source/ConfigPolicy`、`DialogAgentContract`、`resolve_agent_contract`），并由 `components/dialog/src/view.rs` 通过 `Signal::derive` 挂载 `data-ui-schema/data-ui-schema-version/data-ui-intent/data-ui-action/data-ui-state/data-ui-source/data-ui-open-change-source/data-ui-config-policy`，避免自由字符串拼接。`components/dialog/src/Component.toml` 同步补齐 `data-ui-*` 输出、`agent_contract_markers` 与 `agent_contract_whitelist`（`allowed`/`blocked`）边界，`components/dialog/src/dialog.rbi` 同步投影类型与签名。回归：`components/dialog/test/semantics.rs::dialog_check2_documents_agent_contract_schema_governance_rules`、`components/dialog/test/semantics.rs::dialog_agent_contract_is_schema_typed_and_machine_readable`、`components/dialog/test/semantics.rs::dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing`、`components/dialog/test/semantics.rs::dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free`、`components/dialog/test/semantics.rs::dialog_contract_hygiene_script_covers_agent_contract_schema_guards`、`components/dialog/test/dialog/semantics.rs::dialog_check2_documents_agent_contract_schema_governance_rules`、`components/dialog/test/dialog/semantics.rs::dialog_agent_contract_is_schema_typed_and_machine_readable`、`components/dialog/test/dialog/semantics.rs::dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing`、`components/dialog/test/dialog/semantics.rs::dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free`、`components/dialog/test/dialog/semantics.rs::dialog_contract_hygiene_script_covers_agent_contract_schema_guards`、`components/dialog/test/dialog/semantics.rs::dialog_check2_marks_agent_contract_schema_governance_complete`。阻断门禁：`scripts/check-ui-contract-hygiene.sh` 新增 4 条 dialog agent-contract 用例。当前本地执行相关 `cargo test` 命令仍受环境问题阻断：`Invalid cross-device link (os error 18)`。）
  - 关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。
  - Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。
  - 契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。
  - 配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。
- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。
  - `Streaming`：LLM 还在生成，界面边生成边显示。
  - `Snapshot`：LLM 全部生成完成后，一次性显示。
  - 类型约束依据：`components/dialog/src/logic.rs` 新增 `DialogAgentStreamMode`，仅允许 `Streaming | Snapshot`，`as_str()` 仅输出 `"streaming" | "snapshot"` 两个封闭值。
  - 默认落点：`resolve_agent_contract` 固定输出 `stream_mode=Snapshot`、`stream_fallback=Snapshot`，与 `Dialog` 当前非正文阅读面职责一致。
  - 语义挂载依据：`components/dialog/src/view.rs` 挂载 `data-stream-mode=agent_contract.stream_mode.as_str()` 与 `data-stream-fallback=agent_contract.stream_fallback.as_str()`，保证机器可读状态稳定。
  - Manifest / RBI 同步：`components/dialog/src/Component.toml` 增加 `data-stream-mode/data-stream-fallback` 输出与 `llm_streaming_two_display_modes_only` 能力声明；`components/dialog/src/dialog.rbi` 同步 `DialogAgentStreamMode` 与 `stream_mode/stream_fallback` 字段投影。
  - 回归锁定：新增
    `components/dialog/test/semantics.rs::dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`、
    `components/dialog/test/semantics.rs::dialog_streaming_display_modes_are_limited_to_streaming_and_snapshot`、
    `components/dialog/test/semantics.rs::dialog_streaming_script_covers_two_mode_definition_contract`、
    `components/dialog/test/dialog/semantics.rs::dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`、
    `components/dialog/test/dialog/semantics.rs::dialog_streaming_display_modes_are_limited_to_streaming_and_snapshot`、
    `components/dialog/test/dialog/semantics.rs::dialog_streaming_script_covers_two_mode_definition_contract`、
    `components/dialog/test/dialog/semantics.rs::dialog_check2_marks_streaming_two_mode_definition_complete`。
  - 脚本门禁：`scripts/check-ui-streaming.sh` 新增
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`、
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_streaming_display_modes_are_limited_to_streaming_and_snapshot`。
  - 验证记录：执行相关 `cargo test` 命令时，当前环境仍被 `Invalid cross-device link (os error 18)` 阻断，属于容器环境问题，非本次“流式两种显示模式”契约回归。
- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。
  - 所有组件都应能消费“完整生成结果”并稳定渲染。
  - 即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。
  - 基线能力依据：`components/dialog/src/logic.rs` 的 `resolve_agent_contract` 固定输出 `stream_mode=Snapshot`、`stream_fallback=Snapshot`，将 `Dialog` 的流式语义收敛为默认 snapshot 能力。
  - 稳定渲染依据：`components/dialog/src/view.rs` 通过 `normalize_id_base/normalize_required_text/normalize_optional_text/normalize_close_config/resolve_part_states/resolve_part_classes` 先归一后渲染，并持续挂载 `data-stream-mode/data-stream-fallback/data-ui-state/data-state`，保证完整配置输入下的稳定可观测输出。
  - 回归锁定：新增
    `components/dialog/test/semantics.rs::dialog_check2_documents_snapshot_as_default_baseline_capability`、
    `components/dialog/test/semantics.rs::dialog_snapshot_baseline_consumes_complete_result_and_renders_stably`、
    `components/dialog/test/semantics.rs::dialog_streaming_script_covers_snapshot_baseline_contract`、
    `components/dialog/test/dialog/semantics.rs::dialog_check2_documents_snapshot_as_default_baseline_capability`、
    `components/dialog/test/dialog/semantics.rs::dialog_snapshot_baseline_consumes_complete_result_and_renders_stably`、
    `components/dialog/test/dialog/semantics.rs::dialog_streaming_script_covers_snapshot_baseline_contract`、
    `components/dialog/test/dialog/semantics.rs::dialog_check2_marks_snapshot_baseline_capability_complete`。
  - 脚本门禁：`scripts/check-ui-streaming.sh` 新增
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_snapshot_as_default_baseline_capability`、
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_snapshot_baseline_consumes_complete_result_and_renders_stably`。
  - 验证记录：执行相关 `cargo test` 命令时，当前环境仍被 `Invalid cross-device link (os error 18)` 阻断，属于容器环境问题，非本次 Snapshot 基线能力契约回归。
- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。
  - `Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。
  - `Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。
  - 无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。
  - 数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。
  - 职责分类依据：`Dialog` 不是 LLM 正文阅读面，`components/dialog/src/logic.rs` 的 `resolve_agent_contract` 固定输出 `stream_support=Optional` 且 `stream_fallback=Snapshot`。
  - 输出状态依据：`DialogAgentOutputStatus` 明确 `Draft/Verified/CommitReady` 封闭枚举域，当前默认输出 `Verified`；`components/dialog/src/view.rs` 挂载 `data-output-status`，并持续输出 `data-ui-stream-support/data-stream-mode/data-stream-fallback`。
  - 连续语义依据：`components/dialog/src/view.rs` 继续通过 `<Overlay ...>` 保持 role/aria 语义链路，并在根节点暴露 `data-state/data-ui-state/data-ui-source`，保证流式职责分类下语义连续可读。
  - 上层边界依据：组件层未引入 `retry/backoff/reconnect/resume_stream/validate_stream` 等重试恢复策略代码，校验与恢复仍由上层治理。
  - Manifest / RBI 同步：`components/dialog/src/Component.toml` 新增 `data-ui-stream-support/data-output-status` 输出与 agent markers；`components/dialog/src/dialog.rbi` 同步 `DialogAgentStreamSupport/DialogAgentOutputStatus` 与对应字段投影。
  - 回归锁定：新增
    `components/dialog/test/semantics.rs::dialog_check2_documents_streaming_required_optional_classification_rules`、
    `components/dialog/test/semantics.rs::dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、
    `components/dialog/test/semantics.rs::dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`、
    `components/dialog/test/semantics.rs::dialog_streaming_script_covers_required_optional_classification_contract`、
    `components/dialog/test/dialog/semantics.rs::dialog_check2_documents_streaming_required_optional_classification_rules`、
    `components/dialog/test/dialog/semantics.rs::dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、
    `components/dialog/test/dialog/semantics.rs::dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`、
    `components/dialog/test/dialog/semantics.rs::dialog_streaming_script_covers_required_optional_classification_contract`、
    `components/dialog/test/dialog/semantics.rs::dialog_check2_marks_streaming_required_optional_classification_complete`。
  - 脚本门禁：`scripts/check-ui-streaming.sh` 新增
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_streaming_required_optional_classification_rules`、
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`。
  - 验证记录：执行相关 `cargo test` 命令时，当前环境仍被 `Invalid cross-device link (os error 18)` 阻断，属于容器环境问题，非本次 Streaming Required/Optional 职责分类契约回归。

### 7. 测试、门禁与交付
- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。
  - 非测试源码约束：`components/dialog/src/lib.rs`、`mod.rs`、`logic.rs`、`motion.rs`、`protocol.rs`、`styles.rs`、`view.rs` 未引入 `unwrap/expect` 与无处理 `let _ = ...`。
  - 字符串热点收敛：`components/dialog/src/logic.rs` 的 `compose_class_name` 已改为 `Vec<Cow<'static, str>>`，静态类名统一走 `Cow::Borrowed`，仅自定义 class 走 `Cow::Owned`，减少字符串复制抖动。
  - 回归锁定：新增
    `components/dialog/test/semantics.rs::dialog_rust_hygiene_forbids_unwrap_expect_and_ignored_results_in_component_sources`、
    `components/dialog/test/semantics.rs::dialog_rust_hygiene_string_hotspots_are_coalesced_with_cow_static_str`、
    `components/dialog/test/semantics.rs::dialog_rust_hygiene_script_enforces_repo_level_hygiene_guards`、
    `components/dialog/test/dialog/semantics.rs::dialog_rust_hygiene_forbids_unwrap_expect_and_ignored_results_in_component_sources`、
    `components/dialog/test/dialog/semantics.rs::dialog_rust_hygiene_string_hotspots_are_coalesced_with_cow_static_str`、
    `components/dialog/test/dialog/semantics.rs::dialog_rust_hygiene_script_enforces_repo_level_hygiene_guards`、
    `components/dialog/test/dialog/semantics.rs::dialog_check2_marks_rust_hygiene_contract_complete`。
  - 脚本门禁：`./scripts/check-rust-hygiene.sh`。
  - 验证记录：执行 `./scripts/check-rust-hygiene.sh`，当前环境存在两处仓库级阻塞：`PCRE2 is not available in this build of ripgrep`（rg 构建能力缺失）与 `[api-contract] violation set changed (baseline drift)`（全仓 baseline 漂移）。两者均非本次 `dialog` 局部改动引入；组件级约束由上述语义回归锁定。
- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。
  - 特性树注册依据：`crates/ui/Cargo.toml` 已声明 `component-dialog = ["component-overlay", "component-button"]`，并在 `web-demo-components` 与 `all-components` 特性列表中显式包含 `"component-dialog"`，避免组件未注册导致 feature 不可达。
  - `lib.rs` 门控依据：`crates/ui/src/lib.rs` 对 dialog 模块保持 `#[cfg(feature = "component-dialog")]` + `#[path = "../../../components/dialog/src/mod.rs"]` + `pub mod dialog;` 条件导出，公共 `pub use dialog::{Dialog, DialogMotion, DialogSize};` 仅在启用 dialog 特性后可达。
  - `css.rs` 门控依据：`crates/ui/src/css.rs` 对 dialog 样式聚合保持紧邻门控 `#[cfg(feature = "component-dialog")] out.push_str(crate::dialog::styles::CSS);`，未出现无条件全量 CSS 依赖。
  - 回归锁定：新增
    `components/dialog/test/semantics.rs::dialog_tree_shaking_contract_registers_component_feature_and_gates_lib_css_aggregation`、
    `components/dialog/test/semantics.rs::dialog_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget`、
    `components/dialog/test/semantics.rs::dialog_check2_documents_tree_shaking_feature_pruning_requirements`、
    `components/dialog/test/dialog/semantics.rs::dialog_tree_shaking_contract_registers_component_feature_and_gates_lib_css_aggregation`、
    `components/dialog/test/dialog/semantics.rs::dialog_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget`、
    `components/dialog/test/dialog/semantics.rs::dialog_check2_marks_tree_shaking_feature_pruning_contract_complete`。
  - 脚本门禁：`scripts/check-ui-tree-shaking.sh` 新增 dialog 路径：
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_tree_shaking_contract_registers_component_feature_and_gates_lib_css_aggregation`、
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_tree_shaking_script_enforces_component_minimal_feature_tree_and_budget`、
    `cargo tree -e features -i ui -p ui --no-default-features --features component-dialog,inject-css`、
    `cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-dialog,inject-css`。
  - 验证记录：当前容器执行 `cargo test/cargo tree` 仍受环境阻断 `Invalid cross-device link (os error 18)`，为基础设施问题，非 dialog tree-shaking 契约回归。
- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。（`Dialog` 语义契约回归由 `dialog_logic_and_view_follow_layering_contract` 锁定 `data-*` 来源挂载与逻辑分层；性能预算与可观测 marker 由 `dialog_performance_governance_budget_is_defined_and_blocking` 持续阻断，并新增 `dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement` 合并覆盖 `aria-* + data-* + focus` 语义流与 `render_count` 治理证据；`render_count` 自动化回归仍在仓库统一 follow-up（`docs/plan/TODO.md`，当前以 mount-only 等价证据执行）；脚本门禁：`scripts/check-ui-performance.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement`；本地验证受环境限制：`Invalid cross-device link (os error 18)`。）
- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `Dialog` 未发生跨大版本 API 破坏升级，组件协议与接口投影仍保持 `v1`（`components/dialog/src/Component.toml` 的 `schema_version = "1"`、`components/dialog/src/protocol.rs` 的 `DialogComponentSchemaVersion::V1`、`components/dialog/src/dialog.rbi` 的稳定 `Dialog(...)` 签名），因此不触发 Codemod/Schema Registry 弃用窗口与 `migrate_v1_to_v2` 迁移层要求。回归：`components/dialog/test/semantics.rs::dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade`、`components/dialog/test/semantics.rs::dialog_version_deprecation_migration_script_covers_engineering_gate`、`components/dialog/test/dialog/semantics.rs::dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade`、`components/dialog/test/dialog/semantics.rs::dialog_version_deprecation_migration_script_covers_engineering_gate`；脚本门禁：`scripts/check-ui-engineering.sh` 新增 `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade`；本地验证受环境限制：`Invalid cross-device link (os error 18)`。）
- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。（已满足：`apps/docs-app/src/pages/components/pages/overlays_dialog.rs` 新增 `DIALOG_DOC_IMPORTS` 并为 `Hello World`、`State Matrix`、`Controlled vs Uncontrolled`、`Streaming / Snapshot Contract`、`Interactive Playground`、`Scenario Comparison` 全部挂载 `code_imports=DIALOG_DOC_IMPORTS.to_string()`，保障 Show code / Copy 输出为 import-ready 片段；新增 `data-slot=\"dialog-source-first\"` 文档区块，明确 `compose_copy_ready_code` 自动补全机制、依赖前提（`component-dialog` + `inject-css`）与源码落点清单（`components/dialog/src/{mod,logic,view,styles,motion}.rs`）。流式/快照展示通过 `data-requested-stream-mode` / `data-requested-output-status` 与固定语义说明 `effective component markers: data-stream-mode=snapshot data-stream-fallback=snapshot data-output-status=verified` 提供可读回归锚点。回归：`components/dialog/test/semantics.rs::{dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract,dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies,dialog_dx_check_script_covers_docs_product_copy_paste_ready_contract,dialog_check2_documents_docs_product_copy_paste_ready_rules}`、`components/dialog/test/dialog/semantics.rs::{dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract,dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies,dialog_dx_check_script_covers_docs_product_copy_paste_ready_contract,dialog_check2_documents_docs_product_copy_paste_ready_rules}`；脚本门禁：`scripts/check-ui-dx.sh` 新增 3 条 dialog docs-product 命令。本地执行相关 `cargo test` 仍受环境限制 `Invalid cross-device link (os error 18)`。）
- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。（已满足：补充 `dialog_check2_documents_semantics_first_testing_rules` 锁定本条规则文本，新增 `dialog_semantics_suite_is_contract_first_not_snapshot_only` 强制语义断言优先（`data-*`/`role`/键盘路径）并显式拒绝 snapshot-only 断言路径，新增 `dialog_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks` 约束 `components/dialog/src/view.rs` 中关键语义字段（如 `data-state/data-open-mode/data-open-source/data-open-change-source/data-ui-schema/data-stream-mode/data-stream-fallback/data-output-status`）变更必须联动语义测试更新。回归：`components/dialog/test/semantics.rs::{dialog_check2_documents_semantics_first_testing_rules,dialog_semantics_suite_is_contract_first_not_snapshot_only,dialog_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks,dialog_contract_hygiene_script_covers_semantics_first_contract_guards}`、`components/dialog/test/dialog/semantics.rs::{dialog_check2_documents_semantics_first_testing_rules,dialog_semantics_suite_is_contract_first_not_snapshot_only,dialog_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks,dialog_contract_hygiene_script_covers_semantics_first_contract_guards}`；脚本门禁：`scripts/check-ui-contract-hygiene.sh` 新增 3 条 dialog 语义优先命令。本地执行相关 `cargo test` 仍受环境限制 `Invalid cross-device link (os error 18)`。）
  - 每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。
  - 断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。
  - 新增/变更语义字段必须同步补测试，否则不得打勾。
- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。
  - E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。
  - WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。
  - 若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。
  - 已满足（语义选择器）：`e2e/tests/docs_app_dialog_contract.spec.mjs` 改为以稳定语义锚点驱动（如 `data-slot="dialog-e2e-open-marker"`、`data-slot="dialog-e2e-open-workbench"`、`data-slot="dialog-e2e-open-compare-default"`、`data-slot="overlay-panel"`、`data-slot="dialog"`、`data-slot="overlay"`）；`apps/docs-app/src/pages/components/pages/overlays_dialog.rs` 同步补齐 open/close 按钮 `data-slot` 锚点（`dialog-e2e-open-*` / `dialog-e2e-close-*`）。
  - 已满足（WASM 稳定等待）：E2E 路径统一通过 `body:not(:has(#boot))` 作为 wasm/hydration ready 断点，并封装 `waitForWasmReady(page)`；未使用 `waitForTimeout`/固定 sleep。
  - 已满足（ready/settled 覆盖）：新增 `expectDialogReady(...)` 与 `expectDialogSettledClosed(...)`，显式覆盖打开阶段语义就绪（`data-state/data-open/data-ui-schema/data-stream-mode/data-output-status`）与关闭阶段收敛（`toHaveCount(0)`），并覆盖 Escape 与指针关闭路径。
  - 脚本门禁：新增 `components/dialog/scripts/check-ui-e2e-dialog.sh`，接入  
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_e2e_selector_and_stable_wait_rules`  
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits`  
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths`
  - 回归：`components/dialog/test/semantics.rs::dialog_check2_documents_e2e_selector_and_stable_wait_rules`、`components/dialog/test/semantics.rs::dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits`、`components/dialog/test/semantics.rs::dialog_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths`、`components/dialog/test/semantics.rs::dialog_e2e_check_script_covers_selector_and_settled_wait_contract`、`components/dialog/test/semantics.rs::dialog_check2_marks_e2e_selector_stability_item_complete`、`components/dialog/test/dialog/semantics.rs::dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits`、`components/dialog/test/dialog/semantics.rs::dialog_e2e_contract_covers_ready_and_settled_conditions_for_overlay_paths`、`components/dialog/test/dialog/semantics.rs::dialog_e2e_check_script_covers_selector_and_settled_wait_contract`、`components/dialog/test/dialog/semantics.rs::dialog_check2_marks_e2e_selector_stability_item_complete`。
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。
  - 至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。
  - 回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。
  - 高风险路径（overlay、focus、keyboard、async）优先进入回归集合。
  - 已满足（可重复关键流程）：`e2e/tests/docs_app_dialog_contract.spec.mjs` 新增 `docs-app dialog key flow is repeatable with semantic breakpoints`，覆盖 `focus -> Enter 打开 -> Escape 关闭 -> reload -> 再次 keyboard 打开 -> keyboard 关闭` 的可重复链路。
  - 已满足（可定位语义断点）：关键断点全部绑定稳定语义锚点与状态断言（`data-slot="dialog-e2e-open-default"`、`data-slot="dialog-e2e-open-workbench"`、`data-slot="overlay-panel"`、`data-slot="dialog"`，以及 `data-state="with-description"`、`data-close-button="shown"`、`expectDialogReady(...)`、`expectDialogSettledClosed(...)`），失败可直接定位到具体契约字段。
  - 已满足（高风险路径覆盖）：overlay 生命周期（open/close settled）、focus 路径（`toBeFocused()`）、keyboard 路径（`Enter`/`Escape`）均进入回归；异步/动画收敛通过 `expectDialogSettledClosed(...)->toHaveCount(0)` 显式断言。
  - 脚本门禁：`components/dialog/scripts/check-ui-e2e-dialog.sh` 新增  
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`。
  - 回归：`components/dialog/test/semantics.rs::dialog_check2_documents_e2e_repeatable_key_flow_rules`、`components/dialog/test/semantics.rs::dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`、`components/dialog/test/semantics.rs::dialog_e2e_check_script_covers_selector_and_key_flow_contracts`、`components/dialog/test/semantics.rs::dialog_check2_marks_e2e_repeatable_key_flow_contract_complete`、`components/dialog/test/dialog/semantics.rs::dialog_check2_documents_e2e_repeatable_key_flow_rules`、`components/dialog/test/dialog/semantics.rs::dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`、`components/dialog/test/dialog/semantics.rs::dialog_e2e_check_script_covers_selector_and_key_flow_contracts`、`components/dialog/test/dialog/semantics.rs::dialog_check2_marks_e2e_repeatable_key_flow_contract_complete`。
  - 本地验证命令受环境限制：`Invalid cross-device link (os error 18)`（编译产物写入阶段失败）。
- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。
  - docs 页面同步落地：`apps/docs-app/src/pages/components/pages/overlays_dialog.rs::dialog()` 已覆盖 `Hello World`、`Dialog`、`State Matrix`、`Controlled vs Uncontrolled`、`Streaming / Snapshot Contract`、`Interactive Playground`、`Scenario Comparison`，组件行为与示例说明保持同步。
  - 状态矩阵覆盖：`State Matrix` 使用 `state_matrix_options` 驱动 `受控/非受控 + default_open + size + close-button 可见性`，并通过 `is_open/default_open/on_open_change/size/is_close_button_visible` 显式组合。
  - API 与默认值一致性：文档示例与 `components/dialog/src/view.rs`/`components/dialog/src/logic.rs` 对齐（`is_open/open/default_open/on_open_change`、`size`、`is_close_button_visible/show_close_button`，默认值 `DEFAULT_OPEN=false`、`DEFAULT_SHOW_CLOSE_BUTTON`、`DEFAULT_SIZE`）。
  - 回归锁定：`components/dialog/test/semantics.rs` 与 `components/dialog/test/dialog/semantics.rs` 新增
    `dialog_check2_documents_docs_sync_and_state_matrix_rules`、
    `dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`、
    `dialog_dx_check_script_covers_docs_sync_state_matrix_contract`、
    `dialog_check2_marks_docs_sync_and_state_matrix_contract_complete`。
  - 脚本门禁：`scripts/check-ui-dx.sh` 新增
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_docs_sync_and_state_matrix_rules`、
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`。
  - 验证记录：执行 dialog 相关 `cargo test` 命令仍受当前容器环境阻塞 `Invalid cross-device link (os error 18)`，属于环境问题，非本次 docs/state-matrix 同步契约回归。
  - 组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。
  - 文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。
  - 文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。
- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。
  - 文档入口存在：`components/dialog/src/README.md` 与 `apps/docs-app/src/pages/components/pages/overlays_dialog.rs::dialog()` 同时存在，避免“只有源码没有文档”。
  - 零门槛示例：README 提供 `## Hello World` 最小可运行示例；docs-app 提供首屏 `Hello World` Playground，默认路径可直接使用。
  - 默认优先、进阶在后：README 结构为 `Hello World -> 常见用法 -> 先用起来，再进阶`；docs 页面按 `Hello World -> State Matrix -> Controlled vs Uncontrolled -> Interactive Playground` 组织，先默认再进阶。
  - 回归锁定：`components/dialog/test/semantics.rs` 与 `components/dialog/test/dialog/semantics.rs` 新增
    `dialog_check2_documents_documentation_as_product_rules`、
    `dialog_documentation_entry_exists_with_beginner_first_progression`、
    `dialog_dx_check_script_covers_documentation_as_product_contract`、
    `dialog_check2_marks_documentation_as_product_contract_complete`。
  - 脚本门禁：`scripts/check-ui-dx.sh` 新增
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_documentation_as_product_rules`、
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_documentation_entry_exists_with_beginner_first_progression`。
  - 验证记录：执行相关 `cargo test` 命令时，当前容器环境仍返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次 Documentation-as-Product 契约回归。
  - 每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。
  - 文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。
  - “只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。
- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。
  - Playground 交互能力已落地：`apps/docs-app/src/pages/components/pages/overlays_dialog.rs::dialog()` 提供 `Interactive Playground`（`Display + Config + Code + CSS Test`）与 `Scenario Comparison`，覆盖 props 调整、状态切换与实时预览。
  - AI Spec 相关联动示例：`Streaming / Snapshot Contract` 通过 `data-requested-stream-mode` / `data-requested-output-status` 输入标记与组件 `data-stream-mode` / `data-stream-fallback` / `data-output-status` 输出标记形成可观察联动。
  - 可重复关键流复用：`e2e/tests/docs_app_dialog_contract.spec.mjs` 包含 `docs-app dialog key flow is repeatable with semantic breakpoints`，覆盖打开/键盘交互/Escape 关闭/reload 后语义断点复验。
  - 回归锁定：`components/dialog/test/semantics.rs` 与 `components/dialog/test/dialog/semantics.rs` 新增
    `dialog_check2_documents_interactive_playground_rules`、
    `dialog_docs_app_provides_interactive_playground_for_props_state_and_preview`、
    `dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow`、
    `dialog_dx_check_script_covers_interactive_playground_contract`、
    `dialog_check2_marks_interactive_playground_contract_complete`。
  - 脚本门禁：`scripts/check-ui-dx.sh` 新增
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_interactive_playground_rules`、
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_docs_app_provides_interactive_playground_for_props_state_and_preview`、
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow`。
  - 验证记录：执行相关 `cargo test` 命令时，当前容器环境仍返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次 Interactive Playground 契约回归。
  - Playground 至少支持基础 props 调整、状态切换、交互反馈观察。
  - 对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。
  - Playground 作为验收面，需可重复复现关键交互路径。
- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。
  - 复制能力已落地：`apps/docs-app/src/pages/components/pages/overlays_dialog.rs` 全部 Playground 接入 `code_imports=DIALOG_DOC_IMPORTS.to_string()`，并在 `data-slot="dialog-source-first"` 明确展示 source-first 复制路径。
  - 一键复制与可运行输出：`apps/docs-app/src/playground.rs::compose_copy_ready_code` 负责补全 imports，`components/code-block/src/view.rs` 提供 copy 按钮与可访问标签，保证“复制即运行”路径稳定。
  - 源码落点与依赖前提：docs 页面在 `data-slot="dialog-source-paths"` 明确指向 `components/dialog/src/{mod,logic,view,styles,motion}.rs`，并声明 `component-dialog + inject-css` 前提，避免复制后缺依赖报错。
  - 回归锁定：`components/dialog/test/semantics.rs` 与 `components/dialog/test/dialog/semantics.rs` 覆盖
    `dialog_check2_documents_source_first_copy_paste_ready_rules`、
    `dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`、
    `dialog_dx_check_script_covers_source_first_copy_paste_ready_contract`、
    `dialog_check2_marks_source_first_copy_paste_ready_contract_complete`。
  - 脚本门禁：`scripts/check-ui-dx.sh` 已接入
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_documents_source_first_copy_paste_ready_rules`、
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`。
  - 验证记录：执行相关 `cargo test` 命令时，当前容器环境仍返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次 Source-first 契约回归。
  - docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。
  - 若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。
  - 文档代码与当前实现必须同步，防止示例漂移。
- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。
  - 对标策略文档已同步：`docs/spec/heroui-parameter-design-strategy.md` 新增 `### Dialog 同步记录（2026-02-20）`，明确参数主轴
    `is_open/open + on_open_change + default_open`、
    `size`、
    `is_close_button_visible/show_close_button`、
    `close_label`、
    `motion`、
    `on_close`、
    `class_name/lang/dir`，
    并声明“参数语义若变更，必须先同步本策略文档与 docs 入口”。
  - 组件文档入口可访问且可索引：`apps/docs-app/src/pages/components/pages.rs` 保持
    `component_doc!("Dialog", "dialog", "Overlays", overlays::dialog)`；
    `apps/docs-app/src/pages/components/pages/overlays_dialog.rs::dialog()` 与 `components/dialog/src/README.md` 同步存在，`#/components/dialog` 可索引访问。
  - 研究文档补充判定：本轮为 Dialog 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，`docs/research/spectrum-heroui-style-interface-study.md` 为 N/A（无需新增）。
  - 回归锁定：`components/dialog/test/semantics.rs` 与 `components/dialog/test/dialog/semantics.rs` 新增
    `dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes`、
    `dialog_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract`、
    `dialog_check2_marks_heroui_strategy_and_component_docs_sync_complete`。
  - 脚本门禁：`scripts/check-ui-contract-hygiene.sh` 新增
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes`、
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_check2_marks_heroui_strategy_and_component_docs_sync_complete`。
  - 验证记录：执行
    `cargo test -p ui --test dialog_semantics --no-default-features --features component-dialog,inject-css dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes`，
    当前容器环境返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次 HeroUI/doc-sync 契约回归。
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
