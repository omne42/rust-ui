# 从 `bb/packages/ui-web` 借鉴到 `rust-ui` 的意义（要点）

来源（本机路径）：

- `/Users/zyp/code/personal/bb/packages/ui-web/ARCHITECTURE_ZH.md`
- `/Users/zyp/code/personal/bb/packages/ui-web/COLORS.md`
- `/Users/zyp/code/personal/bb/packages/ui-web/DOCS_EVOLUTION.md`

> 这份笔记的目的：把“能落地的约束与接口”抽出来，映射到当前 `rust-ui` 的四层（core/headless/theme/components）上，避免空谈。

## 1) ARCHITECTURE_ZH.md：对我们最重要的 5 件事

### 1.1 “计算 vs 副作用”的硬边界

文档强调：逻辑层只做**纯计算/状态机**；视图层负责 DOM/动效/布局等**副作用**。

对 `rust-ui` 的映射：

- `ui-state-primitives`：只放纯状态与状态机（受控/非受控、集合/选择等）；可单测；不引入 DOM 假设。
- `ui-headless`：专注“行为 + A11y + 事件标准化”（FocusVisible/Press/Overlay dismiss 等），并通过 feature gate 处理 Web/SSR。
- `ui-components`：只负责“长什么样”，组合 `ui-state-primitives + ui-headless + ui-theme`，**不直接使用 `web-sys`**。

落地约束（我们已经在做，但需要持续守住）：

- 任何“业务组合判断”（例如 disabled/loading 组合）优先放在 core/headless；组件层只消费结果。
- 组件层需要少量“视图派生状态”（如图片加载失败、动效时序）时，抽成组件私有的 view-state hook/mod（不要回流到 core）。

### 1.2 透明代理（Transparent Proxy）

逻辑 hook 默认要“透传底层能力”，同时提供更简单的二次封装，避免用户为了一个小定制被迫重写全套逻辑。

对 `rust-ui` 的映射：

- `ui-headless` 的对外返回值尽量包含：
  - `state`（我们封装后的稳定状态）
  - `raw_state`（更接近底层事件/交互的细粒度状态）
  - `attrs/handlers`（可挂载到 view 的属性与事件）
- `ui-components` 对外 props 保持稳定，但要给“通往底层”的出口：
  - 要么通过 `ui-headless` 暴露 raw hooks（第三层 API）
  - 要么在组件 props 上提供少量“插槽/回调”以避免用户 fork 组件

### 1.3 渐进式复杂度（Progressive Complexity）

同一个能力提供三种入口：

1. Default（开箱即用）
2. Composed（可组合拆分件）
3. Raw（纯逻辑 hook）

对 `rust-ui` 的映射建议（Phase 2+）：

- Default：`ui-components::{Button, Select, MenuTrigger, ...}`
- Composed：把一些组合组件拆成可组合 pieces（例如 `SelectRoot/SelectTrigger/SelectPopover/...`）
- Raw：`ui-headless::use_*` 与 `ui-state-primitives::use_*`

### 1.4 Tokens as Interfaces（把 Token 当协议）

样式数据不是“实现细节”，而是可演进的**接口**；组件内部不要散落硬编码色值/阴影/圆角。

我们现状：已经使用 `ui-theme` 产出的 `--ui-*` CSS 变量，并在组件里引用它们。

下一步（Phase 2+）：把变量命名升级为更“语义化”的 token（见 COLORS.md）。

### 1.5 治理与工具链（对抗熵）

文档强调：没有硬性门禁，架构原则会被一点点腐蚀。

我们现状：已有 `scripts/gate.sh` + `githooks/*`（格式/检查/Changelog/Conventional Commits/文件行数上限）。

建议：未来把“分层守卫”也变成门禁（例如简单的依赖扫描、禁止特定 crate 引用等）。

## 2) COLORS.md：对我们最重要的 3 件事

### 2.1 语义色 + content surface 分层

不要在组件里用“色阶 ramp”当 surface；surface 用 `content1..4` 这类“层级面”语义，业务语义色用 `primary/success/warning/danger`。

对 `rust-ui` 的映射（Phase 2+）：

- 在 `ui-theme` 引入语义 token：
  - layout：`background/foreground/divider/focus`
  - surface：`content1..4`
  - intent：`primary/secondary/success/warning/danger` + `*-foreground`
  - ramps：`primary-50..900` 等（可选，晚点做）
- 组件内部优先使用 layout/surface/intents，而不是直接用 ramp。

### 2.2 单一事实来源

颜色 token 必须来自一个地方（代码生成或静态定义皆可），不要散落在组件里。

对 `rust-ui`：继续保持所有色值来自 `ui-theme::Theme`（或未来 token 生成器）。

### 2.3 主题切换是 “class 覆盖一组变量”

不要做复杂主题系统：一个 class 覆盖一组 CSS 变量即可，并且要支持“嵌套主题容器”（例如 `.theme-default.theme-dark`）。

对 `rust-ui` 的映射（Phase 2+）：

- 把 `UiRoot` 的 `:root { ... }` 注入升级为：
  - 默认挂载 `.theme-default`
  - dark 模式用 `.theme-default.theme-dark` 或 `.dark .theme-default` 的组合策略
- 这样能支持局部主题/嵌套容器，也更接近 Web UI 生态的实践。

## 3) DOCS_EVOLUTION.md：对我们最重要的 3 件事

### 3.1 文档系统要和组件库解耦

组件库保持纯粹；文档 runtime、kit、脚本应该独立（避免把“文档工程复杂度”塞进核心库）。

对 `rust-ui` 的映射（Phase 3+）：

- 新建 `apps/docs`（或 `apps/docs-site`）作为独立应用
- 文档的解析/渲染/props 表生成应在独立 crate/工具中完成

### 3.2 Example 沙盒隔离（避免污染）

每个 demo/example 要有独立的 Theme/Context 环境，避免“文档 app 的全局样式”污染组件行为。

对 `rust-ui`：`UiRoot` 已是良好起点；后续 docs app 需要为每个 example 单独包一层 provider。

### 3.3 PropsTable 自动化

不要在文档里手写 props 表；用脚本从源码生成 JSON，再由 docs kit 渲染。

对 `rust-ui` 的映射（Phase 3+）：

- Rust 侧可从组件宏输入/类型定义抽取（`syn`）生成 JSON
- 或者先走“手写少量 + 逐步自动化”的路线

## 4) 可执行的落地 TODO（优先级排序）

> 这里列的是“下一步能干”的动作，不要求本阶段全部完成。

P0（近期，利于持续演进）：

1. 在 `docs/research/README.md` 中补充以上参考（索引化，方便后续查）。
2. 给 `ui-theme` 增加一层“语义 token 兼容映射”（在不破坏现有 `--ui-*` 的前提下，引入 `--background/--foreground/...` 的 alias）。

P1（Phase 2+）：

3. 规划并落地 “渐进式复杂度” API：默认组件 + 可组合 pieces + raw hooks。
4. `UiRoot` 从 `:root` 注入迁移到 “class 覆盖变量” 策略（支持嵌套主题容器）。

P2（Phase 3+）：

5. 新建 docs app（`apps/docs`）与 example 沙盒隔离机制。
6. 建立 props table 自动生成（脚本→JSON→渲染）链路。

