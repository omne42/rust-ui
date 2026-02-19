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
- Source-first / Copy-Paste Ready：Skeleton playground 继续走 `apps/docs-app/src/playground.rs::compose_copy_ready_code` 自动补齐 imports，并为 skeleton/skeleton-group playground 标注 `test_source_path`（`crates/ui-components/src/skeleton/view.rs`、`crates/ui-components/src/skeleton/group/view.rs`）以保证源码可追溯。
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
- Copy-Paste Ready 同步：`Tag` playground 代码通过 `code_signal` 进入 `Playground`，由 `compose_copy_ready_code` 自动补齐 imports；`test_source_path` 指向 `crates/ui-components/src/tag/view.rs` 便于源码追溯。

### TagGroup 同步记录（2026-02-17）

- 参数模型同步：`TagGroup` 维持 collection primitive 定位，参数聚焦 `tags/disabled/on_remove/variant/size/id_base/label/description/error/invalid/required/aria_* /class_name/lang/dir`，继续保持 `on_remove` 驱动的显式组合语义。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("TagGroup", "tag-group", "Collections", collections::tag_group)` 暴露入口；`#/components/tag-group` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/collections.rs` 提供 `Hello World`、`Removable + State`、`Validation + Required`、`Disabled + Empty`，覆盖默认路径与状态矩阵。
- Copy-Paste Ready 同步：`TagGroup` playground 代码通过 `code_signal` 进入 `Playground`，由 `compose_copy_ready_code` 自动补齐 imports；`test_source_path` 指向 `crates/ui-components/src/tag/group/view.rs` 便于源码追溯。

### Swatch 同步记录（2026-02-17）

- 参数模型同步：`Swatch` 保持 display primitive 定位，参数聚焦 `color/label/size/border/rounding/shape/is_nothing/is_mixed_value/is_disabled/is_decorative/selected/default_selected/on_selected_change/lang/dir/class_name/motion`，维持统一 `is_* / on_* / default_*` 语义命名。
- docs 入口同步：`apps/docs-app/src/pages/components/pages.rs` 通过 `component_doc!("Swatch", "swatch", "Display", display_extra_swatch::swatch)` 暴露入口；`#/components/swatch` 可索引访问。
- 示例矩阵同步：`apps/docs-app/src/pages/components/pages/display_extra_swatch.rs` 提供 `Hello World`、`Size + Shape + Rounding`、`Mixed + Nothing + Disabled + Controlled`、`Custom Motion Contract`，覆盖默认路径、状态矩阵与受控示例。
- Copy-Paste Ready 同步：Swatch playground 代码通过 `code_signal` 进入 `Playground`，由 `compose_copy_ready_code` 自动补齐 imports；`test_source_path` 指向 `crates/ui-components/src/color/swatch_core/view.rs` 便于源码追溯。
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
