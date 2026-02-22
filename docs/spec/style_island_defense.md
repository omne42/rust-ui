# 样式孤岛防御规范（Defensive Variables + Lazy Injection）

目标：让组件在“未正确挂载主题提供者”的情况下仍可读、可用、可回退；同时保持设计系统的单一真相源（SSOT）。

## 1. 问题定义

典型样式孤岛问题：

- 应用忘记挂载 `UiRoot` 或主题变量未注入。
- 组件直接读取 `var(--ui-*)`，变量缺失后出现“透明/错位/不可读”。
- 临时修补常用硬编码 Hex，长期导致 fallback 值在组件内四散，品牌升级成本爆炸。

结论：不能把“主题注入成功”当作组件可用性的前提。

## 2. 设计决策（本规范结论）

### 2.1 组件层：防御性变量链（必须）

组件样式必须采用两段回退链，而不是直接写终值：

```css
.ui-button {
  background: var(--ui-button-bg, var(--ui-fallback-primary));
  border-radius: var(--ui-button-radius, var(--ui-fallback-radius-md));
}
```

含义：

- 第一层：消费语义组件变量（可被主题/上下文覆盖）。
- 第二层：回退到全局 fallback 变量（保证裸奔可用）。

禁止在组件 `styles.rs` 直接写 `#007bff`、`4px` 这类硬编码终值。

### 2.2 Token 层：Fallback SSOT（必须）

fallback 的“终值真相源”必须集中，不允许分散到组件。

建议采用轻量 `ui-design-tokens`（无运行时逻辑，常量库）：

```rust
pub const FALLBACK_PRIMARY: &str = "#007bff";
pub const FALLBACK_RADIUS_MD: &str = "4px";
```

再由主题层统一输出：

```css
:root {
  --ui-fallback-primary: #007bff;
  --ui-fallback-radius-md: 4px;
}
```

品牌升级只改一处，不改 50 个组件。

### 2.3 注入层：分层与懒加载注入（必须）

- 默认路径：`UiRoot` 注入 `theme vars + components css + base css`。
- 裸奔容错：即使未注入主题 vars，组件仍通过 fallback 变量链可读。
- 非 `UiRoot` 场景：允许提供“仅注入变量”的最小注入入口（lazy injection / once），但不改变组件对变量命名协议的依赖。

## 3. 与 Spectrum / HeroUI 的对齐

- 学习 Spectrum：token 是样式真相源，组件只消费语义变量。
- 学习 HeroUI：允许局部覆盖与组合，但不破坏变量协议。
- 本仓库策略：`token SSOT + defensive fallback + layer override` 三者同时成立。

## 4. 规范约束（Required）

- `ui/src/**/styles.rs` 中：
  - 必须优先使用 `var(--ui-*)`。
  - 必须为关键视觉属性提供 fallback 变量链（至少背景/前景/边框/圆角/关键排版）。
  - 禁止直接写颜色 Hex/RGB 与裸尺寸终值作为 fallback 终点。
- fallback 终值必须由 token 层统一生成，不允许组件自定义私有终值。
- 主题层变量命名协议必须稳定：`--ui-*`（语义变量）+ `--ui-fallback-*`（兜底变量）。

## 5. 落地建议（迁移顺序）

1. 先在 `ui-theme` 增加 `--ui-fallback-*` 变量输出（由 token 常量生成）。
2. 组件分批改造为双层变量链：`var(--ui-*, var(--ui-fallback-*))`。
3. 增加契约测试：
   - 检查组件样式是否存在关键属性 fallback 链。
   - 阻断新引入硬编码颜色/尺寸终值。

## 6. 验收命令（建议）

```bash
# 样式与契约回归
cargo test -p ui --test style_rules --no-default-features --features inject-css

# 主题变量输出回归
cargo test -p ui-theme
```

## 7. 与现有规范关系

- 本文是 `docs/spec/styling.md` 的补充规范。
- 若本文与历史文档冲突，以“SSOT + 防御性变量链 + 禁止组件硬编码终值”为准。
