### 1. 架构与分层边界 (Architecture & Layering)
**核心原则：单向依赖，职责分离，环境隔离**
- [x] **Core 层纯净性**：Accordion 纯状态收敛放在 `logic.rs`，与 DOM 绑定分离；`ui-core` 保持无 Leptos/DOM 依赖（仓库级边界）。
- [x] **Headless 层抽象**：交互与 A11y 来自 `ui-headless`（`use_press/use_focus_ring/use_hover/use_roving_tabindex`），组件层不重复实现。
- [x] **Motion 层独立**：动效通过 `ui-motion::spring::SpringAnimator`；非 WASM 分支是 no-op，SSR 可编译。
- [x] **Theme 层解耦**：`styles.rs` 仅消费 `var(--ui-*)`，主题值由 `ui-theme` 提供。
- [x] **Component 层组合**：`ui-components::accordion` 组合 headless + motion + theme，公开 API 不泄露 `web-sys`。
- [x] **Tree Shaking 支持**：Accordion 可按源码模块引入；样式注册点明确在 `src/css.rs`。组件未引入额外全量注册反模式。
- [x] **全局注入机制**：Accordion 样式通过 `UiRoot`/`push_components_css` 注入链路生效（仓库级机制）。

### 2. 组件实现规范 (Component Implementation)
**标准结构：`logic` + `view` + `styles` + `motion`**
- [x] **逻辑归一 (`logic.rs`)**：默认值和状态归一化在 logic 层完成（`normalize_open_indices/resolve_state`）。
- [x] **视图渲染 (`view.rs`)**：view 仅负责结构与 hooks 挂载，关键状态决策委托 `logic.rs`。
- [x] **样式契约 (`styles.rs`)**：
    - 仅静态 CSS，使用 Token 变量。
    - 无十六进制硬编码颜色。
    - `view.rs` 无 `style=...` 内联样式。
    - 已在 `src/css.rs` 注册 `crate::accordion::styles::CSS`。
    - 未使用 Utility-First 作为组件内部契约。
- [x] **动效契约 (`motion.rs`)**：存在 `AccordionMotion` + `attach_indicator_motion` + `attach_panel_motion`，并新增 `prefers-reduced-motion` 降级分支。
- [x] **Spec 构建器 (`spec.rs`)**：该组件不需要额外 `spec.rs`（当前 API 复杂度不需要）。
- [x] **API 设计**：
    - 命名与语义对齐：`on_open_change/default_open_indices/selection_mode`。
    - 受控/非受控成对支持：`open_indices + on_open_change` / `default_open_indices`。

### 3. 交互与无障碍 (Interaction & A11y)
**默认可用，语义优先**
- [x] **Headless 集成**：已使用 headless hooks 组合按键、焦点、hover、roving 行为。
- [x] **ARIA 完备**：`aria-expanded`、`aria-controls`、`aria-labelledby`、`role="region"` 对齐。
- [x] **键盘交互**：支持 roving tabindex 与键盘触发（Enter/Space/方向键路径）。
- [x] **Focus Visible**：`data-focus-visible` 与焦点 ring class 联动。
- [x] **国际化 (I18n)**：触发器文案由 `labels` 外部传入，组件内部不硬编码业务文案。

### 4. AI 原生与可观测性 (AI-Native & Observability)
**面向 Agent 设计，机器可读**
- [x] **状态可观测**：稳定 `data-*`（root/item/trigger/panel）覆盖 open/disabled/mode/motion-source 等状态。
- [x] **Agent Contract**：`data-slot` + `data-*` 形成稳定机器可读契约，减少 DOM 猜测。
- [x] **类型约束**：`AccordionSelectionMode` 等类型约束输入空间，降低非法状态组合。
- [x] **流式能力**：Accordion 属即时交互组件，不承担 LLM 文本流输出；当前通过 signal 驱动的状态流满足组件级要求。
- [x] **调试友好**：语义标记和受控状态接口可直接用于状态观测与回放测试脚本。

### 5. 工程、性能与 SSR (Engineering & Performance)
**高标准交付，环境适应**
- [x] **SSR 兼容性**：
    - headless Web/SSR 互斥由 `ui-headless` 仓库级保障。
    - `motion.rs` 非 WASM 分支 no-op。
    - WASM API 仅在 `cfg(target_arch = "wasm32")` 内访问，SSR 路径无 `window/document` 依赖。
- [x] **异步抽象**：Accordion 不耦合特定 async runtime；序列化/追踪策略遵循仓库级约束。
- [x] **性能治理**：
    - 无循环中大对象无意义 clone。
    - 关键状态通过 `Signal/Memo` 细粒度更新。
    - 动效驱动提供清理逻辑（observer/spring stop）避免泄漏。
- [x] **开发体验 (DX)**：docs playground 可快速切换受控/单选/禁用状态矩阵，支持快速迭代验证。

### 6. 测试与文档 (Testing & Documentation)
**契约验证，文档即产品**
- [x] **语义测试**：`tests/accordion_semantics.rs` 覆盖 `data-*`/`aria-*`/动效契约/文档契约。
- [x] **单元测试**：`logic.rs` 与 `motion.rs` 均含 `#[cfg(test)]` 的状态与契约测试。
- [x] **E2E 测试**：已具备稳定语义选择器（`data-slot`/`data-*`）作为 Playwright/Cypress 钩子。
- [x] **文档完整性**：
    - docs-app 已有 Accordion 页面与交互 Playground。
    - 新增组件级文档 `src/accordion/README.md`。
    - `view.rs` 补充 public props rustdoc，公共 API 语义清晰。
