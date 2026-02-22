# UI 组件边界：什么是组件，什么不是

> Status: Draft  
> Scope: 定义 `ui` 的职责边界，并规定跨组件基础设施应归属的层级

## 0. 结论先行（避免争吵）

**UI 组件**：对外暴露可渲染的 View（Leptos `#[component]` 或等价），并提供稳定、可测试的语义契约（`data-*` / `aria-*` / `role`），内部按 `logic/styles/motion/view` 组织来装配 `ui-headless`、`ui-theme`、`ui-motion`。

**不是 UI 组件**：任何“跨组件、跨页面、跨应用”的基础设施与开发工具，例如：

- i18n/l10n 注入基础设施（registry、上下文、格式化策略接口）
- trace/事件回放、调试观测面板（debug overlay）
- 性能预算/测量、探针与采样策略
- 流式 contract（分段配置/增量挂载/恢复协议）与其运行时
- E2E 测试运行器/框架适配（Playwright 配置、helpers、fixtures）
- 兼容层/上游命名对齐（Spectrum/RAC/S1 宏、story/test/utils 等）

这些东西放进 `ui` 的结果只有一个：**污染 API、破坏分层、让未来维护变成屎山**。

## 1. 分层归属（放错层就是 bug）

依赖方向（核心链路）：`ui-state-primitives -> ui-headless -> ui -> apps/*`；横向能力为 `ui-theme/ui-motion`（服务组件，不反向侵入状态与行为层）。

- `ui-state-primitives`：纯状态原语与不变量建模（与语言/DOM/运行时无关）。
- `ui-headless`：交互行为 + A11y 语义契约（输出 attrs/handlers/state），以及跨组件基础设施的**契约与注入点**（例如 i18n registry、trace/perf hooks）。
- `ui-theme`：token 与 CSS 变量生成，不含组件 CSS。
- `ui-motion`：动效运行时（可 wasm/no-op）。
- `ui`：最终“组装与渲染”。只做两件事：
  1) 把 headless 的语义正确挂到 DOM（`role/aria/data-*`）  
  2) 把 theme/motion 作为契约落地到 CSS/动画
- `apps/*`：验收面与回归面。调试 UI、E2E 资产、demo 字典、业务集成全部在这里。

## 2. i18n：不是“翻译系统”，是“文本来源契约”

基础组件需要 i18n/l10n 的原因非常现实：**可访问性**。

- 图标按钮/清除按钮没有 `aria-label` 就是不合格组件。
- 一些组件必须给出默认读屏文案，否则“开箱可用”是假的。

我们要做的不是在组件层塞一个“翻译引擎”，而是把默认文案的来源变成可注入的契约：

优先级（高→低）：
1. 调用方 props（最明确）
2. `UiRoot`/应用注入的 strings bundle（可全局覆盖）
3. 组件默认值（英文兜底，保证开箱可用）

关键约束：**禁止中心化“大字典”**。每组件 strings 类型分散定义，按需引入，避免单文件膨胀与合并冲突。

详见：`docs/spec/i18n.md`

## 3. 组件层的“可验证完成”标准（check2 的正确含义）

`check2.md` 里的 `[x]` 不是“看过了”，而是**有可执行证据**：

- 语义契约：`data-slot`/`data-state`/`aria-*` 稳定且有测试覆盖（单测或语义测试或 E2E）。
- 不硬编码不可替换的用户可见文案（有 props 覆盖或 strings 注入）。
- SSR/WASM 路径不 panic，motion 在非 wasm 下可降级。

不适用的条目必须明确写清楚 “为什么不适用、用什么机制替代保证正确性”，否则就是偷懒。

## 4. 纯逻辑与细粒度响应的阻抗匹配（Reducer + Selector）

这个方向是对的，但要说完整：

- `logic.rs` 应保持纯函数：接收旧状态 + 动作，返回新状态（或最小变更）。
- `logic.rs` 不依赖 `Signal`，这样单测简单、行为可重放、状态转移可审计。
- `view.rs` 负责把纯逻辑接到 Leptos 响应系统，并做 selector 切片。

关键点：Leptos 的追踪粒度是 `Signal`，不是结构体字段。

- 如果所有依赖都直接读同一个 `RwSignal<State>`，一次 `set` 可能唤醒过多订阅者。
- 解决办法是让 `view.rs` 暴露稳定 selector（`Memo/Signal::derive`），只把需要的切片喂给子视图。
- `Memo` 语义边界：切片值实现 `PartialEq` 时，值未变化不会通知下游，因此不会触发对应 DOM 更新。

推荐模式：

```rust
#[derive(Clone, PartialEq)]
pub struct ButtonState {
    pub is_pressed: bool,
    pub is_hovered: bool,
}

pub enum ButtonAction {
    Press,
    Release,
    Hover(bool),
}

pub fn transition(state: &ButtonState, action: ButtonAction) -> ButtonState {
    let mut next = state.clone();
    match action {
        ButtonAction::Press => next.is_pressed = true,
        ButtonAction::Release => next.is_pressed = false,
        ButtonAction::Hover(v) => next.is_hovered = v,
    }
    next
}
```

```rust
// view.rs: reducer dispatch + selector slicing
let state = RwSignal::new(ButtonState { is_pressed: false, is_hovered: false });
let dispatch = Callback::new(move |action: ButtonAction| {
    state.update(|s| *s = transition(s, action));
});

let is_pressed = Memo::new(move |_| state.get().is_pressed);
let is_hovered = Memo::new(move |_| state.get().is_hovered);
```

高频路径补充约束：

- 当某个字段更新频率极高且影响面大，优先拆成独立 `Signal`，不要硬塞进大 `State`。
- “性能损失仅为 clone 开销”只在状态体量小、更新频率适中时成立；高频/大状态场景要评估 `transition + selector` 重算和 clone 成本·。
- 禁止在 `view.rs` 内分散写状态机分支；状态转移规则统一回收至 `logic.rs`。

## 5. 命令式第三方库接入边界（Foreign Zone）

场景：ECharts、Google Maps 等命令式库必须直接持有 DOM ref。  
结论：允许接入，但必须进入“受控外交特区”，不能污染主路径。

归属规则：

- `logic.rs`：只表达意图命令（`YieldControl` / `CleanupForeign`）。
- `view.rs`：通过受控 ref + adapter 执行第三方 `init/update/destroy`。
- 生命周期主导权仍在 Rust 逻辑层（open/close/mount/unmount 由逻辑决定）。

禁止事项：

- 禁止把第三方实例句柄暴露为组件公共 API。
- 禁止第三方库直接回写核心状态机（必须回流为 Action）。
- 禁止为第三方接入破坏分层方向。

详见：`docs/spec/foreign_zone_escape_hatches.md`

## 6. 焦点管理边界（Global Focus Stack）

场景：层叠 Modal/Overlay 中，上层容器可能被异步强制卸载。  
结论：焦点恢复不能由单个组件私有完成，必须交给全局 Focus Manager。

归属规则：

- `ui-headless`：提供全局焦点栈服务（trap push/pop、restore 策略解析、失效目标回收）。
- `logic.rs`：只记录恢复策略（selector/fallback policy），不记录脆弱 DOM 引用。
- `view.rs`：执行 manager 决议到 DOM（focus 调用），并在 scope 销毁时上报 invalidation。

禁止事项：

- 禁止组件私有缓存 `NodeRef` 作为长期 restore target。
- 禁止绕过全局管理器直接做最终 restore 决策。
- 禁止在 focus restore 失败时静默坠落 `document.body` 且无诊断标记。

详见：`docs/spec/focus_global_stack_gc.md`

## 7. AI 可读性边界（Context Projection Layer）

场景：组件分层越细，AI 越容易因上下文过载而遗忘接口细节。  
结论：需要独立“索引层”，但不能让索引层反客为主替代源码。

归属规则：

- 组件目录必须提供机器可读索引（`Component.toml`）与接口投影（`.rbi`）。
- `Component.toml/.rbi` 负责“压缩理解成本”，源码与测试仍是唯一真相源。
- Agent 默认先读索引层；只有需要改实现时，才读取 `logic/view/styles/motion` 源码。

禁止事项：

- 禁止让 AI 默认跨 30+ 文件全量扫源码完成首次理解。
- 禁止把 README 当作唯一索引（它不可机读、不可校验）。
- 禁止长期手写 `.rbi` 且不做与真实接口的一致性校验。

详见：`docs/spec/ai_context_projection_protocol.md`

## 8. 创新试验边界（Controlled Sandbox）

场景：高不确定性组件（如复杂拖拽看板）在早期阶段很难一次满足核心全部约束。  
结论：允许实验，但只能在受控沙盒中演化，不能直接污染核心主路径。

归属规则：

- 试验组件进入 `ui-contrib`（或等价隔离域），不进入核心默认导出。
- 沙盒组件必须携带显式风险标签与生命周期状态，便于审计和清退。
- 稳定并被验证后，再走 Graduation 路径迁入核心 crate。

禁止事项：

- 禁止以“先上线再说”为理由绕过核心分层和质量门禁进入主路径。
- 禁止把 contrib 临时 API 当作长期公共契约。
- 禁止没有 owner/里程碑的长期孵化组件常驻仓库。

详见：`docs/spec/controlled_evolution_sandbox.md`
