# Link

`Link` 是文本链接组件，使用 `ui-headless` 的 hover/focus 语义，输出稳定 `data-*` 状态标记，并支持 `is_disabled` 主 API + `disabled` 兼容别名。

## 展示（Display）

docs-app 页面：`apps/docs-app/src/pages/components/pages/display.rs::link()`

对比场景（同页可见）：

| 场景 | 关键输入 | 预期状态 |
| --- | --- | --- |
| Internal | `href="#/docs/welcome"` | `data-state="enabled"` |
| External | `href="https://..." + target="_blank"` | `data-target="blank"` + 自动安全 `rel` |
| Disabled (`is_*`) | `is_disabled=true` | `data-state="disabled"` |
| Disabled (legacy) | `disabled=true` | `data-disabled-source="legacy-alias"` |
| Missing href | `href="   "` | `data-state="missing-href"` |

## config（Actual Config）

docs-app 的 `Interactive Playground (展示 / Config / Code / CSS Test)` 会实时输出配置快照：

```text
LinkActualConfig {
  href: "#/docs/welcome",
  has_href: true,
  is_disabled: false,
  disabled_source: "default",
  target: "self",
  rel: None,
  rel_source: "auto",
  data_state: "enabled",
  class: "ui-link ui-link--enabled ui-link--rel-auto",
}
```

## code（Copy/Paste）

最小可用：

```rust
<Link href="#/docs/welcome".to_string()>"Internal docs link"</Link>
```

状态对比：

```rust
<Link href="https://example.com".to_string() target="_blank">"External"</Link>
<Link href="#/docs/welcome".to_string() is_disabled=true>"Disabled (is_*)"</Link>
<Link href="#/docs/welcome".to_string() disabled=true>"Disabled (legacy)"</Link>
<Link href="   ".to_string()>"Missing href"</Link>
```

## css test（Scoped CSS Test）

docs-app playground 已接入：

- `test_source_path="crates/ui-components/src/link/styles.rs"`
- `test_css_source=ui_components::link::styles::CSS`

可在测试面板直接写局部样式（推荐 `:scope`）：

```css
:scope .ui-link[data-state="enabled"] {
  text-decoration-thickness: 3px;
}
```

## API 快速表

| Prop | Type | 默认值 |
| --- | --- | --- |
| `href` | `String` | 必填（空白会归一为 missing-href） |
| `is_disabled` | `Option<bool>` | `None`（默认 `false`） |
| `disabled` | `Option<bool>` | `None`（兼容别名） |
| `target` | `Option<&'static str>` | `None` |
| `rel` | `Option<String>` | `None` |
| `aria_label` | `Option<String>` | `None` |
| `class_name` | `Option<String>` | `None` |
| `lang` | `Option<String>` | `None` |
| `dir` | `Option<A11yDirection>` | `None` |
