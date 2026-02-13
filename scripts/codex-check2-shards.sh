#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT_DIR"

MAX_PARALLEL="${1:-8}"
if ! [[ "$MAX_PARALLEL" =~ ^[0-9]+$ ]]; then
  echo "MAX_PARALLEL must be an integer" >&2
  exit 2
fi
if (( MAX_PARALLEL < 1 || MAX_PARALLEL > 20 )); then
  echo "MAX_PARALLEL must be within 1..20" >&2
  exit 2
fi

LOG_DIR="$ROOT_DIR/.codex-shards"
mkdir -p "$LOG_DIR"

mapfile -t COMPONENTS < <(find crates/ui-components/src -mindepth 1 -maxdepth 1 -type d -printf '%f\n' | sort)
if (( ${#COMPONENTS[@]} == 0 )); then
  echo "No component directories found." >&2
  exit 1
fi

TOTAL="${#COMPONENTS[@]}"
SHARDS="$MAX_PARALLEL"
if (( SHARDS > TOTAL )); then
  SHARDS="$TOTAL"
fi

echo "components=$TOTAL shards=$SHARDS log_dir=$LOG_DIR"

for ((i=0; i<SHARDS; i++)); do
  : > "$LOG_DIR/shard-$i.components"
done

for ((idx=0; idx<TOTAL; idx++)); do
  shard=$((idx % SHARDS))
  echo "${COMPONENTS[$idx]}" >> "$LOG_DIR/shard-$shard.components"
done

pids=()
for ((i=0; i<SHARDS; i++)); do
  comp_list="$(tr '\n' ',' < "$LOG_DIR/shard-$i.components" | sed 's/,$//')"
  prompt_file="$LOG_DIR/shard-$i.prompt.txt"
  out_file="$LOG_DIR/shard-$i.log"

  cat > "$prompt_file" <<PROMPT
你是 Rust UI 组件修复子任务（shard $i/$SHARDS）。

工作目录：$ROOT_DIR
负责组件：$comp_list

硬性要求：
1) 逐个组件真实检查并修复，依据该组件目录内 check2.md 的条目逐步打勾（只在真实满足后打勾）。
2) 严禁占位文件（placeholder/stub/todo shell）。新增文件必须是可维护的真实实现或真实转发实现。
3) 优先修复结构规范：logic.rs / styles.rs / view.rs / motion.rs、a11y 语义、命名一致性、受控/非受控契约、样式约束。
4) 只改你负责的组件目录及其必要关联测试文件；避免改动无关模块。
5) 严禁执行以下动作：cargo check / cargo test / cargo clippy、修改 CHANGELOG.md、git commit。
6) 在本 shard 完成后，只允许执行一次：cargo fmt --all。
7) 最终输出：本 shard 修复了哪些组件、哪些条目被打勾、剩余阻塞项是什么（若有）。

开始执行。
PROMPT

  (
    codex exec --full-auto --cd "$ROOT_DIR" "$(cat "$prompt_file")"
  ) > "$out_file" 2>&1 &
  pids+=("$!")
  echo "started shard=$i pid=${pids[-1]} log=$out_file"
done

failed=0
for p in "${pids[@]}"; do
  if ! wait "$p"; then
    failed=$((failed + 1))
  fi
done

echo "all shards completed. failed=$failed"
if (( failed > 0 )); then
  exit 1
fi
