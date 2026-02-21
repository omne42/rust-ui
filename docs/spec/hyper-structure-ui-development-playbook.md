# Hyper-Structure UI 开发手册（AI Verified / Struct-First）

> 目标：把“Rust 类型系统 = AI 幻觉过滤器”落成可执行工程规范，用来指导 `ui-components` 后续所有组件开发。

## 1. North Star（核心原则）

1. **类型先于实现**：先设计 `struct/enum`，再写渲染逻辑。
2. **编译门禁即质量门禁**：AI 生成代码必须通过 `cargo check/test`，否则不允许合入。
3. **语义可反射**：组件不仅要“能显示”，还要“能被 Agent 理解”。
4. **零二开优先**：尽量通过官方参数覆盖高频业务能力，减少业务侧重复封装。

---

## 2. 分层架构（固定不变）

每个组件保持以下分层：

- `logic.rs`：默认值、输入归一、状态计算、class 组合、a11y 兜底
- `view.rs`：只负责渲染和事件绑定，不做复杂业务判断
- `motion.rs`：动效参数和 sanitize（防无效输入）
- `styles.rs`：状态 class / slot / data-* 对应样式
- `spec.rs`：**AI 友好的 Struct-First Builder API**（新增标准层）
- `mod.rs`：模块边界和导出

> 规则：`view.rs` 出现大量 `if/else` 分支时，优先下沉到 `logic.rs`。

---

## 3. Struct-First API 规范（给人和 AI 都友好）

## 3.1 目标形态

每个核心组件必须同时提供：

1. **原生 Leptos 组件 API**（兼容现有调用）
2. **Struct-First Builder API**（AI 优先调用）

例如 Button：

- 原生：`Button(...)`
- Struct-First：`ButtonSpec::new()...render()`

## 3.2 必备类型约束

每个组件的 `spec.rs` 至少包含：

- `*Intent`（语义意图）
- `*Action`（语义动作）
- `*Schema`（Agent 可读结构）
- `*Spec`（Builder 主入口）

命名示例：

- `ButtonIntent`
- `ButtonAction`
- `ButtonSchema`
- `ButtonSpec`

## 3.3 Builder 设计规则

- `new()` 提供可用默认值（必须可直接 `render()`）
- 每个 setter 单一职责（只改一个语义维度）
- 不暴露“半成品状态”（默认值必须完整）
- `render()` 内仅做参数映射，不做复杂业务逻辑

---

## 4. 语义反射规范（Semantic Reflection）

## 4.1 统一输出

所有可交互组件必须输出：

- `data-state`
- 核心行为态：`data-hovered/data-pressed/data-loading/...`
- **`data-ui-schema`**（机器可读 schema 字符串）

## 4.2 Schema 最小字段（v1）

- `element_id`
- `intent`
- `action_signature`
- `requires_confirmation`

> 建议：Schema 使用强类型 struct 生成，不允许手写 JSON 字符串拼接散落在业务代码中。

---

## 5. 类型化设计系统规范（Spectrum × HeroUI）

## 5.1 样式输入只能用语义参数

禁止把不存在的类名作为“自由文本”传给 AI 生成代码；优先使用：

- `variant / size / tone / radius / loading_placement`
- `start_content / end_content / icon_only`
- `class_name`（根）+ `class_names`（slot，后续逐步补齐）

## 5.2 Token 类型化（逐步推进）

设计 token 必须朝以下方向演进：

- `enum ColorToken`
- `enum SpaceToken`
- `enum RadiusToken`

目标：让 AI “只能选对的值”，而不是猜 class 名。

---

## 6. AI 开发闭环（必须执行）

每次 AI 生成代码后，按以下顺序执行：

1. `cargo check -p ui-components`
2. 相关组件单测（例如 Button：`cargo test -p ui-components --test button_semantics`）
3. 相关逻辑单测（例如：`cargo test -p ui-components button::logic::`）
4. 通过后再扩展到 docs/playground

如果失败：

- 先修类型错误（编译错误）
- 再修语义错误（测试失败）
- 不允许跳过失败门禁

---

## 7. 组件落地模板（DoD）

每新增或重构一个组件，必须满足：

- [ ] 保持 `logic/view/motion/styles/mod/spec` 分层
- [ ] 提供 `*Spec` Builder API
- [ ] 提供 `*Schema` 并输出 `data-ui-schema`
- [ ] 提供 `normalize_* / resolve_state / compose_class_name` 可测试函数
- [ ] 至少一组 `*_semantics.rs` 回归测试
- [ ] docs-app playground 提供可配置示例与可复制代码

---

## 8. 当前状态（2026-02-13）

Button 已完成第一轮落地，可作为模板：

- Struct-First：`crates/ui-components/src/button/spec.rs`
- 语义反射：`crates/ui-components/src/button/view.rs`（`data-ui-schema`）
- 逻辑归一：`crates/ui-components/src/button/logic.rs`
- 语义测试：`components/button/test/button_semantics.rs`

下一优先级建议：

1. `Select`（受控/非受控 + schema）
2. `Modal/Popover`（open 契约 + destructive 行为 schema）
3. `ActionButton`（与 ButtonSpec 对齐）

---

## 9. 代码审查 Checklist（PR 必看）

- 是否新增了 AI 友好的 `*Spec`？
- 是否把复杂判断留在 `logic.rs` 而不是 `view.rs`？
- 是否输出了可机器读取的 `data-ui-schema`？
- 是否存在“字符串魔法值”可改为 enum？
- 是否有对应 semantics 测试覆盖新增契约？

---

## 10. 与现有文档关系

- 参数风格与组件能力：`docs/spec/heroui-parameter-design-strategy.md`
- 本手册关注：**工程执行方法与 AI 开发门禁**

两者配合方式：

1. 先在参数策略文档定义“要提供什么能力”
2. 再按本手册定义“怎么实现并保证可持续演进”
