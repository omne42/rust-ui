# Spectrum × HeroUI 样式与接口综合学习（v0）

> 目标：把 Spectrum 的可访问性与状态语义、HeroUI 的可配置与易用接口，融合为 `rust-ui` 的统一设计规范。

## Spec Draft

### Goal

形成一套可执行的“统一样式与接口规范”，让 `ui` 既有 Spectrum 风格的稳定语义与可访问性，又有 HeroUI 风格的高可配置与低上手成本。

### Non-Goals

- 不追求 1:1 复刻 React Spectrum 或 HeroUI 的全部 API。
- 不在本阶段一次性重写全部组件。

### Constraints

- 现有工程分层必须保持：`logic.rs / view.rs / motion.rs / styles.rs / mod.rs`。
- 已发布组件优先向后兼容，新增能力不能破坏已有默认行为。
- Rust/Leptos 下参数设计要减少所有权复杂度，避免“参数越多越难维护”。

### Definition of Done

- [ ] 输出统一参数分层（视觉/状态/行为/a11y/内容/样式/motion）并写入仓库文档。
- [ ] 给出 Button、Select、Modal 三个组件的“目标参数表 + 命名规则 + 默认值规则”。
- [ ] 明确受控/非受控组合规范：`value + on_value_change` / `default_value`，`open + on_open_change` / `default_open`。
- [ ] 明确样式覆盖规范：根 `class_name` + 槽位级 `class_names`。
- [ ] 明确状态暴露规范：统一 `data-*`（hovered/pressed/selected/open/disabled/loading 等）。

### Options (2-3)

#### Option A: Spectrum-first（语义优先）

- **描述**：优先对齐 Spectrum/React Aria 的语义状态与 a11y 契约，配置能力保持适中。
- **数据结构**：
  - `struct CommonA11yProps { aria_label, aria_labelledby, aria_describedby, role }`
  - `struct CommonStateProps { is_disabled, is_invalid, is_readonly, is_required }`
  - `enum Controlled<T> { Controlled { value, on_change }, Uncontrolled { default } }`
- **Pros**：
  - 稳定、可测试、可访问性强。
  - 与你当前 `ui-headless` 方向高度一致。
- **Cons**：
  - 业务同学会觉得“可配但不够爽”。
  - 对样式个性化支持相对保守。
- **工作量**：中。

#### Option B: HeroUI-first（体验优先）

- **描述**：优先对齐 HeroUI 的高可配置调用体验（slots/classNames/内容插槽齐全）。
- **数据结构**：
  - `struct SlotClassNames { root, label, start, end, ... }`
  - `struct ContentSlots { start_content, end_content, ... }`
  - `struct VisualProps { variant, size, color, radius, full_width }`
- **Pros**：
  - API 友好、业务落地快。
  - 减少二次封装概率。
- **Cons**：
  - 参数膨胀风险高。
  - 若缺少统一约束，组件间一致性容易漂移。
- **工作量**：中高。

#### Option C: Hybrid（推荐）

- **描述**：底层采用 Spectrum 风格“语义与状态契约”，上层采用 HeroUI 风格“参数与样式覆盖能力”。
- **数据结构**：
  - `struct CommonSemanticProps { state, a11y, controlled }`
  - `struct CommonStylingProps { class_name, class_names, variant, size, tone }`
  - `struct CommonContentProps { start_content, end_content, icon, icon_only }`
  - `struct CommonMotionProps { motion, disable_animation }`
- **Pros**：
  - 同时满足“稳定性 + 易用性”。
  - 与当前仓库的 `logic`（归一）→`view`（渲染）分层天然兼容。
- **Cons**：
  - 需要先定义并强制执行统一规范。
- **工作量**：中高（但收益最大）。

### Recommendation

选 **Option C（Hybrid）**。

原因：

1. **Spectrum 提供底盘**：状态语义、可访问性、受控/非受控模型、`data-*` 回归测试都更稳。
2. **HeroUI 提供体验**：`start/end` 插槽、`is_icon_only`、`classNames` 等“开箱即用”能力能明显减少业务二开。
3. **符合现状**：你的组件库已经有大量 `data-*` 状态契约和 `logic.rs` 归一逻辑，继续往 Hybrid 走成本最低。

### Open Questions (max 2)

1. `class_names` 你希望采用“强类型 slot enum”还是“字符串 key map”（灵活但弱约束）？
2. 参数命名是否统一坚持 `snake_case`（Rust风格），并在文档示例中同步给出 TS 对应名？

---

## 一、Spectrum 与 HeroUI 的核心差异（样式 + 接口）

### 1) 样式哲学

- **Spectrum（React Aria / React Spectrum 生态）**：更强调“状态语义 + 可访问性优先 + 可测试状态标记”。
- **HeroUI**：更强调“可配置设计系统 + 插槽 + 快速定制主题/样式”。

### 2) 接口哲学

- **Spectrum**：接口倾向语义化和状态驱动，受控/非受控配对明确。
- **HeroUI**：接口倾向业务体验，视觉参数、内容插槽、样式覆盖参数更完整。

### 3) 融合结论

- 基础交互/语义按 Spectrum 思路统一。
- 组件参数表按 HeroUI 思路补齐高频能力。

---

## 二、统一参数分层规范（建议直接执行）

### A. Visual（视觉）

- `variant`, `size`, `color`, `radius`, `full_width`
- 规则：视觉参数只控制“外观”，不改变交互语义。

### B. State（状态）

- `is_disabled`, `is_loading`, `is_selected`, `is_open`, `is_invalid`, `is_readonly`, `is_required`
- 规则：状态字段统一 `is_*` 前缀。

### C. Behavior（行为）

- `on_press`, `on_open_change`, `on_value_change`, `on_selection_change`
- 规则：事件字段统一 `on_*`，避免组件自定义命名漂移。

### D. Controlled/Uncontrolled（受控模型）

- 值类型：`value + on_value_change` / `default_value`
- 开关类型：`open + on_open_change` / `default_open`
- 规则：不得只提供一半。

### E. Content/Slots（内容）

- `start_content`, `end_content`, `icon`, `is_icon_only`, `children`
- 规则：内容插槽与样式槽位一一对应。

### F. Styling Override（样式覆盖）

- 根：`class_name`
- 槽位：`class_names`
- 规则：`class_name` 只影响 root，子节点必须通过 `class_names` 覆盖。

### G. A11y（可访问性）

- `aria_label`, `aria_labelledby`, `aria_describedby`, `role`
- 规则：所有可交互组件都要有可追溯标签来源。

### H. Motion（动效）

- `motion`, `disable_animation`
- 规则：默认动效安全可控，支持 reduced-motion。

---

## 三、对 `Button` 的落地模板（下一步可直接扩到全库）

当前你已补齐：

- `is_icon_only`
- `full_width`
- `start_content`
- `end_content`
- `loading_placement`

建议下一轮继续补齐：

1. `class_names`（`root/label/start/end/spinner`）
2. `color` / `radius`（若设计 token 已稳定）
3. `disable_animation`
4. `is_pressed`（受控按压态，必要时）

---

## 四、治理规范（避免参数爆炸）

新增参数必须满足全部条件：

1. 至少两个业务场景复用。
2. 能映射到统一分层（Visual/State/Behavior/...）。
3. 有默认值与回归测试。
4. 不引入新的命名体系。

否则进入 recipe/组合层，不进核心组件 API。

---

## 五、执行顺序建议

1. `Button`：补 `class_names` + 文档矩阵。
2. `Select`：补齐受控/非受控 + 插槽 + 状态 data 契约。
3. `Modal/Popover`：统一 open/close 接口和 overlay 样式槽位。
4. 文档层：为每个组件提供“参数矩阵 + playground 全控件”。

---

## 参考资料（官方）

- React Aria Components — Button: https://react-spectrum.adobe.com/react-aria/Button.html
- React Aria Components — Select: https://react-spectrum.adobe.com/react-aria/Select.html
- HeroUI — Button: https://www.heroui.com/docs/components/button
- HeroUI — Custom Styles / classNames: https://www.heroui.com/docs/customization/custom-styles
- HeroUI — Select: https://www.heroui.com/docs/components/select
