# 副作用命令模式（Side Effect Command Pattern）

## 问题定义

`logic.rs` 通常被要求保持纯函数：`(OldState, Action) -> NewState`。  
但 UI 交互不只有状态更新，还存在副作用，例如：

- `event.preventDefault()`
- `event.stopPropagation()`
- `element.focus()`
- 播放提示音/触发外部 effect

如果把 `web_sys::Event`、`HtmlElement` 等平台对象传入 `logic.rs`，会污染纯逻辑层并破坏分层边界。  
如果不传，逻辑层又无法声明“我需要阻止默认行为/需要聚焦下一个元素”。

## 标准解法

采用 Command Pattern（可参考 Elm Architecture）：

- 逻辑层只做**决策**，返回 `(State, Vec<Command>)`
- 视图层（或事件适配层）只做**执行**，把 `Command` 映射到具体平台 API

核心契约：`Intent` 与 `Implementation` 分离。

## 契约形态

```rust
pub enum Command {
    PreventDefault,
    StopPropagation,
    FocusById(String),
    EmitSound(SoundType),
}

pub fn update(state: State, action: Action) -> (State, Vec<Command>) {
    match action {
        Action::Click => {
            let next = state.set_pressed(true);
            (next, vec![Command::PreventDefault])
        }
    }
}
```

## 分层要求

- `logic.rs`：
  - 允许：`State`、`Action`、`Command`、纯数据归一化
  - 禁止：`web_sys`、DOM 引用、平台事件对象、直接副作用调用
- `view.rs` / adapter：
  - 允许：匹配 `Command` 并调用 `prevent_default`、`focus` 等 API
  - 禁止：绕过 `Command` 在事件回调里分散写业务决策

## 测试要求

- 逻辑测试（单元测试）断言：
  - 给定 `state + action` 时，返回的 `state` 与 `commands` 是否正确
- 视图/集成测试断言：
  - `Command` 到平台行为的映射是否正确（例如 `PreventDefault` 真正生效）

## 适用与例外

- 若某交互完全无副作用，可返回空命令：`Vec::new()`
- 若副作用是跨组件可复用语义能力，优先落在 `ui-headless` 契约层，不在单组件重复发明

