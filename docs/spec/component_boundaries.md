# UI 组件边界：什么是组件，什么不是

> Status: Draft  
> Scope: 定义 `ui-components` 的职责边界，并规定跨组件基础设施应归属的层级

## 0. 结论先行（避免争吵）

**UI 组件**：对外暴露可渲染的 View（Leptos `#[component]` 或等价），并提供稳定、可测试的语义契约（`data-*` / `aria-*` / `role`），内部按 `logic/styles/motion/view` 组织来装配 `ui-headless`、`ui-theme`、`ui-motion`。

**不是 UI 组件**：任何“跨组件、跨页面、跨应用”的基础设施与开发工具，例如：

- i18n/l10n 注入基础设施（registry、上下文、格式化策略接口）
- trace/事件回放、调试观测面板（debug overlay）
- 性能预算/测量、探针与采样策略
- 流式 contract（分段配置/增量挂载/恢复协议）与其运行时
- E2E 测试运行器/框架适配（Playwright 配置、helpers、fixtures）
- 兼容层/上游命名对齐（Spectrum/RAC/S1 宏、story/test/utils 等）

这些东西放进 `ui-components` 的结果只有一个：**污染 API、破坏分层、让未来维护变成屎山**。

## 1. 分层归属（放错层就是 bug）

依赖方向（核心链路）：`ui-state-primitives -> ui-headless -> ui-components -> apps/*`；横向能力为 `ui-theme/ui-motion`（服务组件，不反向侵入状态与行为层）。

- `ui-state-primitives`：纯状态原语与不变量建模（与语言/DOM/运行时无关）。
- `ui-headless`：交互行为 + A11y 语义契约（输出 attrs/handlers/state），以及跨组件基础设施的**契约与注入点**（例如 i18n registry、trace/perf hooks）。
- `ui-theme`：token 与 CSS 变量生成，不含组件 CSS。
- `ui-motion`：动效运行时（可 wasm/no-op）。
- `ui-components`：最终“组装与渲染”。只做两件事：
  1) 把 headless 的语义正确挂到 DOM（`role/aria/data-*`）  
  2) 把 theme/motion 作为契约落地到 CSS/动画
- `apps/*`：验收面与回归面。调试 UI、E2E 资产、demo 字典、业务集成全部在这里。

## 2. i18n：不是“翻译系统”，是“文本来源契约”

基础组件需要 i18n/l10n 的原因非常现实：**可访问性**。

- 图标按钮/清除按钮没有 `aria-label` 就是不合格组件。
- 一些组件必须给出默认读屏文案，否则“开箱可用”是假的。

我们要做的不是在组件层塞一个“翻译引擎”，而是把默认文案的来源变成可注入的契约：

优先级（高→低）：
1. 调用方 props（最明确）
2. `UiRoot`/应用注入的 strings bundle（可全局覆盖）
3. 组件默认值（英文兜底，保证开箱可用）

关键约束：**禁止中心化“大字典”**。每组件 strings 类型分散定义，按需引入，避免单文件膨胀与合并冲突。

详见：`docs/spec/i18n.md`

## 3. 组件层的“可验证完成”标准（check2 的正确含义）

`check2.md` 里的 `[x]` 不是“看过了”，而是**有可执行证据**：

- 语义契约：`data-slot`/`data-state`/`aria-*` 稳定且有测试覆盖（单测或语义测试或 E2E）。
- 不硬编码不可替换的用户可见文案（有 props 覆盖或 strings 注入）。
- SSR/WASM 路径不 panic，motion 在非 wasm 下可降级。

不适用的条目必须明确写清楚 “为什么不适用、用什么机制替代保证正确性”，否则就是偷懒。
