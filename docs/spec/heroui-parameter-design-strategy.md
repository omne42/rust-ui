# HeroUI 参数设计风格对齐策略

## Spec Draft

### Goal

在 `ui` 中建立一套接近 HeroUI 的参数设计规范：覆盖高频业务需求、保持简单调用、尽量避免业务侧二次封装。

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
- [ ] 给出验收命令并可执行：`cargo fmt --all -- --check`、`cargo clippy --workspace --all-targets -- -D warnings`、`cargo test -p ui`。

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

### Thumbnail 同步记录（2026-02-17）

- 参数模型同步：`Thumbnail` 维持 display primitive 定位，参数聚焦 `size/background/cover/layer/selected/focused/motion/class_name/lang/dir`，避免引入并行别名与额外状态对象接线。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Thumbnail", "thumbnail", "Display", display_extra_thumbnail::thumbnail)` 暴露入口；`#/components/thumbnail` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display_extra_thumbnail.rs` 提供 `Hello World`、`Sizes`、`Cover + Background + Layer + Selected`、`Custom Motion Contract`，并与当前参数语义保持一致。
- HeroUI 对齐结论：保持默认路径简洁可运行，同时保留显式参数扩展路径；禁止实现先漂移、文档后补。

### Spinner 同步记录（2026-02-17）

- 参数模型同步：`Spinner` 维持 display primitive 定位，公开参数为 `size/aria_label/class_name/motion/lang/dir`；本轮补齐 `motion + lang/dir` 语义接入，不引入受控/非受控状态轴，不新增平行别名。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Spinner", "spinner", "Display", display::spinner)` 暴露入口；`#/components/spinner` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display.rs` 提供 `Hello World`、`Size Matrix`、`Custom Label + Class`，并与当前参数和默认行为保持一致。
- HeroUI 对齐结论：默认路径保持零门槛 `<Spinner />`，高级参数按需开启；后续若发生参数语义变更，必须先同步本策略文档再推进实现。

### Skeleton 同步记录（2026-02-18）

- 参数模型同步：`Skeleton` 与 `SkeletonGroup` 维持 display primitive 定位；参数聚焦 `variant/is_shimmer/class_name`（Skeleton）与 `is_loading/is_skeleton_only/variant/layout/density/aria_label/class_name`（SkeletonGroup），不引入业务 store、并行数组或组件外状态对象必填接线。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Skeleton", "skeleton", "Display", display::skeleton)` 与 `component_doc!("SkeletonGroup", "skeleton-group", "Display", display_extra::skeleton_group)` 暴露入口；`#/components/skeleton`、`#/components/skeleton-group` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display.rs` 与 `apps/docs-app/src/pages/components/pages/display_extra.rs` 提供 `Shimmer/Still` 与 `Shimmer + Pulse Layout/Loaded + Skeleton Only`，覆盖默认路径与状态矩阵，并保持 API 名称与默认值一致。
- Source-first / Copy-Paste Ready：Skeleton playground 继续走 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，并为 skeleton/skeleton-group playground 标注 `test_source_path`（`crates/ui/src/skeleton/view.rs`、`crates/ui/src/skeleton/group/view.rs`）以保证源码可追溯。
- HeroUI 对齐结论：保持“默认调用零门槛、复杂状态按需显式开启”的体验目标；参数语义若变更，先同步本策略文档与 docs 入口再推进实现。

### Underlay 同步记录（2026-02-17）

- 参数模型同步：`Underlay` 维持 overlay primitive 定位，公开参数聚焦 `is_open/open/default_open/on_open_change`、`on_close`、`is_transparent/is_disabled`、`motion`、`class_name`、`lang/dir`；本次新增为 Agent/AI 可观测语义标记，不引入破坏性参数别名漂移。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Underlay", "underlay", "Overlays", overlays_extra::underlay)` 暴露入口；`#/components/underlay` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/overlays_extra.rs` 提供 `Scrim + Click To Close`、`Transparent + Disabled + Custom Class`、`State + Source Markers`、`LLM Render Modes (Snapshot + Streaming)`，并与当前实现语义保持一致。
- Copy-Paste Ready 同步：`Underlay` playground 代码通过 `code_signal` 进入 `Playground`，由 `apps/docs-app/src/playground.rs` 的 `compose_copy_ready_code` 自动补齐 imports，并在文档中显式提示 `component-underlay` 依赖和源码落点。

### Surface 同步记录（2026-02-17）

- 参数模型同步：`Surface` 维持基础容器定位，参数保持 `tone/elevation/is_bordered/is_padded/aria_label/class_name/lang/dir/motion`；本次无破坏性参数语义变更，继续保留 `is_*` 优先 + legacy 兜底策略。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Surface", "surface", "Layout", layout_extra::surface)` 暴露入口；`#/components/surface` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/layout_extra_surface.rs` 提供 `Tone + Elevation + Frame` 与 `Custom Aria + Class`，与当前实现参数和默认值保持一致。
- Copy-Paste Ready 同步：`Surface` 示例通过 `code_signal` 进入 `Playground`，由 `apps/docs-app/src/playground.rs` 的 `compose_copy_ready_code` 自动补齐 imports；组件 README 补充真实源码落点与 feature 前提，避免复制即报错。

### Tag 同步记录（2026-02-17）

- 参数模型同步：`Tag` 维持 token primitive 定位，参数聚焦 `variant/size/disabled/removable/on_remove/remove_aria_label/class_name/lang/dir`，避免引入并行别名与组件外状态对象必填接线。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Tag", "tag", "Collections", collections_groups::tag)` 暴露入口；`#/components/tag` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/collections_groups.rs` 提供 `Hello World`、`Variant + Size Matrix`、`Removable + Disabled + Custom Class`，并与当前参数语义保持一致。
- Copy-Paste Ready 同步：`Tag` playground 代码通过 `code_signal` 进入 `Playground`，由 `compose_copy_ready_code` 自动补齐 imports；`test_source_path` 指向 `components/tag/src/view.rs` 便于源码追溯。

### TagGroup 同步记录（2026-02-17）

- 参数模型同步：`TagGroup` 维持 collection primitive 定位，参数聚焦 `tags/disabled/on_remove/variant/size/id_base/label/description/error/invalid/required/aria_* /class_name/lang/dir`，继续保持 `on_remove` 驱动的显式组合语义。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("TagGroup", "tag-group", "Collections", collections::tag_group)` 暴露入口；`#/components/tag-group` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/collections.rs` 提供 `Hello World`、`Removable + State`、`Validation + Required`、`Disabled + Empty`，覆盖默认路径与状态矩阵。
- Copy-Paste Ready 同步：`TagGroup` playground 代码通过 `code_signal` 进入 `Playground`，由 `compose_copy_ready_code` 自动补齐 imports；`test_source_path` 指向 `components/tag/src/group/view.rs` 便于源码追溯。

### Swatch 同步记录（2026-02-17）

- 参数模型同步：`Swatch` 保持 display primitive 定位，参数聚焦 `color/label/size/border/rounding/shape/is_nothing/is_mixed_value/is_disabled/is_decorative/selected/default_selected/on_selected_change/lang/dir/class_name/motion`，维持统一 `is_* / on_* / default_*` 语义命名。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Swatch", "swatch", "Display", display_extra_swatch::swatch)` 暴露入口；`#/components/swatch` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display_extra_swatch.rs` 提供 `Hello World`、`Size + Shape + Rounding`、`Mixed + Nothing + Disabled + Controlled`、`Custom Motion Contract`，覆盖默认路径、状态矩阵与受控示例。
- Copy-Paste Ready 同步：Swatch playground 代码通过 `code_signal` 进入 `Playground`，由 `compose_copy_ready_code` 自动补齐 imports；`test_source_path` 指向 `components/swatch/src/view.rs` 便于源码追溯。
- HeroUI 对齐结论：继续遵循“先用起来，再进阶”，默认 API 路径前置，高级控制后置；参数语义变更需先同步本策略文档与 docs 入口。

### Toaster 同步记录（2026-02-18）

- 参数模型同步：`Toaster` 继续保持宿主定位，公开参数为 `position/portal/max_toasts/aria_label/class_name/lang/dir/motion/store`；本轮未发生破坏性参数语义变更，默认值仍锚定 `ToasterPosition::default()` 与 `DEFAULT_PORTAL/DEFAULT_MAX_TOASTS/DEFAULT_ARIA_LABEL`。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Toaster", "toaster", "Overlays", overlays_extra::toaster)` 暴露入口；`#/components/toaster` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/overlays_extra.rs` 保持 `Hello World / Portal Queue Host / Inline Top-Center Host / State + Source Markers`，并补充 `API Matrix + State Matrix`，与当前实现参数与状态轴保持一致。
- Copy-Paste Ready 同步：Toaster 文档新增 `Source-first / Copy-Paste Ready` 区块，提供 `Snippet(copyable=true)` 的最小可用片段、真实源码落点与 feature 前提；Playground 代码继续通过 `code_signal` + `compose_copy_ready_code` 自动补齐 imports，避免示例漂移。
- HeroUI 对齐结论：保持“默认路径简洁、进阶参数按需开启”的体验目标；若未来发生参数语义变更，必须先同步本策略文档再合入组件实现。

### Sonner 同步记录（2026-02-18）

- 参数模型同步：`Sonner` 继续保持宿主定位，公开参数为 `position/portal/max_toasts/aria_label/class_name/lang/dir/motion/store`；本轮未发生破坏性参数语义变更，默认值仍锚定 `SonnerPosition::default()` 与 `DEFAULT_PORTAL/DEFAULT_MAX_TOASTS/DEFAULT_ARIA_LABEL`。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Sonner", "sonner", "Overlays", overlays_extra::sonner)` 暴露入口；`#/components/sonner` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/overlays_extra.rs` 保持 `Hello World / Portal Queue + Variants / Inline Top-Center + Max Queue / State + Source Markers`，并补充 `API Matrix + State Matrix`，与当前实现参数和状态轴一致。
- Source-first / Copy-Paste Ready：Sonner Playground 代码继续通过 `code_signal` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；组件 README 同步真实源码落点与 feature 前提，避免复制即报错。
- HeroUI 对齐结论：保持“默认路径零门槛、进阶参数按需开启”的体验目标；参数语义若变更，必须先同步本策略文档与组件文档入口，再推进实现与合入。

### Tabs 同步记录（2026-02-17）

- 参数模型同步：`Tabs` 公开参数保持 `labels/id_base`、`keyboard_activation`、`default_selected_index/selected_index/on_selection_change`、`is_disabled/disabled/disabled_indices`、`motion`、`class_name/aria_label/lang/dir`，命名与受控/非受控契约保持一致。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `collections_core_catalog::TABS_DOC` 暴露入口；`#/components/tabs` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/collections.rs` 提供 `Hello World (Uncontrolled)`、`Automatic + Controlled`、`Manual + Disabled`、`Workbench (Isolated Canvas + Optional Persist)`，覆盖默认路径、受控/非受控与 disabled 状态矩阵。
- Copy-Paste Ready 同步：Tabs playground 示例通过 `code_signal` 进入 `Playground`，由 `apps/docs-app/src/playground.rs` 的 `compose_copy_ready_code` 自动补齐 imports，并可直接复制运行。
- HeroUI 对齐结论：遵循“先用起来，再进阶”，默认路径前置，高级控制后置；参数语义变更必须同步更新本对标文档与 docs 入口。

### TextField 同步记录（2026-02-18）

- 参数模型同步：`TextField` 保持表单输入基元定位，公开参数继续围绕 `value + on_value_change + default_value`、`is_disabled/is_read_only/is_required/is_invalid`、`description/error/placeholder/input_type/class_name/lang/dir/motion`；与统一 `is_* / on_* / default_*` 命名契约一致。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("TextField", "text-field", "Forms", forms_text_field::text_field)` 暴露入口；`#/components/text-field` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_text_field.rs` 新增 `API Matrix + State Matrix`，覆盖受控/非受控与状态/来源语义轴，并保持默认 Hello World 路径前置。
- Source-first / Copy-Paste Ready：文档页提供 `Snippet(copyable=true)` 最小片段、源码落点与 feature 前提；Playground 代码继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，避免示例漂移。
- HeroUI 对齐结论：默认路径保持最小接线，复杂能力通过显式受控参数和语义标记按需开启；参数语义若变更，先同步该策略文档再推进实现。

### Textarea 同步记录（2026-02-17）

- 参数模型同步：`Textarea` 保持表单多行输入基元定位，公开参数继续围绕 `value + on_value_change + default_value`、`is_required/is_invalid/is_disabled/is_read_only`、`description/error/placeholder/rows/class_name/lang/dir/motion`；命名与受控/非受控契约保持一致。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Textarea", "textarea", "Forms", forms_extra::textarea)` 暴露入口；`#/components/textarea` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs` 提供 `Basic Textarea` 与 `State + Source Markers`，覆盖默认路径、受控值变更与 invalid 状态切换语义。
- Copy-Paste Ready 同步：`Textarea` playground 示例通过 `code_signal` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，并通过 `CodeBlock` 一键复制，避免“复制即报错”。
- HeroUI 对齐结论：默认调用路径保持最小接线，进阶控制通过显式参数按需开启；参数语义若变更，必须先更新本策略文档与 docs 入口，再推进实现与清单勾选。

### TimeField 同步记录（2026-02-17）

- 参数模型同步：`TimeField` 维持表单时间输入基元定位，公开参数围绕 `value + on_value_change + default_value`、`is_disabled/disabled`、`minute_step`、`tone`、`aria/locale` 与 `motion/class_name`；命名继续遵循统一 `is_* / on_* / default_*` 契约与受控/非受控配对语义。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("TimeField", "time-field", "Forms", forms_extra::time_field)` 暴露入口；`#/components/time-field` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs` 提供 `Hello World`、`Interactive Playground (State + Source Markers)`、`Controlled + Step 15`、`Strong Tone + Custom Placeholder`、`Disabled + Uncontrolled (Default Step)`，覆盖默认路径与关键状态矩阵。
- Source-first / Copy-Paste Ready：`TimeField` docs 已补 `Source-first / Copy-Paste Ready` 区块，提供 `Snippet(copyable=true)` 最小片段、真实源码落点与 feature 前提；Playground 示例继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，避免示例漂移。
- HeroUI 对齐结论：保持“先用起来，再进阶”的路径顺序，默认调用最小接线，复杂控制显式开启；参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选。

### MotionRipple 同步记录（2026-02-17）

- 参数模型同步：`MotionRipple` 参数收敛为 `is_bounded/motion/class_name/lang/dir`；本轮命名统一完成 `bounded -> is_bounded`，避免同义别名漂移并对齐全库 `is_*` 契约。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("MotionRipple", "motion-ripple", "Display", display::motion_ripple)` 暴露入口；`#/components/motion-ripple` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display.rs::motion_ripple()` 保持 `Hello World` 默认路径前置，并提供 `Animation Matrix` + `Custom Boundary + Class` 进阶矩阵，参数与默认行为保持一致。
- Source-first / Copy-Paste Ready：`MotionRipple` 示例代码通过 `code_signal` 接入 `Playground`，由 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，保证复制片段可直接运行且与实现同步。
- HeroUI 对齐结论：默认路径保持零门槛，复杂能力通过显式参数按需开启；后续若参数语义变更，必须先同步本策略文档与 docs 入口再推进实现。

### Slider 同步记录（2026-02-18）

- 参数模型同步：`Slider` 参数围绕 `value + on_value_change + default_value`、`is_disabled`、`min/max/step`、`motion/class_name/lang/dir`；保持统一 `is_* / on_* / default_*` 契约，并兼容 legacy `disabled/set_value/on_change` 映射。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Slider", "slider", "Forms", forms_extra::slider)` 暴露入口；`#/components/slider` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs::slider()` 提供 `Hello World (Uncontrolled)`、`Controlled + Source Markers`、`Disabled + Fine Step`，覆盖默认路径、受控/非受控与 disabled 状态矩阵。
- Source-first / Copy-Paste Ready：Slider playground 代码走 `code_signal` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，`e2e/tests/docs_app_slider_contract.spec.mjs` 已回归 copyable 代码块契约。
- HeroUI 对齐结论：保持“先用起来，再进阶”的 API 路径，默认调用不要求用户手动接线底层状态对象；参数语义变更必须先同步本策略文档与 docs 页面。

### Resizable 同步记录（2026-02-18）

- 参数模型同步：`Resizable` 参数统一到 `value + on_value_change + default_value`（保留 `split_percent + on_split_percent_change + default_split_percent` 兼容别名）；布尔参数采用 `is_*` 前缀（`is_disabled/is_with_handle`），保持全库命名一致。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Resizable", "resizable", "Layout", layout_extra::resizable)` 暴露入口；`#/components/resizable` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/layout_extra.rs::resizable()` 继续覆盖 `Horizontal + Handle Grip` 与 `Controlled + Vertical Bounds`，并新增 `API Matrix + State Matrix` 与 `Source-first / Copy-Paste Ready` 区块。
- Source-first / Copy-Paste Ready：Resizable playground 代码继续走 `code_signal` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；新增 `e2e/tests/docs_app_resizable_contract.spec.mjs` 回归 copyable code block 和语义选择器稳定等待策略。
- HeroUI 对齐结论：保持“默认路径零门槛，复杂控制按需显式开启”；参数模型发生变更时先同步本策略文档与 docs 页面，再推进实现合入。

### ScrollArea 同步记录（2026-02-18）

- 参数模型同步：`ScrollArea` 继续保持 layout primitive 定位，参数主轴收敛为 `orientation/max_height_px/is_disabled/disabled`，其中 `is_disabled` 为主命名，`disabled` 仅作兼容输入。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 继续通过 `layout_extra::SCROLL_AREA_DOC` 暴露入口，并由 `apps/docs-app/src/pages/components/pages/layout_extra.rs::scroll_area()` 维护示例矩阵。
- 示例矩阵同步：`Hello World`、`Vertical + Max Height`、`Horizontal + Both + Disabled`、`Interactive Playground (State + Source Markers)` 与 `Source-first / Copy-Paste Ready` 保持同页覆盖。
- Source-first / Copy-Paste Ready：ScrollArea playground 示例继续通过 `code_signal` 驱动并进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，保证复制即运行。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，不允许实现先漂移文档后补。

### ScrollShadow 同步记录（2026-02-18）

- 参数模型同步：`ScrollShadow` 维持 layout primitive 定位，参数聚焦 `class_name` 与 `max_height_px`，不引入受控/非受控轴与并行别名。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ScrollShadow", "scroll-shadow", "Layout", layout::scroll_shadow)` 暴露入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/layout.rs::scroll_shadow()` 保持 `Hello World`、`Default Scrollable`、`Custom Height + Class`。
- Source-first / Copy-Paste Ready：ScrollShadow playground 示例继续通过 `code_signal` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code`。
- HeroUI 对齐结论：保持“先用起来，再进阶”的体验路径。

### SearchField 同步记录（2026-02-18）

- 参数模型同步：`SearchField` 参数主轴保持 `value + on_value_change + default_value`、`is_disabled`、`clearable`、`placeholder`、`motion/class_name/lang/dir`，并兼容 legacy `disabled/set_value/on_change` 归一映射。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("SearchField", "search-field", "Forms", forms::search_field)` 暴露入口；`#/components/search-field` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms.rs::search_field()` 继续覆盖 `Hello World`、`Controlled + Clear`、`Disabled + Placeholder` 与 `Source-first / Copy-Paste Ready`。
- Source-first / Copy-Paste Ready：SearchField playground 代码通过 `code_signal` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，保证示例复制后可直接运行。
- HeroUI 对齐结论：默认路径保持零门槛，清空交互与可访问性参数按需显式开启；参数语义变更必须先同步本策略文档与 docs 页面。

### Select 同步记录（2026-02-18）

- 参数模型同步：`Select` 参数继续以 `is_disabled/open/on_open_change/default_open`、`selected_key/on_selection_change/default_selected_key`、`placeholder/label/description/error_message` 为主轴，保持 `is_* / on_* / default_*` 命名一致。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Select", "select", "Collections", collections::select)` 暴露入口；`#/components/select` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/collections.rs::select()` 覆盖 `Hello World`、受控开关、禁用态与状态矩阵，保持参数语义与实现一致。
- Source-first / Copy-Paste Ready：Select playground 示例继续通过 `code_signal` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，保证复制后可直接运行。
- HeroUI 对齐结论：默认路径先可用，复杂状态按需显式启用；参数语义变化必须先同步本策略文档与 docs 页面，再推进实现。

### Sheet 同步记录（2026-02-18）

- 参数模型同步：`Sheet` 继续保持 overlay primitive 定位，参数主轴收敛为 `open/on_close/placement/is_dismissable/is_keyboard_dismiss_disabled/motion/aria_labelledby/aria_describedby/on_exit_complete`。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Sheet", "sheet", "Overlays", overlays::sheet)` 暴露入口；`#/components/sheet` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/overlays.rs::sheet()` 继续覆盖基础开关、位置切换、可关闭策略与状态来源矩阵。
- Source-first / Copy-Paste Ready：Sheet 示例仍通过 `code_signal` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 保持复制即运行。
- HeroUI 对齐结论：保持“默认路径零门槛、进阶参数按需开启”的体验目标。

### Autocomplete 同步记录（2026-02-18）

- 参数模型同步：`Autocomplete` 保持 `is_open/open + on_open_change + default_open`、`is_disabled/disabled`、`is_required/required`、`is_invalid/invalid` 轴，并维持 `is_*` 主命名 + legacy 兼容映射。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 继续通过 `component_doc!("Autocomplete", "autocomplete", "Collections", collections::autocomplete)` 暴露入口；`#/components/autocomplete` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/collections.rs::autocomplete()` 补齐 `Hello World`、`Selection + Validation`、`Controlled Open State`、`Disabled + Empty`，覆盖默认路径、受控/非受控与状态矩阵。
- Source-first / Copy-Paste Ready：Autocomplete playground 示例继续通过 `code_signal` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，避免示例漂移。
- HeroUI 对齐结论：默认路径保持零门槛可运行，复杂行为按需显式开启；参数语义变更必须先同步本策略文档与 docs 页面。

### Calendar 同步记录（2026-02-19）

- 参数模型同步：`Calendar` 继续保持月视图基元定位，参数主轴为 `year/month/selected_day/show_outside_days/first_weekday/tone/on_day_press`，并新增 `motion/lang/dir` 接入；保持默认路径可直接运行。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Calendar", "calendar", "Forms", forms_extra::calendar)` 暴露入口；`#/components/calendar` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs::calendar()` 现覆盖 `Default + Outside Days`、`Monday First + Strong Tone`、`Interactive Playground (State + Source Markers)`，并补充 Source-first / Copy-Paste Ready 区块。
- Source-first / Copy-Paste Ready：Calendar playground 代码继续通过 `code_signal` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；源码落点与 feature 前提在文档中显式列出。
- HeroUI 对齐结论：默认调用保持零门槛；高级状态切换与来源标记通过交互 playground 显式验证。参数语义若变更，必须先同步本策略文档与 docs 页面。

### Header 同步记录（2026-02-19）

- 参数模型同步：`Header` 继续保持语义容器头部原语定位，公开参数聚焦 `tone/bordered/aria_label/class_name/motion/lang/dir`，并保持默认路径 `<Header>...</Header>` 可直接运行。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Header", "header", "Layout", layout::header)` 暴露入口；`#/components/header` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/layout.rs::header()` 现覆盖 `Semantic Header + Tone`、`Bordered + Custom Aria/Class`、`Interactive Playground (State + Source Markers)`，并补充 Source-first / Copy-Paste Ready 区块。
- Source-first / Copy-Paste Ready：Header playground 代码继续通过 `code_signal` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档同步给出源码落点与 feature 前提。
- HeroUI 对齐结论：默认路径保持零门槛，状态语义由 `data-* + data-ui-*` 稳定暴露；参数语义若变更，必须先同步本策略文档与 docs 页面。

### Dropdown 同步记录（2026-02-18）

- 参数模型同步：`Dropdown` 参数收敛为 `is_open/open + on_open_change + default_open`、`is_disabled/disabled`、`close_on_action`、`disabled_indices`、`item_kinds`、`motion`、`aria_label/class_name`，保持 `is_*` 主命名与 legacy 兼容。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Dropdown", "dropdown", "Collections", collections_extra::dropdown)` 暴露入口；`#/components/dropdown` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/collections_extra.rs::dropdown()` 补齐 `Hello World`、`Default`、`Controlled + Persistent + Motion`，覆盖默认路径、受控/非受控与状态矩阵。
- Source-first / Copy-Paste Ready：Dropdown playground 示例继续通过 `code_signal` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；新增 `e2e/tests/docs_app_dropdown_contract.spec.mjs` 回归 copyable 代码块与语义选择器稳定等待策略。
- HeroUI 对齐结论：默认路径保持零门槛可运行，复杂能力按需显式开启；参数语义变更必须先同步本策略文档与 docs 页面。

### Badge 同步记录（2026-02-20）

- 参数模型同步：`Badge` 参数继续收敛为 `variant/class_name/lang/dir`，保持默认路径 `<Badge>"New"</Badge>` 与进阶路径显式参数开启；本轮未引入参数语义破坏性变更。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Badge", "badge", "Display", display::badge)` 暴露入口；`#/components/badge` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display.rs::badge()` 覆盖 `Hello World`、`Variant Matrix`、`Custom Class + Outline` 与 `Badge Workbench (Display + Config + Code + CSS Test)`，保持参数语义与实现一致。
- Source-first / Copy-Paste Ready：Badge playground 代码继续通过 `code_signal` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，`components/badge/src/README.md` 同步标注源码落点与依赖前提。
- HeroUI 对齐结论：保持“先用起来，再进阶”的路径顺序；参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选。

### Alert 同步记录（2026-02-20）

- 参数模型同步：`Alert` 参数主轴已统一为 `tone/fill/layout`，并保留 `variant` 兼容映射（`variant -> tone`）；其余参数聚焦 `is_hide_icon/hide_icon`、`title/description`、`icon_label`、`start_content/end_content`、`motion`、`class_name`、`lang/dir`，默认路径保持 `<Alert>...</Alert>` 可直接运行。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Alert", "alert", "Display", display::alert)` 暴露入口；`#/components/alert` 可索引访问，且 `components/alert/src/README.md` 提供等价入口说明。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display.rs::alert()` 已覆盖 `Hello World`、`Interactive Playground (展示 / Config / Code / CSS Test)`、`State Matrix`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Inline Layout`，与当前参数语义和默认值保持一致。
- Source-first / Copy-Paste Ready：`Alert` docs 的 `Source-first / Copy-Paste Ready` 区块继续通过 `Snippet(copyable=true)` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 输出可运行片段，并同步真实源码落点与依赖前提（`component-alert`、`UiRoot`、`inject-css`），避免复制即报错。
- HeroUI 对齐结论：保持“默认路径零门槛、进阶参数按需开启”的体验目标；参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选。

### Avatar 同步记录（2026-02-20）

- 参数模型同步：`Avatar` 参数主轴保持 `name/src/alt/size/class_name/lang/dir`，默认路径仍为零门槛 `<Avatar />`；本轮未引入破坏性参数语义变更，继续维持 `label source = alt -> name -> fallback` 与 `render state = image | fallback` 契约。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Avatar", "avatar", "Display", display::avatar)` 暴露入口；`#/components/avatar` 可索引访问，且 `components/avatar/src/README.md` 提供等价入门入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display.rs::avatar()` 覆盖 `Hello World`、`Image + Fallback`、`State Matrix`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Interactive Playground (Props + State Preview)` 与 `Source-first / Copy-Paste Ready` 区块，参数语义与当前实现保持一致。
- Source-first / Copy-Paste Ready：Avatar 文档通过 `Snippet(copyable=true)` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 提供可运行片段，并显式标注依赖前提（`component-avatar`、`UiRoot`、`inject-css`）与真实源码落点（`components/avatar/src/{mod,logic,view,styles}.rs`），避免复制即报错。
- 研究文档补充判定：本轮仅为 Avatar 参数模型与文档验收面同步，未引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：保持“先用起来，再进阶”的路径顺序；参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### AvatarGroup 同步记录（2026-02-20）

- 参数模型同步：`AvatarGroup` 参数主轴保持 `items/max/size/aria_label/class_name/lang/dir`，默认路径仍为零门槛 `<AvatarGroup items=Vec::<AvatarGroupItem>::new() />`；本轮未引入破坏性参数语义变更，继续维持 `data-state = empty | stable | overflow` 与来源标记 `data-aria-label-source/data-class-source` 契约。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("AvatarGroup", "avatar-group", "Display", display::avatar_group)` 暴露入口；`#/components/avatar-group` 可索引访问，且 `components/avatar-group/src/README.md` 提供等价入门入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display.rs::avatar_group()` 覆盖 `Hello World`、`Overflow Stack`、`Sizes Without Overflow`、`State Matrix`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional (fallback=snapshot)`、`Interactive Playground (Props + State + Preview)` 与 `Source-first Starter (Copy-Paste Ready)`，参数语义与当前实现保持一致。
- Source-first / Copy-Paste Ready：AvatarGroup 文档通过 `Snippet(copyable=true)` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 提供可运行片段，并显式标注依赖前提（`component-avatar-group`、`UiRoot`、`inject-css`）与真实源码落点（`components/avatar-group/src/{mod,logic,view,styles}.rs`），避免复制即报错。
- 研究文档补充判定：本轮仅为 AvatarGroup 参数模型与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ActionBar 同步记录（2026-02-20）

- 参数模型同步：`ActionBar` 参数主轴保持 `selected_count + on_selected_count_change + default_selected_count`（受控/非受控）、`on_clear_selection`、`position`、`is_force_visible`、`selection_text`、`clear_label`、`motion`、`aria_label`、`class_name`、`lang/dir`；默认路径仍为 `<ActionBar default_selected_count=1>...</ActionBar>`。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ActionBar", "action-bar", "Actions", ax::action_bar)` 暴露入口；`#/components/action-bar` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/actions_extra.rs::action_bar()` 已覆盖 `Hello World`、`Controlled vs Uncontrolled`、`State Matrix`、`Interactive Playground (Props + State + Spec Preview)` 与 `Source-first Copy-Paste` 区块，参数语义与默认值与实现保持一致。
- Source-first / Copy-Paste Ready：ActionBar playground 继续通过 `code_signal + code_imports` 接入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档已显式标注真实源码落点与 feature 前提（`component-action_bar`、`inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅是 ActionBar 参数/文档对齐与验收面补全，未引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ColorHandle 同步记录（2026-02-20）

- 参数模型同步：`ColorHandle` 参数主轴保持 `id_base/color` + 状态参数 `is_disabled/is_focused/is_dragging/is_loupe_visible` + 几何参数 `x_percent/y_percent` + 扩展参数 `aria_label/lang/dir/class_name/motion`；默认值继续锚定 `is_loupe_visible=true`、`x_percent=50.0`、`y_percent=50.0`、`motion=ColorHandleMotion::default()`，未引入破坏性命名漂移。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ColorHandle", "color-handle", "Forms", forms_color::color_handle)` 暴露入口；`#/components/color-handle` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_handle()` 已覆盖 `Hello World`、`State Matrix`、`Controlled vs Uncontrolled (N/A)`、`Disabled + Custom Class + Loupe Off`、`Parameter Matrix Workbench`、`Source-first / Copy-Paste Ready`，与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：ColorHandle playground 代码通过 `code_signal` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，并在文档中标注真实源码落点 `components/color-handle/src/{view,logic,styles}.rs`，避免复制即报错。
- 研究文档补充判定：本轮仅为 ColorHandle 参数模型与文档入口对齐，未引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ColorArea 同步记录（2026-02-20）

- 参数模型同步：`ColorArea` 参数主轴保持 `value + on_value_change + default_value`（受控/非受控）、`is_disabled`、`step`、`grid_size`、`preview_color`、`label/aria_label/x_axis_label/y_axis_label`、`class_name`、`motion`、`lang/dir`；默认路径继续为 `<ColorArea id_base=\"...\" />`，未引入破坏性命名漂移。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ColorArea", "color-area", "Forms", forms_color::color_area)` 暴露入口；`#/components/color-area` 可索引访问，且 `components/color-area/src/README.md` 提供等价入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_area()` 已覆盖 `Hello World`、`Controlled Grid Selection`、`Controlled vs Uncontrolled`、`State Matrix`、`Interactive Playground`、`Source-first / Copy-Paste Ready`，并保持 API 命名与默认值语义一致。
- Source-first / Copy-Paste Ready：ColorArea playground 继续通过 `code_signal` + `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档显式给出源码落点与依赖前提，避免复制即报错。
- 研究文档补充判定：本轮仅为 ColorArea 参数模型与文档入口同步，未引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ColorLoupe 同步记录（2026-02-20）

- 参数模型同步：`ColorLoupe` 参数主轴保持 `id_base/color/is_open/is_disabled/x_percent/y_percent/aria_label/class_name/lang/dir/output_state`；继续维持统一 `is_*` 命名并拒绝回退到 `open/disabled` 别名，默认路径保持 `<ColorLoupe id_base=\"...\" />` 可直接运行。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ColorLoupe", "color-loupe", "Forms", forms_color::color_loupe)` 暴露入口；`#/components/color-loupe` 可索引访问，且 `components/color-loupe/src/README.md` 提供等价文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_loupe()` 已覆盖 `Hello World`、`Open + Position Buckets`、`Disabled + Custom Label + Custom Class`、`Controlled vs Uncontrolled（N/A）`、`State Matrix`、`Streaming Optional / Snapshot`、`Interactive Playground`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：ColorLoupe playground 继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档已显式标注依赖前提（`component-color_loupe + inject-css`、`UiRoot`）与源码落点（`components/color-loupe/src/{view,logic,styles}.rs`），避免复制即报错。
- 研究文档补充判定：本轮仅为 ColorLoupe 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ColorPicker 同步记录（2026-02-20）

- 参数模型同步：`ColorPicker` 参数主轴保持 `value + on_value_change + default_value`、`selected_color + on_selected_change + default_selected_color`、`open + on_open_change + default_open`、`is_disabled (disabled legacy alias)`、`label/aria_label/class_name/lang/dir/motion`；默认值继续由 `logic.rs` 统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ColorPicker", "color-picker", "Forms", forms_color::color_picker)` 暴露入口；`#/components/color-picker` 可索引访问，且 `apps/docs-app/src/pages/components/pages/forms_color.rs::color_picker()` 与 `components/color-picker/src/README.md` 提供等价文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_picker()` 已覆盖 `Hello World（默认路径）`、`State Matrix`、`Controlled vs Uncontrolled`、`Interactive Workbench (DX)` 与 `Source-first / Copy-Paste Ready`，并与当前参数语义和默认值保持一致。
- Source-first / Copy-Paste Ready：ColorPicker playground 继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档显式标注真实源码落点（`components/color-picker/src/{mod,view,logic,styles,motion}.rs`）与 feature 前提（`component-color_picker`、按需 `inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 ColorPicker 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ColorEditor 同步记录（2026-02-20）

- 参数模型同步：`ColorEditor` 参数主轴保持 `selected_color + on_selected_change + default_selected_color`、`format + on_format_change + default_format`、`is_disabled / is_alpha_channel_hidden`、`label/aria_label/class_name/lang/dir/motion`；默认值继续在 `logic.rs::normalize_default_inputs` 统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ColorEditor", "color-editor", "Forms", forms_color::color_editor)` 暴露入口；`#/components/color-editor` 可索引访问，且 `apps/docs-app/src/pages/components/pages/forms_color.rs::color_editor()` 为对应文档页入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_editor()` 已覆盖 `Hello World（默认路径）`、`State Matrix`、`Controlled vs Uncontrolled`、`Interactive Playground`，并与当前参数语义和默认值保持一致。
- Source-first / Copy-Paste Ready：ColorEditor playground 继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档页提供 `Source-first / Copy-Paste Ready` 区块并标注真实源码落点与 feature 前提，避免复制即报错。
- 研究文档补充判定：本轮仅为 ColorEditor 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ColorSlider 同步记录（2026-02-20）

- 参数模型同步：`ColorSlider` 参数主轴保持 `value + on_value_change + default_value`（受控/非受控）与 `channel`；范围与精度参数继续为 `min/max/step`，无障碍与视觉扩展参数保持 `label/aria_label/is_disabled(disabled legacy alias)/track_start_color/track_end_color/motion/class_name/lang/dir`；默认值由 `logic.rs` 统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ColorSlider", "color-slider", "Forms", forms_color::color_slider)` 暴露入口；`#/components/color-slider` 可索引访问，且 `apps/docs-app/src/pages/components/pages/forms_color.rs::color_slider()` 与 `components/color-slider/src/README.md` 提供等价文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_slider()` 已覆盖 `Hello World`、`State Matrix`、`Controlled vs Uncontrolled`、`Streaming Optional / Snapshot`、`Interactive Workbench (DX)` 与 `Source-first / Copy-Paste Ready`，并与当前参数语义和默认值保持一致。
- Source-first / Copy-Paste Ready：ColorSlider playground 继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档显式标注真实源码落点（`components/color-slider/src/{mod,logic,view,styles,motion}.rs`）与 feature 前提（`component-color_slider`、按需 `inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 ColorSlider 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ColorWheel 同步记录（2026-02-20）

- 参数模型同步：`ColorWheel` 参数主轴保持 `value + on_value_change + default_value`（受控/非受控）、`step`、`is_disabled`、`is_value_label_visible`、`label/aria_label`、`class_name`、`motion`、`lang/dir`；默认路径继续为 `<ColorWheel id_base=\"...\".to_string() />`，默认值归一保持在 `logic.rs`。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ColorWheel", "color-wheel", "Forms", forms_color::color_wheel)` 暴露入口；`#/components/color-wheel` 可索引访问，且 `components/color-wheel/src/README.md` 提供等价文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_wheel()` 已覆盖 `Hello World`、`State Matrix`、`Parameter Matrix`、`Controlled vs Uncontrolled`、`Streaming Optional / Snapshot`、`Interactive Workbench (DX)` 与 `Source-first / Copy-Paste Ready`，并与当前参数语义和默认值保持一致。
- Source-first / Copy-Paste Ready：ColorWheel playground 继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档显式标注真实源码落点 `components/color-wheel/src/{mod,logic,view,styles,motion}.rs` 与 feature 前提（`component-color_wheel`、按需 `inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 ColorWheel 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ColorThumb 同步记录（2026-02-20）

- 参数模型同步：`ColorThumb` 参数主轴保持 `id_base/color/is_disabled/is_focused/is_dragging/x_percent/y_percent/is_loupe_visible/motion/aria_label/aria_value_text/class_name/lang/dir`；默认值与来源标记继续由 `components/color-thumb/src/logic.rs::resolve_component_state` 与 `normalize_position_percent` 统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ColorThumb", "color-thumb", "Forms", forms_color::color_thumb)` 暴露入口；`#/components/color-thumb` 可索引访问，且 `apps/docs-app/src/pages/components/pages/forms_color.rs::color_thumb()` 维持 `title="ColorThumb"` 与 `slug="color-thumb"`。
- 组件文档同步：`components/color-thumb/src/README.md` 提供等价文档入口，保留 `Hello World -> 常见用法 -> 进阶参数` 的新手优先路径。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_color.rs::color_thumb()` 已覆盖 `Hello World`、`State Matrix`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Interactive Workbench (DX)` 与 `Source-first Starter (Copy-Paste Ready)`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：ColorThumb playground 继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档显式标注真实源码落点 `components/color-thumb/src/{mod,logic,view,styles,motion}.rs` 与 feature 前提（`component-color_thumb`、按需 `inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 ColorThumb 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ColorSwatch 同步记录（2026-02-20）

- 参数模型同步：`ColorSwatch` 维持 display color preview primitive 定位，参数主轴保持 `color/color_name/size/rounding/shape/is_bordered/is_decorative/aria_label/class_name/lang/dir/motion`，继续遵循统一 `is_*` 状态命名；本轮无破坏性别名漂移。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ColorSwatch", "color-swatch", "Display", display_extra::color_swatch)` 暴露入口；`#/components/color-swatch` 可索引访问，且 `components/color-swatch/src/README.md` 提供等价文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display_extra.rs::color_swatch()` 已覆盖 `Hello World`、`Interactive Playground`、`Comparison Matrix`、`Controlled vs Uncontrolled Contrast (N/A)`、`Streaming / Snapshot Contract` 与 `Source-first Starter`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：ColorSwatch playground 继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档显式标注真实源码落点 `components/color-swatch/src/{mod,logic,view,styles,motion}.rs` 与 feature 前提（`component-color_swatch`、按需 `inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 ColorSwatch 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ColorSwatchPicker 同步记录（2026-02-20）

- 参数模型同步：`ColorSwatchPicker` 维持单选色板 primitive 定位，参数主轴保持 `swatches`、`selected_color + on_selected_change + default_selected_color`、`is_disabled`、`is_bordered`、`shape`、`rounding`、`aria_label/class_name/lang/dir`、`motion`，命名继续遵循统一 `is_*` / `on_*` / `default_*` 契约，不引入平行别名。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ColorSwatchPicker", "color-swatch-picker", "Display", display_extra::color_swatch_picker)` 暴露入口；`#/components/color-swatch-picker` 可索引访问，且 `components/color-swatch-picker/src/README.md` 提供等价文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display_extra.rs::color_swatch_picker()` 已覆盖 `Hello World`、`Basic Selection`、`Transparency + Disabled + Custom Class`、`State Matrix`、`Controlled vs Uncontrolled Contrast`、`Streaming / Snapshot Contract`、`Interactive Playground` 与 `Source-first Starter`，并与当前参数语义和默认值保持一致。
- Source-first / Copy-Paste Ready：ColorSwatchPicker playground 继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档显式标注真实源码落点 `components/color-swatch-picker/src/{mod,logic,view,styles,motion}.rs` 与 feature 前提（`component-color_swatch_picker`、按需 `inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 ColorSwatchPicker 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ContextualHelp 同步记录（2026-02-20）

- 参数模型同步：`ContextualHelp` 参数主轴保持 `variant/placement/open + on_open_change + default_open/is_disabled/aria_label/class_name/lang/dir/id/motion`；布尔禁用语义以 `is_disabled` 为主命名，`disabled` 仅作兼容别名，归一优先级保持 `is_disabled > disabled > false`。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ContextualHelp", "contextual-help", "Overlays", overlays::contextual_help)` 暴露入口；`#/components/contextual-help` 可索引访问，且 `components/contextual-help/src/README.md` 提供等价文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/overlays.rs::contextual_help()` 已覆盖 `Hello World (Default API)`、`Info Variant + Controlled`、`Workbench (Display + Config + Code + CSS Test)`、`API Matrix`、`State Matrix` 与 `Streaming/Snapshot Display`，参数语义与默认值与实现保持一致。
- Source-first / Copy-Paste Ready：文档页提供 `Source-first / Copy-Paste Ready` 区块与 `Snippet(copyable=true)`；Playground 代码继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，且显式给出源码落点与 feature 前提（`component-contextual_help`、`inject-css`），避免复制即报错。
- 研究文档补充判定：本轮为参数语义命名与文档验收面同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### FieldLabel 同步记录（2026-02-21）

- 参数模型同步：`FieldLabel` 维持 form primitive 定位，参数主轴保持 `text/for_id/is_required/is_disabled/tone/required_indicator/aria_label/class_name/lang/dir`；命名继续遵循统一 `is_*` 状态前缀，不引入平行别名。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("FieldLabel", "field-label", "Forms", fxl::field_label)` 暴露入口；`#/components/field-label` 可索引访问，且 `components/field-label/src/README.md` 提供等价入门文档。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_extra_field_label.rs` 已覆盖 `Hello World (Default API)`、`Tone + Required`、`Custom Indicator + Aria + Class`、`Controlled vs Uncontrolled (N/A)`、`Streaming / Snapshot Contract`、`Workbench (Display + Config + Code + CSS Test)`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：FieldLabel playground 代码继续通过 `code_imports=FIELD_LABEL_DOC_IMPORTS` 与 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 输出可运行片段，并在 Workbench 标注 `test_source_path` 指向真实源码落点，避免复制即报错。
- 研究文档补充判定：本轮仅为 FieldLabel 参数模型与组件文档同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Label 同步记录（2026-02-21）

- 参数模型同步：`Label` 维持 form primitive 定位，参数主轴保持 `text/for_id/is_required/is_disabled/emphasis/required_indicator/class_name/lang/dir/motion`；命名继续遵循统一 `is_*` 状态前缀，不引入平行别名。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Label", "label", "Forms", forms_extra::label)` 暴露入口；`#/components/label` 可索引访问，且 `components/label/src/README.md` 提供等价入门文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs::label()` 已覆盖 `Hello World`、`Interactive Playground`、`Emphasis + Required`、`Custom Indicator + Class`、`Controlled vs Uncontrolled (N/A for Label)`、`Streaming Optional (fallback=snapshot)`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：Label playground 代码继续通过 `code_imports=label_imports.clone()` 与 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 输出可运行片段；文档显式标注真实源码落点 `components/label/src/{mod,logic,view,styles,motion}.rs` 与 feature 前提（`component-label`、`inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 Label 参数模型与组件文档同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### FieldError 同步记录（2026-02-20）

- 参数模型同步：`FieldError` 维持 form feedback primitive 定位，参数主轴保持 `tone/is_visible/is_disabled/is_icon_visible/message/aria_label/class_name/lang/dir`；兼容别名 `visible/disabled/show_icon` 仅作 legacy 输入，归一优先级保持 `is_visible > visible`、`is_disabled > disabled`、`is_icon_visible > show_icon`。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("FieldError", "field-error", "Forms", forms_extra::field_error)` 暴露入口；`#/components/field-error` 可索引访问，且 `components/field-error/src/README.md` 提供等价入门文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs::field_error()` 已覆盖 `Hello World (Snapshot Baseline)`、`State Matrix (Visible / Hidden / Disabled)`、`Controlled vs Uncontrolled (Stateless Contract)`、`Interactive Playground (Props + State + Source Markers)`、`Source-first / Copy-Paste Ready`，并与当前参数语义保持一致。
- 研究文档补充判定：本轮为参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ErrorMessage 同步记录（2026-02-21）

- 参数模型同步：`ErrorMessage` 维持 form feedback primitive 定位，参数主轴保持 `text/tone/is_disabled/is_truncated/element/aria_label/class_name/lang/dir/motion`；状态命名以 `is_*` 为主，`disabled/truncate` 仅作兼容别名并保持 `is_disabled/is_truncated` 优先归一语义。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ErrorMessage", "error-message", "Forms", forms_extra::error_message)` 暴露入口；`#/components/error-message` 可索引访问，且 `components/error-message/src/README.md` 提供等价入门文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs::error_message()` 已覆盖 `Hello World (Default API)`、`Tone Variants`、`Truncate + Disabled + Element + Custom Class`、`Display Comparisons (Tone / State / Element)`、`Controlled / Uncontrolled (Input-Driven N/A)`、`Streaming Optional + Snapshot Fallback` 与 `Config + Code + CSS Test Workbench`，参数语义与当前实现保持一致。
- Source-first / Copy-Paste Ready：文档页提供 `data-slot="error-message-source-first"` 区块与 `Snippet(copyable=true)`；Playground 代码继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，并显式给出源码落点与 feature 前提（`component-error_message`、`inject-css`），避免复制即报错。
- 研究文档补充判定：本轮为 ErrorMessage 参数语义命名统一与文档验收面对齐，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ErrorView 同步记录（2026-02-20）

- 参数模型同步：`ErrorView` 维持 display feedback primitive 定位，参数主轴保持 `is_invalid/tone/is_compact/is_bordered/message/aria_label/class_name/icon/actions/motion/lang/dir`；本轮未引入新的参数语义变更，默认路径仍为零门槛 `<ErrorView is_invalid=true message=... />`。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ErrorView", "error-view", "Display", display_extra::error_view)` 暴露入口；`#/components/error-view` 可索引访问，且 `components/error-view/src/README.md` 提供等价文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display_extra.rs::error_view()` 已覆盖 `Hello World`、`State Matrix`、`Interactive Playground`、`Source-first Starter (Copy-Paste Ready)`、`Streaming / Snapshot Contract`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：文档页提供 `data-slot="error-view-source-first-contract"` 区块；Playground 代码继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，并显式给出源码落点与 feature 前提（`component-error_view`、`inject-css`），避免复制即报错。
- 研究文档补充判定：本轮为参数模型与文档入口同步校验，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Empty 同步记录（2026-02-20）

- 参数模型同步：`Empty*` 继续保持 display composition primitive 定位，公开参数保持 `class_name/lang/dir`（各槽位）与 `variant`（`EmptyMedia`）；本轮未引入破坏性参数语义变更，默认路径仍为零门槛组合 `<Empty><EmptyHeader><EmptyTitle>...</EmptyTitle></EmptyHeader></Empty>`。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `display_extra_empty_catalog::{EMPTY_DOC, EMPTY_HEADER_DOC, EMPTY_MEDIA_DOC, EMPTY_TITLE_DOC, EMPTY_DESCRIPTION_DOC, EMPTY_CONTENT_DOC}` 暴露入口；`#/components/empty` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display_extra_empty.rs` 已覆盖 `Hello World`、`Interactive Playground`、`Parameter Matrix`、`State Matrix`、`Controlled vs Uncontrolled (N/A)`、`Streaming/Snapshot` 与 `Source-first / Copy-Paste Ready`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：`Empty` playground 代码继续通过 `code_signal + code_imports` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档同步给出 feature 前提（`component-empty`）与源码落点（`components/empty/src/{mod,logic,view,styles}.rs`），避免复制即报错。
- 研究文档补充判定：本轮仅为 Empty 文档验收面与对标策略同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### EmptyState 同步记录（2026-02-20）

- 参数模型同步：`EmptyState` 维持 display primitive 定位，参数主轴保持 `title/description/tone/align/is_compact/is_bordered/aria_label/class_name/icon/actions/motion/lang/dir`；命名遵循统一 `is_*` 规则，不引入平行别名漂移。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("EmptyState", "empty-state", "Display", display_extra::empty_state)` 暴露入口；`#/components/empty-state` 可索引访问，且 `components/empty-state/README.md` 提供等价文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display_extra.rs::empty_state()` 已覆盖 `Hello World (Default Path)`、`State Matrix`、`Tone + Alignment + Actions`、`Compact + Bordered + Custom Class`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Interactive Playground` 与 `Source-first Starter (Copy-Paste Ready)`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：文档继续通过 `code_signal + code_imports` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；`data-slot="empty-state-source-first-contract"` 区块显式给出源码落点与 feature 前提（`component-empty_state`、`inject-css` + `UiRoot`），避免复制即报错。
- 研究文档补充判定：本轮仅为 EmptyState 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Item 同步记录（2026-02-21）

- 参数模型同步：`Item` 参数主轴保持 `variant/size/class_name/lang/dir`，默认值继续由 `logic.rs` 的 `normalize_item_variant/normalize_item_size`（`unwrap_or_default`）统一归一；本轮未引入破坏性参数语义漂移。
- docs 入口同步：`apps/docs-app/src/pages/components/pages/collections_item_catalog.rs` 通过 `ITEM_DOC`（`slug = "item"`）暴露入口，`apps/docs-app/src/pages/components/pages.rs` 保持 `collections_item_catalog::ITEM_DOC` 聚合，`#/components/item` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/collections_item_primitives.rs` 已覆盖 `Hello World`、`Media + Content + Actions`、`Header + Footer Layout`、`State Matrix (Variant + Size)`、`Controlled vs Uncontrolled (N/A)`、`Streaming / Snapshot Display`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：item playground 继续通过 `code_signal + code_imports` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档显式标注源码落点与 `component-item` feature 前提，避免复制即报错。
- 研究文档补充判定：本轮仅为 Item 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### HelpText 同步记录（2026-02-20）

- 参数模型同步：`HelpText` 参数主轴保持 `tone/is_invalid/is_disabled/is_error_icon_visible/description/error_message/aria_label/motion/class_name/lang/dir`；命名统一收敛到 `is_*` 轴，旧别名 `invalid/disabled/show_error_icon` 已移除并由文档迁移说明承接。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("HelpText", "help-text", "Forms", forms_extra::help_text)` 暴露入口；`#/components/help-text` 可索引访问，且 `components/help-text/src/README.md` 提供等价入门入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs::help_text()` 已覆盖 `Hello World (Default API)`、`State Matrix (Description / Error / Disabled)`、`Controlled vs Uncontrolled (Stateless Contract)`、`Interactive Playground` 与 `Source-first / Copy-Paste Ready`，参数语义与默认值与实现保持一致。
- Source-first / Copy-Paste Ready：HelpText playground 继续通过 `code_imports=help_text_imports.clone()` 与 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 输出可运行片段，并显式给出源码落点（`components/help-text/src/{mod,logic,view,styles,motion}.rs`），避免复制即报错。
- 研究文档补充判定：本轮为参数语义命名与文档验收面对齐，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：保持“默认路径简洁、进阶参数按需开启”的体验目标；参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Icon 同步记录（2026-02-21）

- 参数模型同步：`Icon` 参数主轴保持 `size/tone/is_disabled/is_decorative/aria_label/class_name/lang/dir/slot`；命名继续遵循统一 `is_*` 状态前缀与 `aria_*` 语义命名，不引入平行别名或破坏性回退。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Icon", "icon", "Display", display_extra::icon)` 暴露入口；`#/components/icon` 可索引访问，且 `components/icon/src/README.md` 提供等价入门文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display_extra.rs::icon()` 已覆盖 `Hello World (Default Path)`、`Size + Tone Matrix`、`Accessible + Disabled + Custom Class`、`Workbench (Display + Config + Code + CSS Test)` 与 `Source-first Starter (Copy-Paste Ready)`，参数语义与默认路径保持一致。
- Source-first / Copy-Paste Ready：`icon` 文档通过 `code_imports=icon_code_imports` 与 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 输出可运行片段（含 `use leptos::prelude::*;`、`use ui::{Icon, IconSize, IconTone};`）；源码落点固定为 `components/icon/src/styles.rs` 并在文档中声明依赖前提（`ui`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `Icon` 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### ComboBox 同步记录（2026-02-20）

- 参数模型同步：`ComboBox` 参数主轴保持 `items + selected_index + set_selected_index`，并维持受控/非受控 open 轴 `is_open + on_open_change + default_open`。`is_disabled/disabled_indices/placeholder/empty_message/class_name/motion` 继续保持统一命名与显式可配置路径，不引入破坏性参数别名。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("ComboBox", "combo-box", "Collections", collections::combo_box)` 暴露入口；`#/components/combo-box` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/collections.rs::combo_box()` 保持 `Hello World (Uncontrolled)`、`展示：多场景对比`、`Workbench（展示 + Config + Code + CSS Test）`、`Streaming/Snapshot Display` 与 `Source-first / Copy-Paste Ready`，参数语义与当前实现保持一致。
- Source-first / Copy-Paste Ready：ComboBox Playground 代码继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，并在文档中显式给出源码落点与 feature 前提，避免复制即报错。
- 研究文档补充判定：本轮为 ComboBox 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Command 同步记录（2026-02-21）

- 参数模型同步：`Command` 参数主轴保持 `id_base/groups` 必填 + `query/on_query_change/default_query` 受控/非受控成对轴，并维持 `on_action/is_disabled/motion/placeholder/empty_label/aria_label/lang/dir/class_name` 作为按需扩展参数；命名继续遵循统一 `is_*`、`on_*`、`default_*` 约定，不引入同义别名漂移。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Command", "command", "Collections", collections_command::command)` 暴露入口；`#/components/command` 可索引访问，且 `components/command/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/collections_command.rs::command()` 已覆盖 `Hello World (Default API)`、`State Matrix`、`Controlled vs Uncontrolled`、`Interactive Playground`、`Source-first / Copy-Paste Ready` 与 `Streaming / Snapshot Contract`，参数语义与当前实现保持一致。
- Source-first / Copy-Paste Ready：`command` 文档继续通过 `code_signal + code_imports=COMMAND_DOC_IMPORTS` 接入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；源码落点固定为 `components/command/src/{mod,logic,view,styles,motion}.rs`，并显式给出 feature 前提（`component-command`、`inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `Command` 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### CommandDialog 同步记录（2026-02-20）

- 参数模型同步：`CommandDialog` 参数主轴保持 `open + on_open_change + default_open`、`close_on_action`、`is_disabled/disabled`、`on_action`、`placeholder/empty_label/aria_label`、`command_motion/overlay_motion`、`class_name`；默认值继续由 `logic.rs` 统一归一（`DEFAULT_CLOSE_ON_ACTION`、`DEFAULT_DISABLED`、`DEFAULT_DEFAULT_OPEN`），不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("CommandDialog", "command-dialog", "Collections", collections_command::command_dialog)` 暴露入口；`#/components/command-dialog` 可索引访问，且 `components/command-dialog/src/README.md` 提供等价文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/collections_command.rs::command_dialog()` 已覆盖 `Hello World (Default API)`、`State Matrix`、`Controlled Open + Action Close`、`State + Source Markers`、`Controlled vs Uncontrolled`、`Streaming / Snapshot Contract`、`Workbench` 与 `Source-first Copy-Paste`，参数语义与当前实现保持一致。
- Source-first / Copy-Paste Ready：`command-dialog` playground 继续通过 `code_signal + code_imports=COMMAND_DIALOG_DOC_IMPORTS` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档显式标注源码落点与 feature 前提（`component-command_dialog`、`inject-css`），避免复制即报错。
- 研究文档补充判定：本轮为 CommandDialog 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Keyboard 同步记录（2026-02-20）

- 参数模型同步：`Keyboard` 维持 display primitive 定位，参数主轴保持 `tone/is_compact/aria_label/class_name`；命名继续遵循统一 `is_*` 规则（仅 `is_compact`），默认值由 `components/keyboard/src/logic.rs::normalize_root_state` 统一归一（`tone.unwrap_or_default()`、`is_compact.unwrap_or(false)`、`normalize_aria_label(...)`、`normalize_optional_text(...)`）。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Keyboard", "keyboard", "Display", display_extra::keyboard)` 暴露入口；`#/components/keyboard` 可索引访问，且 `components/keyboard/src/README.md` 提供等价文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display_extra.rs::keyboard()` 已覆盖 `Hello World (Default Path)`、`State Matrix (Tone / Compact / Source Markers)`、`Interactive Playground (展示 / Config / Code / CSS Test)` 与 `Source-first Starter (Copy-Paste Ready)`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：Keyboard playground 代码继续通过 `code_signal + code_imports` 接入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；README 显式标注源码落点与 feature 前提（`component-keyboard`），避免复制即报错。
- 研究文档补充判定：本轮仅为 Keyboard 参数模型与文档验收面对齐，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：保持“先用起来，再进阶”的路径顺序；参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Form 同步记录（2026-02-20）

- 参数模型同步：`Form` 维持表单上下文容器定位，参数主轴保持 `is_disabled/is_read_only/is_required/label_position/label_align/class_name/lang/dir`；默认值继续由 `components/form/src/logic.rs::resolve_props` 统一归一（`is_* = false`、`label_position = Top`、`label_align = Start`），不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Form", "form", "Forms", forms::form)` 暴露入口；`#/components/form` 可索引访问，且 `components/form/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms.rs::form()` 已覆盖 `Hello World（默认路径）`、`Interactive Playground (展示 / Config / Code / CSS Test)`、`Comparison Matrix (Default / Required / Disabled / ReadOnly)`，并与当前参数语义和默认值保持一致。
- Source-first / Copy-Paste Ready：`Form` playground 代码继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档显式标注 `test_source_path="/root/autodl-tmp/zjj/p/rust-ui/components/form/src/styles.rs"`，避免复制即报错与源码落点漂移。
- 研究文档补充判定：本轮仅为 Form 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Legend 同步记录（2026-02-21）

- 参数模型同步：`Legend` 继续保持语义标题组件定位，参数主轴保持 `text/tone/is_required/is_disabled/required_indicator/class_name/lang/dir/motion`；默认值由 `components/legend/src/logic.rs` 统一归一（`DEFAULT_IS_REQUIRED = false`、`DEFAULT_IS_DISABLED = false`），不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Legend", "legend", "Forms", forms_groups_extra::legend)` 暴露入口；`#/components/legend` 可索引访问，且 `components/legend/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::legend()` 已覆盖 `Hello World`、`Required Legend`、`Tone + Custom Indicator + Disabled`、`Controlled vs Default (Comparison)`，与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：Legend playground 代码继续通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，并在 docs 页面显式给出源码落点与 feature 前提（`component-legend` + 可选 `inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 Legend 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Kbd 同步记录（2026-02-20）

- 参数模型同步：`Kbd` 维持 display primitive 定位，参数主轴保持 `size/keys/class_name/children`；默认值继续由 `components/kbd/src/logic.rs` 统一归一（`normalize_size -> unwrap_or_default()`、`normalize_optional_text(keys/class_name)`），不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Kbd", "kbd", "Display", display::kbd)` 暴露入口；`#/components/kbd` 可索引访问，且 `components/kbd/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display.rs::kbd()` 已覆盖 `Hello World (Default API)`、`State Matrix (Size + Keys + Label-only)`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Source-first Starter (Copy-Paste Ready)` 与 `Workbench (Display + Config + Code + CSS Test)`，与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：`kbd` playground 代码继续通过 `code_signal + code_imports=kbd_imports` 接入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，并在 `data-slot="kbd-source-first"` 区块显式给出源码落点与 feature/style 前提（`component-kbd`、`UiRoot + inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 Kbd 参数模型与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Meter 同步记录（2026-02-20）

- 参数模型同步：`Meter` 维持 display primitive 定位，参数主轴保持 `id`（必填）+ `label/aria_label/lang/dir/value/min/max/variant/size/motion/is_value_label_visible/show_value_label/value_label/class_name`；默认值继续由 `components/meter/src/logic.rs::normalize_inputs` 统一归一（`DEFAULT_MIN=0.0`、`DEFAULT_MAX=100.0`、`DEFAULT_SHOW_VALUE_LABEL=true`），并保持 `is_value_label_visible > show_value_label` 的兼容优先级，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Meter", "meter", "Display", display::meter)` 暴露入口；`apps/docs-app/src/pages/components/pages/display.rs::meter()` 通过 `slug="meter"` 可索引访问，且 `components/meter/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display.rs::meter()` 已覆盖 `Hello World (Default API)`、`Variant + Size Matrix`、`Custom Label + Motion + Class`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Workbench (Display + Config + Code + CSS Test)`、`State Matrix`、`Parameter Matrix` 与 `Source-first / Copy-Paste Ready`，参数语义与当前实现保持一致。
- 研究文档补充判定：本轮仅为 Meter 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Code 同步记录（2026-02-20）

- 参数模型同步：`Code` 维持 display primitive 定位，参数主轴保持 `variant/class_name/lang/dir/children`；默认值继续由 `components/code/src/logic.rs::resolve_view_state` 统一归一（`variant.unwrap_or_default()`、`normalize_optional_text(input.class_name)`），不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Code", "code", "Display", display::code)` 暴露入口；`apps/docs-app/src/pages/components/pages/display.rs::code()` 通过 `slug="code"` 可索引访问，且 `components/code/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display.rs::code()` 已覆盖 `Hello World (Default API)`、`Variant Matrix`、`Interactive Playground`、`State Matrix`、`Parameter Matrix` 与 `Source-first Starter (Copy-Paste Ready)`，参数语义与当前实现保持一致。
- Source-first / Copy-Paste Ready：`Code` 文档继续通过 `code_signal + code_imports` 接入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，并在 `data-slot="code-source-prerequisites"` 显式给出 `component-code + inject-css + UiRoot` 前提，避免复制即报错。
- 研究文档补充判定：本轮仅为 Code 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### CodeBlock 同步记录（2026-02-20）

- 参数模型同步：`CodeBlock` 参数主轴保持 `code` 必填 + `label/language/class_name/lang/dir`，并维持受控/非受控复制状态轴 `is_copied + on_copied_change + default_copied`；复制开关继续以 `is_copyable` 为主命名，`copyable` 仅作兼容别名归一。AI 输出轴保持 `output_mode/output_status` 显式可选，默认回落 `Snapshot + Validated`。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("CodeBlock", "code-block", "Display", display::code_block)` 暴露入口；`apps/docs-app/src/pages/components/pages/display.rs::code_block()` 维持 `title="CodeBlock"` 与 `slug="code-block"` 可索引访问；`components/code-block/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display.rs::code_block()` 已覆盖 `Hello World (Default API)`、`State Matrix`、`Controlled vs Uncontrolled (Copied State)`、`Streaming Optional / Snapshot`、`Workbench (Display + Config + Code + CSS Test)` 与 `Source-first Starter (Copy-Paste Ready)`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：CodeBlock playground 代码继续通过 `code_signal + code_imports` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；`data-slot="code-block-source-first"` 区块显式给出 feature/style 前提（`component-code_block`、`UiRoot + inject-css`）与真实源码落点，避免复制即报错。
- 研究文档补充判定：本轮仅为 `CodeBlock` 参数模型与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Description 同步记录（2026-02-20）

- 参数模型同步：`Description` 参数主轴保持 `text/tone/is_disabled/is_truncated/element/aria_label/class_name/lang/dir`；状态命名统一使用 `is_*`，并由 `components/description/src/logic.rs` 与 `crates/ui-state-primitives/src/description.rs` 统一归一默认值与来源标记。
- docs 入口同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs` 通过 `description()` 暴露 `slug="description"` 页面入口；`components/description/src/README.md` 提供等价组件文档入口，保证可索引可访问。
- 示例矩阵同步：文档页持续覆盖 `Hello World`、`State Matrix (Tone / Disabled / Truncate)`、`Controlled vs Uncontrolled (Stateless Contract)`、`Workbench` 与 `Source-first / Copy-Paste Ready`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：Description playground 代码通过 `code_imports=description_imports.clone()` 与 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档显式给出源码落点（`components/description/src/{mod,logic,view,styles}.rs`）与 feature 前提（`component-description`、`inject-css`），避免复制即报错。
- 研究文档补充判定：本轮为 Description 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### IllustratedMessage 同步记录（2026-02-20）

- 参数模型同步：`IllustratedMessage` 维持 display empty-state primitive 定位，参数主轴保持 `title/description/illustration/actions/orientation/motion/class_name/lang/dir`；本轮未引入破坏性参数语义变更，默认值与来源归一继续由 `components/illustrated-message/src/logic.rs::resolve_view_model` 统一收敛（`missing/blank -> hidden`）。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("IllustratedMessage", "illustrated-message", "Display", display::illustrated_message)` 暴露入口；`#/components/illustrated-message` 可索引访问，且 `components/illustrated-message/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display.rs::illustrated_message()` 已覆盖 `Hello World (Default API)`、`State Matrix`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Source-first Starter (Copy-Paste Ready)` 与 `Interactive Playground (Props + State + Preview)`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：`illustrated_message` 文档通过 `code_signal + code_imports` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；`data-slot="illustrated-message-source-first"` 区块显式给出源码落点与 feature/style 前提（`component-illustrated_message`、`UiRoot + inject-css`），避免复制即报错。
- 研究文档补充判定：本轮为参数模型与组件文档入口同步校验，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Direction 同步记录（2026-02-20）

- 参数模型同步：`DirectionProvider` 继续保持语义 provider 定位，参数主轴为 `direction/dir/lang/class_name`；归一优先级保持 `direction > dir > DirectionMode::default()`，并通过 `data-direction-source=direction|dir-alias|default` 暴露来源契约。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `layout_extra_direction::DIRECTION_PROVIDER_DOC` 暴露 `slug="direction-provider"`；`apps/docs-app/src/pages/components/test/mod.rs` 保持 `"direction" => &["direction-provider"]` 可检索映射。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/layout_extra_direction.rs` 已覆盖 `Hello World`、`State Matrix`、`Interactive Playground`、`Source-first / Copy-Paste Ready` 与参数/默认值规则，参数语义与当前实现保持一致。
- Source-first / Copy-Paste Ready：Direction playground 代码继续通过 `code_signal + code_imports=DIRECTION_COPY_IMPORTS` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档显式给出源码落点与 feature 前提（`component-direction`、`inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `DirectionProvider` 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Field 同步记录（2026-02-20）

- 参数模型同步：`Field` 参数主轴保持 `orientation/tone/is_required/is_disabled/is_invalid`，并保留兼容别名 `required/disabled/invalid`；内容与可访问性参数保持 `label/description/error_message/aria_label/lang/dir/class_name/motion`。默认值与优先级统一由 `components/field/src/logic.rs::{resolve_is_required,resolve_is_disabled,resolve_is_invalid}` 归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Field", "field", "Forms", forms_extra::field)` 暴露入口；`apps/docs-app/src/pages/components/pages/forms_extra.rs::field()` 维持 `title="Field"` 与 `slug="field"` 可索引访问；`components/field/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs::field()` 已覆盖 `Hello World (Default API)`、`State Matrix (Required / Invalid / Disabled)`、`Controlled vs Uncontrolled (Stateless Contract)`、`Streaming Optional (fallback=snapshot)`、`Workbench (Display + Config + Code + CSS Test)` 与 `Source-first / Copy-Paste Ready`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：`Field` docs 继续通过 `code_imports=field_imports + apps/docs-app/src/playground.rs::compose_copy_ready_code` 保障复制代码 import-ready；`data-slot="field-source-first"` 区块明确 `Show code + Copy` 路径、源码落点 `components/field/src/{mod,logic,view,styles,motion}.rs` 与依赖前提 `component-field + inject-css`，避免复制即报错。
- 研究文档补充判定：本轮仅为 `Field` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Fieldset 同步记录（2026-02-20）

- 参数模型同步：`Fieldset` 参数主轴保持 `orientation/tone/is_required/default_is_required/on_is_required_change/is_disabled/default_is_disabled/on_is_disabled_change/is_invalid/default_is_invalid/on_is_invalid_change`，并维持 `legend/description/error_message/actions/aria_label/class_name/lang/dir/motion` 语义轴；默认值与来源标记统一由 `components/fieldset/src/logic.rs::resolve_view_state` 与 `ui-state-primitives::fieldset::normalize_boolean_axis` 收敛。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Fieldset", "fieldset", "Forms", forms_extra::fieldset)` 暴露入口；`apps/docs-app/src/pages/components/pages/forms_extra.rs::fieldset()` 维持 `title="Fieldset"` 与 `slug="fieldset"` 可索引访问；`components/fieldset/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/forms_extra.rs::fieldset()` 已覆盖 `Hello World`、`Legend + Description`、`Horizontal + Invalid + Actions`、`Controlled vs Uncontrolled (Snapshot Contrast)`、`Streaming Optional (fallback=snapshot)`、`Fieldset Workbench (Display + Config + Code + CSS Test)` 与 `Source-first / Copy-Paste Ready`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：`Fieldset` 文档通过 `Show code + copy` 与 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 保持 import-ready；`data-slot="fieldset-source-first"` 区块明确源码落点与 feature 前提（`component-fieldset + inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `Fieldset` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### FormField 同步记录（2026-02-20）

- 参数模型同步：`FormField` 参数主轴保持 `is_selected/default_selected/on_selected_change`、`is_disabled/is_invalid`、`tone/indicator_variant/indicator_placement`、`label/description/error_message/aria_label/class_name/lang/dir`；默认值与来源标记继续由 `components/form-field/src/logic.rs::normalize_selected_axis + resolve_state` 统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("FormField", "form-field", "Forms", forms_groups_extra::form_field)` 暴露入口；`apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::form_field()` 维持 `title="FormField"` 与 `slug="form-field"` 可索引访问；`components/form-field/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`forms_groups_extra.rs::form_field()` 已覆盖 `Hello World（默认路径）`、`Switch Indicator + Description`、`Checkbox Indicator + Quiet + Invalid/Disabled`、`Controlled vs Default (Comparison)`、`FormField Workbench (Display + Config + Code + CSS Test)`、`Streaming Optional (fallback=snapshot)` 与 `Source-first / Copy-Paste Ready`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：`FormField` docs 继续通过 `code_signal + apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；`data-slot="form-field-source-paths"` 与 `data-slot="form-field-source-prerequisites"` 明确源码落点和依赖前提（`component-form_field + inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `FormField` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### FlipCard 同步记录（2026-02-20）

- 参数模型同步：`FlipCard` 参数主轴保持 `front/back/is_flipped/default_is_flipped/on_is_flipped_change/is_disabled/flip_mode/is_flip_on_hover/motion/class_name/id/lang/dir`，并保留兼容别名 `default_flipped/disabled/flip_on_hover`；默认值与优先级继续由 `components/flip-card/src/logic.rs::normalize_flipped_axis` 与 `crates/ui-state-primitives/src/flip_card.rs::normalize_behavior_flags` 统一归一。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("FlipCard", "flip-card", "Display", display_extra::flip_card)` 暴露入口；`apps/docs-app/src/pages/components/pages/display_extra.rs::flip_card()` 维持 `title="FlipCard"` 与 `slug="flip-card"` 可索引访问；`components/flip-card/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`display_extra.rs::flip_card()` 已覆盖 `Hello World (Default Path)`、`State Matrix (Default / Hover / Disabled / Dramatic Motion)`、`Controlled vs Uncontrolled Contrast`、`Streaming / Snapshot Contract`、`Source-first Starter (Copy-Paste Ready)` 与 `Interactive Playground (展示 / Config / Code / CSS Test)`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：`FlipCard` 文档继续通过 `code_signal + code_imports=flip_card_imports` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补全 imports；`data-slot="flip-card-source-first-contract"` 区块明确源码落点与依赖前提（`component-flip_card + inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### DateInputGroup 同步记录（2026-02-20）

- 参数模型同步：`DateInputGroup` 参数主轴保持 `variant/is_full_width/is_disabled/is_invalid/is_segmented/aria_label/lang/dir/class_name/prefix/suffix/motion`，并继续通过 `components/date-input-group/src/view.rs` 的 `logic::resolve_*` 归一映射到 `ui-state-primitives::date_input_group` 状态轴（`variant/width/status/segmented`）。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("DateInputGroup", "date-input-group", "Forms", forms_groups::date_input_group)` 暴露入口；`apps/docs-app/src/pages/components/pages/forms_groups.rs::date_input_group()` 维持 `title="DateInputGroup"` 与 `slug="date-input-group"` 可索引访问。
- 示例矩阵同步：`forms_groups.rs::date_input_group()` 已覆盖 `Hello World (Default API)`、`State Matrix (Default / Prefix-Suffix / Secondary+Invalid)`、`Controlled vs Uncontrolled (Child Field Axis)`、`Streaming / Snapshot Contract` 与 `Source-first Starter (Copy-Paste Ready)`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：`DateInputGroup` 文档继续通过 `code_signal + code_imports=date_input_group_imports` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补全 imports；`data-slot="date-input-group-source-first"` 区块明确源码落点与依赖前提（`component-date_input_group + inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `DateInputGroup` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### NativeSelect 同步记录（2026-02-20）

- 参数模型同步：`NativeSelect` 参数主轴保持 `selected_index/on_selected_index_change/default_selected_index`、`is_disabled/is_required/is_invalid/size`、`aria_label/name/placeholder/class_name/lang/dir`；命名继续遵循统一 `is_* / on_* / default_*` 契约，不引入平行别名。
- 受控/非受控同步：默认路径保持零接线 `<NativeSelect id_base=... options=... />`；进阶路径显式走 `selected_index + on_selected_index_change` 与 `default_selected_index`，避免半受控隐式写回。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("NativeSelect", "native-select", "Forms", forms_native::native_select)` 暴露目录入口；`apps/docs-app/src/pages/components/pages/forms_native.rs::native_select()` 维持 `title="NativeSelect"` 与 `slug="native-select"` 可索引访问。
- 组件文档同步：`components/native-select/src/README.md` 已与 docs-app 对齐，保留 `Hello World`、`Controlled + Placeholder`、`State Matrix` 与 API 表，保证“先用起来，再进阶”路径可用。
- 研究文档补充判定：本轮仅为参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 `components/native-select/src/README.md`、docs 入口，再推进实现；接口变更场景下禁止“仅代码更新无文档更新”合入。

### Image 同步记录（2026-02-20）

- 参数模型同步：`Image` 参数主轴保持 `src/alt/fallback_src/is_skeleton_disabled/is_blurred/is_zoomed/radius/shadow/motion/class_name/lang/dir`；默认值继续由 `components/image/src/logic.rs::normalize_props` 与 `ui-state-primitives::image` 枚举默认值统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Image", "image", "Display", display::image)` 暴露入口；`apps/docs-app/src/pages/components/pages/display.rs::image()` 维持 `title="Image"` 与 `slug="image"` 可索引访问；`components/image/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`display.rs::image()` 已覆盖 `Hello World (Default API)`、`State Matrix`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Source-first Starter (Copy-Paste Ready)` 与 `Workbench (Display + Config + Code + CSS Test)`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：`Image` 文档继续通过 `code_signal + code_imports` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；`data-slot="image-source-first"` 区块明确源码落点与依赖前提（`component-image + inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `Image` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步 `docs/spec/heroui-parameter-design-strategy.md` 与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Dialog 同步记录（2026-02-20）

- 参数模型同步：`Dialog` 参数主轴保持 `is_open/open + on_open_change + default_open`、`size`、`is_close_button_visible/show_close_button`、`close_label`、`motion`、`on_close`、`class_name/lang/dir`；默认值继续由 `components/dialog/src/logic.rs` 统一归一（`DEFAULT_OPEN`、`DEFAULT_SHOW_CLOSE_BUTTON`、`DEFAULT_SIZE`、`DEFAULT_CLOSE_LABEL`），不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Dialog", "dialog", "Overlays", overlays::dialog)` 暴露入口；`#/components/dialog` 可索引访问，且 `components/dialog/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/overlays_dialog.rs::dialog()` 已覆盖 `Hello World`、`State Matrix`、`Controlled vs Uncontrolled`、`Streaming / Snapshot Contract`、`Interactive Playground` 与 `Scenario Comparison`，参数语义与当前实现保持一致。
- Source-first / Copy-Paste Ready：`dialog` playground 继续通过 `code_signal + code_imports=DIALOG_DOC_IMPORTS` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；文档显式标注源码落点与 feature 前提（`component-dialog`、`inject-css`），避免复制即报错。
- 研究文档补充判定：本轮为 Dialog 参数语义与文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### AlertDialog 同步记录（2026-02-20）

- 参数模型同步：`AlertDialog` 参数主轴保持 `open + on_close`、`id_base/title/description`、`confirm_label/on_confirm`、`secondary_label/on_secondary`、`is_confirm_disabled/is_secondary_disabled`、`variant/auto_focus_button/motion/on_exit_complete/class_name/lang/dir`；默认值继续由 `components/alert-dialog/src/logic.rs` 统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("AlertDialog", "alert-dialog", "Overlays", overlays::alert_dialog)` 暴露入口；`apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs::alert_dialog()` 保持 `title="AlertDialog"` 与 `slug="alert-dialog"` 可索引访问。
- 组件文档同步：`components/alert-dialog/src/README.md` 提供等价组件文档入口（含 `Hello World（最小可用）` 与“先用起来，再进阶”路径）。
- Source-first / Copy-Paste Ready：`apps/docs-app/src/pages/components/pages/overlays_alert_dialog.rs` 的 source-first 区块继续通过 `ALERT_DIALOG_DOC_IMPORTS + apps/docs-app/src/playground.rs::compose_copy_ready_code` 保障一键复制可运行，并显式标注真实源码落点与 `component-alert_dialog + inject-css` 依赖前提。
- 研究文档补充判定：本轮仅为 `AlertDialog` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Modal 同步记录（2026-02-20）

- 参数模型同步：`Modal` 参数主轴保持 `is_open/default_open/on_open_change`，并在组件层维持 `id_base/title/description/on_close/motion/on_exit_complete/class_name/lang/dir` 的显式输入边界；受控/非受控契约继续由 `components/modal/src/logic.rs` 统一归一。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Modal", "modal", "Overlays", overlays::modal)` 暴露入口；`apps/docs-app/src/pages/components/pages/overlays.rs::modal()` 保持 `title="Modal"` 与 `slug="modal"` 可索引访问。
- 组件文档同步：`components/modal/src/README.md` 提供等价组件文档入口（含 `Hello World` 与进阶路径），确保“先用起来，再进阶”。
- Source-first / Copy-Paste Ready：`overlays.rs::modal()` 的 source-first 区块继续标注真实源码落点与依赖前提，并通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，避免示例漂移。
- 研究文档补充判定：本轮仅为 `Modal` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Drawer 同步记录（2026-02-20）

- 参数模型同步：`Drawer` 参数主轴保持 `is_open/default_open/on_open_change`，并维持 `id_base/title/description/placement/motion/on_close/on_exit_complete/class_name/lang/dir` 的显式边界；受控/非受控语义继续由 `components/drawer/src/logic.rs` 统一归一。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Drawer", "drawer", "Overlays", overlays::drawer)` 暴露入口；`apps/docs-app/src/pages/components/pages/overlays.rs::drawer()` 保持 `title="Drawer"` 与 `slug="drawer"` 可索引访问。
- 组件文档同步：`components/drawer/src/README.md` 提供等价组件文档入口（含 `Hello World` 与“先用起来，再进阶”路径）。
- Source-first / Copy-Paste Ready：`overlays.rs::drawer()` 继续通过 `DRAWER_DOC_IMPORTS + compose_copy_ready_code` 保障一键复制可运行，并标注真实源码落点与 `component-drawer + inject-css` 依赖前提。
- 研究文档补充判定：本轮仅为 `Drawer` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### BottomSheet 同步记录（2026-02-20）

- 参数模型同步：`BottomSheet` 参数主轴保持 `open/on_close`，并继续在组件层显式暴露 `id_base/title/description/footer/motion/is_handle_visible/is_close_button_visible/is_detached/bottom_inset_px/is_dismissable/is_keyboard_dismiss_disabled/class_name/lang/dir`；默认值由 `components/bottom-sheet/src/logic.rs` 统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 保持 `component_doc!("BottomSheet", "bottom-sheet", "Overlays", overlays_extra::bottom_sheet)` 目录索引；`apps/docs-app/src/pages/components/pages/overlays_extra.rs::bottom_sheet()` 维持 `title="BottomSheet"` 与 `slug="bottom-sheet"` 可索引访问。
- 组件文档同步：`components/bottom-sheet/src/README.md` 提供等价组件文档入口，并保留 `Hello World` 与“先用起来，再进阶”的默认路径。
- Source-first / Copy-Paste Ready：docs 页面继续通过 `BOTTOM_SHEET_DOC_IMPORTS + apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，保持复制即运行与源码落点可追溯。
- 研究文档补充判定：本轮仅为 `BottomSheet` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### HoverCard 同步记录（2026-02-20）

- 参数模型同步：`HoverCard` 参数主轴保持 `is_open/open + on_open_change + default_open`、`is_disabled/disabled`（兼容别名，`is_disabled` 优先）、`open_delay_ms/close_delay_ms`、`placement/motion/class_name/lang/dir`；默认值继续由 `components/hover-card/src/logic.rs` 统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("HoverCard", "hover-card", "Overlays", overlays::hover_card)` 暴露入口；`apps/docs-app/src/pages/components/pages/overlays_hover_card.rs::hover_card()` 维持 `title="HoverCard"` 与 `slug="hover-card"` 可索引访问。
- 组件文档同步：`components/hover-card/src/README.md` 提供等价组件文档入口，保留 `Hello World`、`先用起来，再进阶`、`常见用法` 与受控进阶路径。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/overlays_hover_card.rs::hover_card()` 已覆盖 `Hello World (Minimal Path)`、`State Matrix`、`Controlled vs Uncontrolled` 与 `Streaming / Snapshot Contract`，参数语义与当前实现保持一致。
- 研究文档补充判定：本轮仅为 `HoverCard` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### DropZone 同步记录（2026-02-20）

- 参数模型同步：`DropZone` 参数主轴保持 `label/aria_label/is_disabled/disabled/motion/on_drop_files/lang/dir`；默认值与来源标记继续由 `components/drop-zone/src/logic.rs::resolve_props` 与 `classify_disabled_input` 统一归一（`is_disabled` 优先于兼容别名 `disabled`）。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("DropZone", "drop-zone", "Files", files::drop_zone)` 暴露入口；`apps/docs-app/src/pages/components/pages/files.rs::drop_zone()` 维持 `title="DropZone"` 与 `slug="drop-zone"` 可索引访问；`components/drop-zone/src/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`files.rs::drop_zone()` 已覆盖 `Hello World`、`Quick Start (Default API)`、`State Matrix (Disabled / Motion / Callback)`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional (fallback=snapshot)`、`Source-first Starter (Copy-Paste Ready)` 与 `Workbench（展示 + Config + Code + CSS Test）`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：`DropZone` 文档继续通过 `code_signal + code_imports=source_first_imports` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补全 imports；`data-slot="drop-zone-source-paths"` 区块明确源码落点与依赖前提（`component-drop_zone + inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `DropZone` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Breadcrumb 同步记录（2026-02-20）

- 参数模型同步：`Breadcrumb` 参数主轴保持 `items/aria_label/class_name/separator/lang/dir`；默认值与优先级继续由 `components/breadcrumb/src/logic.rs::resolve_root_state + resolve_separator` 统一归一（`class -> "ui-breadcrumb"`、`separator -> "/"`）。
- docs 入口同步：`apps/docs-app/src/pages/components/pages/collections_breadcrumb_catalog.rs::BREADCRUMB_DOC` 作为目录入口，`apps/docs-app/src/pages/components/pages.rs` 收录 `collections_breadcrumb_catalog::BREADCRUMB_DOC`；`apps/docs-app/src/pages/components/pages/collections_breadcrumb.rs::breadcrumb()` 维持 `title="Breadcrumb"` 与 `slug="breadcrumb"` 可索引访问。
- 示例矩阵同步：`collections_breadcrumb.rs::breadcrumb()` 已覆盖 `Hello World`、`Trail`、`State Matrix (Linked / Label-only / Empty)`、`Controlled vs Uncontrolled (N/A)`、`Streaming / Snapshot Contract` 与 `Source-first Starter (Copy-Paste Ready)`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：`Breadcrumb` 文档继续通过 `code_imports=BREADCRUMB_DOC_IMPORTS` 与 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；`data-slot="breadcrumb-source-first"` 区块明确源码落点与依赖前提（`component-breadcrumb + inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `Breadcrumb` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Asset 同步记录（2026-02-20）

- 参数模型同步：`Asset` 保持 display primitive 定位，参数主轴为 `variant/size/label/is_selected/is_focused/motion/class_name/lang/dir/children`；当前语义继续由 `components/asset/src/logic.rs::resolve_view_state` 与 `ui-state-primitives::asset::resolve_state` 统一归一，本轮未引入破坏性参数语义变更。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Asset", "asset", "Display", display_extra_asset::asset)` 暴露入口；`apps/docs-app/src/pages/components/pages/display_extra_asset.rs` 维持 `title="Asset"` 与 `slug="asset"` 可索引访问；`components/asset/README.md` 提供等价组件文档入口。
- 示例矩阵同步：`display_extra_asset.rs::asset()` 已覆盖 `Hello World`、`State + Source Markers`、`Interactive Playground (Props + State + Spec Preview)` 与 `Source-first Copy-Paste`，并与当前参数语义保持一致。
- Source-first / Copy-Paste Ready：Asset playground 继续通过 `code_signal + code_imports=ASSET_PLAYGROUND_IMPORTS` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；`data-slot="asset-source-first"` 区块明确源码落点与依赖前提（`component-asset + inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `Asset` 文档同步与索引可达性校验，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Checkbox 同步记录（2026-02-20）

- 参数模型同步：`Checkbox` 参数主轴保持 `is_checked/default_checked/on_checked_change`、`is_disabled`（兼容别名 `disabled`）、`variant/size/motion/class_name/lang/dir`；默认值与来源标记继续由 `components/checkbox/src/logic.rs::resolve_checked_control + normalize_is_disabled` 统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Checkbox", "checkbox", "Forms", forms::checkbox)` 暴露入口；`apps/docs-app/src/pages/components/pages/forms.rs::checkbox()` 维持 `title="Checkbox"` 与 `slug="checkbox"` 可索引访问。
- 组件文档同步：`components/checkbox/src/README.md` 提供等价组件文档入口，保留 `# Checkbox`、`## Hello World（最小可用）` 与 `## 先用起来，再进阶`，保证默认路径先于进阶控制参数。
- Source-first / Copy-Paste Ready：`forms.rs::checkbox()` 的 `data-slot="checkbox-source-first"` 区块继续标注真实源码落点与依赖前提（`component-checkbox + inject-css`），并通过 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，避免复制即报错与示例漂移。
- 研究文档补充判定：本轮仅为 `Checkbox` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### CheckboxField 同步记录（2026-02-20）

- 参数模型同步：`CheckboxField` 参数主轴保持 `is_checked/default_checked/on_checked_change`，并继续与 `is_disabled/is_invalid` 统一命名；参数语义归一保持在 `components/checkbox-field/src/logic.rs`，避免 view 层漂移。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("CheckboxField", "checkbox-field", "Forms", forms_groups_extra::checkbox_field)` 暴露入口；`apps/docs-app/src/pages/components/pages/forms_groups_extra.rs::checkbox_field()` 保持 `title="CheckboxField"` 与 `slug="checkbox-field"` 可索引访问。
- 组件文档同步：`components/checkbox-field/src/README.md` 提供等价组件文档入口，并显式声明 docs-app 页面函数与路由。
- Source-first / Copy-Paste Ready：`forms_groups_extra.rs::checkbox_field()` 继续通过 `data-slot="checkbox-field-copy-ready|checkbox-field-source-paths|checkbox-field-source-prerequisites"` 标记真实源码落点与 feature 前提，复制链路由 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports。
- 研究文档补充判定：本轮仅为 `CheckboxField` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：仅代码更新无文档更新在接口变更场景下不允许合入；参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选。

### CheckboxGroup 同步记录（2026-02-20）

- 参数模型同步：`CheckboxGroup` 参数主轴保持 `is_required/is_invalid/is_disabled`、`aria_describedby`、`description/error`、`motion/class_name`；参数语义归一继续在 `components/checkbox-group/src/logic.rs` 收敛，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("CheckboxGroup", "checkbox-group", "Forms", forms::checkbox_group)` 暴露入口；`apps/docs-app/src/pages/components/pages/forms.rs::checkbox_group()` 保持 `title="CheckboxGroup"` 与 `slug="checkbox-group"` 可索引访问。
- 组件文档同步：`components/checkbox-group/src/README.md` 提供等价组件文档入口，并显式声明 docs-app 页面函数与路由。
- Source-first / Copy-Paste Ready：`forms.rs::checkbox_group()` 持续通过 `data-slot="checkbox-group-source-first|checkbox-group-copy-ready|checkbox-group-source-paths|checkbox-group-source-prerequisites"` 标记真实源码落点与 feature 前提，复制链路由 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports。
- 研究文档补充判定：本轮仅为 `CheckboxGroup` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：仅代码更新无文档更新在接口变更场景下不允许合入；参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选。

### CircularProgress 同步记录（2026-02-20）

- 参数模型同步：`CircularProgress` 参数主轴保持 `aria_label/size_px/thickness_px/class_name/lang/dir`，状态语义固定为 indeterminate；默认值与来源标记继续由 `components/circular-progress/src/logic.rs::resolve_component_contract` 与 `ui-state-primitives::circular_progress::{resolve_state, sanitize_dimension}` 统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("CircularProgress", "circular-progress", "Display", display::circular_progress)` 暴露入口；`apps/docs-app/src/pages/components/pages/display.rs::circular_progress()` 维持 `title="CircularProgress"` 与 `slug="circular-progress"` 可索引访问。
- 组件文档同步：`components/circular-progress/src/README.md` 提供等价组件文档入口，保留 `Hello World（先用起来） -> 常见用法 -> 进阶用法` 新手优先路径，并指向 docs-app 路由入口。
- 示例矩阵同步：`display.rs::circular_progress()` 已覆盖 `Hello World`、`Size + Thickness Matrix`、`Custom Label + Class`、`Controlled vs Uncontrolled (N/A)`、`Streaming Optional / Snapshot`、`Interactive Playground (Props / State / Preview)` 与 `Source-first Starter (Copy-Paste Ready)`，参数语义与当前实现保持一致。
- Source-first / Copy-Paste Ready：文档示例继续通过 `code_signal + code_imports` 进入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；`data-slot="circular-progress-source-first"` 区块明确源码落点与依赖前提（`component-circular_progress + inject-css + UiRoot`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `CircularProgress` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Coachmark 同步记录（2026-02-20）

- 参数模型同步：`Coachmark` 参数主轴保持 `variant/open + on_open_change + default_open/is_disabled`（兼容别名 `disabled`）、`current_step/total_steps`、`primary_cta/secondary_cta`、`asset_variant/asset_src`、`class_name/lang/dir`；默认值与来源继续由 `components/coachmark/src/logic.rs` 统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Coachmark", "coachmark", "Overlays", overlays_extra_coachmark::coachmark)` 暴露入口；`apps/docs-app/src/pages/components/pages/overlays_extra_coachmark.rs::coachmark()` 维持 `title="Coachmark"` 与 `slug="coachmark"` 可索引访问。
- 组件文档同步：`components/coachmark/src/README.md` 提供等价组件文档入口，保留 `Hello World（最小可用）` 与“先用起来，再进阶”路径，保证默认 API 前置。
- Source-first / Copy-Paste Ready：`overlays_extra_coachmark.rs` 通过 `code_signal + code_imports` 接入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，并通过 `data-slot="coachmark-source-first"` 暴露源码路径与依赖前提，避免复制即报错。
- 研究文档补充判定：本轮仅为 `Coachmark` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Chart 同步记录（2026-02-20）

- 参数模型同步：`Chart` 维持 display data-visual primitive 定位，参数主轴保持 `points/kind/active_index/default_active_index/on_active_index_change/is_disabled/is_show_grid/id_base/aria_label/class_name/on_action/lang/dir/motion`；默认值与来源标记继续由 `components/chart/src/logic.rs::normalize_input_boundary + derive_state_from_boundary` 统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 保持 `component_doc!("Chart", "chart", "Display", display_extra::chart)` 目录索引；`apps/docs-app/src/pages/components/pages/display_extra.rs::chart()` 维持 `title="Chart"` 与 `slug="chart"`，确保 `#/components/chart` 可索引访问。
- 组件文档同步：`components/chart/src/README.md` 提供等价组件文档入口，并保留 `Hello World（最小可用） -> 常见用法 -> 再进阶` 的默认优先路径。
- Source-first / Copy-Paste Ready：`display_extra.rs::chart()` 继续通过 `code_imports + apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；`data-slot="chart-source-paths"` 明确源码落点 `components/chart/src/{mod,logic,view,styles,motion}.rs` 与依赖前提（`component-chart + inject-css + UiRoot`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `Chart` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Carousel 同步记录（2026-02-20）

- 参数模型同步：`Carousel` 参数主轴保持 `selected_index/default_selected_index/on_selected_index_change`、`orientation/is_loop_navigation`、`aria_label/controls_aria_label/indicators_aria_label`、`motion/class_name/lang/dir`；默认值与来源标记继续由 `components/carousel/src/logic.rs` 统一归一，不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 保持 `component_doc!("Carousel", "carousel", "Collections", collections_command::carousel)` 目录索引；`apps/docs-app/src/pages/components/pages/collections_command.rs::carousel()` 维持 `title="Carousel"` 与 `slug="carousel"`，确保 `#/components/carousel` 可索引访问。
- 组件文档同步：`components/carousel/src/README.md` 提供等价组件文档入口，并保留 `Hello World（最小可用） -> 先用起来，再进阶` 的新手优先路径。
- Source-first / Copy-Paste Ready：`collections_command.rs::carousel()` 继续通过 `CAROUSEL_DOC_IMPORTS + apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；`data-slot="carousel-source-first"` 区块显式标注源码落点 `components/carousel/src/{mod,logic,view,styles,motion}.rs` 与依赖前提（`component-carousel + inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `Carousel` 参数语义与组件文档入口同步，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

### Collapsible 同步记录（2026-02-20）

- 参数模型同步：`Collapsible` 参数主轴保持 `open/default_open/on_open_change`、`is_disabled`（兼容别名 `disabled`）、`motion`、`aria_label/class_name/lang/dir`；默认值与来源标记继续由 `components/collapsible/src/logic.rs` 统一归一（`open > default_open > primitive fallback` 与 `is_disabled.unwrap_or(disabled)`），不在 `view.rs` 分散兜底。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Collapsible", "collapsible", "Collections", collections_groups::collapsible)` 暴露入口；`apps/docs-app/src/pages/components/pages/collections_groups.rs::collapsible()` 维持 `title="Collapsible"` 与 `slug="collapsible"` 可索引访问；`components/collapsible/src/README.md` 保持等价文档入口。
- 示例矩阵同步：`collections_groups.rs::collapsible()` 持续覆盖 `Hello World`、`Parameter Matrix`、`State Matrix`、`Controlled vs Uncontrolled Contrast`、`State + Source Markers`、`Streaming / Snapshot Contract`、`Interactive Playground` 与 `Source-first Starter (Copy-Paste Ready)`，参数语义与当前实现保持一致。
- Source-first / Copy-Paste Ready：`Source-first Starter` 继续通过 `code_signal + code_imports=collapsible_imports.clone()` 接入 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports；`data-slot="collapsible-source-paths"` 明确源码落点 `components/collapsible/src/{mod,logic,view,styles,motion}.rs` 与依赖前提（`component-collapsible + inject-css`），避免复制即报错。
- 研究文档补充判定：本轮仅为 `Collapsible` 参数语义与组件文档入口同步校验，不引入新的 Spectrum/HeroUI 风格结论，不需要追加 `docs/research/spectrum-heroui-style-interface-study.md`。
- HeroUI 对齐结论：参数语义若变更，必须先同步本策略文档与 docs/README 入口，再推进实现与清单勾选；仅代码更新无文档更新在接口变更场景下不允许合入。

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
