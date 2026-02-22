# Chip

`Chip` 是一个 tag/pill 展示组件，支持 `variant/size`、可选 dismiss 动作、以及稳定状态语义标记。

## API (Table)

| Prop | Type | Default |
| --- | --- | --- |
| `children` | `Children` | required |
| `variant` | `ChipVariant` (`Default`/`Accent`/`Danger`/`Outline`) | `Default` |
| `size` | `ChipSize` (`Sm`/`Md`/`Lg`) | `Md` |
| `is_disabled` | `bool` | `false` |
| `on_dismiss` | `Option<OnPress>` | `None` |
| `dismiss_aria_label` | `Option<String>` | `"Remove tag"` |
| `class_name` | `Option<String>` | `None` |

## docs-app 展示区（Display）

- 页面：`apps/docs-app/src/pages/components/pages/display.rs` 的 `chip()`
- 区块：`Interactive Playground (展示 / Config / Code / CSS Test)`
- 区块：`Comparison Matrix (Variant / Size / Disabled / Custom)`

## docs-app Config 区（Settings）

Interactive Playground 可调：

- `Variant`（default/accent/danger/outline）
- `Size`（sm/md/lg）
- `is_disabled`
- `Dismiss action`
- `Custom dismiss aria label`
- `Custom class_name`

## docs-app Code 区（Code）

`code_signal` 会根据当前配置实时生成可复制代码：

- 非默认参数按需输出
- 布尔参数统一 `is_*` 命名
- dismiss 场景会显式展示 `on_dismiss`

## docs-app CSS Test 区（CSS Test）

- `test_css_source`: `ui::chip::styles::CSS`
- `test_source_path`: `components/chip/src/styles.rs`
- `test_config_signal`: `ChipActualConfig`

## 多场景对比

Comparison Matrix 覆盖至少四种场景：

- Default / Static
- Accent / Removable
- Danger / Disabled
- Outline / Custom (aria + class)
