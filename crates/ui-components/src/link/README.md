# Link

`Link` 是文本导航组件，统一使用 canonical API：`is_disabled`。

## 展示（Display）

docs-app 页面：`apps/docs-app/src/pages/components/pages/display.rs::link()`

对比场景：

| 场景 | 关键输入 | 预期状态 |
| --- | --- | --- |
| Internal | `href="#/docs/welcome"` | `data-state="enabled"` |
| External | `href="https://..." + target="_blank"` | `data-target="blank"` |
| Disabled | `is_disabled=true` | `data-state="disabled"` |
| Missing href | `href="   "` | `data-state="missing-href"` |

## config（Actual Config）

Playground 会实时输出：

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
}
```

## code（Copy/Paste）

```rust
<Link href="#/docs/welcome".to_string()>"Internal docs link"</Link>
<Link href="https://example.com".to_string() target="_blank">"External link"</Link>
<Link href="#/docs/welcome".to_string() is_disabled=true>"Disabled"</Link>
<Link href="   ".to_string()>"Missing href"</Link>
```

## css test（Scoped CSS Test）

- `test_source_path="crates/ui-components/src/link/styles.rs"`
- `test_css_source=ui_components::link::styles::CSS`

```css
:scope .ui-link[data-state="enabled"] {
  text-decoration-thickness: 3px;
}
```
