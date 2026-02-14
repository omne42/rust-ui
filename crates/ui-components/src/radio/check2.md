### 1. 架构与分层边界 (Architecture & Layering)
**核心原则：单向依赖，职责分离，环境隔离**
- [x] **Core 层纯净性**：`crates/ui-core` 仅包含纯状态原语（Toggle/Selection/List 等），**零依赖** `Leptos/DOM/web-sys`，确保逻辑可单测、可移植。
- [x] **Headless 层抽象**：`crates/ui-headless` 封装交互与 A11y（Press/Focus/Hover/Menu 等），输出 `attrs + handlers + state`。**严禁**包含样式或动效。
- [x] **Motion 层独立**：`crates/ui-motion` 实现动效引擎（Spring/WAAPI）。非 WASM 环境提供 no-op 实现，确保 SSR 可编译。
- [x] **Theme 层解耦**：`crates/ui-theme` 仅负责 Token 定义与 CSS 变量生成（Light/Dark/OLED），不包含组件具体 CSS。
- [ ] **Component 层组合**：`crates/ui-components` 作为最终组装层。对外 API **严禁暴露** `web-sys` 类型，DOM 细节封装在内部。
- [ ] **Tree Shaking 支持**：
    - Package 模式：`ui-components` 支持组件级 feature，按需编译。
    - Source 模式：源码拉取天然支持裁剪。
    - 样式裁剪：禁止无条件聚合所有 CSS，需随组件按需加载。
- [ ] **全局注入机制**：`UiRoot` (`src/root.rs`) 统一负责 Base CSS + Theme Vars + Component CSS 的注入。

### 2. 组件实现规范 (Component Implementation)
**标准结构：`logic` + `view` + `styles` + `motion`**
- [x] **逻辑归一 (`logic.rs`)**：负责 Props 归一化、状态派生、语义计算。默认值在此处处理，而非 View 层。
- [x] **视图渲染 (`view.rs`)**：纯 Leptos 结构渲染，挂载 Headless 提供的 `attrs/handlers`。禁止内联复杂逻辑。
- [ ] **样式契约 (`styles.rs`)**：
    - 仅包含静态 CSS 字符串，完全由 Token (`var(--ui-*)`) 驱动。
    - **禁止** 16进制颜色硬编码。
    - **禁止** `style="..."` 内联样式（仅允许通过 `style` 传递 `--*` 变量）。
    - 必须在 `src/css.rs` 中注册聚合。
    - 不使用 Utility-First（Tailwind）作为组件内部范式。
- [x] **动效契约 (`motion.rs`)**：定义组件专属的 `XxxMotion` 结构体及 `attach_motion` 方法。遵守 `prefers-reduced-motion`。
- [ ] **Spec 构建器 (`spec.rs`)**：(可选) 为复杂组件提供结构化构建入口（如 Button Spec）。
- [ ] **API 设计**：
    - 命名统一：`is_*` (状态), `on_*` (事件), `default_*` (默认值)。
    - 模式统一：受控 (`value` + `on_change`) 与非受控 (`default_value`) 成对出现。

### 3. 交互与无障碍 (Interaction & A11y)
**默认可用，语义优先**
- [x] **Headless 集成**：必须消费 `ui-headless` 的 Hooks，不得重复造轮子。
- [ ] **ARIA 完备**：正确暴露 `aria-*` 属性，`role` 定义准确。
- [x] **键盘交互**：支持 Tab 焦点管理、方向键导航（Roving Tabindex）、快捷键操作。
- [x] **Focus Visible**：正确处理 `data-focus-visible`，区分鼠标点击与键盘聚焦。
- [ ] **国际化 (I18n)**：存在 I18n/L10n 注入点，不硬编码文本。

### 4. AI 原生与可观测性 (AI-Native & Observability)
**面向 Agent 设计，机器可读**
- [x] **状态可观测**：使用稳定的 `data-*` 属性显式标记组件状态（Open/Selected/Loading）和来源。
- [ ] **Agent Contract**：语义标记 Schema 化，使 Agent 无需猜测 DOM 结构即可理解组件意图。
- [x] **类型约束**：利用 Rust 类型系统（Enum 等）限制输入空间，通过编译器反馈指导 AI 生成。
- [ ] **流式能力**：支持结构流（Config 分段）、状态流（校验中/可提交）、结果流（增量挂载）。流式输出必须可恢复、可验证。
- [ ] **调试友好**：
    - 开发模式提供可视化调试入口（信号/Props/State 观测）。
    - 关键交互事件可回放。
    - 避免依赖 `console.log` 堆砌，使用结构化追踪。

### 5. 工程、性能与 SSR (Engineering & Performance)
**高标准交付，环境适应**
- [ ] **SSR 兼容性**：
    - `ui-headless` 中 Web/SSR feature 互斥 (`compile_error!` 保护)。
    - 非 WASM 环境下 `ui-motion` 为 no-op。
    - 确保无 `window/document` 全局对象依赖，降级实现不 Panic。
- [ ] **异步抽象**：使用 `serde` 处理序列化，`tracing` 处理链路追踪。异步运行时（Tokio/Async-std）解耦。
- [ ] **性能治理**：
    - 无不必要的 `.clone()`。
    - 使用细粒度更新（Signals/Memos）而非整体重算。
    - 关键组件定义性能预算（内存/渲染耗时）。
- [ ] **开发体验 (DX)**：支持样式热重载（无需重编 WASM），提供 Workbench 模式隔离开发。

### 6. 测试与文档 (Testing & Documentation)
**契约验证，文档即产品**
- [x] **语义测试**：验证 `data-*` / `aria-*` 等语义契约，而不仅仅是视觉快照。
- [x] **单元测试**：`logic.rs` 中的纯逻辑/状态机必须有 `#[test]` 覆盖。
- [ ] **E2E 测试**：关键业务流纳入 Playwright/Cypress 回归集合，使用稳定语义选择器。
- [ ] **文档完整性**：
    - `docs-app` 包含对应文档页。
    - 包含至少一个可交互的 Playground 示例。
    - API 文档清晰，无暴露底层实现细节。
    - 每个组件都需要有heroui水平的单组件文档