# AI 上下文压缩协议（Manifest + RBI）

> Status: Draft  
> Scope: 为 AI 协作提供“低成本可读索引”，避免在细粒度分层代码库中因上下文过载导致幻觉与错误组装。

## 0. 核心判断

这个问题是真问题，值得做。  
组件分层越细，AI 组装一个业务能力时就越容易被上下文长度击穿。  
没有索引层，AI 只能“扫源码+猜接口”，最终就是幻觉。

## 1. 问题定义（为什么会溢出）

在 `logic/view/styles/motion/mod` 的细分架构下，一个业务场景通常要跨多个组件协作：

- Dropdown（5+ 文件）
- Input（5+ 文件）
- List（5+ 文件）
- Popover（5+ 文件）
- 以及 `ui-headless` / `ui-state-primitives` 的契约

AI 若直接读完整源码，读取量会快速膨胀；超出上下文预算后会遗忘关键接口细节（如事件名、能力边界、slot 约束），从而开始“编造正确性”。

## 2. 协议设计（类型系统作为压缩算法）

### 2.1 `Component.toml`（组件清单）

每个组件必须提供机器可读清单文件，描述“能做什么”，而不是“怎么实现”。  
建议路径：`crates/<pkg>/src/<component>/Component.toml`。

最小字段：

- `schema_version`
- `component.name`
- `inputs`（props 名称、类型、默认值、是否受控）
- `outputs`（事件/回调契约）
- `slots`（可插槽区域与约束）
- `capabilities`（focus/keyboard/overlay/async 等）
- `dependencies`（依赖的 headless/theme/motion 能力）

示例：

```toml
schema_version = "1"

[component]
name = "Button"
crate = "ui-components"

[[inputs]]
name = "size"
ty = "ButtonSize"
default = "md"

[[outputs]]
name = "on_press"
ty = "Callback<()>"

[[capabilities]]
name = "keyboard_activation"
enabled = true
```

### 2.2 `.rbi`（Rust Interface Projection）

每个组件必须提供“仅签名、无实现”的接口投影文件。  
建议路径：`crates/<pkg>/src/<component>/<component>.rbi`。

要求：

- 只保留公共类型与函数签名（`struct/enum/type fn`）
- 不包含逻辑实现、样式细节、平台分支细节
- 可由脚本从真实源码提取，禁止长期手写漂移

示例：

```rust
pub struct ButtonProps {
    pub size: ButtonSize,
    pub disabled: bool,
}

pub fn Button(props: ButtonProps) -> impl IntoView;
```

## 3. AI 读取协议（先索引，后源码）

默认流程：

1. 先读 `Component.toml + .rbi`（理解接口）
2. 仅当需要改内部逻辑时再读 `logic.rs/view.rs/styles.rs/motion.rs`
3. 仅当跨组件改动时再增量读取依赖组件的索引文件

禁止默认“全量扫源码”。

## 4. 漂移控制（没有门禁就等于没有协议）

必须建立两类校验：

- 结构校验：`Component.toml` 字段完整、可解析、版本有效
- 一致性校验：`.rbi` 与真实公开接口一致（签名漂移即失败）

建议门禁（待脚本化）：

- `scripts/check-component-manifest.sh`
- `scripts/check-rbi-sync.sh`

## 5. 边界与反模式

边界：

- Manifest/RBI 是“索引层”，不是替代源码的真相源
- 真相源仍是 Rust 源码与测试

反模式：

- 只维护 README，不维护机器可读索引
- 手工维护 `.rbi` 且长期不校验
- 让 AI 直接跨 30+ 文件做首次理解

## 6. 与现有规范关系

- 组件边界：`docs/spec/component_boundaries.md`
- AI 执行手册：`docs/spec/hyper-structure-ui-development-playbook.md`
- 硬规则入口：`docs/RULES_ZH.md`
