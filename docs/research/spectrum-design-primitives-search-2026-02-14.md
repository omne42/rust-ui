# Spectrum 设计元语 Search 调研记录（2026-02-14）

> 范围：记录本轮关于 “Spectrum 是否有可落地的设计样式基准” 的快速检索结果，作为 `docs/research/*` 输入材料，不直接覆盖 `spec` 与 `rules`。

## 调研问题

1. Spectrum 是否存在明确的样式基准，而不是仅有视觉理念？
2. 这些基准是否能映射到本仓库分层（`ui-core/ui-headless/ui-theme/ui-motion/ui-components`）？

## 核心结论

Spectrum 存在可执行的样式基准，核心在 token 与上下文轴，而不是单一组件样式表。

可归纳为以下五点：

1. `Design Tokens` 是统一样式基线。
2. 存在三条上下文轴：`system`（如 `spectrum` / `express` / `spectrum-two`）、`color`（`light` / `dark`）、`scale`（`medium` / `large`）。
3. Token 分类完整，至少覆盖颜色、排版、布局、图标与组件级别 token。
4. 有可量化尺寸基准（示例）：`font-size-200` 在 `medium=16px`、`large=19px`；`component-height-100` 在 `medium=32px`、`large=40px`。
5. 可访问性是硬约束：颜色对比需满足 `WCAG 2.1 AA`，并支持 `lang` 与 `dir`（LTR/RTL）。

## 对 rust-ui 的意义

结论修正：应当全量学习 Spectrum 的设计基准，但不做外部系统的全量搬运。

更准确的策略是：

1. 全量学习：语义状态、A11y 契约、token 体系、命名与不变量。
2. 分层落地：按 `ui-core -> ui-headless -> ui-theme -> ui-motion -> ui-components` 单向映射。
3. 分批迁移：按组件切片推进，不做一次性重写。

建议分层映射如下：

1. `ui-core`：定义稳定语义元语（如 `ColorRole`、`Density`、`InteractionState`、`MotionPreset`）。
2. `ui-headless`：把交互/A11y 状态绑定到元语状态机，避免语义漂移。
3. `ui-theme`：将 “语义角色 -> token/CSS 变量” 固化为单向映射。
4. `ui-motion`：以语义动效（enter/exit/emphasis）消费元语，而非散落时长参数。
5. `ui-components`：只做组装，不再重复发明状态词典。

## 硬约束基线（需进入规范层）

以下 5 条应从调研结论升级为 `spec` 级硬约束：

1. `Design Tokens` 是统一样式真相源，禁止组件层硬编码样式值。
2. 三条上下文轴 `system/color/scale` 必须显式建模，不使用自由字符串。
3. Token 分类至少覆盖颜色、排版、布局、图标、组件级 token，并保持命名空间稳定。
4. 关键尺寸基准必须可量化、可回归验证（如 `font-size-200`、`component-height-100`）。
5. A11y 是合并门禁：满足 `WCAG 2.1 AA`，并支持 `lang` 与 `dir`（LTR/RTL）。

## 风险与边界

1. 最大风险不是缺功能，而是多层语义不一致（`headless/theme/components` 各自命名）。
2. 若在 `ui-components` 直接硬编码 token，会破坏分层并提高回归成本。
3. 若元语数量失控，会把简单问题抽象成复杂框架。

## 落地建议（全量学习，分层落地）

1. 在 `ui-core` 建立语义词典与状态 `enum`，先消灭松散 `Option<bool>` 组合状态。
2. 在 `ui-theme` 固化 token 映射矩阵（覆盖 `system/color/scale`）。
3. 在 `ui-headless` 与 `ui-motion` 对齐状态语义与 reduced-motion 契约。
4. 在 `ui-components` 仅消费语义，不新增私有状态命名。
5. 为 token、A11y、状态映射建立契约测试，作为 CI 门禁。

## 检索来源

- Spectrum Web Components Theme Tool  
  https://opensource.adobe.com/spectrum-web-components/tools/theme/
- Spectrum Web Components Styles Tool  
  https://opensource.adobe.com/spectrum-web-components/tools/styles/
- Spectrum 2 Tokens Viewer  
  https://opensource.adobe.com/spectrum-design-data/s2-tokens-viewer/
- Spectrum CSS（GitHub）  
  https://github.com/adobe/spectrum-css
- Adobe Design: Naming colors in design systems  
  https://adobe.design/stories/design-for-scale/naming-colors-in-design-systems
