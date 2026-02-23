# LabeledValue

`LabeledValue` 是一个基于 `ui-state-primitives` + `ui-headless` + `ui-motion` 组合的标签-值展示组件。

## 目标 / 非目标 / 风险边界

- 目标：提供可访问、可测试、语义稳定的 label/value 展示基元。
- 非目标：不在组件层实现业务状态管理或全局主题/动效系统。
- 风险边界：跨层契约漂移（primitives/headless/motion）优先在对应层修复，不在 `view.rs` 临时补逻辑。

## Architecture Layers

- `ui-state-primitives/src/labeled_value.rs`：方向、语气、文本归一与状态来源标记。
- `ui-headless/src/labeled_value.rs`：A11y 语义契约（`role/aria/lang/dir` + `data-*`）。
- `logic.rs`：桥接 primitives，负责 class 组装。
- `view.rs`：Leptos 结构渲染与 headless 契约挂载。
- `motion.rs`：`LabeledValueMotion` 与 wasm attach（SSR no-op、reduced-motion 降级）。
- `styles.rs`：token-first 静态 CSS。
- `mod.rs`：公开最小稳定 API。

## API (Table)

### LabeledValue Props

| Prop | Type | Default |
| --- | --- | --- |
| `label` | `Option<String>` | `"Label"` |
| `value` | `Option<String>` | `"—"` |
| `description` | `Option<String>` | `None` |
| `orientation` | `LabeledValueOrientation` (`Stacked` / `Inline`) | `Stacked` |
| `tone` | `LabeledValueTone` (`Default` / `Subtle` / `Strong`) | `Default` |
| `aria_label` | `Option<String>` | `"Labeled value"` |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
| `motion` | `LabeledValueMotion` | `LabeledValueMotion::default()` |

### LabeledValue Events

| Event | Type | Default |
| --- | --- | --- |
| `N/A` | 展示型组件，无用户交互事件回调 | `-` |

## Hello World（最小可用）

```rust
<LabeledValue label="Project".to_string() value="Omne".to_string() />
```

## Interactive Playground（展示区）

### 展示区（Display）

- docs 页面：`apps/docs-app/src/pages/components/pages/display_extra.rs`
- Playground：
  - `Orientation + Tone`
  - `Description + Custom Aria/Class`
  - `Interactive Playground`
- `Interactive Playground` 默认同时渲染 1 个可调实例 + 2 个固定对照实例（Inline/Subtle、Stacked/Strong）。

### Config 区（Config）

- `Orientation`：`Stacked / Inline`
- `Tone`：`Default / Subtle / Strong`
- `Description` 开关
- `Custom aria_label` 开关
- `Custom class` 开关

### Code 区（Code）

- Workbench 会实时生成当前配置对应的 `LabeledValue` 代码片段，便于直接复制到业务代码。
- 代码内容与 `Config` 控件严格一一对应，避免“UI 状态与示例代码不一致”。

### CSS Test 区（CSS Test）

- `test_source_path` 指向：`components/labeled-value/src/styles.rs`
- 支持在 playground 内局部覆写 `:scope` CSS，验证 token、状态标记与样式分支。
- 显示 `Actual config`（实时配置快照），方便回归与排障。

## 多种不同情况下的对比显示

| 场景 | 关键输入 | 预期对比点 |
| --- | --- | --- |
| 默认基线 | `Stacked + Default` | 标签/值纵向排布，基础语义标记 |
| 信息密度 | `Inline + Subtle` | 横向排布、弱化语气 |
| 强强调态 | `Stacked + Strong + Description` | 强语气 + 描述文案状态 |
| 可访问覆盖 | `Custom aria_label` | `data-aria-source="custom"` |
| 样式覆盖 | `Custom class` | `data-class-source="custom"` + 自定义类分支 |

## Semantics and Accessibility

- 根节点挂载 `role="group"` 与 `aria-label`。
- 支持 `lang/dir`，不假设单语言单方向。
- 暴露稳定契约：`data-orientation`、`data-tone`、`data-state`、`data-label-source`、`data-value-source`、`data-aria-source`、`data-class-source`。

## Motion and Fallback

- wasm 下根据描述区显隐做轻量 keyframe 过渡。
- `prefers-reduced-motion` 或 `motion.enabled=false` 时跳过动画。
- non-wasm 路径为 no-op，确保 SSR/tooling 编译稳定。

## Docs / Test References

- docs page: `apps/docs-app/src/pages/components/pages/display_extra.rs` (`slug="labeled-value"`)
- semantics test: `components/labeled-value/tests/semantics.rs`
- state primitive test: `crates/ui-state-primitives/src/labeled_value.rs`
- headless test: `crates/ui-headless/src/labeled_value.rs`
