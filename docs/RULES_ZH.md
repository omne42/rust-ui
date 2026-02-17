# rust-ui 规则文档（v0）

> 目标：用 Rust + Leptos 复刻 React Spectrum 的分层（Stately/Aria/Spectrum），并通过 Tauri 覆盖 Web/桌面/Android(WebView)。  
> 风格取向：现代化（OKLCH + OLED）、强交互（Framer Motion/HeroUI 级别的“弹簧手感”）。

## 0. 总则（必须遵守）

- **低耦合 / 高内聚**：每个 crate 只做一件事，并且边界清晰。
- **分层不破**：状态原语（state-primitives）→ 行为与可访问性（headless）→ 组件（components）→ 应用（apps）。
- **不把实现细节透传到上层**：上层依赖稳定的“契约”（struct/enum/trait），而不是下层内部类型。
- **动效优先物理（Spring Physics）**：交互反馈尽量使用弹簧驱动（非 CSS transition / 非纯 duration/easing）。
- **默认可访问性（A11y）**：headless 输出语义、键盘与焦点行为；components 负责视觉表达。

## 1. 仓库结构与目录职责

```
.
├── crates/
│   ├── ui-state-primitives  # 纯状态（Stately）
│   ├── ui-headless          # 行为 + A11y（Aria）
│   ├── ui-theme             # 设计系统 tokens → CSS vars（OKLCH + OLED）
│   ├── ui-motion            # 高级动效引擎/后端（WAAPI + Spring runtime）
│   ├── ui-components        # 最终组件（Spectrum）
│   └── ui-compat            # 兼容层（provider/rac/s2/story/test/utils）
├── apps/
│   ├── web-demo             # 可提交的 Web demo（Trunk CSR）
│   ├── docs-app             # 文档与组件工作台入口
│   └── tauri-demo           # 可提交的 Tauri 壳（桌面验证入口）
├── examples/          # 本地调研/参考（默认不提交，见 .gitignore）
└── docs/
    ├── plan/          # 计划与 DAG
    ├── spec/          # 规格冻结（motion/mvp…）
    └── research/      # 调研笔记（调研定位等）
```

- **`apps/*`**：必须可运行、可展示真实交互与 A11y（用来验收）。
- **`examples/_upstream/*`**：只用于本地 clone 调研仓库（React Spectrum / motion / heroui / shadcn / animate-ui 等）；**默认不进 git**（`.gitignore` 忽略 `examples/`）。

## 2. 分层与依赖规则（最重要）

### 2.1 依赖方向（单向）

- `ui-state-primitives`：**不依赖任何内部 crate**（平台无关；禁止 DOM/web-sys）。
- `ui-theme`：不依赖 `ui-components`（tokens 不知道组件存在）。
- `ui-headless`：可选依赖 `ui-state-primitives`；**禁止依赖** `ui-components` / `ui-theme`。
- `ui-motion`：不依赖 `ui-components`（引擎不关心组件）。
- `ui-components`：允许依赖 `ui-headless + ui-theme + ui-motion`（必要时才依赖 `ui-state-primitives`）。
- `ui-compat`：兼容层可依赖 `ui-components/ui-headless`，但核心分层不得反向依赖 `ui-compat`。
- `apps/*`：依赖 `ui-components`（上层不直接接触 `web-sys`）。

### 2.2 每层职责（对标 React Spectrum）

#### `ui-state-primitives`（React Stately）

- 只做：**状态建模**（受控/非受控、选择、集合、开关等）。
- 不做：DOM、事件标准化、样式、动画。
- 要求：单元测试覆盖关键状态机/受控行为。

#### `ui-headless`（React Aria）

- 只做：**交互与 A11y**（press/focus-visible/roving tabindex/aria-* 等）。
- 输出形态：**handlers + attrs 的结构体**，由组件层显式挂载（不要隐式 spread 魔法）。
- 不做：视觉表现（不写 class、不写 CSS、不做动画编排）。
- **Feature gating**：
  - 默认 `web`（CSR）可用。
  - `ssr` 下提供降级实现：能编译、能返回合理默认值，但不注册 window/document 监听。

#### `ui-theme`（Design Tokens）

- 只做：tokens → CSS Variables（字符串输出）。
- 颜色规范：**OKLCH**；新增 **OLED** 主题（真黑背景）。
- 不做：组件 CSS（组件 CSS 在 `ui-components`）。

#### `ui-motion`（Motion Engine）

- 只做：动效执行与运行时（Web 后端等）。
- 必须支持 `prefers-reduced-motion`：reduce 时应跳过/降级。
- 非 wasm/SSR：允许 no-op（保持编译通过）。

#### `ui-components`（最终组件库）

- 只做：把 `ui-state-primitives` 状态 + `ui-headless` 行为 + `ui-theme` 样式 + `ui-motion` 动效组合成最终组件。
- 对外 API：尽量小而稳（v0 冻结后避免破坏性改动）。
- 公开 API 禁止暴露 `web-sys` 类型；DOM 细节只存在于 `cfg(wasm32)` 的内部实现中。

## 3. 组件内部结构（ARCHITECTURE_ZH 风格）

每个组件建议拆为：

- `logic.rs`：props 归一化、派生状态、组合 headless hooks、决定 class/variant。
- `styles.rs`：组件的**静态 CSS 字符串**（只使用 tokens：`var(--ui-*)`）。
- `motion.rs`：组件 motion contract（`XxxMotion`/`XxxMotionPreset`）+ `attach_motion(...)`。
- `view.rs`：纯 Leptos view（HTML 结构 + class/attrs/handlers 挂载）。

CSS 注入规则：

- `ui-components/src/css.rs` 聚合所有组件 CSS。
- `<UiRoot>`（`crates/ui-components/src/root.rs`）统一注入：
  - `ui-theme` 生成的 CSS variables
  - 组件 CSS
  - 最小全局 base（body 背景/字体）
- **Cascade Layers（默认）**：
  - 组件 CSS 注入在 `@layer ui`（低优先级层）。
  - 应用侧覆盖推荐：不分 layer 直接写 overrides；如应用也使用 layers，则声明 `@layer ui, app;` 并把 overrides 放进 `@layer app`。
- **禁止 inline CSS（组件层）**：
  - `ui-components` 中禁止在 `view!` 里写 `style="..."` / `style=...`（字符串形式的 inline style）。
  - 组件所有样式规则（selector + 声明）必须位于该组件的 `styles.rs`，并通过 `ui-components/src/css.rs` 聚合后由 `<UiRoot>` 注入；组件内部不得写 `<style>` 标签。
  - 禁止使用 `style:<prop>=...` 绑定普通 CSS 属性（`padding/background/position/...` 等）；样式切换通过 `class`/`data-*` + `styles.rs` 完成。
  - 如必须传递运行时数值（例如 popover 位置 / motion 数值），只允许设置 **CSS variables（custom properties，`--*`）**：
    - 推荐：`style:--x=...`（如果语法可用）
    - 允许：`style=...` 但内容必须 **只包含** `--*` 变量赋值（禁止出现 `top/left/padding/background/...` 等普通属性）

- **Inline CSS forbidden (component layer):**
  - `ui-components` must not use `style="..."` / `style=...` inside `view!`
  - Do not bind normal CSS properties via `style:<prop>=...`
  - Only CSS variables (custom properties, `--*`) are allowed

## 4. 颜色与主题（OKLCH + OLED）

### 4.1 规范

- 颜色 token **必须使用 OKLCH**：`oklch(L% C h)`；透明度用 `oklch(... / a)`。
- 主题必须设置 `color-scheme`（由 `ui-theme` 输出），让浏览器表单控件/滚动条更一致。
- 组件禁止硬编码颜色（hex/rgb 等）；只能使用 `var(--ui-*)`。

### 4.2 当前 tokens（v0）

`ui-theme` 输出（示例）：

- `--ui-fg`, `--ui-fg-muted`
- `--ui-bg`, `--ui-bg-muted`
- `--ui-accent`, `--ui-accent-fg`, `--ui-accent-soft`
- `--ui-border`, `--ui-focus-ring`
- `--ui-radius-*`, `--ui-space-*`, `--ui-shadow-*`

主题入口：

- `Theme::light()`
- `Theme::dark()`
- `Theme::oled()`（真黑背景，暗色 scheme）

### 4.3 OLED 规则（方向）

- OLED 主题 `--ui-bg` 为真黑（`oklch(0% 0 0)`）。
- Surface（`bg-muted`）必须比背景更亮，避免“所有东西都融进黑里”。

## 5. Motion：接口与实现（Framer/HeroUI 方向）

### 5.1 分层规则

- **契约（contract）在组件层**：每个组件定义自己的 `XxxMotion`（例如 `ButtonMotion`）。
- **引擎（engine）在 `ui-motion`**：组件通过 `attach_motion` 把 DOM ref + 状态信号连接到引擎。
- `ui-headless` 不做动画编排（它只输出“状态变化/事件语义”）。

### 5.2 `ui-motion` 当前实现（v0）

- Web（wasm32）：
  - **WAAPI**：`ui_motion::web::animate(...)`（keyframes/options → `element.animate(...)`）。
  - **Spring runtime**：`ui_motion::spring::SpringAnimator`（rAF 驱动、stiffness/damping/mass/precision）。
  - `prefers-reduced-motion`：reduce 时应跳过或直接 set 到目标值。
- 非 wasm：
  - `ui_motion::web` 为 no-op；`prefers_reduced_motion()` 默认视为 true（避免误触发动画逻辑）。

### 5.3 组件侧用法模式（以 Button 为例）

- 组件定义 motion contract：`XxxMotion`（默认值合理、对外可覆盖）。
  - Button：`ButtonMotion { spring, hover_scale, tap_scale }`
  - Checkbox：`CheckboxMotion { spring, hover_scale, tap_scale, indicator_spring }`
  - Switch：`SwitchMotion { spring }`（thumb translate/width）
  - Overlay/Popover：`OverlayMotion` / `PopoverMotion`（opacity/scale/translate）
  - 列表类：`ActiveHighlightMotion`（active highlight 的 y/height/opacity）
- `attach_motion(...)` 的硬规则：
  - 只在 `cfg(wasm32)` 生效（SSR/非 wasm 为 no-op 或立即完成）。
  - per-frame 更新应尽量**只写 CSS variables（custom properties）**，避免触发组件重渲染。
  - 长生命周期、非 Send/Sync 的运行时对象必须用 `StoredValue::new_local(...)` 存放。
  - 需要“exit 动画后再卸载”时：组件提供 `on_exit_complete` 回调，上层用 presence（例如 `use_presence`）决定何时 unmount。

### 5.4 动效准则（方向）

- 交互反馈（press/drag/hover/selection highlight）优先 Spring（参数可主题化：未来 motion tokens）。
- Presence（enter/exit）与 layout motion（FLIP）后续补齐，但仍遵守：contract 在组件，引擎在 `ui-motion`。

## 6. 排版与布局

### 6.1 全局排版（当前）

`<UiRoot>` 提供最小全局样式（`system-ui` 字体栈 + 背景/前景来自 tokens）。

规则：

- 组件库不“接管页面布局”；布局应由应用（`apps/*`）决定。
- 组件内部排版使用 tokens（space/radius/shadow），避免散落的 magic numbers。

### 6.2 Safe Area（移动端/Tauri Android）

- `ui-theme` 提供 `SAFE_AREA_CSS`（使用 `env(safe-area-inset-*)`）。
- `<UiRoot safe_area=true>` 时应用 `.safe-area`，用于刘海屏/沉浸式场景。

## 7. 全局配置与 Provider（应用必须做）

应用入口（例如 `apps/web-demo/src/main.rs`）必须在 root 初始化：

- `provide_focus_visible()`：全局交互 modality 推断（键盘显示 focus ring，指针不显示）。
- `provide_overlay_stack()`：overlay 栈管理（只让 topmost 响应 Esc 等）。
- 使用 `<UiRoot theme=... safe_area=...>` 注入 tokens + CSS + base。

## 8. 工程化与协作约束（提交即门禁）

### 8.1 Git hooks（必须启用）

- 安装：`./scripts/setup-githooks.sh`
- `commit-msg`：Conventional Commits
- `pre-commit`：
  - **必须同时提交 `CHANGELOG.md`**（只允许改 `[Unreleased]`）
  - 限制单个 `.rs` 文件行数（默认 1000，可用 `RUST_UI_MAX_RS_LINES=<N>` 临时放宽）
  - 自动跑 `scripts/gate.sh`（进而跑 `scripts/check.sh`）

### 8.2 质量门禁（Stop Gates）

- `./scripts/check.sh`：fmt → clippy → test → ssr compile → wasm compile
- 变量：
  - `SKIP_WASM=1` 可跳过 wasm gates（本机未装 wasm target 时）
  - `RUST_UI_ALLOW_CHANGELOG_RELEASE_EDIT=1` 仅在 cut release 时允许改已发布 changelog 段落

---

## 附：相关规格与入口

- 计划：`docs/plan/TODO.md`（骨架→血肉）
- Motion 规格：`docs/spec/motion.md`
- 调研入口：`docs/research/README.md`
