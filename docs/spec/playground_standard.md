# Playground Standard（docs-app）

本规范用于约束 `apps/docs-app` 里组件 Playground 的最低工程质量，并通过测试强制执行。

## 目标

每个组件 Playground 必须同时满足：

1. 一个简单、真实、可读的展示（非样式滑块假交互）。
2. 一个 Config 面板，且对外 API 必须可追踪覆盖。
3. 一个多参数对比展示（State Matrix / Comparison Matrix）。

## 命名与结构约束

推荐每个组件至少包含以下区块：

- `Hello World`（或等价默认路径）
- `Workbench`（带 `controls` + `test_config_signal`）
- `State Matrix`（或 `Comparison Matrix`）

区块顺序是硬性约束：必须按 `Showcase -> Workbench -> Matrix` 出现。

Workbench 必须有稳定 `data-slot`：

- `<component>-workbench-controls`
- `<component>-workbench-config-preview`
- `<component>-workbench-canvas`

所有带 `controls` 的 Playground 必须声明 `test_config_signal`，用于输出“实时实际配置（actual config）”并进入测试面板。

如果组件存在难以在主 Workbench 同时覆盖的子 API（如初始化型 API、生命周期回调），必须补充专门 API Workbench，并提供独立 `data-slot`。

## API 覆盖规则

以组件 `pub fn Component(...)` / `pub fn SubComponent(...)` 签名为准：

- 除 `children` 外，其余参数都必须在 Playground 配置输出中可追踪。
- 配置输出可用 `...Config { prop: ... }` 文本形式，但字段名必须与 API 名一致（如 `on_open_change`，不是 `has_on_open_change`）。
- 回调类 API 不能只“声明支持”，必须有可见反馈（日志、计数、状态回显之一）。

## 强制检测

本规范提供两层检测：

- 组件级测试（当前已落地）：`cargo test -p docs-app --test playground_standard`
- 全组件巡检脚本（独立 gate，不并入 `scripts/check.sh`）：
  `./scripts/check-playground-standard.sh`

`check-playground-standard.sh` 的检测范围是 docs catalog 全组件，且必须失败快：

1. 校验每个组件存在“简单展示 + Config Workbench + 多参数对比”三类 Playground。
2. 对每个组件从 `pub fn Component(...)` 提取 API 参数，并强制要求：
   - Playground 页面存在 API 的真实使用痕迹；
   - Workbench 的 `test_config_signal` 输出覆盖 API 字段（忽略 `children`）。
3. 发现任一违规即以非零退出码失败。

## 首个落地组件

- `Accordion`：主 Workbench 覆盖 root API，`Item API Workbench` 覆盖 `default_open` 与 `on_panel_lifecycle`。
