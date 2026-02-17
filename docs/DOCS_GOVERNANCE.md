# 文档治理规范

本文定义仓库 Markdown 文档的组织方式与维护规则。

## 1. 适用范围

纳入治理：
- 根目录自有 Markdown（如 `README.md`、`CHANGELOG.md`、`todo.md`）
- `docs/**/*.md`
- `crates/*/README.md`
- `apps/*/README.md`

不纳入治理（外部文档）：
- `examples/_upstream/**`
- vendor 文档（如 `vendor/tachys/README.md`）

## 2. 文档分类（Taxonomy）

- Core：总览、规则、哲学、文档系统本身
- Spec：规格与约束（定义“做什么”）
- Plan：执行计划与任务分解（定义“怎么做”）
- Research：调研输入与参考资料（非最终规范）
- Package/App：各 crate/app 的使用文档
- External：第三方文档（登记但不治理）

## 3. 冲突时的优先级

当文档定义冲突时，按以下顺序执行：

1. `docs/philosophy.md`
2. `docs/RULES_ZH.md`
3. 对应 `docs/spec/*`
4. 对应 `docs/plan/*`
5. `docs/research/*`

## 4. 文档状态

索引中的状态统一使用：

- Active：当前有效并维护
- Draft：正在演进，结构可能变化
- Reference：参考性质，不是主决策来源
- Archive：历史归档，不用于新决策
- External：外部文档，不在本仓库治理范围

## 5. 放置与命名约定

- 设计/能力策略放 `docs/spec/`
- 执行/里程碑/任务放 `docs/plan/`
- 调研材料放 `docs/research/`
- crate/app 使用说明放各自目录 `README.md`

## 6. 变更规则

新增或修改 Markdown 时必须同步：

1. 更新 `docs/DOCS_INDEX.md`
2. 检查 `docs/README.md` 的导航是否仍有效
3. 若涉及行为契约变更，按优先级更新对应 spec/plan/rules
4. 若文档过时，必须在索引中调整状态（如转 Reference/Archive）

## 7. 合并前检查清单

- 链接可达
- 无重复或冲突的“单一真相”定义
- 文档放置在正确层级目录
- 关键交叉引用已更新（README/索引/相关规范）
- Research 文档未越权覆盖 Spec/Rules

## 8. 建议责任归属

- `docs/RULES_ZH.md`：架构维护者
- `docs/philosophy.md`：架构维护者
- `docs/spec/*`：对应能力 owner + 架构评审
- `docs/plan/*`：交付 owner
- `docs/research/*`：调研贡献者

## 9. 文档清点命令

仅统计仓库自有 Markdown（排除research mirror/vendor）：

```bash
rg --files -g '*.md' \
  -g '!examples/_upstream/**' \
  -g '!vendor/**' | sort
```

统计仓库内全部 Markdown（含外部目录）：

```bash
find . -name '*.md' -not -path '*/target/*' | sort
```
