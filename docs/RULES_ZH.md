# rust-ui 规则文档（v0）

> 目标：用 Rust + Leptos 复刻 React Spectrum 的分层（Stately/Aria/Spectrum），并通过 Tauri 覆盖 Web/桌面/Android(WebView)。  
> 风格取向：现代化（OKLCH + OLED）、强交互（Framer Motion/HeroUI 级别的“弹簧手感”）。

## 0. 总则（必须遵守）

- **低耦合 / 高内聚**：每个 crate 只做一件事，并且边界清晰。
- **分层不破**：状态原语（state-primitives）→ 行为与可访问性（headless）→ 组件（components）→ 应用（apps）。
- **不把实现细节透传到上层**：上层依赖稳定的“契约”（struct/enum/trait），而不是下层内部类型。
- **动效优先物理（Spring Physics）**：交互反馈尽量使用弹簧驱动（非 CSS transition / 非纯 duration/easing）。
- **默认可访问性（A11y）**：headless 输出语义、键盘与焦点行为；components 负责视觉表达。

## 1. 仓库结构与目录职责

```
.
├── crates/
│   ├── ui-state-primitives  # 纯状态（Stately）
│   ├── ui-headless          # 行为 + A11y（Aria）
│   ├── ui-theme             # 设计系统 tokens → CSS vars（OKLCH + OLED）
│   ├── ui-motion            # 高级动效引擎/后端（WAAPI + Spring runtime）
│   ├── ui-components        # 最终组件（Spectrum）
├── apps/
│   ├── web-demo             # 可提交的 Web demo（Trunk CSR）
│   ├── docs-app             # 文档与组件工作台入口
│   └── tauri-demo           # 可提交的 Tauri 壳（桌面验证入口）
├── examples/          # 本地调研/参考（默认不提交，见 .gitignore）
└── docs/
    ├── plan/          # 计划与 DAG
    ├── spec/          # 规格冻结（motion/mvp…）
    └── research/      # 调研笔记（调研定位等）
```

- **`apps/*`**：必须可运行、可展示真实交互与 A11y（用来验收）。
- **`examples/_upstream/*`**：只用于本地 clone 调研仓库（React Spectrum / motion / heroui / shadcn / animate-ui 等）；**默认不进 git**（`.gitignore` 忽略 `examples/`）。

## 2. 分层与依赖规则（最重要）

### 2.1 依赖方向（单向）

- `ui-state-primitives`：**不依赖任何内部 crate**（平台无关；禁止 DOM/web-sys）。
- `ui-theme`：不依赖 `ui-components`（tokens 不知道组件存在）。
- `ui-headless`：可选依赖 `ui-state-primitives`；**禁止依赖** `ui-components` / `ui-theme`。
- `ui-motion`：不依赖 `ui-components`（引擎不关心组件）。
- `ui-components`：允许依赖 `ui-headless + ui-theme + ui-motion`（必要时才依赖 `ui-state-primitives`）。
- `apps/*`：依赖 `ui-components`（上层不直接接触 `web-sys`）。

### 2.2 每层职责（对标 React Spectrum）

#### `ui-state-primitives`（React Stately）

- 只做：**状态建模**（受控/非受控、选择、集合、开关等）。
- 不做：DOM、事件标准化、样式、动画。
- 要求：单元测试覆盖关键状态机/受控行为。

#### `ui-headless`（React Aria）

- 只做：**交互与 A11y**（press/focus-visible/roving tabindex/aria-* 等）。
- 输出形态：**handlers + attrs 的结构体**，由组件层显式挂载（不要隐式 spread 魔法）。
- 不做：视觉表现（不写 class、不写 CSS、不做动画编排）。
- **Feature gating**：
  - 默认 `web`（CSR）可用。
  - `ssr` 下提供降级实现：能编译、能返回合理默认值，但不注册 window/document 监听。

#### `ui-theme`（Design Tokens）

- 只做：tokens → CSS Variables（字符串输出）。
- 颜色规范：**OKLCH**；新增 **OLED** 主题（真黑背景）。
- 不做：组件 CSS（组件 CSS 在 `ui-components`）。

#### `ui-motion`（Motion Engine）

- 只做：动效执行与运行时（Web 后端等）。
- 必须支持 `prefers-reduced-motion`：reduce 时应跳过/降级。
- 非 wasm/SSR：允许 no-op（保持编译通过）。

#### `ui-components`（最终组件库）

- 只做：把 `ui-state-primitives` 状态 + `ui-headless` 行为 + `ui-theme` 样式 + `ui-motion` 动效组合成最终组件。
- 对外 API：尽量小而稳（v0 冻结后避免破坏性改动）。
- 公开 API 禁止暴露 `web-sys` 类型；DOM 细节只存在于 `cfg(wasm32)` 的内部实现中。

## 3. 组件内部结构（ARCHITECTURE_ZH 风格）

每个组件建议拆为：

- `logic.rs`：props 归一化、派生状态、组合 headless hooks、决定 class/variant。
- `styles.rs`：组件的**静态 CSS 字符串**（只使用 tokens：`var(--ui-*)`）。
  - 样式孤岛防御：关键视觉属性必须使用防御性变量链（`var(--ui-*, var(--ui-fallback-*))`）。
  - 禁止在组件 `styles.rs` 直接写 Hex/RGB/裸尺寸作为 fallback 终值。
  - fallback 终值必须来自 token 层统一输出（SSOT），不得在组件内分散维护。
- `motion.rs`：组件 motion contract（`XxxMotion`/`XxxMotionPreset`）+ `attach_motion(...)`。
- `view.rs`：纯 Leptos view（HTML 结构 + class/attrs/handlers 挂载）。
- 副作用分层硬规则：
  - `logic.rs` 只表达副作用意图（`Command`），不得依赖 `web_sys`/DOM 事件对象。
  - 副作用相关更新推荐返回 `(State, Vec<Command>)`（或等价结构），避免在 `view.rs` 回调里分散业务决策。
  - `view.rs`/adapter 负责执行 `Command` 到平台 API 的映射（如 `PreventDefault`、`FocusById`）。
  - 规范详见：`docs/spec/side_effect_command_pattern.md`。
- 受控外交特区硬规则（Foreign Zone / Escape Hatches）：
  - 命令式第三方库（ECharts/Maps 等）只能在显式 Foreign Zone 中接入，禁止散落在通用 `view.rs` 路径。
  - `logic.rs` 只发出 `YieldControl/CleanupForeign` 意图，不直接持有第三方实例与 DOM 句柄。
  - `view.rs` adapter 负责 `init/update/destroy`，并在组件卸载时强制清理，防止内存泄漏。
  - 禁止第三方库反向写入组件核心状态机；状态回流必须经 Action/Command 桥接。
  - 规范详见：`docs/spec/foreign_zone_escape_hatches.md`。
- 焦点连续性硬规则（Global Focus Stack + Graveyard GC）：
  - 焦点是全局单例资源，禁止组件私有保存可腐烂 DOM 引用作为 restore 真相源。
  - 焦点恢复目标必须记录“策略”（selector/fallback policy），而非 `NodeRef` 快照。
  - 层叠 overlay 必须通过全局 Focus Manager `push/pop trap` 协调；仅 topmost trap 可控制焦点。
  - 容器强制卸载时必须触发焦点墓地回收（invalidate/re-parent），防止恢复到 Zombie Node。
  - 规范详见：`docs/spec/focus_global_stack_gc.md`。
- WASM 泛型体积硬规则：
  - 组件核心逻辑默认优先具体类型（`bool`/`enum`/`f64`），避免无收益泛型扩散。
  - 单态化会放大 wasm 包体；在核心行为接口中可用 `&dyn Trait` 等边界收敛泛型。
  - 只有确认存在复用收益时才保留泛型，禁止“为泛型而泛型”。
  - 规范详见：`docs/spec/wasm_generic_bloat.md`。
- 几何决策硬规则（Two-Pass Rendering）：
  - 几何依赖组件（tooltip/popover/menu 等）必须走“Intent -> Measure -> Rectification”流程，禁止一次状态更新直接拍板最终布局。
  - `logic.rs` 只处理 `LayoutSnapshot` 纯数据与几何计算，禁止直接读取 DOM / `web_sys` 测量对象。
  - `view.rs`/adapter 负责测量并回传 `LayoutSnapshot`，不得在视图层绕过逻辑层直接决定 `actual_placement`。
  - 必须有收敛保护：修正逻辑幂等 + 稳定相等门，避免测量/更新死循环。
  - 规范详见：`docs/spec/ui_physics_two_pass_rendering.md`。
- 异步阻抗硬规则（State as Data, Async as Command）：
  - `logic.rs` 严禁持有 `Future`/runtime 句柄；状态只保存请求元数据（如 `RequestId`、loading/error/result）。
  - 异步触发必须走命令契约（如 `FetchData`/`CancelRequest`），由 `view.rs`/effect adapter 执行实际异步任务。
  - 任务回包必须回流为 `Action` 再进入 `logic.rs`，禁止在 view 层绕过逻辑直接改业务状态。
  - 必须做竞态仲裁：仅接受当前活动 `RequestId` 的返回，过期响应必须丢弃；新请求需可取消旧请求或至少逻辑可忽略其回包。
  - 规范详见：`docs/spec/async_state_as_data_command.md`。
- Headless 去状态化硬规则（Headless Purification）：
  - `ui-headless` 只负责语义映射与输入归一，禁止成为第二状态源；复杂交互状态机必须上交 `ui-state-primitives`/`logic.rs`。
  - 禁止在组件公共路径直接引入“自带内部状态”的 headless hook（包括第三方与仓库内实现），避免 headless 与 logic 双状态漂移。
  - `ui-headless` 输出以 `attrs/handlers/action-intent` 为主；业务状态更新必须通过 action 回流 `logic.rs` 决策。
  - `focus trap`、`roving tabindex`、`grid nav` 等交互不变量应在 primitives/logic 可单测验证，headless 层仅做 ARIA/键盘映射。
  - 规范详见：`docs/spec/headless_purification.md`。
- 宏观/微观双状态机硬规则（Macro/Micro Duality）：
  - 高频连续交互（drag/resize/gesture）禁止每帧走完整 `View -> Action -> logic -> View` 往返链路，避免桥接开销导致卡顿。
  - `logic.rs` 负责宏观离散状态（`Open/Closed/Dragging`）与边界决策（`DragStart/DragEnd`），不负责每帧像素物理更新。
  - `view.rs`/`ui-motion` 在 `Dragging` 期间可本地持有微观物理态（offset/velocity）并直接驱动渲染。
  - 交互结束必须回流 `Action::DragEnd { final_offset, final_velocity }` 与逻辑层和解，恢复宏观状态单一真相源。
  - 规范详见：`docs/spec/macro_micro_dual_state_machine.md`。
- 集合组件注册协议硬规则（Registration Protocol）：
  - 集合类组件（Accordion/Tabs/Menu/Select）不得假设父层天然拥有完整子项列表；动态子项必须走 `Register/Unregister` 生命周期上报。
  - `logic.rs`/primitives 必须维护显式 `items_order`（或等价有序结构），禁止依赖 `HashSet` 迭代顺序做键盘导航。
  - `view.rs`/adapter 负责把挂载/卸载与顺序变化回传逻辑层；逻辑层据此收敛 `focused_id`、roving index、expanded 集合。
  - 顺序重排与动态删除必须幂等可恢复：`Register` 重入安全、`Unregister` 可重入、失效焦点自动迁移到有效项。
  - 规范详见：`docs/spec/collection_registration_protocol.md`。
- 环境订阅流硬规则（Environment as Phantom Input）：
  - 环境输入（resize/media/intersection）必须先在 `view.rs`/adapter 语义化与采样，禁止把原始高频事件直接洪泛到 `logic.rs`。
  - `logic.rs` 只消费高层环境 action（如 `BreakpointChanged`/`ColorSchemeChanged`/`VisibilityChanged`），不持有 observer/runtime 监听句柄。
  - 高频连续环境跟随必须走 Pull 模式：logic 发 `Command::Start/Stop*`，由 view/motion 本地循环执行；低频变化走 Push action。
  - 环境流必须有门控策略（阈值、节流、去抖），并可收敛回稳定宏观状态，避免状态风暴。
  - 规范详见：`docs/spec/environment_subscription_streams.md`。
- Kernel/Shell 总线硬规则（Industrial Contract）：
  - 基础设施层保证“物理隔离 + 可重放”：workspace 分层与 token 体系必须保持稳定边界，不得回退为跨层耦合实现。
  - Kernel 只管理离散状态与命令意图（state machine + command + registry），禁止直连 DOM/runtime/observer。
  - Shell 负责切片渲染、命令执行、物理测量与环境桥接，禁止绕过 Kernel 直接写宏观业务状态。
  - 高频连续反馈默认本地执行，边界事件回流 Kernel 收敛，禁止每帧强穿 `View -> Action -> logic -> View`。
  - 规范详见：`docs/spec/kernel_shell_architecture.md`。
- SSR Hydration 一致性硬规则（Hydration Discontinuity）：
  - 逻辑初始化禁止直接依赖 `now/random` 等非确定性源；必须通过可重放 provider（如 `IdProvider/NowProvider`）注入。
  - SSR 必须输出状态传输载荷（`server-state` 序列化快照），客户端 hydration 必须走 `Logic::hydrate(...)` 恢复，而非重新 `new()`。
  - 组件状态结构必须可序列化/反序列化（`Serialize + Deserialize`），并保证首帧语义与 SSR HTML 一致。
  - ID 相关语义（`id`/`aria-labelledby`/`aria-controls`）必须跨 SSR/CSR 稳定一致，禁止 hydration 后漂移。
  - 规范详见：`docs/spec/ssr_hydration_discontinuity.md`。
- 插槽投影策略硬规则（Phantom Projection）：
  - 容器类组件（Tabs/Accordion/Stepper 等）必须显式声明内容投影策略（`Lazy/KeepAlive/Eager`），禁止在 `view.rs` 临时硬编码分支。
  - `logic.rs` 负责策略与生命周期意图（含 `NotifyHidden/NotifyVisible` 语义），`view.rs` 负责执行挂载/隐藏实现。
  - `KeepAlive` 场景必须提供“隐藏即降耗”通道，避免隐藏面板继续执行高成本动画/轮询/观测。
  - `Lazy` 场景需明确状态保留语义，避免无意重建导致内容状态丢失。
  - 规范详见：`docs/spec/slot_projection_strategy.md`。
- 四层能力基线硬规则（Core/Shell/Protocol/Infrastructure）：
  - Core 必须满足：纯状态机、可回放（time-travel 友好）、可序列化状态、确定性 ID 生成。
  - Shell 必须满足：细粒度切片、Foreign Zone 受控接入、Projection 生命周期管理。
  - Protocol 必须满足：Command Pattern 统一副作用桥接，Agent Contract 统一语义输出。
  - Infrastructure 必须满足：workspace 分层不破、release-plz fixed-mode 版本级联、token 防御链稳定。
  - 规范详见：`docs/spec/core_shell_protocol_infra_baseline.md`。
- 事件光锥硬规则（Event Light Cone）：
  - 大规模集合组件（Table/Grid/Tree）禁止依赖父链层层 props 传播全局状态变更；默认采用 Context Bus + Selector 订阅。
  - 子节点只订阅最小切片（selector），未变化不得触发更新；禁止把整块状态复制下发到 Row/Cell 级 props。
  - 批量状态操作（全选/全取消）应优先状态压缩表达（如 `SelectionState::All`），避免 N 次布尔写入。
  - 复杂度目标：常见批量操作更新路径 O(1)/O(log N)，拒绝 O(N) 广播式通信。
  - 规范详见：`docs/spec/event_light_cone_signal_bus.md`。
- 统一因果总线硬规则（Unified Causality Bus）：
  - 每个用户初始输入事件必须分配全局 `TraceId`，并在 Action/Command/SignalBus/Manager 链路中持续透传。
  - 任意副作用执行与总线广播日志都必须携带 `trace_id`，禁止匿名派生事件导致因果链断裂。
  - 调试入口必须支持按 `TraceId` 聚合链路，定位“触发源 -> 执行器 -> 订阅者 -> 受影响组件”完整路径。
  - 追踪实现不得阻塞主交互路径；开发全量、生产可采样，但采样后仍要保证链路可关联。
  - 规范详见：`docs/spec/unified_causality_bus.md`。
- 协议演化硬规则（Architectural Heat Death Guard）：
  - 协议版本演进必须走编译期治理：Schema Registry 记录版本窗口与迁移路径，禁止无限期运行时兼容分支堆叠。
  - 引入 `vN+1` 时必须同步提供纯函数迁移（`migrate_vN_to_vNplus1`）与回归测试，禁止“新版本先上，迁移后补”。
  - 工具链必须具备 codemod/lint 能力，自动扫描并改写旧协议调用点；超过支持窗口的版本在 CI 中直接失败。
  - 版本淘汰策略必须固定：`deprecated -> compile error -> remove`，确保系统可忘却历史负担。
  - 规范详见：`docs/spec/compile_time_evolution_migration.md`。
- 语义意图分层硬规则（Intent Stack）：
  - 通用组件只允许输出通用交互语义意图（如 `InteractionSubmitted`），禁止直接输出业务命令（如购物车/支付/实验策略命令）。
  - 业务组件层负责“通用意图 -> 业务意图”翻译；应用编排层负责结合策略配置发出最终行为命令。
  - A/B 或业务策略变更不应迫使修改底层组件 `logic.rs`；若出现该情况视为职责污染。
  - 意图链建议与 `TraceId` 联动，保证从组件意图到最终命令的可观测性。
  - 规范详见：`docs/spec/intent_stack_semantic_layering.md`。
- 架构健身函数硬规则（Architectural Fitness Functions）：
  - 架构原则必须有对应可执行检查（测试/脚本/lint），禁止只停留在文档约定。
  - 关键健身函数（分层依赖、framework-agnostic、目录契约、feature gate 边界）必须接入 CI，失败即阻断合并。
  - 新增架构规则时，必须同步新增对应健身函数（或明确 N/A 证明），否则视为未落地。
  - 健身函数失败信息必须可定位违规源与最小修复路径，避免“知道坏了但不知道哪里坏”。
  - 规范详见：`docs/spec/architectural_fitness_functions.md`。
- 平台退位硬规则（Platform Abdication）：
  - 平台核心交付优先是“法则 + 工具 + 脚手架 + 验证系统”，而非垄断所有领域组件实现。
  - 领域扩展（如 `ui-financial-primitives`）必须有标准接入路径，并沿用同一套分层约束与质量门禁。
  - 成功指标包含“外部团队可独立产出高质量组件”，而不仅是核心团队产出数量。
  - 治理文档（`philosophy`/`contributing`）与脚手架工具需长期维护，确保生态按同一物理法则演化。
  - 规范详见：`docs/spec/platform_abdication_ecosystem.md`。
- AI 上下文压缩硬规则（Manifest + RBI）：
  - 组件必须提供机器可读索引：`Component.toml`（Inputs/Outputs/Slots/Capabilities）与 `.rbi`（仅签名接口投影）。
  - Agent 默认只读索引层，禁止默认全量扫源码；只有在改内部实现时才升级读取 `logic/view/styles/motion`。
  - `Component.toml/.rbi` 必须可校验并与真实公开接口一致，禁止长期手写漂移。
  - 规范详见：`docs/spec/ai_context_projection_protocol.md`。
- 受控演化沙盒硬规则（Cleanroom vs Sandbox）：
  - 核心质量门禁不降级；不完美探索只能进入受控沙盒（建议 `crates/ui-contrib`），不得直接污染核心 `ui-components` 主路径。
  - 实验组件必须显式标记风险（A11y/SSR/性能/命令式依赖）与生命周期状态（`incubating/adopted/graduated/retired`）。
  - `ui-contrib` 组件默认不进入核心导出，必须显式 opt-in；禁止通过实验需求反向放宽核心规则。
  - 进入核心前必须完成 Graduation 审查与重构，补齐语义测试与门禁。
  - 规范详见：`docs/spec/controlled_evolution_sandbox.md`。
- 状态原语 Core/Satellite 硬规则：
  - `ui-state-primitives` 作为 Core，必须优先保持“通用交互原语 + 低依赖”纯度。
  - 领域型原语若需要重依赖（日期/i18n/虚拟化等），必须拆入 `ui-logic-*` 卫星包，不得直接把重依赖灌入 Core。
  - 模块归属使用 Litmus Test：是否“必须引入新的非序列化外部依赖”。
  - 拆分必须提供稳定迁移路径（门面/特性化/分阶段收敛），禁止一次性破坏调用面。
  - 规范详见：`docs/spec/state_primitives_core_satellite_split.md`。

CSS 注入规则：

- `ui-components/src/css.rs` 聚合所有组件 CSS。
- `<UiRoot>`（`crates/ui-components/src/root.rs`）统一注入：
  - `ui-theme` 生成的 CSS variables
  - 组件 CSS
  - 最小全局 base（body 背景/字体）
- **Cascade Layers（默认）**：
  - 组件 CSS 注入在 `@layer ui`（低优先级层）。
  - 应用侧覆盖推荐：不分 layer 直接写 overrides；如应用也使用 layers，则声明 `@layer ui, app;` 并把 overrides 放进 `@layer app`。
- **禁止 inline CSS（组件层）**：
  - `ui-components` 中禁止在 `view!` 里写 `style="..."` / `style=...`（字符串形式的 inline style）。
  - 组件所有样式规则（selector + 声明）必须位于该组件的 `styles.rs`，并通过 `ui-components/src/css.rs` 聚合后由 `<UiRoot>` 注入；组件内部不得写 `<style>` 标签。
  - 禁止使用 `style:<prop>=...` 绑定普通 CSS 属性（`padding/background/position/...` 等）；样式切换通过 `class`/`data-*` + `styles.rs` 完成。
  - 如必须传递运行时数值（例如 popover 位置 / motion 数值），只允许设置 **CSS variables（custom properties，`--*`）**：
    - 推荐：`style:--x=...`（如果语法可用）
    - 允许：`style=...` 但内容必须 **只包含** `--*` 变量赋值（禁止出现 `top/left/padding/background/...` 等普通属性）

- **Inline CSS forbidden (component layer):**
  - `ui-components` must not use `style="..."` / `style=...` inside `view!`
  - Do not bind normal CSS properties via `style:<prop>=...`
  - Only CSS variables (custom properties, `--*`) are allowed

样式孤岛防御规范（必读）：`docs/spec/style_island_defense.md`

## 4. 颜色与主题（OKLCH + OLED）

### 4.1 规范

- 颜色 token **必须使用 OKLCH**：`oklch(L% C h)`；透明度用 `oklch(... / a)`。
- 主题必须设置 `color-scheme`（由 `ui-theme` 输出），让浏览器表单控件/滚动条更一致。
- 组件禁止硬编码颜色（hex/rgb 等）；只能使用 `var(--ui-*)`。

### 4.2 当前 tokens（v0）

`ui-theme` 输出（示例）：

- `--ui-fg`, `--ui-fg-muted`
- `--ui-bg`, `--ui-bg-muted`
- `--ui-accent`, `--ui-accent-fg`, `--ui-accent-soft`
- `--ui-border`, `--ui-focus-ring`
- `--ui-radius-*`, `--ui-space-*`, `--ui-shadow-*`

主题入口：

- `Theme::light()`
- `Theme::dark()`
- `Theme::oled()`（真黑背景，暗色 scheme）

### 4.3 OLED 规则（方向）

- OLED 主题 `--ui-bg` 为真黑（`oklch(0% 0 0)`）。
- Surface（`bg-muted`）必须比背景更亮，避免“所有东西都融进黑里”。

## 5. Motion：接口与实现（Framer/HeroUI 方向）

### 5.1 分层规则

- **契约（contract）在组件层**：每个组件定义自己的 `XxxMotion`（例如 `ButtonMotion`）。
- **引擎（engine）在 `ui-motion`**：组件通过 `attach_motion` 把 DOM ref + 状态信号连接到引擎。
- `ui-headless` 不做动画编排（它只输出“状态变化/事件语义”）。

### 5.2 `ui-motion` 当前实现（v0）

- Web（wasm32）：
  - **WAAPI**：`ui_motion::web::animate(...)`（keyframes/options → `element.animate(...)`）。
  - **Spring runtime**：`ui_motion::spring::SpringAnimator`（rAF 驱动、stiffness/damping/mass/precision）。
  - `prefers-reduced-motion`：reduce 时应跳过或直接 set 到目标值。
- 非 wasm：
  - `ui_motion::web` 为 no-op；`prefers_reduced_motion()` 默认视为 true（避免误触发动画逻辑）。

### 5.3 组件侧用法模式（以 Button 为例）

- 组件定义 motion contract：`XxxMotion`（默认值合理、对外可覆盖）。
  - Button：`ButtonMotion { spring, hover_scale, tap_scale }`
  - Checkbox：`CheckboxMotion { spring, hover_scale, tap_scale, indicator_spring }`
  - Switch：`SwitchMotion { spring }`（thumb translate/width）
  - Overlay/Popover：`OverlayMotion` / `PopoverMotion`（opacity/scale/translate）
  - 列表类：`ActiveHighlightMotion`（active highlight 的 y/height/opacity）
- `attach_motion(...)` 的硬规则：
  - 只在 `cfg(wasm32)` 生效（SSR/非 wasm 为 no-op 或立即完成）。
  - per-frame 更新应尽量**只写 CSS variables（custom properties）**，避免触发组件重渲染。
  - 长生命周期、非 Send/Sync 的运行时对象必须用 `StoredValue::new_local(...)` 存放。
  - 需要“exit 动画后再卸载”时：组件提供 `on_exit_complete` 回调，上层用 presence（例如 `use_presence`）决定何时 unmount。

### 5.4 动效准则（方向）

- 交互反馈（press/drag/hover/selection highlight）优先 Spring（参数可主题化：未来 motion tokens）。
- Presence（enter/exit）与 layout motion（FLIP）后续补齐，但仍遵守：contract 在组件，引擎在 `ui-motion`。

## 6. 排版与布局

### 6.1 全局排版（当前）

`<UiRoot>` 提供最小全局样式（`system-ui` 字体栈 + 背景/前景来自 tokens）。

规则：

- 组件库不“接管页面布局”；布局应由应用（`apps/*`）决定。
- 组件内部排版使用 tokens（space/radius/shadow），避免散落的 magic numbers。

### 6.2 Safe Area（移动端/Tauri Android）

- `ui-theme` 提供 `SAFE_AREA_CSS`（使用 `env(safe-area-inset-*)`）。
- `<UiRoot safe_area=true>` 时应用 `.safe-area`，用于刘海屏/沉浸式场景。

## 7. 全局配置与 Provider（应用必须做）

应用入口（例如 `apps/web-demo/src/main.rs`）必须在 root 初始化：

- `provide_focus_visible()`：全局交互 modality 推断（键盘显示 focus ring，指针不显示）。
- `provide_overlay_stack()`：overlay 栈管理（只让 topmost 响应 Esc 等）。
- `provide_focus_manager()`：全局焦点栈与恢复策略管理（push/pop trap + invalidation/GC）。
- 使用 `<UiRoot theme=... safe_area=...>` 注入 tokens + CSS + base。

## 8. 工程化与协作约束（提交即门禁）

### 8.1 Git hooks（必须启用）

- 安装：`./scripts/setup-githooks.sh`
- `commit-msg`：Conventional Commits
- `pre-commit`：
  - **必须同时提交 `CHANGELOG.md`**（只允许改 `[Unreleased]`）
  - 限制单个 `.rs` 文件行数（必须 **小于 1000**，不可放宽）
  - 自动跑 `scripts/gate.sh`（进而跑 `scripts/check.sh`）

### 8.2 质量门禁（Stop Gates）

- `./scripts/check.sh`：fmt → clippy → test → ssr compile → wasm compile
- `./scripts/check-rust-hygiene.sh`（死命令，非测试代码）：
  - 禁止 `unwrap/expect`
  - 禁止 `let _ =`（副作用调用必须有明确可观测策略）
  - 禁止字符串克隆热点（优先 `Cow<'static, str>`，拒绝 `to_string/to_owned/String::from` 式扩散）
- 变量：
  - `SKIP_WASM=1` 可跳过 wasm gates（本机未装 wasm target 时）
  - `RUST_UI_ALLOW_CHANGELOG_RELEASE_EDIT=1` 仅在 cut release 时允许改已发布 changelog 段落

### 8.3 发布版本治理（Release Cascade）

- 发布自动化统一使用 `release-plz`，禁止手工批量改版本号。
- UI 包采用 **Fixed Mode**（同步版本）：`release-plz.toml` 中 `version_group = "ui-workspace"` 且 `release_always = true` 的包必须保持同版本。
- 版本级联由 release PR 自动完成：核心包版本变化时，workspace 内部依赖版本必须自动更新，不允许人工漏改。
- 发布入口固定为 `.github/workflows/release-plz.yml`；发布策略与约束详见 `docs/spec/release_versioning.md`。

---

## 附：相关规格与入口

- 计划：`docs/plan/TODO.md`（骨架→血肉）
- Motion 规格：`docs/spec/motion.md`
- 调研入口：`docs/research/README.md`
