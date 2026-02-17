- [x] 流式在这里仅指 LLM 输出渲染（只看两种显示模式）。
  - `Streaming`：LLM 还在生成，界面边生成边显示。
  - `Snapshot`：LLM 全部生成完成后，一次性显示。
- [x] `Snapshot` 是所有组件的基础能力（默认必须支持）。
- [x] `Streaming` 是否强制，按组件职责判断（不能一刀切）。

Button 归类为 `Streaming Optional` 且当前实现为 `N/A`（snapshot-only）。
