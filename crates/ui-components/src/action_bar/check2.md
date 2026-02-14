### 1. 架构与分层边界 (Architecture & Layering)
**核心原则：单向依赖，职责分离，环境隔离**
- [x] **Core 层纯净性**：`crates/ui-state-primitives` 仅包含纯状态原语（Toggle/Selection/List 等），**零依赖** `Leptos/DOM/web-sys`，确保逻辑可单测、可移植。
- [x] **Headless 层抽象**：`crates/ui-headless` 封装交互与 A11y（Press/Focus/Hover/Menu 等），输出 `attrs + handlers + state`。**严禁**包含样式或动效。
- [x] **Motion 层独立**：`crates/ui-motion` 实现动效引擎（Spring/WAAPI）。非 WASM 环境提供 no-op 实现，确保 SSR 可编译。
- [x] **Theme 层解耦**：`crates/ui-theme` 仅负责 Token 定义与 CSS 变量生成（Light/Dark/OLED），不包含组件具体 CSS。
- [x] **Component 层组合**：`crates/ui-components` 作为最终组装层。对外 API **严禁暴露** `web-sys` 类型，DOM 细节封装在内部。
- [x] **Tree Shaking 支持**：
    - Package 模式：`ui-components` 支持组件级 feature，按需编译。
    - Source 模式：源码拉取天然支持裁剪。
    - 样式裁剪：禁止无条件聚合所有 CSS，需随组件按需加载。
- [x] **全局注入机制**：`UiRoot` (`src/root.rs`) 统一负责 Base CSS + Theme Vars + Component CSS 的注入。

### 2. 组件实现规范 (Component Implementation)
**标准结构：`logic` + `view` + `styles` + `motion`**
- [x] **逻辑归一 (`logic.rs`)**：负责 Props 归一化、状态派生、语义计算。默认值在此处处理，而非 View 层。
- [x] **视图渲染 (`view.rs`)**：纯 Leptos 结构渲染，挂载 Headless 提供的 `attrs/handlers`。禁止内联复杂逻辑。
- [x] **样式契约 (`styles.rs`)**：
    - 仅包含静态 CSS 字符串，完全由 Token (`var(--ui-*)`) 驱动。
    - **禁止** 16进制颜色硬编码。
    - **禁止** `style="..."` 内联样式（仅允许通过 `style` 传递 `--*` 变量）。
    - 必须在 `src/css.rs` 中注册聚合。
    - 不使用 Utility-First（Tailwind）作为组件内部范式。
- [x] **动效契约 (`motion.rs`)**：定义组件专属的 `XxxMotion` 结构体及 `attach_motion` 方法。遵守 `prefers-reduced-motion`。
- [x] **Spec 构建器 (`spec.rs`)**：(N/A) ActionBar 无复杂构建需求，直接 props 组合即可覆盖目标场景。
- [x] **API 设计**（已审计）：
    - 命名统一：`is_*` (状态), `on_*` (事件), `default_*` (默认值)。
    - 模式统一：受控 (`value` + `on_change`) 与非受控 (`default_value`) 成对出现。

### 3. 交互与无障碍 (Interaction & A11y)
**默认可用，语义优先**
- [x] **Headless 集成**：消费 `ui-headless` hooks（clear action 按钮：press/hover/focus）。
- [x] **ARIA 完备**：`role="toolbar"` + `aria-label` + `aria-hidden`，clear action 提供可覆盖的 `aria-label`。
- [x] **键盘交互**：clear action 按钮支持 Enter/Space 触发（headless press）。
- [x] **Focus Visible**：clear action 输出 `data-focus-visible`，并保留 `:focus-visible` 样式契约。
- [x] **国际化 (I18n)**：通过 `ActionBarStrings` + `UiRoot i18n` 注入兜底文案，props 仍可逐个覆盖。

### 4. AI 原生与可观测性 (AI-Native & Observability)
**面向 Agent 设计，机器可读**
- [x] **状态可观测**：稳定 `data-*` 输出（state/position/selection/count/sources）。
- [x] **Agent Contract**：`data-slot`/`data-state`/`data-selection` 等 schema 化选择器足够稳定用于 E2E/Agent。
- [x] **类型约束**：位置/阶段/选择类型使用 enum 建模，避免裸字符串。
- [x] **流式能力**：(N/A) ActionBar 不承担流式 contract；该能力应落在 `ui-headless`/`ui-state-primitives` 与 app 层。
- [x] **调试友好**：(N/A) 调试观测属于 app 层（docs-app debug overlay）；本组件提供可观测 data-attr 作为最低保障。

### 5. 工程、性能与 SSR (Engineering & Performance)
**高标准交付，环境适应**
- [x] **SSR 兼容性**：
    - `ui-headless` 中 Web/SSR feature 互斥 (`compile_error!` 保护)。
    - 非 WASM 环境下 `ui-motion` 为 no-op。
    - 确保无 `window/document` 全局对象依赖，降级实现不 Panic。
- [x] **异步抽象**：(N/A) ActionBar 不涉及异步边界；跨组件链路追踪在 `ui-headless` 提供注入点。
- [x] **性能治理**（已审计）：
    - 无不必要的 `.clone()`。
    - 使用细粒度更新（Signals/Memos）而非整体重算。
    - 关键组件定义性能预算（内存/渲染耗时）。
- [x] **开发体验 (DX)**：(N/A) DX 由 apps 层脚手架提供；组件仅输出稳定契约与可注入参数。

### 6. 测试与文档 (Testing & Documentation)
**契约验证，文档即产品**
- [x] **语义测试**：验证 `data-*` / `aria-*` 等语义契约，而不仅仅是视觉快照。
- [x] **单元测试**：`logic.rs` 中的纯逻辑/状态机必须有 `#[test]` 覆盖。
- [x] **E2E 测试**：Playwright 覆盖可见性切换与 clear selection 键盘触发（语义选择器）。
- [x] **文档完整性**：
    - `docs-app` 包含对应文档页。
    - 包含至少一个可交互的 Playground 示例。
    - API 文档清晰，无暴露底层实现细节。
    - 每个组件都需要有heroui水平的单组件文档
