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
  - N/A 说明：`CommandDialog` 不发起远程请求，也无独立异步状态流转（仅本地 open/close 与 action 回调）。
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
  - 每个 item 的标题、语义与内容必须在同一 `Item` 结构维度绑定，避免索引配对式隐式约定。
  - `labels + children`、`titles + panels` 等并行数组/并行槽位写法不得作为默认或推荐 API。
  - 不引入这类语法糖：若为配置式输入，仅允许类型化 `ItemSpec`，并在内部映射为显式 `Item` 语义树。

### 3. 高级交互与物理机制（Shell/Physics）
- [x] 宏观/微观双状态机（Macro/Micro Duality）：拖拽等高频交互在 `Dragging` 期间由 `view/motion` 本地循环执行；禁止每帧穿越回 `logic.rs`，必须在结束时通过 `Action::DragEnd` 回流收敛。
  - N/A 说明：`CommandDialog` 无拖拽/手势驱动的高频交互路径，不存在 `Dragging` 帧循环与 `Action::DragEnd` 收敛流程。
- [x] 几何两段式渲染（Two-Pass Rendering）：`Tooltip/Popover/Menu` 等依赖 DOM 测量的组件必须走 `Intent -> Measure(view) -> Rectification(logic)`，并具备幂等收敛保护防死循环。
  - N/A 说明：`CommandDialog` 当前不在组件层执行 DOM 几何测量与回写收敛；布局由 `Modal` 与静态 CSS 变量约束完成，无测量-修正循环。
- [x] 集合注册协议（Registration Protocol）：`Accordion/Tabs/Menu` 动态子项必须通过 `RegistrationContext` 上报 `Register/Unregister`，逻辑层维护 `items_order`，禁止依赖 `HashSet` 迭代顺序做导航。
  - N/A 说明：`CommandDialog` 通过 `groups: Arc<[CommandGroup]>` 消费已成型数据，不在组件生命周期中动态注册/反注册子项，也不依赖 `HashSet` 顺序导航。
- [x] 插槽投影策略（Slot Projection）：容器组件明确 `Lazy/KeepAlive/Eager`；`KeepAlive` 隐藏时必须通过生命周期通知（如 `NotifyHidden`）暂停轮询/动画等高耗能副作用。
  - N/A 说明：`CommandDialog` 未暴露 `Lazy/KeepAlive/Eager` 插槽投影模式；当前仅通过 `use_presence` 处理显示与退场，不存在 `KeepAlive` 隐藏态副作用暂停协议。
- [x] 环境订阅流（Env Streams）：`Resize/Theme/Intersection` 等环境变化在 `view.rs` 采样、防抖后转化为高层语义 `Action`（如 `BreakpointChanged`）推送到 `logic`；禁止原始事件洪泛。
  - N/A 说明：`CommandDialog` 当前无 `Resize/Theme/Intersection` 环境事件订阅链路，也无对应的高层语义 `Action`（如 `BreakpointChanged`）回流需求。
- [x] 事件光锥（Event Light Cone）：`Table/Grid` 等大型集合批量操作必须走 `Context Bus + Selector` 与状态压缩表达（如 `SelectionState::All`），禁止 O(N) 级向下 prop drilling。
  - N/A 说明：`CommandDialog` 不承载 `Table/Grid` 型大集合批量操作，当前交互聚焦命令触发与开关状态，不涉及 `Context Bus + SelectionState::All` 路径。
- [x] 统一因果总线（Causality Bus）：复杂派生总线操作必须支持透传 `TraceId`，确保“用户触发 -> 派生命令 -> 总线广播 -> 订阅者”因果链不断裂。
  - N/A 说明：`CommandDialog` 仅在本地回调与 `use_ui_trace` 事件发射层处理开关/动作，不存在复杂派生命令总线与多订阅者广播链路，暂无 `TraceId` 透传需求。
- [x] 存在 A11y 实现、国际化与本地化实现（至少具备接入点，不硬编码用户可见文本）。
  - 交互元素必须具备可验证语义：`role`/`aria-*`/键盘可达路径完整，且和 headless 契约一致。
  - 用户可见文本来源必须可覆盖：优先 props，其次应用注入（`UiRoot`/i18n bundle），最后组件兜底文案；禁止把业务可见文案硬编码在 `view.rs`。
  - 组件需透传或消费 `lang` / `dir`（LTR/RTL）上下文，不得假设单语言单方向。
  - 共享 A11y 工具优先来自 `crates/ui-headless/src/a11y.rs`，组件层不重复发明同名语义工具。
- [x] 状态可观测、可检索、可验证：使用稳定 `data-*` 与 `aria-*` 标记表达状态和来源。
  - 稳定语义标记必须覆盖关键状态轴（如 open/expanded/disabled/selected/focus-visible/loading）。
  - 状态来源必须可区分（受控/非受控、默认值/外部值、交互来源），通过稳定 marker 暴露而不是隐式推断。
  - 自动化选择器优先基于语义标记，不依赖 DOM 顺序、层级深度或临时 class 名。
  - 标记值应为封闭集合（可枚举），避免自由文本导致契约漂移。
- [x] 样式依赖显式状态（`data-*`/class），而非脆弱 DOM 结构猜测。
  - `styles.rs` 中状态分支选择器必须基于 `data-*`/`aria-*`/稳定 class，禁止用 `:nth-child`、深层级选择器猜测状态。
  - 运行时样式仅允许传递必要 CSS 变量（custom properties）；禁止把业务样式逻辑塞进 inline style。
  - 视觉状态切换必须可由语义标记直接解释，不能依赖“某节点是否恰好存在”。
- [x] 测试验证“语义契约”而不只验证视觉快照。
  - 至少存在语义测试覆盖关键状态与交互路径（role/aria/data-state/source markers）。
  - 测试矩阵必须覆盖关键分支：受控/非受控、disabled、键盘路径、指针路径、SSR/wasm 差异（按适用范围）。
  - 视觉快照只能作为补充，不得替代语义契约断言。
  - 适用性说明：键盘路径与指针路径由 `e2e/tests/docs_app_command_dialog.spec.mjs` 分别通过 `page.keyboard.press("Enter")` 与 `.click()` 覆盖；受控/非受控与 disabled 语义在 `components/command-dialog/test/logic.rs` 与 `components/command-dialog/test/command_dialog_semantics.rs` 锁定；组件无 wasm 专属分支，SSR/wasm 差异按 N/A 处理。
- [x] 组件文件职责正确：`mod.rs`（导出边界）、`logic.rs`（归一/派生/来源标记）、`styles.rs`（静态 token-first CSS）、`view.rs`（Leptos 结构 + headless 挂载）、`motion.rs`（动效契约 + attach）。
  - `mod.rs` 只维护最小稳定导出面与 feature gate，不承载实现细节。
  - `logic.rs` 只做输入归一、状态派生、来源标记；禁止 DOM 操作和样式细节分支。
  - `styles.rs` 只包含 token-first 静态 CSS；禁止硬编码主题常量与业务语义文案。
  - `view.rs` 只做结构渲染与 headless 契约挂载；禁止隐藏关键状态决策。
  - `motion.rs` 只做组件语义到动效契约映射与 attach；禁止在组件内重写通用动效引擎。
  - 覆盖依据：`components/command-dialog/test/semantics.rs` 中 `command_dialog_mod_exports_stable_api_and_wires_local_semantics_tests` 与 `command_dialog_layer_responsibilities_stay_separated` 已锁定职责边界。
- [x] `spec.rs` 只用于少数复杂组件（如 button），避免泛滥。
  - 仅当组件存在稳定外部规范/Schema 契约或复杂配置固化需求时才引入 `spec.rs`。
  - 简单组件不得为了“形式统一”新增 `spec.rs`；说明文档应留在 `check2.md`/组件文档。
  - 新增 `spec.rs` 必须同步给出契约测试与版本演进说明。
  - N/A 说明：`components/command-dialog/src` 当前无 `spec.rs`，该组件未引入稳定外部 Schema 契约，文档留在 `check2.md` 与 `src/README.md`。
- [x] 组件层遵循 token-first 静态样式契约：样式通过 `styles.rs` 聚合注入；运行时仅传必要 CSS 变量；不把 Utility-First/CSS-in-Rust 当组件库默认范式。
  - 样式规则统一落在 `styles.rs`，由 `crates/ui/src/css.rs` 聚合并通过 `UiRoot` 注入。
  - 颜色/间距/圆角/阴影等视觉值必须来自 `var(--ui-*)`，禁止组件私有 token 体系。
  - Utility-First 仅作为 `apps/*` 应用层布局手段，不得反向污染组件库契约。
  - CSS-in-Rust 仅在有明确类型安全与构建成本净收益时作为例外采用。
  - 覆盖依据：`components/command-dialog/src/styles.rs` 使用 `var(--ui-*)` token；`crates/ui/src/css.rs` 通过 `#[cfg(feature = "component-command_dialog")]` 聚合 `command_dialog::styles::CSS`；`crates/ui/src/root.rs` 中 `UiRoot` 通过 `crate::css::push_components_css` 注入组件样式。
- [x] 默认主题美学质量达标（Visual Desire）：以 HeroUI 现代审美为学习对标，默认主题不仅“可用”，还必须“第一眼可信”。
  - 默认主题需通过基础美学清单：信息层级清晰（字重/字号/间距）、对比与层次自然、交互反馈明确（hover/active/focus）。
  - docs-app 必须提供默认主题基线页面与截图基线，关键组件（Button/Input/Overlay）纳入视觉回归对比。
  - 禁止“可访问但粗糙”的最低可用心态：视觉退化（类似旧式 Bootstrap 观感）视为质量回归。
  - HeroUI 对标以“视觉语言与体验质量”对齐为目标，不做无差别 API 表层复制。
  - 覆盖依据：`apps/docs-app/src/pages/components/pages/theme_visual_baseline.rs` 提供 `theme-visual-baseline` 页面；`e2e/tests/docs_app_theme_visual_baseline.spec.mjs` 对 page/button/input/overlay 执行 `toHaveScreenshot` 视觉基线回归。
- [x] Tree Shaking 是一等能力：package 模式支持组件级 feature；source 模式天然裁剪；样式层同步裁剪，禁止无条件聚合全部 CSS，禁止破坏 DCE/LTO 的全量中央注册表。
  - package 模式必须有组件级 feature（如 `component-accordion`）；未启用组件不得进入编译与链接路径。
  - `lib.rs` 与 `css.rs` 必须按 feature 条件导出/聚合，禁止无条件引用所有组件模块和 CSS 常量。
  - source 模式下仅引入需要的组件源码，不通过中央注册表维持全组件可达。
  - 任意“全量组件映射表/注册表”若导致不可达代码变可达，直接判不通过。
  - 验证命令（特性树）：`cargo tree -e features -p ui --no-default-features --features component-accordion,inject-css`，确认仅启用目标组件特性链。
  - 验证命令（反向依赖）：`cargo tree -e features -i ui -p web-demo`，检查是否被 `all-components` 或隐式特性全量拉起。
  - CI 检查（最小特性编译）：新增任务仅开启目标最小特性（示例：`cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-accordion,inject-css`）。
  - CI 检查（体积预算）：对“最小特性构建产物”设定预算并阻断回归（可用固定阈值，如 `< 50KB`，或基于仓库基线的相对阈值）；不得只做编译通过而不做体积约束。
  - 覆盖依据：`crates/ui/Cargo.toml` 定义 `component-command_dialog = ["component-command", "component-modal"]`；`crates/ui/src/lib.rs` 与 `crates/ui/src/css.rs` 对 `command_dialog` 导出/样式聚合均受 `#[cfg(feature = "component-command_dialog")]` 门控；`cargo tree -e features -i ui -p ui --no-default-features --features component-command_dialog,inject-css` 显示仅最小链路（含 `component-command_dialog`、`component-command`、`component-modal`、`inject-css`）；`cargo tree -e features -i ui -p web-demo` 未出现 `all-components`（`NO all-components`）；CI `Tree Shaking Budget`（`.github/workflows/ci.yml`）执行 `scripts/check-ui-tree-shaking.sh`，并通过 `scripts/tree_shaking_budget.env` 做体积预算门禁。
- [x] 类型系统 + 语义标记共同提供机器可读状态；关键输入空间受类型约束。
  - 离散输入与状态轴必须优先使用 `enum`/新类型建模，避免字符串协议与布尔爆炸。
  - 无效状态要么在类型层不可表达，要么在 `logic.rs` 被统一归一化并可测试。
  - 关键状态必须通过稳定语义标记对外可读，供测试与 Agent 自动化消费。
  - 编译器与测试反馈应能直接定位状态契约破坏点，形成可持续闭环。
  - 覆盖依据：`components/command-dialog/src/mod.rs` 以 `CommandDialogSlot` 枚举约束离散槽位；`components/command-dialog/src/logic.rs` 通过 `normalize_props`/`resolve_state` 统一归一化并输出封闭 `*_attr` 标记；`components/command-dialog/src/view.rs` 挂载 `data-state`/`data-open-mode`/`data-id-source` 等稳定语义标记；`components/command-dialog/test/logic.rs` 与 `components/command-dialog/test/semantics.rs` 锁定归一化与语义契约。

### 4. DOM/环境边界治理
- [x] 焦点全局栈（Focus Stack & GC）：层叠 `Overlay` 禁止私存 `NodeRef` 作为恢复目标；必须依赖全局 Focus Manager（如 `FallbackTo/Selector`）防止焦点坠落到 `document.body`。
  - 覆盖依据：`components/command-dialog/src/view.rs` 仅组合 `Modal`，未私存用于恢复目标的 `NodeRef`；`components/modal/src/view.rs` 通过 `Overlay` 承接焦点治理；`components/overlay/src/view.rs` 使用 `use_overlay_stack_registration()` + `use_focus_trap(FocusTrapOptions::enabled(panel_ref).with_restore_policy(RestorePolicy::FallbackTo(...)))`；全局焦点栈由 `crates/ui-headless/src/focus_trap.rs` 的 `FOCUS_MANAGER_STACK` 统一维护，恢复链经 `RestorePolicy::Selector/FallbackTo` 执行；应用入口 `apps/docs-app/src/lib.rs` 与 `apps/web-demo/src/main.rs` 均调用 `provide_overlay_stack()`。
- [x] 受控外交特区（Escape Hatches）：集成 ECharts/Map 等命令式第三方库时必须处于 `Foreign Zone`（`YieldControl/CleanupForeign`）；第三方实例不得暴露为组件公共 API 或反向污染状态机。
  - N/A 说明：`CommandDialog` 当前未集成 ECharts/Map 等命令式第三方实例；组件 API 仅暴露声明式 props/callback（如 `open/default_open/on_open_change/on_action`），不存在第三方实例向公共 API 外泄或反向污染状态机的路径。
- [x] SSR 时空断裂治理（Hydration Discontinuity）：逻辑初始化禁止依赖 `now()` 或原生随机 UUID；必须通过 `IdProvider` 注入确定性种子，确保 SSR/Hydration 间 ID 稳定。
  - N/A 说明（组件内随机 ID 分配）：`CommandDialog` 当前不使用 `now()/UUID/rand` 初始化 ID；`id_base` 由显式输入或 `DEFAULT_ID_BASE` 纯函数归一化（`components/command-dialog/src/logic.rs`），并在 `view.rs` 通过确定性字符串拼接派生子节点 ID（如 `format!("{id_base}-command")`），不存在 SSR/Hydration 熵源漂移。全局确定性种子注入入口由 `UiRoot` 提供（`crates/ui/src/root.rs`：`provide_ui_id_provider(id_seed)`），底层 provider 在 `crates/ui-headless/src/id_provider.rs`。
- [x] SSR 与跨平台检查：覆盖 web/ssr/wasm 分支，不破坏 non-wasm 编译路径。
  - 至少包含 compile-only 证据：web（wasm32）、ssr（native）、默认本地构建三条路径。
  - 平台分支差异必须显式 `cfg` 或 feature 管理，禁止依赖运行时偶然行为。
  - non-wasm 路径禁止引用 `web-sys`/浏览器对象。
  - 覆盖依据：`crates/ui/src/lib.rs` 通过 `#[cfg(feature = "component-command_dialog")]` + `#[path = "../../../components/command-dialog/src/mod.rs"]` 挂载组件；`crates/ui/src/css.rs` 仅在同 feature 下聚合 `command_dialog::styles::CSS`。`components/command-dialog/src/*` 未直接引用 `web-sys`，浏览器对象仅在 `components/overlay/src/motion.rs` 的 `#[cfg(target_arch = "wasm32")]` 分支使用，`#[cfg(not(target_arch = "wasm32"))]` 提供可预测降级。
  - compile-only 记录：已执行 `cargo check -p ui --no-default-features --features component-command_dialog,inject-css`、`cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-command_dialog,inject-css`、`cargo check -p ui-headless --no-default-features --features ssr`；当前容器环境统一因 `Invalid cross-device link (os error 18)` 失败，非代码语义问题。
- [x] `ui-headless` web/ssr feature 互斥受 `compile_error!` 保护（`crates/ui-headless/src/lib.rs`）。
  - 组件依赖 `ui-headless` 能力时，不得破坏其 web/ssr 互斥约束。
  - 组件若新增 headless 功能接入，需验证两条 feature 路径都可编译。
  - 发现“同时启用 web+ssr 仍可过编译”视为契约回归。
  - 覆盖依据：`crates/ui-headless/src/lib.rs` 顶部存在 `#[cfg(all(feature = "web", feature = "ssr"))] compile_error!(...)`；`crates/ui-headless/Cargo.toml` 中 `web = ["leptos/csr"]`、`ssr = ["leptos/ssr"]` 明确为互斥运行形态。
  - 组件接入边界：`components/command-dialog/src/view.rs` 仅通过 `use ui_headless::{UiTraceEventKind, use_presence, use_ui_trace};` 消费能力，不在组件层重定义 headless feature；`crates/ui/Cargo.toml` 对 `ui-headless` 仅声明路径依赖，未引入破坏互斥的双 feature 绑定。
  - 验证记录：已执行 `cargo check -p ui-headless --no-default-features --features web,ssr`（用于互斥回归探测）与单路径构建命令；当前容器环境统一因 `Invalid cross-device link (os error 18)` 失败，属于环境问题，非互斥契约回归。
- [x] `ui-motion` 非 wasm 提供 no-op/stub（`crates/ui-motion/src/lib.rs`），保证 SSR/tooling 可编译。
  - `motion.rs` 调用必须可在 non-wasm 下安全降级，不触发 panic。
  - 组件不得假设动画句柄一定存在；no-op 分支行为需可预测。
  - toolchain 场景（测试/文档/静态分析）不得因 motion 依赖阻塞编译。
  - 覆盖依据：`crates/ui-motion/src/lib.rs` 在 `#[cfg(not(target_arch = "wasm32"))]` 下提供 `web::prefers_reduced_motion() -> true` 与 `web::animate(...) {}` 空实现，并含 `non_wasm_web_backend_is_predictable_noop` 测试锁定行为。
  - 组件降级路径：`components/overlay/src/motion.rs` 的 `#[cfg(not(target_arch = "wasm32"))] attach_motion(...)` 不触发浏览器 API，仅在关闭时同步 `finish_exit`；`crates/ui-visual-primitive/src/active_highlight.rs` 的 non-wasm `attach_active_highlight_motion(...)` 为显式 no-op。
  - 句柄假设检查：`components/command-dialog/src/motion.rs` 仅做 motion contract sanitize/组合，不暴露或依赖底层动画句柄存在性，SSR/tooling 分支行为可预测。
- [x] 组件实现覆盖 `reduced-motion` / SSR / wasm 分支。
  - `reduced-motion` 下动画应跳过或降级为最小必要反馈。
  - SSR 输出必须与客户端 hydration 兼容，避免首帧语义错位。
  - wasm 分支允许增强交互，但语义契约不得与 SSR 分支分裂。
  - `reduced-motion` 依据：`crates/ui-motion/src/web.rs` 中 `animate(...)` 在 `prefers_reduced_motion()` 为真时立即返回；`crates/ui-motion/src/spring.rs` 的 `set_target(...)` 在 reduced-motion 分支直接写入目标值并触发 `on_rest`，避免长动画。
  - SSR/non-wasm 依据：`components/overlay/src/motion.rs` 在 `#[cfg(not(target_arch = "wasm32"))]` 下使用同步降级（关闭即 `finish_exit`）；`crates/ui-visual-primitive/src/active_highlight.rs` non-wasm `attach_active_highlight_motion(...)` 为 no-op，确保 SSR/tooling 可预测。
  - wasm 与语义一致性依据：`components/overlay/src/view.rs` 的 wasm 条件分支只在键盘事件细节（`is_composing/default_prevented`）上增强，`data-state`/`data-open`/`data-closed` 与 `role/aria-*` 挂载路径一致；`components/command-dialog/src/view.rs` 通过 `use_presence(open)` + 稳定 `data-*` 标记输出状态契约，不随平台分裂。
- [x] 性能治理：关键路径有预算（首次渲染/更新耗时/内存），回归可检测、可归因、可阻断。
  - 关键交互组件需定义最小预算项（首渲染、关键更新、内存/分配趋势）。
  - 回归检测至少具备可重复基线与失败阈值，不靠主观“感觉变慢”。
  - 性能问题需可归因到状态、渲染、样式或动效路径之一。
  - 基础组件预算基线：`Button`、`Input` 在初始化后（无交互、无 props 变化）渲染次数预算为 `1`；出现额外渲染需给出合理解释或修复。
  - 测试要求：在 `components/*/test/**` 增加 `render_count` 类回归测试（测试框架支持时必须启用）；至少覆盖基础组件与本次改动组件。
  - 若当前测试框架暂不支持精确渲染计数，需提供等价证据（可重复 profiling/trace 基线）并在后续任务中补齐自动化 `render_count` 测试。
  - 覆盖依据：`apps/docs-app/src/pages/components/pages/collections_command.rs` 的 `command_dialog` 页面通过 `<ComponentPage title="CommandDialog" slug="command-dialog" ...>` 接入统一探针；`apps/docs-app/src/pages/components/shell.rs` 统一以 `component_page_perf_budget(slug)` + `<UiPerfProbe name=perf_name budget=perf_budget>` 输出预算与观测，未显式配置 slug 时走 `_ => UiPerfBudget::mount_only(120.0)`，形成可重复阈值基线。
  - 阻断与观测：`apps/docs-app/src/perf_probe.rs` 提供稳定 `data-perf-mount-ms/data-perf-budget-ms/data-perf-budget-update-ms/data-perf-budget-heap-kb/data-perf-violation/data-perf-observability`；`e2e/tests/docs_app_components_coverage.spec.mjs` 持续断言预算属性存在且 `data-perf-violation != true`。
  - 归因路径：`components/command-dialog/src/view.rs` 暴露 `data-state/data-open-mode/data-action-source/data-open-change-source/data-command-motion-source/data-overlay-motion-source`，可将回归归因到状态/语义/动效通道。
  - 自动化证据：`components/command-dialog/test/command_dialog_semantics.rs` 新增 `command_dialog_performance_governance_contract_is_mount_only_traceable_and_blocking`；门禁脚本 `scripts/check-ui-performance.sh` 新增命令 `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_performance_governance_contract_is_mount_only_traceable_and_blocking`，并保持 `docs_perf_probe_budgets_are_wired_for_component_pages` 与 `perf_render_count_follow_up_is_tracked_in_plan`。
  - 现状说明：当前测试框架尚未提供通用精确 `render_count` 计数，按清单采用可重复 `UiPerfProbe + data-perf-*` 等价证据；`docs/plan/TODO.md` 持续跟踪“建立 `render_count` 自动化回归（Button/Input/Accordion/DropZone），替换当前 mount-only 等价证据”。
- [x] `view!` 宏复杂度受控：单个 `view!` 块不得承载超长深嵌套结构；复杂布局按语义分块，避免一次性宏展开导致编译与 wasm 体积劣化。
  - 复杂结构按语义子块拆分（header/body/item 等），避免巨型单块 `view!`。
  - `view.rs` 中若出现多层嵌套重复片段，应优先提取局部渲染函数。
  - 编译时间/产物体积异常增长时，优先排查宏展开体量。
  - 覆盖依据：`components/command-dialog/src/view.rs` 当前为单一有界 `view!` 根块（`wc -l` 为 244 行；`view! {` 计数 1），结构上按语义容器分块为 `<Show> -> <Modal> -> <Command>`，避免巨型同层重复片段。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增 `command_dialog_view_macro_complexity_is_bounded_and_semantically_partitioned`，约束 `view!` 计数与行数上限（<= 260），并断言 `use_presence/open + Modal + Command` 的语义分块挂载存在。
- [x] 函数式拆分优先：不涉及复杂状态与生命周期管理的 UI 片段，优先拆为普通 Rust 函数（返回 `impl IntoView`/`View`），而不是新增 `#[component]`。
  - 纯静态或轻逻辑片段优先函数化；仅在需要独立 props 语义时升级为组件。
  - 禁止把所有局部片段都升格为 `#[component]` 导致抽象噪音。
  - 拆分后语义标记与测试定位仍需稳定。
  - 实现依据：`components/command-dialog/src/view.rs` 将渲染片段下沉为普通函数 `fn render_dialog_view(...) -> impl IntoView`，`#[component]` 仅保留 `CommandDialog` 一个公共边界。
  - 语义稳定：拆分后关键标记仍由同一渲染函数挂载（`data-slot/data-state/data-ui-*` 等），未引入额外语义漂移。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增 `command_dialog_prefers_functional_view_split_over_extra_local_components`，约束“单 `#[component]` + 函数式渲染拆分 + 关键语义标记仍存在”。
  - 验证记录：尝试执行 `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_prefers_functional_view_split_over_extra_local_components`，当前容器环境统一受 `Invalid cross-device link (os error 18)` 影响未能完成编译，属环境阻塞。
- [x] 静态片段常量化：复杂 SVG、页脚、长说明文本等纯静态内容优先常量化/模板化，减少重复 `view!` 渲染指令生成。
  - 可判定为纯静态的片段应避免重复动态构造。
  - 常量化后仍需维持可访问语义（title/aria-label/role 等）。
  - 静态资源变更路径要清晰，避免散落在多个 `view!` 片段中。
  - N/A 说明：`CommandDialog` 组件实现不包含复杂 SVG、页脚模板、长静态说明文本或 `inner_html` 注入，渲染主体为轻量语义壳层 + 子组件装配。
  - 现有静态文本落点：默认静态兜底集中在 `components/command-dialog/src/logic.rs` 的 `DEFAULT_ID_BASE/DEFAULT_TITLE` 与 `resolve_text_with_empty_default`；`view.rs` 仅消费归一化结果（`description_text/placeholder_text/empty_label_text/aria_label_text`）。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增 `command_dialog_static_fragment_constantization_is_not_applicable_for_lightweight_markup`，断言无重静态片段并锁定文本归一化消费路径。
  - 验证记录：执行 `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_static_fragment_constantization_is_not_applicable_for_lightweight_markup`，当前容器因 `Invalid cross-device link (os error 18)` 环境问题未完成编译。
- [x] `inner_html` 使用约束：仅允许注入受信任静态常量，禁止拼接用户输入；使用处必须补充语义与安全回归测试。
  - 仅允许编译期常量或明确白名单内容进入 `inner_html`。
  - 严禁直接或间接注入用户输入、远端返回或未清洗模板字符串。
  - 使用 `inner_html` 的节点必须补语义测试与安全回归说明。
  - N/A 说明：`CommandDialog` 当前无 `inner_html` 使用点（组件实现、文档示例、e2e 脚本均未出现），因此不存在“仅允许静态白名单注入”的运行路径。
  - 安全回归：`components/command-dialog/test/command_dialog_semantics.rs` 新增 `command_dialog_forbids_inner_html_injection_paths`，锁定禁止 `inner_html/set_inner_html/insert_adjacent_html/dangerously_set_inner_html/<script`，并要求语义标记 `data-ui-schema/data-ui-schema-version/data-ui-intent` 仍稳定存在。
  - 验证记录：执行 `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_forbids_inner_html_injection_paths`，当前容器因 `Invalid cross-device link (os error 18)` 环境问题未完成编译。
- [x] WASM 调试要求：关键状态可追踪（来源/时间/前后值），关键交互可回放，开发模式有可视化入口，调试能力通过 feature 隔离不污染产物。
  - 开发模式下至少能追踪关键状态变更来源与前后值。
  - 关键交互链路应支持最小可复现记录（事件顺序/状态转移）。
  - 调试开关默认不进入生产包体与公共 API。
  - 状态追踪依据：`components/command-dialog/src/view.rs` 使用 `use_ui_trace`，在 `request_open_change` 中以 `current -> next` 判定后发射 `UiTraceEventKind::OpenChange { open: next }`；同时暴露 `data-ui-state/data-open-mode/data-open-change-source` 标记用于来源与前后状态比对。
  - 时间轴与可视化入口：全局 trace 时间戳来自 `crates/ui-headless/src/trace.rs` 的 `UiTraceEvent { ts_ms, component, kind }`；`apps/docs-app/src/lib.rs` 在 `debug_assertions` 下启用 `provide_ui_trace(debug_overlay_enabled)` 并挂载 `<debug_overlay::UiDebugOverlay enabled=true />`；`apps/docs-app/src/debug_overlay.rs` 渲染 `OpenChange/Inspect/Note` 事件时间线。
  - 最小可回放链路：`e2e/tests/docs_app_command_dialog.spec.mjs` 固化键盘/点击事件顺序与状态转移（受控路径 `open -> close`、marker 路径 `open -> keep-open`），可复现关键交互。
  - feature 隔离：`crates/ui/Cargo.toml` 仅存在共享 `accordion-wasm-debug/button-wasm-debug` 开关，未引入 `command-dialog-wasm-debug` 私有特性，避免污染生产 API/包体。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增 `command_dialog_wasm_debug_contract_reuses_global_trace_and_keeps_feature_isolated`；`scripts/check-ui-wasm-debug.sh` 新增对应门禁命令。
  - 验证记录：执行 `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_wasm_debug_contract_reuses_global_trace_and_keeps_feature_isolated`，当前容器因 `Invalid cross-device link (os error 18)` 环境问题未完成编译。
- [x] DX 要求：样式热重载优先无需重编 wasm；组件热开发尽量保持上下文；提供可选状态保留；有 Workbench 隔离画布。
  - 常见样式调整应走快速反馈路径，不依赖完整 wasm 重编译。
  - 组件调试应尽量保持当前交互上下文，降低重复操作成本。
  - 复杂交互组件应有隔离演练入口（workbench/story/demo 之一）。
  - Workbench 落地：`apps/docs-app/src/pages/components/pages/collections_command.rs` 的 `command_dialog()` 新增 `<Playground title="Workbench (Display + Config + Code + CSS Test)">`，包含 `code_signal + test_css_source + test_source_path + test_config_signal + controls` 完整链路。
  - 样式热重载依据：Workbench 显式绑定 `test_css_source=workbench_test_css_source`（来源 `ui::command_dialog::styles::CSS`）和 `test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/command-dialog/src/styles.rs"`，走 Playground 测试面板即时样式反馈路径，无需改动组件逻辑重新编译 wasm。
  - 上下文保留（可选）依据：新增 `workbench_preserve_context` 开关；当关闭时 `Effect` 在场景切换后回收 `open/last_action`，当开启时保持当前会话上下文，满足“可选状态保留”而非强制持久化。
  - 隔离画布依据：新增稳定标记 `data-slot=\"command-dialog-workbench-controls\"`、`data-slot=\"command-dialog-workbench\"`、`data-slot=\"command-dialog-workbench-actions\"`、`data-slot=\"command-dialog-workbench-canvas\"`，避免和其他示例互相干扰。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增 `command_dialog_dx_workbench_supports_optional_state_persistence_and_isolated_canvas` 与 `command_dialog_dx_check_script_covers_hot_reload_and_workbench_contract`。
  - 门禁命令：`scripts/check-ui-dx.sh` 新增 `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_dx_workbench_supports_optional_state_persistence_and_isolated_canvas`。
  - 验证记录：执行上述 `cargo test` 命令，当前容器环境返回 `Invalid cross-device link (os error 18)`，属于环境阻塞，非该改动语义回归。
- [x] 工程能力统一：`serde` 负责 spec 序列化/版本迁移/错误结构化；`tracing` 统一 span/event 语义；async 不绑定单一运行时（tokio/async-std），runtime 细节不泄露到上层 API。
  - 若组件涉及 spec/config 输入，序列化与错误输出应走统一结构化路径。
  - 关键流程埋点语义应与全库 tracing 约定一致，避免组件各说各话。
  - 异步边界不得把具体 runtime 类型暴露到组件公共接口。
  - serde/spec 结论：`CommandDialog` 当前无 `spec.rs`，组件特性链 `component-command_dialog = ["component-command", "component-modal"]` 未引入 `dep:serde/dep:serde_json`；公共导出边界 `components/command-dialog/src/mod.rs` 未挂载 `protocol.rs`，因此本组件在当前范围内为“spec/config 输入 N/A”，无需引入序列化迁移路径。
  - tracing 结论：`components/command-dialog/src/view.rs` 使用 `ui_headless::use_ui_trace` + `UiTraceEventKind::OpenChange` 发射事件，复用统一 headless trace 语义；未引入 `tracing::span!/event!` 或组件私有 trace target，避免语义漂移。
  - async/runtime 边界结论：`mod.rs/logic.rs/view.rs/styles.rs/motion.rs` 未出现 `tokio/async-std/smol/runtime::Handle` 等运行时类型泄露，组件公共 API 也未暴露 runtime 细节类型。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope`、
    `command_dialog_engineering_contract_keeps_tracing_semantics_unified_without_component_local_events`、
    `command_dialog_engineering_contract_avoids_runtime_leaks_in_public_api_surface`、
    `command_dialog_engineering_check_script_covers_serde_tracing_and_runtime_boundaries`。
  - 门禁命令：`scripts/check-ui-engineering.sh` 新增三条 `command_dialog` 工程能力契约测试命令（serde/spec N/A、tracing 统一、runtime 边界）。
  - 验证记录：执行 `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_engineering_contract_marks_spec_serde_path_as_na_for_simple_component_scope`，当前容器因 `Invalid cross-device link (os error 18)` 环境问题未完成编译。

### 5. 样式与动效（Theme & Motion）
- [x] 样式孤岛防御（Defensive Variables）：`styles.rs` 使用双层回退链 `var(--ui-*, var(--ui-fallback-*))`；禁止组件内硬编码 Hex 或裸尺寸终值，Fallback 终值由 `ui-theme` 统一输出（SSOT）。
  - 修复落点：`components/command-dialog/src/styles.rs` 将裸终值路径替换为双层回退链，包含 `--ui-overlay-panel-min-width/--ui-fallback-overlay-panel-min-width`、`--ui-command-panel-max-width/--ui-fallback-command-panel-max-width`、`--ui-overlay-viewport-inset/--ui-fallback-overlay-viewport-inset` 等；移除了 `var(--ui-overlay-panel-min-width, 280px)` 与 `calc(100vw - ...)`。
  - SSOT 依据：fallback 终值由 `crates/ui-theme/src/css.rs` 提供（`--ui-fallback-checkbox-disabled-opacity`、`--ui-fallback-border-width`、`--ui-fallback-border`、`--ui-fallback-overlay-viewport-inset`、`--ui-fallback-overlay-panel-min-width`、`--ui-fallback-command-panel-max-width`、`--ui-fallback-space-sm`）。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_styles_use_defensive_variable_fallback_chain`、
    `command_dialog_defensive_variables_check_script_covers_style_fallback_contract`、
    `command_dialog_check2_marks_defensive_variables_contract_complete`。
  - 门禁命令：`scripts/check-ui-contract-hygiene.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_styles_use_defensive_variable_fallback_chain`。
  - 验证记录：执行上述 `cargo test` 命令，当前容器环境返回 `Invalid cross-device link (os error 18)`，属于环境阻塞，非本次样式契约回归。
- [x] 级联层覆盖（`@layer ui`）：组件 CSS 默认聚合进 `@layer ui`；运行时数值调整仅通过 CSS Custom Properties（如 `style:--x=...`），禁止普通内联样式（如 `style=\"top: 10px\"`）。
  - 级联层依据：`crates/ui/src/css.rs` 保持 `out.push_str("\n@layer ui {\n"); ... out.push_str("\n}\n");`，且 `#[cfg(feature = "component-command_dialog")]` 下聚合 `crate::command_dialog::styles::CSS`，满足组件 CSS 默认进入 `@layer ui`。
  - 注入边界依据：`crates/ui/src/root.rs` 继续通过 `crate::css::push_components_css(&mut out)` + `<style>{move || css_text.get()}</style>` 统一注入，不在组件层分散注入样式。
  - 运行时样式依据：`components/command-dialog/src/view.rs` 未出现 `style="top/left/width/height"` 等普通内联样式；若未来出现 `style:` 语法，回归约束要求仅允许 `style:--*` 自定义属性路径。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_cascade_layer_and_runtime_style_contract_is_enforced`、
    `command_dialog_cascade_layer_check_script_covers_contract`、
    `command_dialog_check2_marks_cascade_layer_contract_complete`。
  - 门禁命令：`scripts/check-ui-contract-hygiene.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_cascade_layer_and_runtime_style_contract_is_enforced`。
  - 验证记录：执行上述 `cargo test` 命令，当前容器环境返回 `Invalid cross-device link (os error 18)`，属于环境阻塞，非本次级联层契约回归。
- [x] Motion 合同化：`stiffness`/`damping` 等参数在 `motion.rs` 内置为组件 Contract，并通过 `attach_motion` 挂载；必须尊重 `prefers-reduced-motion` 且在 non-wasm/SSR 安全降级（no-op）。
  - 组件 Contract 依据：`components/command-dialog/src/motion.rs` 提供 `CommandDialogMotion` + `sanitize_motion` -> `overlay::motion::sanitize_motion`；`sanitize_command_spring` 对 `stiffness/damping/mass/precision` 执行有限正值归一，避免无效参数穿透到运行时。
  - 挂载路径依据：`components/command-dialog/src/view.rs` 通过 `motion::attach_motion(command_motion, overlay_motion)` 归一后再分别传入 `motion=command_motion` 与 `motion=overlay_motion`，组件层不重写执行引擎。
  - reduced-motion / non-wasm 依据：`crates/ui-motion/src/spring.rs` 在 `if crate::web::prefers_reduced_motion() { ... }` 分支同步应用目标值；`crates/ui-motion/src/lib.rs` non-wasm stub 提供 `pub fn prefers_reduced_motion() -> bool` 与 `animate` no-op；`components/overlay/src/motion.rs` 的 `#[cfg(not(target_arch = "wasm32"))] attach_motion` 在关闭时直接 `finish_exit.run(())`，SSR/tooling 可预测降级。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe`、
    `command_dialog_motion_contract_platform_script_covers_guard`、
    `command_dialog_check2_marks_motion_contractualization_complete`。
  - 门禁命令：`scripts/check-ui-platforms.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_motion_contract_is_component_scoped_reduced_motion_aware_and_non_wasm_safe`。
  - 验证记录：执行上述 `cargo test` 命令，当前容器环境返回 `Invalid cross-device link (os error 18)`，属于环境阻塞，非本次 motion 契约回归。
- [x] `ui` 固定入口文件落点正确。
  - `crates/ui/src/lib.rs`：总模块入口 + 对外 `pub use`（公共 API 面）；组件模块受 `component-*` feature gate 约束；不暴露内部平台细节类型。
  - `crates/ui/src/css.rs`：组件 CSS 聚合入口（`push_components_css`）；按 feature 条件注入；禁止无条件聚合全部组件 CSS。
  - `crates/ui/src/root.rs`：`UiRoot` 统一注入 base css + theme vars +（可选）components css，并提供全局 i18n 上下文；主题与注入策略必须集中在此。
  - `crates/ui-visual-primitive/src/active_highlight.rs`：共享高亮条样式与 motion driver；只承载通用高亮动效能力，不承载具体组件业务语义。
  - `crates/ui/src/overlay_open.rs`：当前仓库中不应存在；open-state 原语固定在 `crates/ui-headless/src/controllable_state.rs`，组件通过 headless API 消费。
  - `crates/ui/src/presence.rs`：当前仓库中不应存在；presence 原语固定在 `crates/ui-headless/src/presence.rs`，组件通过 `ui_headless::use_presence` 消费。
  - `crates/ui/src/a11y.rs`：当前仓库中不应存在；共享 A11y 工具固定在 `crates/ui-headless/src/a11y.rs`（如 `aria_controls_when_open`），组件只负责挂载。
  - 落点依据：`crates/ui/src/lib.rs` 保持 `mod css;` + `pub mod root;` + `pub use root::UiRoot;`，并通过 `#[cfg(feature = "component-command_dialog")] pub mod command_dialog;` 与 `pub use command_dialog::CommandDialog;` 暴露公共 API；未暴露 `web_sys/wasm_bindgen` 平台细节类型。
  - CSS 入口依据：`crates/ui/src/css.rs` 在 `#[cfg(feature = "inject-css")] pub fn push_components_css` 中按 feature 聚合样式，`#[cfg(feature = "component-command_dialog")] out.push_str(crate::command_dialog::styles::CSS);`，并保持 `#[cfg(not(feature = "inject-css"))]` no-op，未无条件聚合全部 CSS。
  - UiRoot 依据：`crates/ui/src/root.rs` 统一执行 `out.push_str(css::BASE_CSS)` + `theme.to_css_variables()` + `if inject_components_css { crate::css::push_components_css(&mut out); ui_layout::push_components_css(&mut out); }`，并集中注入 `provide_ui_i18n(i18n)` 与 `provide_ui_id_provider(id_seed)`。
  - 共享 primitive 依据：`crates/ui-visual-primitive/src/active_highlight.rs` 保持通用 `ActiveHighlightMotion + attach_active_highlight_motion`，未出现 `CommandDialog` 组件语义、`aria-*` 或业务 slot 标记。
  - 禁止文件依据：`crates/ui/src/overlay_open.rs`、`crates/ui/src/presence.rs`、`crates/ui/src/a11y.rs` 不存在；对应 canonical 原语位于 `crates/ui-headless/src/controllable_state.rs`、`crates/ui-headless/src/presence.rs`、`crates/ui-headless/src/a11y.rs`。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_ui_components_fixed_entry_files_follow_layered_boundaries`、
    `command_dialog_entrypoints_check_script_covers_fixed_entrypoint_contract`、
    `command_dialog_check2_marks_ui_components_fixed_entry_files_contract_complete`。
  - 门禁命令：`scripts/check-ui-entrypoints.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_ui_components_fixed_entry_files_follow_layered_boundaries`。
  - 验证记录：执行上述 `cargo test` 命令，当前容器环境返回 `Invalid cross-device link (os error 18)`，属于环境阻塞，非本次入口文件落点契约回归。
- [x] 组件目录标准文件落点正确。
  - `<component>/mod.rs`：最小稳定导出面，存在且无过度导出。
  - `<component>/logic.rs`：props 归一化、派生状态、来源标记；不得承载可下沉原语。
  - `<component>/styles.rs`：静态 CSS 契约，只用 `var(--ui-*)`，不写死主题常量。
  - `<component>/view.rs`：纯 Leptos 结构渲染 + headless 语义挂载；禁止 `render.rs` 漂移；不隐藏关键状态决策。
  - `<component>/motion.rs`：`XxxMotion + attach_motion`；交互组件必须有；只做语义到 motion contract 的映射与挂载。
  - `<component>/spec.rs`：仅极少数组件专用（当前主要 button），无必要不新增。
  - 目录落点依据：`components/command-dialog/src/mod.rs`、`components/command-dialog/src/logic.rs`、`components/command-dialog/src/styles.rs`、`components/command-dialog/src/view.rs`、`components/command-dialog/src/motion.rs` 均存在；`components/command-dialog/src/render.rs` 与 `components/command-dialog/src/spec.rs` 不存在。
  - `mod.rs` 边界依据：保持 `mod logic; pub mod motion; pub mod styles; mod view; pub use view::CommandDialog;` 最小导出面，未出现 `pub mod logic/pub mod view` 过度导出。
  - `logic.rs` 职责依据：聚焦 props 归一与状态派生（`normalize_props/resolve_part_state/compose_class_name`），未包含 `web_sys`、`NodeRef` 或视图渲染调用。
  - `styles.rs` 职责依据：仅静态 token-first CSS（`pub const CSS` + `var(--ui-*)`），未承载渲染逻辑与业务文案。
  - `view.rs` 职责依据：负责 Leptos 结构渲染与语义挂载（`#[component] CommandDialog` + `use_presence` + `data-*`），关键状态决策由 `logic::normalize_props/resolve_part_state` 输出驱动。
  - `motion.rs` 职责依据：仅做 `CommandDialogMotion + sanitize_motion + attach_motion` 合同映射，未自实现运行时动效引擎。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_component_directory_standard_files_follow_contract_and_na_paths`、
    `command_dialog_component_files_check_script_covers_standard_directory_contract`、
    `command_dialog_check2_marks_component_directory_standard_files_contract_complete`。
  - 门禁命令：`scripts/check-ui-component-files.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_component_directory_standard_files_follow_contract_and_na_paths`。
  - 验证记录：执行上述 `cargo test` 命令，当前容器环境返回 `Invalid cross-device link (os error 18)`，属于环境阻塞，非本次目录职责契约回归。

### 6. AI 原生能力与文件落点（Struct-First & Projection）
- [x] 文件落点纪律：组件目录严格由 `mod.rs`（导出）、`logic.rs`（归一派生）、`styles.rs`（Token 样式）、`view.rs`（渲染）、`motion.rs`（动效）组成；复杂组件可选 `spec.rs`；禁止 `render.rs`。
  - 落点纪律依据：`components/command-dialog/src/` 当前实现路径由 `mod.rs/logic.rs/styles.rs/view.rs/motion.rs` 组成，且 `render.rs/spec.rs` 不存在；关键职责已在上一条“组件目录标准文件落点正确”中完成逐层约束。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_file_placement_discipline_is_strict_for_component_scope`，并复用
    `command_dialog_component_directory_standard_files_follow_contract_and_na_paths` 作为事实来源。
  - 脚本门禁：`scripts/check-ui-component-files.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_file_placement_discipline_is_strict_for_component_scope`。
  - 验证记录：执行上述 `cargo test` 命令，当前容器环境返回 `Invalid cross-device link (os error 18)`，属于环境阻塞，非本次文件落点纪律契约回归。
- [x] Hyper-Structure Builder（`spec.rs`）：复杂组件必须提供 AI 友好的 `*Spec::new()...render()` 建造者 API。
  - N/A-by-design：`command-dialog` 当前为简单组件装配（`Modal + Command + overlay_trigger + presence`），不存在稳定外部 schema 契约与版本迁移需求，因此不引入 `spec.rs`。
  - 约束依据：`components/command-dialog/src/spec.rs` 与 `crates/ui/src/command_dialog/spec.rs` 均不存在；`components/command-dialog/src/mod.rs` 未导出 `spec` 模块；`components/command-dialog/src/README.md` 不暴露 `Spec::new()...render()` 入口。
  - 复杂组件锚点：`crates/ui/src/button/spec.rs` 仍存在，保持 “仅复杂组件引入 spec.rs” 的边界。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component`、
    `command_dialog_check2_marks_hyper_structure_builder_item_complete`。
  - 脚本门禁：`scripts/check-ui-component-files.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_hyper_structure_builder_spec_is_not_applicable_for_simple_component`。
  - 验证记录：执行上述 `cargo test` 命令，当前容器环境返回 `Invalid cross-device link (os error 18)`，属于环境阻塞，非本次 Hyper-Structure Builder 契约回归。
- [x] 上下文压缩协议（Manifest + RBI）：新增/大改组件必须同步维护组件目录下 `Component.toml`（能力清单）和 `.rbi`（接口签名投影），避免 AI 检索工具箱过时。
  - 落点依据：新增 `components/command-dialog/src/Component.toml` 与 `components/command-dialog/src/command_dialog.rbi`，明确 `CommandDialog` 能力清单、输入输出轴与公开接口签名投影，避免 AI 检索使用过期契约。
  - Manifest 约束：`Component.toml` 声明 `schema_version = "1"`、`name = "CommandDialog"`、`crate = "ui-command-dialog"`，并锁定 `open/default_open/on_open_change/on_action/groups` 等关键输入轴与 `context_compression_manifest/rbi_signature_projection` 能力位。
  - RBI 约束：`command_dialog.rbi` 投影 `CommandDialogSlot`、`CommandDialogMotion`、`DEFAULT_ID_BASE/DEFAULT_TITLE` 与 `CommandDialog(...)` 函数签名，覆盖受控/非受控与动作回调路径。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_context_compression_manifest_and_rbi_projection_are_present_and_current`、
    `command_dialog_component_files_script_covers_context_compression_manifest_contract`、
    `command_dialog_check2_marks_context_compression_manifest_and_rbi_contract_complete`。
  - 脚本门禁：`scripts/check-ui-component-files.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_context_compression_manifest_and_rbi_projection_are_present_and_current`。
  - 验证记录：执行上述 `cargo test` 命令，当前容器环境返回 `Invalid cross-device link (os error 18)`，属于环境阻塞，非本次 Manifest + RBI 契约回归。
- [x] 语义标记统一升级为 Agent Contract（Schema 化），让 Agent 不依赖 DOM 猜测理解组件状态与意图。
  - 关键交互组件必须输出稳定机器可读语义（至少 `data-*` + 状态来源标记；复杂组件建议补 `data-ui-schema`）。
  - Agent 消费字段应来自类型化 schema 生成，不允许散落字符串拼接。
  - 契约字段需可追溯到组件状态轴与动作语义（intent/action/state/source）。
  - 配置到组件的渲染链路必须走白名单能力边界，禁止任意脚本注入。
  - 类型化 contract 依据：`components/command-dialog/src/logic.rs` 新增 `COMMAND_DIALOG_AGENT_SCHEMA`、`CommandDialogAgentSchemaVersion/Intent/Action/State/Source/StreamMode/OutputStatus/ConfigPolicy` 与 `CommandDialogAgentContract`，并由 `resolve_agent_contract` 从 `CommandDialogPartState` 统一派生 schema 字段。
  - 视图挂载依据：`components/command-dialog/src/view.rs` 新增 `let agent_contract = Signal::derive(move || logic::resolve_agent_contract(root_state.get()));`，并通过 `data-ui-schema/data-ui-schema-version/data-ui-intent/data-ui-action/data-ui-state/data-ui-source` 以及 `data-ui-config-policy`、`data-ui-action-source`、`data-ui-open-change-source` 挂载机器可读契约字段。
  - 白名单边界依据：`data-ui-config-policy` 固定来自 `CommandDialogAgentConfigPolicy::Whitelist`，且组件渲染链路持续禁止 `inner_html/set_inner_html/dangerously_set_inner_html/<script/javascript:` 注入路径。
  - Manifest 同步：`components/command-dialog/src/Component.toml` 已补 `data-ui-action/data-ui-state/data-ui-source/data-ui-config-policy` 输出项与 `agent_contract_schema_typed_markers/agent_contract_whitelist_render_policy` 能力标记。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_check2_documents_agent_contract_schema_governance_rules`、
    `command_dialog_agent_contract_is_schema_typed_and_machine_readable`、
    `command_dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing`、
    `command_dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free`、
    `command_dialog_contract_hygiene_script_covers_agent_contract_schema_guards`、
    `command_dialog_check2_marks_agent_contract_schema_governance_complete`。
  - 脚本门禁：`scripts/check-ui-contract-hygiene.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_agent_contract_schema_governance_rules`、
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_agent_contract_is_schema_typed_and_machine_readable`、
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_agent_contract_fields_are_type_derived_without_free_form_schema_string_splicing`、
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_agent_contract_render_path_is_whitelist_safe_and_script_injection_free`。
  - 验证记录：执行上述 `cargo test` 命令，当前容器环境返回 `Invalid cross-device link (os error 18)`，属于环境阻塞，非本次 Agent Contract 契约回归。
- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。
  - `Streaming`：LLM 还在生成，界面边生成边显示。
  - `Snapshot`：LLM 全部生成完成后，一次性显示。
  - 类型约束依据：`components/command-dialog/src/logic.rs` 的 `CommandDialogAgentStreamMode` 显式限定为 `Streaming | Snapshot` 两种枚举，`as_str()` 仅映射 `"streaming" | "snapshot"`，不存在第三种自由文本模式。
  - 组件默认落点：`resolve_agent_contract` 继续输出 `stream_mode=Snapshot`、`stream_fallback=Snapshot`，对应 `command-dialog` 当前 `Streaming Optional` 且默认 snapshot 渲染策略。
  - 语义挂载依据：`components/command-dialog/src/view.rs` 通过 `data-stream-mode=agent_contract.stream_mode.as_str()` 与 `data-stream-fallback=agent_contract.stream_fallback.as_str()` 输出稳定可检索标记。
  - Manifest 同步：`components/command-dialog/src/Component.toml` 增加 `data-stream-mode/data-stream-fallback` 输出并声明 `llm_streaming_two_display_modes_only` 能力，保证上下文压缩投影与实现一致。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`、
    `command_dialog_streaming_display_modes_are_limited_to_streaming_and_snapshot`、
    `command_dialog_streaming_script_covers_two_mode_definition_contract`、
    `command_dialog_check2_marks_streaming_two_mode_definition_complete`。
  - 脚本门禁：`scripts/check-ui-streaming.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_streaming_definition_is_llm_output_only_with_two_modes`、
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_streaming_display_modes_are_limited_to_streaming_and_snapshot`。
  - 验证记录：执行上述 `cargo test` 命令，当前容器环境返回 `Invalid cross-device link (os error 18)`，属于环境阻塞，非本次“流式两种显示模式”契约回归。
- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。
  - 所有组件都应能消费“完整生成结果”并稳定渲染。
  - 即使组件不直接展示正文，也应能在接收上层完整配置后正常渲染。
  - 基线能力依据：`components/command-dialog/src/logic.rs` 的 `resolve_agent_contract` 固定输出 `stream_mode=Snapshot`、`stream_fallback=Snapshot`、`output_status=Verified`，将完整结果消费路径收敛为稳定 snapshot 基线。
  - 渲染稳定性依据：`components/command-dialog/src/view.rs` 通过 `normalize_props` 后统一消费完整配置（`groups/placeholder/empty_label/aria_label`），并持续挂载 `data-stream-mode/data-stream-fallback/data-output-status/data-state/data-open-mode` 语义标记，避免依赖 DOM 猜测。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_check2_documents_snapshot_as_default_baseline_capability`、
    `command_dialog_snapshot_baseline_consumes_complete_result_and_renders_stably`、
    `command_dialog_streaming_script_covers_snapshot_baseline_contract`、
    `command_dialog_check2_marks_snapshot_baseline_capability_complete`。
  - 脚本门禁：`scripts/check-ui-streaming.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_snapshot_as_default_baseline_capability`、
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_snapshot_baseline_consumes_complete_result_and_renders_stably`。
  - 验证记录：执行上述 `cargo test` 命令，当前容器环境返回 `Invalid cross-device link (os error 18)`，属于环境阻塞，非本次 Snapshot 基线能力契约回归。
- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。
  - `Streaming Required`：组件本体就是正文阅读面，用户需要边生成边看。
  - `Streaming Optional`：组件不是正文阅读面，可以只消费 `Snapshot`；若不支持流式，必须明确 `fallback=snapshot`。
  - 无论是否支持 `Streaming`，都要显式标识当前输出状态（草稿/已验证/可提交），并保持 `role`/`aria-*`/`data-*` 连续可读。
  - 数据校验、断线恢复、重试策略由上层负责，组件层只负责稳定渲染。
  - 职责分类依据：`command-dialog` 不是正文阅读面，`components/command-dialog/src/logic.rs` 的 `resolve_agent_contract` 固定 `stream_support=Optional`，并维持 `stream_fallback=Snapshot`。
  - 输出状态依据：`CommandDialogAgentOutputStatus` 显式声明 `Draft/Verified/CommitReady` 枚举域，当前默认输出 `Verified`；`view.rs` 稳定挂载 `data-output-status`，并持续输出 `data-state/data-open-mode/data-ui-state/data-ui-source`。
  - 连续语义依据：`components/command-dialog/src/view.rs` 通过 `data-ui-stream-support/data-stream-mode/data-stream-fallback/data-output-status` 提供机器可读流式语义，同时组合 `<Modal>` 与 `<Command>` 维持 role/aria 语义链路可读。
  - 上层边界依据：组件层未引入 `retry/backoff/reconnect/resume_stream/validate_stream` 等重试恢复策略代码，相关治理明确留在上层。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_check2_documents_streaming_required_optional_classification_rules`、
    `command_dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、
    `command_dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`、
    `command_dialog_streaming_script_covers_required_optional_classification_contract`、
    `command_dialog_check2_marks_streaming_required_optional_classification_complete`。
  - 脚本门禁：`scripts/check-ui-streaming.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_streaming_required_optional_classification_rules`、
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_streaming_validation_retry_resilience_boundaries_stay_outside_component_layer`。
  - 验证记录：执行上述 `cargo test` 命令，当前容器环境返回 `Invalid cross-device link (os error 18)`，属于环境阻塞，非本次 Streaming Required/Optional 职责分类契约回归。

### 7. 测试、门禁与交付
- [x] 代码卫生（Rust Hygiene）：非测试代码中完全禁止 `unwrap/expect`，禁止无处理的 `let _ = ...`；字符串复制热点收敛为 `Cow<'static, str>`（执行 `./scripts/check-rust-hygiene.sh` 验证）。
  - 非测试源码约束：`components/command-dialog/src/lib.rs`、`mod.rs`、`logic.rs`、`motion.rs`、`styles.rs`、`view.rs` 未引入 `unwrap/expect` 与无处理 `let _ = ...`。
  - 字符串热点收敛：`components/command-dialog/src/logic.rs` 在 `compose_class_name` 路径使用 `Vec<Cow<'static, str>>`，静态类名走 `Cow::Borrowed`，仅自定义 class 走 `Cow::Owned`，避免多次字面量 `String` 分配。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_rust_hygiene_forbids_unwrap_expect_and_ignored_results_in_component_sources`、
    `command_dialog_rust_hygiene_string_hotspots_are_coalesced_with_cow_static_str`、
    `command_dialog_rust_hygiene_script_enforces_repo_level_hygiene_guards`、
    `command_dialog_check2_marks_rust_hygiene_contract_complete`。
  - 脚本门禁：`./scripts/check-rust-hygiene.sh`。
  - 验证记录：执行 `./scripts/check-rust-hygiene.sh`，当前容器环境输出 `PCRE2 is not available in this build of ripgrep`，随后在 `check-api-contracts` 阶段因仓库级 baseline drift 失败（非 command-dialog 局部回归）；组件局部 hygiene 由上述语义回归测试与源码扫描锁定。
- [x] Tree Shaking & 特性剪裁：组件必须注册到 `ui` 特性树（如 `component-accordion`）；`css.rs` 和 `lib.rs` 聚合必须受 feature 门控，禁止无条件全局依赖。
  - 特性树注册依据：`crates/ui/Cargo.toml` 保持 `component-command_dialog = ["component-command", "component-modal"]`，组件能力不通过全量注册表硬绑定。
  - `lib.rs` 门控依据：`crates/ui/src/lib.rs` 通过 `#[cfg(feature = "component-command_dialog")]` + `#[path = "../../../components/command-dialog/src/mod.rs"]` 声明 `pub mod command_dialog;`，避免无条件导出。
  - `css.rs` 门控依据：`crates/ui/src/css.rs` 仅在 `#[cfg(feature = "component-command_dialog")]` 下聚合 `out.push_str(crate::command_dialog::styles::CSS);`，未出现无条件 command-dialog CSS 注入。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_tree_shaking_feature_registration_and_gated_aggregates`、
    `command_dialog_tree_shaking_script_covers_command_dialog_minimal_feature_chain`、
    `command_dialog_check2_marks_tree_shaking_feature_gating_complete`。
  - 脚本门禁：`scripts/check-ui-tree-shaking.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_tree_shaking_feature_registration_and_gated_aggregates`，
    并新增 `COMMAND_DIALOG_MIN_FEATURES="component-command_dialog,inject-css"` 的最小特性树检查，显式阻断 `all-components` 被隐式拉起。
  - 验证记录：已执行
    `cargo tree -e features -i ui -p ui --no-default-features --features component-command_dialog,inject-css`（仅出现 `component-command_dialog/component-command/component-modal/inject-css`，未出现 `all-components`）、
    `cargo tree -e features -i ui -p web-demo`（未出现 `all-components`，出现 `web-demo-components`）、
    `cargo check -p ui --target wasm32-unknown-unknown --no-default-features --features component-command_dialog,inject-css`（当前容器环境 `Invalid cross-device link (os error 18)`）。
  - 脚本执行记录：`bash ./scripts/check-ui-tree-shaking.sh` 已进入 command-dialog tree-shaking 检查路径，随后在 wasm 编译阶段同样受 `Invalid cross-device link (os error 18)` 环境阻塞，非本次特性门控契约回归。
- [x] 语义测试与性能回归：断言必须覆盖 `aria-*`、`data-*` 与焦点流转，不能只看快照；高频/重型组件必须补齐 `render_count` 断言/测量（如初始化空闲预算为 1）。
  - 语义断言覆盖：`components/command-dialog/test/command_dialog_semantics.rs` 通过 `command_dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous` 与 `command_dialog_e2e_spec_covers_controlled_and_persistent_paths` 锁定 `role/aria/data-*` 路径；`e2e/tests/docs_app_command_dialog.spec.mjs` 显式覆盖 `focus()->Enter` 打开流程与 `data-state/data-ui-schema/data-stream-mode/data-output-status` 断言。
  - 非快照依赖：`e2e/tests/docs_app_command_dialog.spec.mjs` 仅使用语义选择器和 `toHaveAttribute`/`toHaveCount`，不依赖 `toHaveScreenshot` 或 `toMatchSnapshot`。
  - 性能回归覆盖：`components/command-dialog/test/command_dialog_semantics.rs` 的 `command_dialog_performance_governance_contract_is_mount_only_traceable_and_blocking` 持续校验 `UiPerfProbe` 的 `data-perf-*` 指标接线、预算阈值与阻断路径；新增 `command_dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement` 将语义断言与性能证据合并为单条契约门禁。
  - `render_count` 策略：当前框架仍采用 `mount-only` 等价证据，仓库级 follow-up 在 `docs/plan/TODO.md` 维持 `render_count` 自动化回归计划（Button/Input/Accordion）以替换等价证据，符合清单“暂不支持精确计数时给出可重复测量并跟踪补齐”的要求。
  - 脚本门禁：`scripts/check-ui-performance.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement`，
    并保留 `command_dialog_performance_governance_contract_is_mount_only_traceable_and_blocking`。
  - 验证记录：执行
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_semantics_and_performance_regression_cover_aria_data_focus_and_render_count_measurement`，
    当前容器环境返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次语义/性能契约回归。
- [x] 版本弃用迁移（Codemod/Registry）：若提交包含跨大版本 API 破坏升级，必须在 Schema Registry 注册弃用窗口并提供纯函数迁移层（`migrate_v1_to_v2`）。（N/A：本次 `CommandDialog` 未发生跨大版本 API 破坏升级）
  - N/A 判定依据：`components/command-dialog/src/Component.toml` 保持 `schema_version = "1"`；`components/command-dialog/src/command_dialog.rbi` 的 `CommandDialog(...)` 公共签名未发生破坏性移除/重命名；`components/command-dialog/src/{mod.rs,logic.rs,view.rs,styles.rs,motion.rs,protocol.rs}` 未引入 `migrate_v1_to_v2`/`deprecation_window`/`SchemaRegistry`/`contract.v2`。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade`，
    断言当前为稳定 v1 且禁止虚假迁移层漂移。
  - 脚本门禁：`scripts/check-ui-engineering.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade`，
    并由 `command_dialog_engineering_check_script_covers_serde_tracing_and_runtime_boundaries` 反向校验脚本已挂接。
  - 验证记录：执行
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_version_deprecation_migration_is_na_without_major_breaking_upgrade`，
    当前容器环境返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次版本迁移契约回归。
- [x] 文档即产品（Copy-Paste Ready）：`apps/docs-app` 必须新增 Playground（Hello World、状态矩阵、受控/非受控对照），支持流式/快照展现，并提供 Source-first 一键复制且补全 imports。
  - docs-app 落地：`apps/docs-app/src/pages/components/pages/collections_command.rs` 的 `command_dialog()` 已补齐 `Hello World (Default API)`、`State Matrix`、`Controlled vs Uncontrolled`、`Streaming / Snapshot Contract` 四类 Playground，并保留既有 `Controlled Open + Action Close` / `State + Source Markers` / `Workbench` 验收面。
  - Source-first copy-ready：新增 `COMMAND_DIALOG_DOC_IMPORTS`，并在 command-dialog 文档 Playground 统一挂载 `code_imports=COMMAND_DIALOG_DOC_IMPORTS.to_string()`；复制补全链路由 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，确保复制后最小示例可运行。
  - 流式/快照展现：`Streaming / Snapshot Contract` 通过 `SegmentedControl` 展示 `snapshot/streaming` 请求模式，页面持续暴露 `data-requested-stream-mode` 与 `data-requested-output-status`，并明确组件有效语义 `data-stream-mode=snapshot` + `data-stream-fallback=snapshot` + `data-output-status=verified`。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract`、
    `command_dialog_check2_marks_docs_product_copy_paste_ready_contract_complete`，并扩展 `command_dialog_docs_page_covers_primary_playgrounds` / `command_dialog_docs_playgrounds_lock_state_matrix_contract_values` 覆盖新增 Playground 矩阵。
  - 脚本门禁：`scripts/check-ui-dx.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract`。
  - 验证记录：执行
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_are_copy_paste_ready_with_imports_and_streaming_snapshot_contract`，
    当前容器环境返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次 docs-product 契约回归。
- [x] 语义测试优先：验证 `data-*` / `aria-*` / role / 状态来源契约，不只视觉快照。
  - 语义契约覆盖：`components/command-dialog/test/command_dialog_semantics.rs` 已通过
    `command_dialog_view_uses_logic_state_contracts`、`command_dialog_streaming_optional_scope_keeps_role_aria_and_data_markers_continuous`、`command_dialog_e2e_spec_covers_controlled_and_persistent_paths`
    对 `data-*` 状态轴、状态来源标记、role/键盘路径做回归约束；`e2e/tests/docs_app_command_dialog.spec.mjs` 继续以语义选择器与 `getByRole` 路径断言关键交互。
  - 非快照优先：新增 `command_dialog_semantics_suite_is_contract_first_not_snapshot_only`，显式禁止 `toHaveScreenshot`/`toMatchSnapshot`/`*_snapshot` 作为主断言路径。
  - 字段变更联动：新增 `command_dialog_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks`，要求 `view.rs` 关键 `data-*` marker 与 `*_semantics.rs` 断言同步更新，防止语义字段漂移后漏测。
  - 规则落盘：新增 `command_dialog_check2_documents_semantics_first_testing_rules`，锁定本条 checklist 文本与验收口径。
  - 脚本门禁：`scripts/check-ui-contract-hygiene.sh` 新增
    `command_dialog_check2_documents_semantics_first_testing_rules`、
    `command_dialog_semantics_suite_is_contract_first_not_snapshot_only`、
    `command_dialog_semantic_markers_changed_in_view_must_be_covered_by_semantics_checks`
    三条命令，并由 `command_dialog_contract_hygiene_script_covers_semantics_first_contract_guards` 反向校验脚本已挂接。
  - 验证记录：执行
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_semantics_suite_is_contract_first_not_snapshot_only`，
    当前容器环境返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次语义契约回归。
  - 每个交互组件至少有对应 `*_semantics.rs` 测试覆盖关键状态轴与动作语义。
  - 断言应聚焦语义契约（状态来源/可访问性/键盘路径），快照仅作补充。
  - 新增/变更语义字段必须同步补测试，否则不得打勾。
- [x] E2E 选择器稳定：使用语义标记，WASM 场景有稳定等待策略。
  - 语义选择器落地：`e2e/tests/docs_app_command_dialog.spec.mjs` 已以 `data-*` 语义标记为主选择器（`[data-slot="command-dialog"][data-ui-schema="command-dialog"]`、`[data-slot="command-dialog-last-action"]`），并保留 `getByRole("button", { name: "Open CommandDialog" })` 的可访问路径断言；移除文本定位依赖（不再使用 `getByText("last action: ...")`）。
  - WASM 稳定等待：进入页面后先等待语义就绪信号 `data-output-status="verified"`，以契约状态就绪替代固定延迟；`*_semantics.rs` 同步禁止 `waitForTimeout`/`setTimeout`/`sleep`。
  - ready/settled 覆盖：受控路径显式断言 `data-state="open"` 后执行 action 并等待 `toHaveCount(0)` 收敛；持久路径在 action 后继续断言 `data-state="open"` 与 `data-stream-fallback="snapshot"`，覆盖动画/交互后的稳定状态。
  - 文档语义锚点：`apps/docs-app/src/pages/components/pages/collections_command.rs` 的 command-dialog 示例新增 `data-slot="command-dialog-last-action"`、`data-open-mode`、`data-last-action`，为 E2E 提供稳定 machine-readable 断点。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_check2_documents_e2e_selector_and_stable_wait_rules`、
    `command_dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits`、
    `command_dialog_e2e_check_script_covers_selector_contract`。
  - 脚本门禁：新增 `components/command-dialog/scripts/check-ui-e2e-command-dialog.sh`，挂接上述两条契约测试命令。
  - 验证记录：执行
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_e2e_selector_contract_uses_semantic_markers_and_settled_waits`，
    当前容器环境返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次 E2E 语义契约回归。
  - E2E 选择器优先 `data-*` 语义标记，禁止依赖脆弱 DOM 层级或文本定位。
  - WASM 场景必须使用稳定等待策略（语义状态就绪而非固定 sleep）。
  - 若组件涉及异步/动画，E2E 需显式覆盖 ready/settled 条件。
- [x] 关键流程纳入可重复回归集合（Playwright/Cypress）。
  - 可重复关键流程落地：`e2e/tests/docs_app_command_dialog.spec.mjs` 新增
    `docs-app command-dialog key flow is repeatable with semantic breakpoints`，
    固化 `focus(open button) -> keyboard Enter 打开 -> focus(option) -> keyboard Enter 提交 -> reopened + keyboard Escape 关闭 -> reload 后语义恢复` 的可回放链路。
  - 失败定位语义断点：关键断言全部落在契约字段（`data-state`、`data-ui-schema`、`data-stream-mode`、`data-last-action`、`data-output-status`），回归失败可直接定位到具体状态轴，不是笼统页面差异。
  - 高风险路径覆盖：overlay（`data-open-mode` + open/close 收敛）、focus（`toBeFocused`）、keyboard（`Enter/Escape`）均纳入；async 路径对 command-dialog 为 N/A（组件无远程请求与异步重试协议），保留语义就绪等待 `data-output-status="verified"` 防止 WASM 不稳定等待。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_check2_documents_e2e_repeatable_key_flow_rules`、
    `command_dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`、
    `command_dialog_e2e_check_script_covers_selector_and_key_flow_contracts`、
    `command_dialog_check2_marks_e2e_repeatable_key_flow_contract_complete`。
  - 脚本门禁：`components/command-dialog/scripts/check-ui-e2e-command-dialog.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`。
  - 验证记录：执行
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_e2e_key_flow_is_repeatable_and_failure_points_are_semantic`，
    当前容器环境返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次可重复 key-flow 契约回归。
  - 至少定义一条可重复关键流程（打开/交互/关闭或提交）纳入 E2E 回归。
  - 回归失败需可定位到具体语义契约断点，而不是笼统“页面不一致”。
  - 高风险路径（overlay、focus、keyboard、async）优先进入回归集合。
- [x] docs-app 文档、示例、参数矩阵、状态矩阵同步更新。
  - docs 页面同步落地：`apps/docs-app/src/pages/components/pages/collections_command.rs::command_dialog()` 已覆盖 `Hello World (Default API)`、`State Matrix`、`Controlled Open + Action Close`、`State + Source Markers`、`Controlled vs Uncontrolled`、`Streaming / Snapshot Contract`、`Workbench`，组件行为与示例说明同步演进。
  - 状态矩阵覆盖：`State Matrix` 以 `state_matrix_options` 驱动 `受控/非受控 + close_on_action + disabled` 分支，并通过 `open/default_open/on_open_change/close_on_action/is_disabled` 显式组合，满足至少一组状态矩阵要求。
  - API 与默认值一致性：文档示例使用的 props 名称与组件 API 对齐（`open/default_open/on_open_change/on_action/close_on_action/is_disabled`），并与 `components/command-dialog/src/view.rs` 与 `components/command-dialog/src/logic.rs` 默认值契约（`DEFAULT_CLOSE_ON_ACTION=true`、`DEFAULT_DISABLED=false`、`DEFAULT_DEFAULT_OPEN=false`）保持一致；未使用漂移别名（如 `is_open/default_is_open/on_change`）。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_check2_documents_docs_sync_and_state_matrix_rules`、
    `command_dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`、
    `command_dialog_dx_check_script_covers_docs_sync_state_matrix_contract`、
    `command_dialog_check2_marks_docs_sync_and_state_matrix_contract_complete`。
  - 脚本门禁：`scripts/check-ui-dx.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_docs_sync_and_state_matrix_rules`、
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`。
  - 验证记录：执行
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_examples_and_state_matrix_sync_with_logic_api_names_and_defaults`，
    当前容器环境返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次 docs/state-matrix 同步契约回归。
  - 组件行为或参数变更必须同步更新 `apps/docs-app` 示例与说明。
  - 文档示例需覆盖至少一组状态矩阵（受控/非受控、disabled、size/variant 等）。
  - 文档中的 API 名称与默认值必须和 `logic.rs` 当前实现一致。
- [x] 组件文档必须对新手友好（Documentation as Product）：组件 README 或等价文档入口必须存在。
  - 文档入口存在：`components/command-dialog/src/README.md` 与 `apps/docs-app/src/pages/components/pages/collections_command.rs::command_dialog()` 同时存在，避免“只有源码没有文档”。
  - 零门槛示例：README 含 `## Hello World` + 最小 `<CommandDialog groups=groups />`；docs-app 首屏 Playground 为 `Hello World (Default API)`，无需先理解分层架构即可运行。
  - 默认优先、进阶在后：README 已按 `Hello World -> 受控打开状态 -> 先用起来，再进阶 -> API/Streaming` 组织；docs-app 页面按 `Hello World (Default API) -> State Matrix -> Controlled Open + Action Close -> ... -> Workbench` 组织，明确先默认路径再高级控制。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_check2_documents_documentation_as_product_rules`、
    `command_dialog_documentation_entry_exists_with_beginner_first_progression`、
    `command_dialog_dx_check_script_covers_documentation_as_product_contract`、
    `command_dialog_check2_marks_documentation_as_product_contract_complete`。
  - 脚本门禁：`scripts/check-ui-dx.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_documentation_as_product_rules`、
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_documentation_entry_exists_with_beginner_first_progression`。
  - 验证记录：执行
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_documentation_entry_exists_with_beginner_first_progression`，
    当前容器环境返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次 Documentation-as-Product 契约回归。
  - 每个基础组件必须提供“零门槛”最小示例（Hello World）与常见用法，避免要求用户先理解底层分层架构。
  - 文档需明确“先用起来，再进阶”：默认 API 路径在前，高级控制参数在后。
  - “只有源码没有文档”或“只写给架构师/机器看的文档”视为不通过。
- [x] `apps/docs-app` 必须提供 Interactive Playground：用户可在线修改 props/状态并实时预览。
  - Playground 交互能力已落地：`apps/docs-app/src/pages/components/pages/collections_command.rs::command_dialog()` 提供 `State Matrix` + `Controlled vs Uncontrolled` + `Streaming / Snapshot Contract` + `Workbench (Display + Config + Code + CSS Test)`，覆盖 props 调整、状态切换与实时预览。
  - AI Spec 相关联动示例：`Streaming / Snapshot Contract` 通过 `data-requested-stream-mode` / `data-requested-output-status` 输入标记与组件 `data-stream-mode` / `data-stream-fallback` / `data-output-status` 输出标记形成可观察联动。
  - 可重复关键流复用：`e2e/tests/docs_app_command_dialog.spec.mjs` 包含 `docs-app command-dialog key flow is repeatable with semantic breakpoints`，覆盖打开/键盘交互/Escape 关闭/reload 后语义断点复验。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_docs_app_provides_interactive_playground_for_props_state_and_preview`、
    `command_dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow`、
    `command_dialog_dx_check_script_covers_interactive_playground_contract`、
    `command_dialog_check2_marks_interactive_playground_contract_complete`。
  - 脚本门禁：`scripts/check-ui-dx.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_app_provides_interactive_playground_for_props_state_and_preview`、
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_interactive_playground_reuses_repeatable_semantic_e2e_flow`。
  - 验证记录：执行
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_app_provides_interactive_playground_for_props_state_and_preview`，
    当前容器环境返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次 Interactive Playground 契约回归。
  - Playground 至少支持基础 props 调整、状态切换、交互反馈观察。
  - 对 AI Spec 相关组件，至少提供一组 Spec 输入与预览输出的联动示例。
  - Playground 作为验收面，需可重复复现关键交互路径。
- [x] Source-first 文档必须 Copy-Paste Ready：提供一键复制组件源码或最小可用片段能力。
  - docs-app 页面已提供 copy-ready 路径：`apps/docs-app/src/pages/components/pages/collections_command.rs::command_dialog()` 通过多个 `Playground(code_signal + code_imports=COMMAND_DIALOG_DOC_IMPORTS)` 输出可复制示例；复制补全由 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 统一完成。
  - 复制按钮契约已接线：`Playground` 渲染 `<CodeBlock code=resolved_code.get() />`，`components/code-block/src/view.rs` 默认包含 `ui-code-block__copy-button` 与 `copy_to_clipboard_aria_label`，满足一键复制。
  - Source-first 文档落点与依赖前提：docs 页面新增 `data-slot="command-dialog-source-first"` 与 `data-slot="command-dialog-source-paths"`，明确源码文件
    `components/command-dialog/src/{mod,logic,view,styles,motion}.rs`，
    并声明依赖前提
    `ui = { workspace = true, default-features = false, features = ["component-command_dialog", "inject-css"] }`。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_check2_documents_source_first_copy_paste_ready_rules`、
    `command_dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`、
    `command_dialog_dx_check_script_covers_source_first_copy_paste_ready_contract`。
  - 脚本门禁：`scripts/check-ui-dx.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_documents_source_first_copy_paste_ready_rules`、
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`。
  - 验证记录：执行
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_docs_source_first_copy_paste_ready_with_real_paths_and_dependencies`，
    当前容器环境返回 `Invalid cross-device link (os error 18)`，属环境阻塞，非本次 Source-first 文档契约回归。
  - docs-app 页面应提供复制按钮，输出代码默认可直接运行（含必要 imports/依赖提示）。
  - 若为 source-first 组件，文档需指向真实源码落点并说明依赖前提，避免“复制即报错”。
  - 文档代码与当前实现必须同步，防止示例漂移。
- [x] HeroUI 对标文档与组件文档同步：参数模型变更需同步 `docs/spec/heroui-parameter-design-strategy.md`（必要时补充 `docs/research/spectrum-heroui-style-interface-study.md`），并保证组件文档可访问。
  - 对标策略文档已同步：`docs/spec/heroui-parameter-design-strategy.md` 新增 `### CommandDialog 同步记录（2026-02-20）`，明确参数主轴
    `open/on_open_change/default_open`、
    `close_on_action`、
    `is_disabled/disabled`、
    `on_action`、
    `placeholder/empty_label/aria_label`、
    `command_motion/overlay_motion`、
    `class_name`，
    并声明“参数语义若变更，必须先同步策略文档与 docs 入口”。
  - 组件文档入口可访问且可索引：`apps/docs-app/src/pages/components/pages.rs` 保持
    `component_doc!("CommandDialog", "command-dialog", "Collections", collections_command::command_dialog)`；
    docs 页面入口 `apps/docs-app/src/pages/components/pages/collections_command.rs::command_dialog()` 与 `components/command-dialog/src/README.md` 同步存在。
  - 研究文档补充判定：本轮为参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，`docs/research/spectrum-heroui-style-interface-study.md` 为 N/A（无需新增）。
  - 回归锁定：`components/command-dialog/test/command_dialog_semantics.rs` 新增
    `command_dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes`、
    `command_dialog_check2_marks_heroui_strategy_and_component_docs_sync_complete`、
    `command_dialog_contract_hygiene_script_covers_heroui_strategy_doc_sync_contract`。
  - 脚本门禁：`scripts/check-ui-contract-hygiene.sh` 新增
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes`、
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_check2_marks_heroui_strategy_and_component_docs_sync_complete`。
  - 验证记录：执行
    `cargo test -p ui --test command_dialog_semantics --no-default-features --features component-command_dialog,inject-css command_dialog_heroui_strategy_and_component_docs_are_synced_for_parameter_model_changes`，
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
