# Styling / CSS 规格（v0）

目标：在 **可发布（crate 用户零配置可用）** 与 **开发体验（快速微调样式）** 之间取得平衡，并保证分层与可维护性。

> TL;DR
> - 发布默认：组件 CSS 仍由 `ui` 统一注入（开箱即用）。
> - 覆盖策略：组件 CSS 注入在 `@layer ui`；应用侧样式（不分 layer，或放在更高优先级 layer）可直接覆盖，避免 `!important`/高 specificity。
> - 开发体验：不要在开发阶段频繁改 `styles.rs`；用应用侧热更新 CSS 覆盖来迭代，收敛后再回填到 `styles.rs`。
> - 体积策略：组件 CSS 注入应与组件 feature 同步裁剪（见 `docs/spec/tree_shaking.md`）。

## 样式孤岛防御（强制补充）

详见：`docs/spec/style_island_defense.md`。

本文件在样式孤岛场景下的硬约束补充如下：

- 组件 `styles.rs` 必须采用防御性变量链：`var(--ui-*, var(--ui-fallback-*))`。
- 禁止在组件层把 Hex/RGB/裸尺寸写成 fallback 终值。
- fallback 终值必须由 token 层统一产出（SSOT），不允许组件私有一套默认值。
- 注入层采用“默认注入 + 裸奔容错”：即使上层漏挂 `UiRoot`，组件仍应可读可用。

## Design Tokens 基线（强制）

`ui-theme` 是仓库内 **唯一** 的设计 token 与主题上下文层，负责：

- 主题三轴上下文：`system/color/scale`（`spectrum|express|spectrum-two` × `light|dark|oled` × `medium|large`）。
- Token 分类（可追溯、可审计）。
- 间距基线（`space-3xs/2xs/xs/sm/md/lg`）由 `ui-theme` 统一定义并输出 CSS 变量，组件只消费。
- 排版与 Overlay 基线（如 `--ui-font-size-100/150/200`、`--ui-overlay-panel-min-width`、`--ui-overlay-viewport-inset`、`--ui-overlay-enter-offset-y`）必须在 `ui-theme` 定义并输出变量。
- 组件视觉 token 分类（示例：Button 的 `layout` + `motion` token 在 `crates/ui-theme/src/tokens.rs` 定义，映射在 `theme.rs`，变量输出在 `css.rs`）。
- 输入类组件同理：`TextField` 的 motion token（如 `--ui-text-field-motion-duration` / `--ui-text-field-motion-easing`）也必须走 `tokens.rs -> theme.rs -> css.rs` 链路，组件侧仅消费变量。
- 标签组件同理：`Label` 的 motion token（如 `--ui-label-motion-color-duration` / `--ui-label-motion-weight-duration` / `--ui-label-motion-easing`）也必须走 `tokens.rs -> theme.rs -> css.rs` 链路，组件侧仅消费变量。
- 表单控件布局同理：`Checkbox` 的 layout token（如 `--ui-checkbox-size-*` / `--ui-checkbox-radius-*` / `--ui-checkbox-gap`）也必须走 `tokens.rs -> theme.rs -> css.rs` 链路，组件侧仅消费变量。
- 色彩预览控件同理：`ColorSwatch` 的 layout token（如 `--ui-color-swatch-size-*` / `--ui-color-swatch-radius-*` / `--ui-color-swatch-wide-multiplier`）也必须走 `tokens.rs -> theme.rs -> css.rs` 链路，组件侧仅消费变量。
- 复合表单容器同理：`CheckboxGroup` 的 layout/motion token（如 `--ui-checkbox-group-gap` / `--ui-checkbox-group-motion-duration`）也必须走 `tokens.rs -> theme.rs -> css.rs` 链路，组件侧仅消费变量。
- 文件投放区同理：`DropZone` 的 layout/motion token（如 `--ui-drop-zone-min-height` / `--ui-drop-zone-focus-outline-width`）也必须走 `tokens.rs -> theme.rs -> css.rs` 链路，组件侧仅消费变量。
- 翻转卡片同理：`FlipCard` 的 layout token（如 `--ui-flip-card-max-inline-size` / `--ui-flip-card-perspective`）也必须走 `tokens.rs -> theme.rs -> css.rs` 链路，组件侧仅消费变量。
- 颜色类控件同理：`ColorWheel` 的 layout/hue token（如 `--ui-color-wheel-size` / `--ui-color-wheel-track-thickness` / `--ui-color-wheel-hue-*`）也必须走 `tokens.rs -> theme.rs -> css.rs` 链路，组件侧仅消费变量。
- 视觉细粒度语义同理：`Separator` 的装饰透明度 token（`--ui-separator-decorative-opacity`）必须走 `tokens.rs -> theme.rs -> css.rs` 链路，组件 `styles.rs` 只消费变量。
- 三轴到 token 的映射（集中在一个地方做决策）。
- CSS 变量输出（组件只消费变量，不重建主题）。

**组件检查硬规则（必须执行）**：

- Token 统一基线落点固定：`crates/ui-theme/src/tokens.rs` 定义，`crates/ui-theme/src/theme.rs` 映射，`crates/ui-theme/src/css.rs` 输出变量；组件只在 `crates/ui/src/<component>/styles.rs` 消费。
- 三轴上下文（`system/color/scale`）在 `theme.rs` 定义；组件在 `logic.rs` 选择并在 `view.rs` 生效，`styles.rs` 只消费变量，不重建主题。
- Token 分类必须可追溯：分类源在 `tokens.rs`，规范同步本文件；组件不得引入平行私有 token 命名体系。
- 量化尺寸基准必须可回归：尺寸基准在 `tokens.rs` 与 `theme.rs` 定义，主题回归在 `crates/ui-theme/tests/token_scale_baseline.rs`，组件语义回归在 `components/*/test/*`。
- 主题调色与语义色对比必须满足 `WCAG 2.1 AA` 基线，并覆盖 Light/Dark/OLED 主题变体。
- 主题层只输出 `theme/tokens/base css` 与变量；不实现组件结构、交互逻辑、组件级动效编排。
- 新增视觉语义先补 token，再由组件消费；禁止“组件临时值先落地、后补 token”的倒序流程。

## 背景与现状

当前仓库的样式来源分三层：

1. **Theme tokens → CSS variables**：`ui-theme` 生成 `--ui-*`，由 `<UiRoot>` 注入（支持运行时切换主题）。
2. **组件样式（ui）**：每个组件的 `styles.rs` 产出 `pub const CSS: &str`，经 `ui/src/css.rs` 聚合，由 `<UiRoot>` 注入。
3. **应用布局（apps/*）**：页面布局/排版（grid/flex/响应式）由应用侧 CSS 决定（例如 `apps/web-demo/app.css`）。

这种设计对 crate 用户友好（只加依赖、用 `<UiRoot>` 即可），但对开发者有一个明显痛点：**改 `styles.rs` 必须重新编译（尤其 wasm）**，微调成本高。

## 关键困难（为什么“不直接改成静态资源”）

把 `ui` 的 CSS 变成静态资源（`*.css`）确实能带来更快的样式迭代，但会引入发布与集成成本：

- **对外部项目不透明**：使用者必须知道要额外引入某个 CSS 文件（Trunk/Vite/Webpack/Tauri/路径/顺序等）。
- **打包与版本同步**：CSS 文件与 crate 版本需要强绑定，否则容易出现“升级依赖但没升级样式文件”的错配。
- **注入顺序与覆盖规则**：`<UiRoot>` 注入的 `<style>` 往往出现在 document 末尾；为保证应用侧可覆盖，组件 CSS 默认注入在 `@layer ui`（低优先级层）。

因此：**发布默认不建议强制静态资源**；更现实的做法是同时满足“零配置发布”和“开发热更新”。

## 方案对比（思考）

### 方案 A：CSS 继续以 `styles.rs` 注入（当前）

优点：
- 对外部项目：零配置、最稳定。
- 对仓库内：分层清晰，主题 tokens 与组件 CSS 统一由 `<UiRoot>` 注入。

缺点：
- 开发时微调样式需要重新编译（尤其 wasm）。
- 应用侧想覆盖组件样式：应通过 layer 规则实现（推荐不分 layer 的覆盖样式），不要依赖 `!important`/高 specificity。

### 方案 B：组件 CSS 纯静态资源（`ui.css`）

优点：
- CSS 可以热更新；微调快。
- 可以接入 PostCSS、CSS Modules、Tailwind 等生态（如需要）。

缺点：
- 破坏开箱即用：对外部项目有额外集成要求。
- 打包/路径/版本/注入顺序增加复杂度（尤其多平台：Web + Tauri）。

### 方案 C：Hybrid（推荐）

核心：**默认保持注入以保证发布友好**，同时提供一条 **应用侧热更新覆盖** 的开发路径（必要时再提供 feature 关闭注入）。

## 推荐决策（解决方式）

### 1) 发布默认：保持注入（不强制静态资源）

- `ui-theme`：继续由 `<UiRoot>` 注入主题变量（运行时切换主题必需）。
- `ui`：组件 CSS 继续由 `<UiRoot>` 注入（crate 用户零配置可用）。
- 当启用组件级 feature 后，`ui/src/css.rs` 必须按 feature 条件聚合，仅注入启用组件的 CSS。

**可选：关闭组件 CSS 注入（高级用法）**

有些应用希望完全自己管理 CSS（静态文件 / PostCSS / Tailwind / 多入口打包等），这时可以关闭 `ui` 的内置 CSS 注入。

`ui` 默认启用 `inject-css` feature；禁用默认 feature 后，`<UiRoot>` 仍会注入 theme variables + base，但**不会**再注入组件 CSS：

```toml
ui = { path = "...", default-features = false }
```

### 2) 开发时：用应用侧 CSS 覆盖进行热更新迭代

建议工作流：

1. 在 `apps/*` 增加一个开发覆盖文件（例如 `dev-overrides.css`），通过 Trunk/Vite 等热更新机制加载。
2. 先在 `dev-overrides.css` 里快速试样式（不改 Rust，不触发 wasm 重新编译）。
3. 样式稳定后，再把规则回填到对应组件的 `styles.rs`，并删除/收敛 overrides。

> 注意：组件 CSS 默认注入在 `@layer ui`；应用侧 overrides **不分 layer**（或放在更高优先级 layer）即可覆盖，避免 `!important`/高 specificity。

### 3) 让覆盖更干净：CSS Cascade Layers（已采用）

为避免 `!important`/高 specificity，`ui` 注入的组件 CSS **默认包在** `@layer ui { ... }`：

- 组件 CSS 在 `@layer ui` 中（低优先级层）。
- 应用侧 overrides 推荐 **不放进任何 `@layer`**（未分层样式），即可自然覆盖 `@layer ui`（即使加载顺序更早）。

如果应用本身也使用 layers，推荐显式声明顺序并放入更高优先级层：

```css
@layer ui, app;

@layer app {
  /* overrides */
}
```

原则：`@layer ui` 只属于组件库；应用侧不要把自己的样式写进 `ui` layer（避免再次回到“比注入顺序/更高 specificity”来抢优先级）。

## Rules (Required)

- **Inline CSS is forbidden in components:**
  - `ui` must not use inline style for normal CSS properties.
  - Do not bind normal CSS properties via `style:<prop>=...`.
  - Style switching must use `class`/`data-*` + `styles.rs`.
- **Runtime values must use CSS variables (custom properties) only:**
  - When passing runtime values, use custom properties (`--*`).
  - Recommended: `style:--x=...`.
  - Allowed: `style=...` only when it contains **only** `--*` variable assignments.
- Quick violation check: search the repo for `style=` and `style:`.

## 规范（必须遵守）

> 该部分与 `docs/RULES_ZH.md` 保持一致；这里给出更具体的落地约束与建议。

- **组件禁止 inline CSS**：
  - `ui` 中禁止写“普通属性”的 inline style（`top/left/padding/background/...`）。
  - 允许传递运行时数值时使用 custom properties（`--*`）：
    - 推荐：`style:--x=...`（如果语法可用）
    - 允许：`style=...` 但内容必须 **只包含** `--*` 变量赋值（禁止普通属性）
  - 禁止使用 `style:<prop>=...` 绑定普通 CSS 属性（`padding/background/position/...` 等）。
- **组件样式必须集中**：
  - 所有样式 selector/声明必须位于组件的 `styles.rs`（`pub const CSS: &str`）。
  - `ui/src/css.rs` 负责聚合，最终由 `<UiRoot>` 统一注入。
  - 聚合必须具备组件级条件拼接能力；禁止无条件拼接全量组件 CSS。
- **状态表达优先 class/data-attrs**：
  - 离散状态（hover/pressed/selected/disabled/open 等）用 `class:` 或 `data-*`，由 `styles.rs` 控制样式。
- **运行时数值只允许用 CSS variables（custom properties）**：
  - 例如 popover 定位、motion 参数、测量结果等，只能通过 `--*` 变量传入，再在 CSS 中消费（如 `top: var(--ui-popover-top)`）。
  - 具体绑定方式依赖 Leptos attribute 语法（可用 `style:` 或 `attr:style` 等）；但语义必须是“只写变量，不写普通样式属性”。

## 迁移与验收建议（落地路线）

- 迁移顺序建议：`Overlay/Popover`（定位/portal）→ `ListBox/Menu/Select`（高频交互状态）→ 其它组件。
- 快速排查违规：在仓库中搜索 `style=` 与 `style:`（目标：`ui` 里不存在普通 inline style）。
- 统一收敛：当 overrides 稳定后，把规则回填 `styles.rs`，删除 `dev-overrides.css` 中对应内容，避免长期分叉。

补充（与 Tree Shaking 协同）：

- 最小特性集（例如 `component-button,component-input`）下，聚合 CSS 不应出现 `select/modal/chart` 的选择器。
- `inject-css` 仅控制“是否注入”，不应被用作“全量样式开关”。
