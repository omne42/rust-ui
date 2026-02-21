#!/usr/bin/env bash
set -euo pipefail

usage() {
  echo "用法:"
  echo "  $0 --keys <k1,k2,...> [--queue <path>] [--logs-dir <dir>] -- <codex_queue.sh 参数...>"
  echo "  $0 --keys-file <file> [--queue <path>] [--logs-dir <dir>] -- <codex_queue.sh 参数...>"
  echo ""
  echo "说明:"
  echo "  1. 批量模式会对不同 key 并行调用 codex_queue.sh。"
  echo "  2. queue 参数中不要传 --key，脚本会自动注入。"
  echo ""
  echo "示例:"
  echo "  $0 --keys keyA,keyB -- --jsonl prompts.jsonl --workdir /repo --tmpdir /tmp/codex"
  echo "  $0 --keys-file keys.txt -- --jsonl prompts.jsonl --workdir /repo"
}

trim() {
  local s="$1"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf '%s' "$s"
}

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
queue_path="$script_dir/codex_queue.sh"
keys_csv=""
keys_file=""
logs_dir=""
auto_logs_dir=0
keys=()
queue_args=()

while [ "$#" -gt 0 ]; do
  case "$1" in
    --keys)
      if [ "$#" -lt 2 ]; then
        echo "错误: --keys 需要参数" >&2
        exit 1
      fi
      keys_csv="$2"
      shift 2
      ;;
    --keys-file|--key-file)
      if [ "$#" -lt 2 ]; then
        echo "错误: --keys-file 需要文件路径" >&2
        exit 1
      fi
      keys_file="$2"
      shift 2
      ;;
    --queue)
      if [ "$#" -lt 2 ]; then
        echo "错误: --queue 需要脚本路径" >&2
        exit 1
      fi
      queue_path="$2"
      shift 2
      ;;
    --logs-dir)
      if [ "$#" -lt 2 ]; then
        echo "错误: --logs-dir 需要目录参数" >&2
        exit 1
      fi
      logs_dir="$2"
      shift 2
      ;;
    --)
      shift
      queue_args=("$@")
      break
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "错误: 未知参数 $1" >&2
      usage
      exit 1
      ;;
  esac
done

if [ -n "$keys_csv" ]; then
  IFS=',' read -r -a csv_items <<< "$keys_csv"
  for raw in "${csv_items[@]}"; do
    key="$(trim "$raw")"
    if [ -n "$key" ]; then
      keys+=("$key")
    fi
  done
fi

if [ -n "$keys_file" ]; then
  if [ ! -f "$keys_file" ]; then
    echo "错误: keys 文件不存在: $keys_file" >&2
    exit 1
  fi

  while IFS= read -r line || [ -n "$line" ]; do
    key="$(trim "$line")"
    if [ -z "$key" ]; then
      continue
    fi
    keys+=("$key")
  done < "$keys_file"
fi

if [ "${#keys[@]}" -eq 0 ]; then
  echo "错误: 没有可用 key，请传 --keys 或 --keys-file" >&2
  exit 1
fi

if [ "${#queue_args[@]}" -eq 0 ]; then
  echo "错误: 缺少 codex_queue.sh 参数，请使用 -- 后传入" >&2
  exit 1
fi

for arg in "${queue_args[@]}"; do
  if [ "$arg" = "--key" ]; then
    echo "错误: queue 参数中不允许 --key，key 由 codex_batch 统一注入" >&2
    exit 1
  fi
done

if [ ! -f "$queue_path" ]; then
  echo "错误: codex_queue.sh 不存在: $queue_path" >&2
  exit 1
fi

if [ -z "$logs_dir" ]; then
  logs_dir="$(mktemp -d -t codex-batch-XXXXXX)"
  auto_logs_dir=1
else
  mkdir -p "$logs_dir"
fi

run_queue() {
  local k="$1"
  if [ -x "$queue_path" ]; then
    "$queue_path" "${queue_args[@]}" --key "$k"
  else
    bash "$queue_path" "${queue_args[@]}" --key "$k"
  fi
}

sanitize_key_for_file() {
  local key="$1"
  key="${key//\//_}"
  key="${key//:/_}"
  key="${key// /_}"
  printf '%s' "$key" | tr -cd '[:alnum:]_.-'
}

declare -A pid_to_key=()
declare -A pid_to_log=()
last_started_pid=""

start_job() {
  local key="$1"
  local idx="$2"
  local safe_key log_file pid
  safe_key="$(sanitize_key_for_file "$key")"
  if [ -z "$safe_key" ]; then
    safe_key="key"
  fi
  log_file="$logs_dir/${idx}_${safe_key}.log"

  run_queue "$key" >"$log_file" 2>&1 &
  pid="$!"
  pid_to_key["$pid"]="$key"
  pid_to_log["$pid"]="$log_file"
  last_started_pid="$pid"
}

pids=()
for idx in "${!keys[@]}"; do
  key="${keys[$idx]}"
  start_job "$key" "$idx"
  pids+=("$last_started_pid")
done

failed=0
for pid in "${pids[@]}"; do
  if wait "$pid"; then
    rc=0
  else
    rc=$?
  fi

  key="${pid_to_key[$pid]:-unknown}"
  log_file="${pid_to_log[$pid]:-}"
  unset "pid_to_key[$pid]"
  unset "pid_to_log[$pid]"

  if [ "$rc" -ne 0 ]; then
    echo "错误: key 执行失败: $key (exit=$rc)" >&2
    if [ -n "$log_file" ] && [ -f "$log_file" ]; then
      echo "----- $key 最近日志 (tail -n 40) -----" >&2
      tail -n 40 "$log_file" >&2
      echo "----- 完整日志: $log_file -----" >&2
    fi
    failed=1
  fi
done

if [ "$failed" -eq 0 ] && [ "$auto_logs_dir" -eq 1 ]; then
  rm -rf "$logs_dir"
else
  echo "日志目录: $logs_dir"
fi

exit "$failed"
