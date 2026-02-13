# HeroUI 参数设计风格对齐策略（草案）

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
