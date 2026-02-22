# Focus 全局栈与墓地回收规范（Global Focus Stack + Graveyard GC）

> Status: Draft  
> Scope: 规范层叠 Overlay/Modal 场景下的焦点连续性，避免 Zombie Node 焦点坠落

## 1. 问题定义（Focus Singularity）

组件局部“自己记住焦点并 restore”在简单场景可用，但在嵌套弹层 + 异步销毁下会崩：

1. A 打开 Modal 1  
2. Modal 1 打开 Modal 2  
3. Modal 1 因外部状态变化被强制卸载  
4. Modal 2 关闭，尝试恢复到 Modal 1 内按钮（已不存在）

结果：浏览器将焦点丢到 `document.body`，读屏用户失去上下文。

结论：焦点是全局单例资源，不是组件私有资源。

## 2. 核心决策

引入 **Global Focus Manager**（建议落在 `ui-headless` 顶层服务）：

- 维护 `Stack<FocusTrapFrame>`（topmost 优先生效）。
- 组件通过协议申请/释放焦点陷阱，不直接私自 restore。
- 恢复目标存“策略”，不存脆弱 DOM 引用。

## 3. 契约模型（Required）

### 3.1 FocusTrapFrame（示意）

```rust
struct FocusTrapFrame {
    trap_id: String,
    scope_id: String,
    restore_policy: RestorePolicy,
}
```

### 3.2 RestorePolicy（示意）

```rust
enum RestorePolicy {
    Selector(String),          // 例如 "#btn-b"
    NearestFocusableSibling,   // 就近兄弟可聚焦节点
    FallbackTo(String),        // 应用级安全落点（如 "#app-main-focus-anchor"）
}
```

禁止在逻辑层存储 `NodeRef<Element>` 作为恢复目标真相源。

### 3.3 Manager API（示意）

- `push_trap(frame)`
- `pop_trap(trap_id)`
- `invalidate_scope(scope_id)`（scope 销毁通知）
- `resolve_restore_target(policy)`（按策略找可用焦点）

## 4. 墓地回收（Graveyard Collection）

当上层容器（如 Modal 1）被卸载时，Manager 必须：

- 扫描栈中引用该 scope 的恢复策略。
- 将失效目标重定向（re-parent）或标记失效。
- 保证后续 `pop_trap` 不会把焦点送到虚空。

推荐回退顺序：

1. `restore_policy` 指向的存活可聚焦节点  
2. 最近可聚焦兄弟/祖先 fallback  
3. 上一层 trap 的 opener 策略  
4. 应用级安全锚点（显式配置）  
5. `document.body`（仅最后兜底，并应记录诊断）

## 5. 与 Overlay 栈协同

- 焦点栈与 overlay 栈必须一致：仅 topmost trap 拥有焦点控制权。
- overlay 弹出：`push_trap`
- overlay 关闭：`pop_trap` + manager restore
- overlay 强制销毁：必须触发 `invalidate_scope`

## 6. 边界约束

- 组件不得绕过 Manager 直接做最终 restore 决策。
- `logic.rs` 只表达焦点意图（open/close/restore policy），不直接调用 DOM focus API。
- `view.rs`/adapter 负责执行 manager 决议到 DOM。

## 7. 验收门禁（建议）

```bash
cargo test -p ui-headless
cargo test -p ui
cargo check -p docs-app --target wasm32-unknown-unknown
```

关键回归用例应覆盖：

- 双层 modal 正常恢复
- 内层关闭前外层被销毁（Zombie target）
- 无有效目标时回退到应用锚点而非直接 body

## 8. 关联规范

- `docs/spec/component_boundaries.md`
- `docs/RULES_ZH.md`
- `docs/plan/TODO.md`（12.2 焦点管理扩展条目）

