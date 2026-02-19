# 异步阻抗治理（Async Impedance）

## 问题定义

`logic.rs` 期望输出可预测的状态快照，但 Rust `async/await` 产物是运行时驱动的 `Future`。  
`Future` 不是组件逻辑层应持有的业务状态，也不应进入纯逻辑结构体。

典型场景：Combobox 输入触发远程搜索。

- 用户输入 -> 发起请求 -> 等待返回 -> 展示结果

如果把 `Future` 放进 `State`，逻辑层会被运行时和平台实现污染。  
如果逻辑层不记录请求元数据，又无法处理取消、去抖和竞态返回。

## 核心判断

- 逻辑层管理“请求元数据”，不管理“请求执行实体”。
- 状态即数据（State as Data），异步即命令（Async as Command）。

## 标准解法

### 1) 状态只存元数据，不存 Future

```rust
enum SearchState {
    Idle,
    Loading(RequestId),
    Loaded(Vec<Item>),
    Failed { id: RequestId, reason: ErrorKind },
}
```

### 2) 异步触发由命令表达

```rust
enum Command {
    FetchData { id: RequestId, query: String },
    CancelRequest { id: RequestId },
}
```

`logic.rs` 在输入变化时返回 `(State, Vec<Command>)`，只声明“要发起/取消什么请求”。

### 3) 执行与回调在 view/effect 层

- `view.rs` 或专用 effect adapter 执行 `spawn_local`/runtime task。
- 任务完成后回发 `Action::DataReceived { id, data }` 或 `Action::DataFailed { id, ... }` 给 `logic.rs`。

## 竞态与取消规则

- 必须使用递增或唯一 `RequestId` 标识一次请求生命周期。
- 逻辑层收到结果时必须核对当前挂起请求 ID：
  - 匹配：接受并落地状态
  - 不匹配：判定为过期响应并丢弃
- 输入更新触发新请求时，旧请求应通过命令可取消（或至少逻辑可忽略其回包）。

## 分层约束

- `logic.rs`：
  - 允许：`RequestId`、请求状态枚举、竞态仲裁、错误状态归一
  - 禁止：持有 `Future`、调用 runtime API、直接网络请求
- `view.rs` / effect adapter：
  - 允许：执行异步任务、取消任务、回传结果 action
  - 禁止：绕过逻辑层直接写业务状态

## 测试要求

- 逻辑单测：
  - 输入变化时是否正确产生 `FetchData`/`CancelRequest` 命令
  - 过期 `RequestId` 回包是否被丢弃
  - 最新 `RequestId` 回包是否正确更新状态
- 集成测试：
  - 快速连续输入下，最终显示结果来自最后一次请求
  - 取消/失败路径有稳定回退与可恢复行为
