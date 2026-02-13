# 调研笔记（v0）

目标：用 Rust + Leptos 复刻 React Spectrum 的 3 层架构（Stately / Aria / Spectrum），并通过 Tauri 覆盖 Web、桌面（Win/Mac）与 Android(WebView)。

文档系统入口：`docs/README.md`  
文档索引：`docs/DOCS_INDEX.md`

> 说明：本目录文档属于 **Research 输入层**，用于提供背景、比较和参考材料。  
> 它们不是最终规范来源；规范与执行以 `docs/RULES_ZH.md`、`docs/spec/*`、`docs/plan/*` 为准。

实施计划：见 `docs/plan/IMPLEMENTATION_PLAN.md`；MVP 规格：`docs/spec/mvp.md`。

## 本机其它项目参考（设计/治理/文档）

- `bb/packages/ui-web` 的架构/色彩/文档演进笔记：`docs/research/bb_ui-web_notes.md`

## Upstream 参考仓库（本地 clone）

这些仓库被 clone 到 `examples/_upstream/`，并且整个 `examples/` 已在 `.gitignore` 中忽略（用于本地调研，不进版本库）。

- Adobe React Spectrum（monorepo，包含 react-aria / react-stately / react-spectrum）：`examples/_upstream/adobe-react-spectrum`
- Adobe Spectrum CSS：`examples/_upstream/adobe-spectrum-css`
- Adobe Spectrum Web Components：`examples/_upstream/adobe-spectrum-web-components`
- React：`examples/_upstream/facebook-react`
- Vue core：`examples/_upstream/vue-core`
- Leptos：`examples/_upstream/leptos`
- Tauri：`examples/_upstream/tauri`
- leptos-use：`examples/_upstream/leptos-use`
- Motion（Framer Motion）：`examples/_upstream/motion`
- HeroUI：`examples/_upstream/heroui`
- shadcn/ui：`examples/_upstream/shadcn-ui`
- animate-ui：`examples/_upstream/animate-ui`
- WAI-ARIA Authoring Practices：`examples/_upstream/w3c-aria-practices`

## React Spectrum 关键文件定位（第一批）

### 交互标准化（Press / Modality / Focus Visible）

- Press（跨 mouse/touch/pointer/keyboard 的统一“按压”语义）：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/interactions/src/usePress.ts`
- PressResponder（上下文合并与嵌套）：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/interactions/src/PressResponder.tsx`
- Interaction modality / Focus visible（键盘显示焦点环、鼠标不显示）：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/interactions/src/useFocusVisible.ts`
- Focus ring hook（消费 focus-visible，并返回 focusProps + 状态）：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/focus/src/useFocusRing.ts`

### Button（组合 Press + Focusable + ARIA）

- Button hook：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/button/src/useButton.ts`

### Overlay（弹层/模态基础设施）

- Overlay 基础：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/overlays/src/useOverlay.ts`
- Trigger：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/overlays/src/useOverlayTrigger.ts`
- Modal overlay：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/overlays/src/useModalOverlay.ts`
- Overlay 容器/Portal：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/overlays/src/Overlay.tsx`
- Overlay Provider（aria-hidden 管理）：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/overlays/src/useModal.tsx`
- Overlay 定位：`examples/_upstream/adobe-react-spectrum/packages/@react-aria/overlays/src/useOverlayPosition.ts`

### Stately（纯状态）

- Toggle state：`examples/_upstream/adobe-react-spectrum/packages/@react-stately/toggle/src/useToggleState.ts`
- Overlay trigger state：`examples/_upstream/adobe-react-spectrum/packages/@react-stately/overlays/src/useOverlayTriggerState.ts`

## Leptos / leptos-use / Tauri 对应点（第一批）

- NodeRef：`examples/_upstream/leptos/tachys/src/reactive_graph/node_ref.rs`
- DOM 事件绑定：`examples/_upstream/leptos/tachys/src/html/event.rs`
- Portal（Overlay/弹层渲染入口）：`examples/_upstream/leptos/leptos/src/portal.rs`
- Context（全局 modality / overlay stack）：`examples/_upstream/leptos/leptos/src/provider.rs`
- 通用事件监听与自动清理：`examples/_upstream/leptos-use/src/use_event_listener.rs`
- Click outside（Overlay dismiss）：`examples/_upstream/leptos-use/src/on_click_outside.rs`

## Rust/Leptos 分层映射（v0 草案）

> 先落地最小切片：`Press + FocusVisible + Button`，再进入 Overlay。

- ui-core（React Stately）
  - `use_toggle_state`：纯状态与受控/非受控模式（类比 `useToggleState`）。
- ui-headless（React Aria）
  - `use_press`：统一 pointer/mouse/touch/keyboard，产出 `is_pressed` + 一组事件处理器/属性。
  - `use_focus_visible`（全局）：监听 `keydown/pointerdown/click/focus/blur` 推断 modality，并在 Context 中暴露 `is_focus_visible`。
  - `use_button`：组合 `use_press + focusable + aria-*`，并处理非 `<button>` 元素的键盘触发与 `aria-disabled`。
- ui-theme（设计系统）
  - 以 Spectrum tokens/CSS 为输入，输出 CSS variables（优先）或 Tailwind preset（后续）。
- ui-components（最终组件）
  - Button v0：对接 headless，提供 Spectrum 风格 class/tokens。

## MVP 建议（避免过早过度工程）

- 先锁定 1 个核心用户流程（例如：打开应用 → 点击/输入 → 展示结果）。
- MVP 只要求键盘 + 鼠标可用；触控与 Android 特性（ripple/haptics/safe-area）延期到第二阶段。
- Overlay 先做“单一弹层 + Esc 关闭 + 点击外部关闭”的 v1，第二个真实用例再抽象框架。

## 任务 DAG（v0，摘要）

1. `t1-workspace`：workspace + `ui-core/ui-headless/ui-components/ui-theme/demo-tauri` crate 壳（`cargo check -p ...` 全过）
2. `t2-headless-interaction`：最小交互系统（Press + FocusVisible）
3. `t3-theme-tokens`：最小 tokens（颜色/圆角/间距…）
4. `t4-components-button`：Button v0（组合 t2+t3）
5. `t5-overlay-v1`：Overlay v1（最小 dismiss/stack）
6. `t6-tauri-demo`：Tauri demo 骨架（引用 Button/Overlay）
