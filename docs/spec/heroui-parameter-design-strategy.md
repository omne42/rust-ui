# HeroUI 参数设计风格对齐策略

## Spec Draft

### Goal

在 `ui-components` 中建立一套接近 HeroUI 的参数设计规范：覆盖高频业务需求、保持简单调用、尽量避免业务侧二次封装。

### Non-Goals

- 一次性把所有组件都重写为 HeroUI 完全同构 API。
- 为“可能永远不会用到”的低频场景预先加入复杂抽象。

### Constraints

- 技术约束：现有组件采用 `logic.rs / view.rs / motion.rs / styles.rs / mod.rs` 分层，需保持该结构稳定。
- 兼容性约束：已有参数和行为尽量不破坏，新增能力优先“向后兼容”。
- 可维护性约束：参数扩展不能显著增加 `borrow checker` 复杂度，避免在组件层引入过多 `Arc<dyn Fn...>` 风格的动态分发。
- 性能约束：默认配置不应增加明显运行时开销（尤其 overlay、list、autocomplete 等高交互组件）。

### Research Notes（HeroUI 风格提炼）

- 参数分层明显：视觉（`variant/size/color/radius`）、状态（`isDisabled/isLoading/isInvalid`）、行为（`onPress/onOpenChange`）、可访问性（`aria-*`）、结构插槽（`startContent/endContent`）、样式覆盖（`classNames` + slots）。
- 受控/非受控成对出现：典型是 `isOpen + onOpenChange` 与 `defaultOpen` 并存。
- 设计系统可覆盖性强：通过 slot + `classNames`，用户可精准覆写子结构，而不是只能改根节点 class。
- 数据态可观察：大量 `data-*` 状态标记，便于样式和测试。

### Definition of Done

- [ ] 输出统一参数分层规范（视觉/状态/行为/a11y/内容/样式/motion）并写入仓库文档。
- [ ] 至少 3 个代表组件（`Button`、`Select`、`Modal/Popover`）给出目标参数清单与迁移策略。
- [ ] 明确受控/非受控规则：每个可开关状态组件必须定义 `controlled + uncontrolled` 行为优先级。
- [ ] 明确插槽策略：含内容型组件必须支持 `start_content/end_content`（或等价 slot）与 `class_names`（slot 级覆盖）。
- [ ] 明确默认值规范：所有 `#[prop(optional)]` 的默认来源可追溯（`Default` / `#[prop(default=...)]` / 逻辑层兜底）。
- [ ] 给出验收命令并可执行：`cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo test -p ui-components`。

### Options (2-3)

#### Option A: 组件内“全量参数一次到位”

- **描述**：每个组件直接暴露尽可能完整参数，目标是“业务零二开”。
- **数据结构**：
  - `struct ButtonPropsV2 { visual, state, behavior, a11y, slots, style, motion }`
  - `struct SelectPropsV2 { ... }`
  - 各组件独立维护一套完整字段。
- **Pros**：
  - 单组件体验直观，短期“看起来最全”。
  - 文档展示上容易对齐 HeroUI 参数表。
- **Cons**：
  - 参数膨胀快，组件间重复字段多，维护成本高。
  - 一致性容易漂移（同名参数在不同组件语义不一致）。
  - 易引入“伪需求参数”，违背 YAGNI。
- **工作量**：高（全量组件逐个扩展 + 回归测试面大）。

#### Option B: 分层参数模型 + 组件特化（推荐）

- **描述**：先定义跨组件公共参数协议，再在每个组件追加特化参数，保持“统一心智 + 场景差异”。
- **数据结构**：
  - `struct CommonVisualProps { variant, size, color, radius }`
  - `struct CommonStateProps { is_disabled, is_loading, is_invalid, is_readonly }`
  - `struct CommonBehaviorProps { on_press, on_open_change, ... }`
  - `struct CommonA11yProps { aria_label, aria_labelledby, aria_describedby, ... }`
  - `struct CommonStyleProps { class_name, class_names: SlotClassNames }`
  - `enum Controlled<T> { Controlled { value: Signal<T>, on_change: Callback<T> }, Uncontrolled { default: T } }`
  - 组件输入：`struct ButtonStateInput { common: Common..., has_start_content, has_end_content, ... }`
- **Pros**：
  - 统一参数语义，学习成本低，文档可模板化。
  - 便于逐步迁移，风险可控，兼容现有分层架构。
  - 能覆盖“简单调用 + 高级覆写”两类用户。
- **Cons**：
  - 需要先建设公共类型和命名规范，前期设计成本较高。
  - 部分组件会出现“公共字段未使用”的裁剪问题。
- **工作量**：中高（先建规范和基类，再分批落组件）。

#### Option C: 最小核心 + 官方配方（Recipes）

- **描述**：核心组件保持精简，复杂需求由仓库内置 recipe/组合组件提供。
- **数据结构**：
  - `struct MinimalButtonProps { variant, size, disabled, on_press }`
  - `struct ButtonRecipeProps { start_content, end_content, icon_only, loading_pattern, ... }`
  - `mod recipes::{button_with_icon, async_submit_button, ...}`
- **Pros**：
  - 核心库简洁稳定，低维护。
  - 复杂能力以组合方式沉淀，避免主 API 过重。
- **Cons**：
  - 用户仍可能感知“需要二次封装”。
  - 配方与核心 API 的边界需要长期治理。
- **工作量**：中（核心改动小，配方建设较多）。

### Recommendation

选 **Option B（分层参数模型 + 组件特化）**，因为它最平衡：

- 能吸收 HeroUI 的高可配置优点（slots、`classNames`、受控/非受控、状态参数统一）。
- 不会把每个组件都推向“巨型 props”，长期可维护性更高。
- 与当前 Rust 代码结构天然兼容：
  - `mod.rs` 负责对外 API 暴露；
  - `logic.rs` 负责默认值与状态归一；
  - `view.rs` 只消费已归一参数；
  - `motion.rs`、`styles.rs` 独立演进。

建议按三阶段落地：

1. **Phase 1（规范）**：冻结命名与分层（尤其 `is_*`、`default_*`、`on_*_change`、slot 命名）。
2. **Phase 2（试点）**：先改 `Button`、`Select`、`Modal` 三个代表组件并产出迁移示例。
3. **Phase 3（推广）**：批量迁移其余组件，统一文档与 playground 用例矩阵。

### Accordion 对齐记录（2026-02）

- 参数命名保持统一契约：根级保留 `selection_mode`，item 级承载 `open` + `on_open_change` + `default_open`。
- 默认调用路径保持 0 接线：`<Accordion labels=... id_base=...>` 即可运行。
- 高级能力按需开启：`selection_mode`、`disabled_indices`、`motion`，不把内部状态对象暴露为必填参数。
- docs 对齐入口：`apps/docs-app` 的 `#/components/accordion` 页面包含 Hello World、受控、多开/单开状态矩阵示例。

### Button 同步记录（2026-02-16）

- 参数模型同步：`Button` 参数已对齐到统一分层命名（视觉 `variant/color/radius/size`、状态 `is_disabled/is_loading/is_icon_only/is_full_width`、行为 `on_press`、A11y `aria_label`）。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Button", "button", "Actions", actions::button)` 暴露入口；`#/components/button` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/actions.rs` 包含 `Hello world`、`Variants & sizes`（含交互 workbench）、`Colors`、`Radius`、`Sizes`，与当前实现参数保持一致。
- Copy-Paste Ready 同步：Button playground 代码快照由 `code_signal` 实时生成，并通过 `apps/docs-app/src/playground.rs` 的 `compose_copy_ready_code` 注入缺失 imports，避免示例漂移。

### ButtonCopy 同步记录（2026-02-17）

- 参数模型同步：`ButtonCopy` 维持 `Button` 特化定位，主能力复用 `Button`（`variant/size/is_icon_only/is_loading/on_press`），组件特化参数聚焦复制语义（`text`、`label`、`copied_label`、`aria_label`、`mode`、`is_disabled`、`motion`、`lang/dir`）。
- 模式语义同步：`mode` 统一为 `TextOnly` / `IconOnly` / `IconAndText` 三态，默认走 `IconAndText`，与文档矩阵示例一致。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ButtonCopy", "button-copy", "Actions", actions::button_copy)` 暴露入口；`#/components/button-copy` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/actions.rs` 包含 `Hello World`、`Label + variant`、`Disabled + empty matrix`、`Mode matrix`、`Workbench (Isolated Canvas + Optional Persist)`，与当前参数模型保持一致。
- Copy-Paste Ready 同步：`button-copy` 各 playground 代码通过 `code_signal` 进入 `Playground`，并由 `apps/docs-app/src/playground.rs` 的 `compose_copy_ready_code` 自动补齐 imports，避免示例漂移。

### FieldButton 同步记录（2026-02-17）

- 参数模型同步：`FieldButton` 保持 `Button` 语义薄封装定位，复用 `Button` 的交互/A11y/motion 能力；组件特化参数收敛为 `is_quiet/is_invalid/is_active/is_disabled` 与 `on_press`、`aria_label`、`class_name`。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("FieldButton", "field-button", "Actions", actions_extra::field_button)` 暴露入口；`#/components/field-button` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/actions_extra.rs` 提供 `Default + Quiet` 与 `Invalid + Active + Disabled` 两组示例，覆盖默认路径与状态矩阵，参数命名与实现保持一致。
- HeroUI 对齐结论：保持统一 `is_*` 状态前缀与 `on_press/aria_label` 行为/A11y 命名，避免平行别名与组件特化参数扩散。

### FlipButton 同步记录（2026-02-17）

- 参数模型同步：`FlipButton` 作为 `Button` 扩展能力，参数收敛为 `from`、`motion`、`class_name` 与显式 `front/back` 槽位；默认路径无需接线底层状态对象。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("FlipButton", "flip-button", "Actions", actions::flip_button)` 暴露入口；`#/components/flip-button` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/actions.rs` 提供 `Top flip`、`Direction matrix`、`Custom Class`、`Interactive Playground`，并显式标注 `Streaming Optional; fallback=snapshot.`。
- HeroUI 对齐结论：保持简洁参数面与显式槽位组合，避免并行数组式隐式约定；动效与交互语义优先复用 `Button/ui-motion/ui-headless` 既有契约。

### Open Questions (max 2)

1. 你希望“参数尽可能全”的优先级高于“API 精简稳定”吗？（这会决定是否把 Option A 的部分策略并入）
2. 你希望先覆盖哪些组件族作为第一批试点：`Button + Overlay` 还是 `Form + Selection`？

## 附：拟定参数命名规范（v0）

- 状态统一前缀：`is_disabled/is_loading/is_invalid/is_readonly`。
- 受控/非受控统一：`value + on_value_change` / `default_value`；开关类用 `open + on_open_change` / `default_open`。
- 内容插槽统一：`start_content/end_content`，必要时补 `top_content/bottom_content`。
- 样式覆盖统一：根节点 `class_name`，子槽位 `class_names`（`HashMap<Slot, String>` 或等价结构）。
- 可访问性优先：输入型组件必须显式支持 `aria_label` / `aria_labelledby` / `aria_describedby`。
- 默认值单一来源：优先 `Default` + 逻辑层归一，避免 view 层散落默认值。

## 参考链接

- HeroUI Button Docs: https://www.heroui.com/docs/components/button
- HeroUI Modal Docs: https://www.heroui.com/docs/components/modal
- HeroUI Select Docs: https://www.heroui.com/docs/components/select
- HeroUI Introduction: https://www.heroui.com/docs/guide/introduction

## 执行手册（必读）

- 开发流程与 AI 门禁：`docs/spec/hyper-structure-ui-development-playbook.md`
- 本文回答“做什么参数能力”，执行手册回答“怎么稳定地做出来并持续演进”。
