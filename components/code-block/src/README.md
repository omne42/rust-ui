# CodeBlock

`CodeBlock` 用于稳定展示代码片段，并提供可访问的复制反馈语义（`aria-*` + `data-*`）。

## 先用起来（默认路径）

### Hello World（最小可用）

```rust
view! {
    <CodeBlock code="cargo check -p ui".to_string() />
}
```

- 默认 API 路径优先：先传 `code` 就能工作。
- 不需要用户手动接线 `ui-state-primitives` / `ui-headless`。

## 常见用法

### 带标签与语言

```rust
view! {
    <CodeBlock
        code="fn main() { println!(\"hello\"); }".to_string()
        label="main.rs".to_string()
        language="rust".to_string()
    />
}
```

### 关闭复制按钮

```rust
view! {
    <CodeBlock
        code="read-only snippet".to_string()
        is_copyable=false
    />
}
```

## 再进阶（高级控制）

```rust
let (copied, set_copied) = signal(false);

view! {
    <CodeBlock
        code="cargo test -p ui --test code_block_semantics".to_string()
        is_copied=Signal::derive(move || copied.get()).into()
        on_copied_change=Callback::new(move |next| set_copied.set(next))
        output_mode=ui::code_block::protocol::CodeBlockAgentOutputMode::Snapshot.into()
        output_status=ui::code_block::protocol::CodeBlockAgentOutputStatus::Validated.into()
    />
}
```

- 受控/非受控复制状态成对：`is_copied + on_copied_change + default_copied`。
- `output_mode` 默认 `Snapshot`，`Streaming` 为可选增强路径。

## docs-app 文档入口

- 组件文档页面：`apps/docs-app/src/pages/components/pages/display.rs`（`code_block()`）。
- README 渲染入口：`apps/docs-app/src/pages/components/shell.rs`（`component_readme_markdown("code-block")`）。
