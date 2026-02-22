# Foreign Zone / Escape Hatches 规范（受控外交特区）

> Status: Draft  
> Scope: 规范“命令式第三方库接入”在 `logic/view` 分层中的合法路径

## 1. 问题陈述

理想架构要求：`Action -> Logic -> View`，禁止随意 DOM 直连。  
现实场景要求：ECharts/Google Maps 等命令式库必须持有原始 DOM 容器并自管理内部状态。

如果没有官方“合法出口”，团队会走两条坏路：

- 在视图层散落临时 hack（生命周期失控、泄漏难查）
- 用 unsafe 或隐式全局状态绕过分层（契约崩坏）

结论：需要一个受控且可审计的“不洁之地”。

## 2. 核心决策

引入 **Controlled Foreign Zone（受控外交特区）**：

- 默认路径仍是 `Action -> Logic -> View`。
- 仅在显式 Foreign Zone 范围内，允许 View 拿到裸 DOM ref 交给第三方库。
- 生命周期主导权仍在 Rust 逻辑层，第三方只接管“容器内部渲染”。

## 3. 契约模型（Required）

### 3.1 View 层：`use_foreign_ref`

- 提供标准 hook（或等价 helper）暴露受控容器 ref。
- 该 ref 只能用于 Foreign Zone adapter，不得扩散为通用组件 API。

### 3.2 Logic 层：Foreign Commands

逻辑层只表达意图，不执行第三方命令：

- `YieldControl { zone_id, adapter_kind, payload }`
- `CleanupForeign { zone_id }`

说明：

- `YieldControl`：声明“此区域交由 adapter 初始化/更新”。
- `CleanupForeign`：声明“必须销毁第三方实例并解绑监听”。

### 3.3 生命周期硬约束

- mount/open：必须先由 logic 发出 `YieldControl`，view adapter 才可 `init`.
- update：只允许在 zone 范围内调用第三方 `setOption/update`。
- unmount/close：必须执行 `CleanupForeign`，调用 `destroy/dispose/remove`.
- 禁止组件销毁后保留第三方实例、计时器、全局监听。

## 4. 边界与防污染规则

- Foreign Zone 不能反向写核心状态机；状态回流必须走 Action/Command 桥接。
- 禁止把第三方对象句柄暴露到 `ui` 公共 API。
- 禁止为第三方库破坏 `core -> headless -> components -> apps` 依赖方向。
- Escape Hatch 默认关闭，必须显式开启并带风险标注。

## 5. 可观测性与审计

接入 Foreign Zone 的组件必须暴露稳定标记：

- `data-foreign-zone`
- `data-foreign-state`（`idle|yielded|active|cleaning`）
- `data-foreign-source`（`default|custom` / adapter 来源）

并在 docs-app 提供最小工作台示例（可复现 init/update/cleanup 路径）。

## 6. 最小伪代码

```rust
// logic.rs
enum Command {
    YieldControl { zone_id: String, adapter: ForeignAdapterKind },
    CleanupForeign { zone_id: String },
}
```

```rust
// view.rs
let zone_ref = use_foreign_ref("map-zone");
run_foreign_adapter(zone_ref, commands);
```

## 7. 验收门禁（建议）

```bash
cargo test -p ui
cargo check -p docs-app --target wasm32-unknown-unknown
```

## 8. 与其他规范关系

- 分层归属：`docs/spec/component_boundaries.md`
- 硬规则：`docs/RULES_ZH.md`
- 执行任务：`docs/plan/TODO.md`（7.0.1 受控外交特区）

