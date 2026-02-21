#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
用法:
  codex_recover_stalled.sh --logs-dir <dir> --jsonl <file> [--stale-minutes <n>] [--workdir <dir>] [--tmpdir <dir>] [--retries <n>] [--retry-delay <sec>] [--dry-run]

说明:
  1. 扫描 logs-dir 下超过 stale-minutes 未更新的 .log。
  2. 从日志中提取 key / session id / [queue] 进度。
  3. 从断点继续执行剩余 prompt，并追加写回原日志文件。

示例:
  ./codex_recover_stalled.sh \
    --logs-dir /root/autodl-tmp/tmp/codex-batch-logs \
    --jsonl /root/code/personal/omne/prompts.jsonl \
    --stale-minutes 20 \
    --workdir /root/autodl-tmp/zjj/p/rust-ui \
    --tmpdir /root/autodl-tmp/tmp
USAGE
}

is_positive_int() {
  [[ "$1" =~ ^[1-9][0-9]*$ ]]
}

is_nonnegative_int() {
  [[ "$1" =~ ^[0-9]+$ ]]
}

trim() {
  local s="$1"
  s="${s#"${s%%[![:space:]]*}"}"
  s="${s%"${s##*[![:space:]]}"}"
  printf '%s' "$s"
}

format_cmd() {
  local part out=""
  for part in "$@"; do
    out+=$(printf '%q ' "$part")
  done
  printf '%s' "${out% }"
}

prompt_preview() {
  local text="$1"
  local normalized
  normalized="${text//$'\n'/ }"
  normalized="${normalized//$'\r'/ }"
  if [ "${#normalized}" -gt 120 ]; then
    printf '%s...' "${normalized:0:120}"
  else
    printf '%s' "$normalized"
  fi
}

logs_dir=""
jsonl_file=""
stale_minutes=20
workdir=""
tmpdir=""
max_retries="${CODEX_QUEUE_RETRIES:-3}"
retry_delay_sec="${CODEX_QUEUE_RETRY_DELAY_SEC:-2}"
dry_run=0

while [ "$#" -gt 0 ]; do
  case "$1" in
    --logs-dir)
      [ "$#" -ge 2 ] || { echo "错误: --logs-dir 需要目录参数" >&2; exit 1; }
      logs_dir="$2"
      shift 2
      ;;
    --jsonl)
      [ "$#" -ge 2 ] || { echo "错误: --jsonl 需要文件参数" >&2; exit 1; }
      jsonl_file="$2"
      shift 2
      ;;
    --stale-minutes)
      [ "$#" -ge 2 ] || { echo "错误: --stale-minutes 需要数字参数" >&2; exit 1; }
      stale_minutes="$2"
      shift 2
      ;;
    --workdir|-C|--cd)
      [ "$#" -ge 2 ] || { echo "错误: --workdir 需要目录参数" >&2; exit 1; }
      workdir="$2"
      shift 2
      ;;
    --tmpdir|--tmp-dir)
      [ "$#" -ge 2 ] || { echo "错误: --tmpdir 需要目录参数" >&2; exit 1; }
      tmpdir="$2"
      shift 2
      ;;
    --retries)
      [ "$#" -ge 2 ] || { echo "错误: --retries 需要数字参数" >&2; exit 1; }
      max_retries="$2"
      shift 2
      ;;
    --retry-delay)
      [ "$#" -ge 2 ] || { echo "错误: --retry-delay 需要秒数参数" >&2; exit 1; }
      retry_delay_sec="$2"
      shift 2
      ;;
    --dry-run)
      dry_run=1
      shift
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

[ -n "$logs_dir" ] || { echo "错误: 必须传 --logs-dir" >&2; exit 1; }
[ -n "$jsonl_file" ] || { echo "错误: 必须传 --jsonl" >&2; exit 1; }
[ -d "$logs_dir" ] || { echo "错误: logs-dir 不存在: $logs_dir" >&2; exit 1; }
[ -f "$jsonl_file" ] || { echo "错误: jsonl 文件不存在: $jsonl_file" >&2; exit 1; }

is_positive_int "$stale_minutes" || { echo "错误: --stale-minutes 必须是 >=1 的整数" >&2; exit 1; }
is_positive_int "$max_retries" || { echo "错误: --retries 必须是 >=1 的整数" >&2; exit 1; }
is_nonnegative_int "$retry_delay_sec" || { echo "错误: --retry-delay 必须是 >=0 的整数秒" >&2; exit 1; }

if [ -n "$workdir" ] && [ ! -d "$workdir" ]; then
  echo "错误: workdir 不存在或不是目录: $workdir" >&2
  exit 1
fi

if [ -n "$tmpdir" ] && [ ! -d "$tmpdir" ]; then
  echo "错误: tmpdir 不存在或不是目录: $tmpdir" >&2
  exit 1
fi

if [ -n "$tmpdir" ]; then
  export TMPDIR="$tmpdir"
  export TMP="$tmpdir"
  export TEMP="$tmpdir"
fi

mapfile -d '' -t base_prompts < <(python3 - "$jsonl_file" <<'PY'
import json
import sys
from pathlib import Path

p = Path(sys.argv[1])
for line_no, raw in enumerate(p.read_text(encoding='utf-8').splitlines(), 1):
    if not raw.strip():
        continue
    try:
        obj = json.loads(raw)
    except json.JSONDecodeError as e:
        print(f"错误: jsonl 第 {line_no} 行不是合法 JSON: {e}", file=sys.stderr)
        sys.exit(1)
    prompt = obj.get('prompt')
    if not isinstance(prompt, str) or prompt == '':
        print(f"错误: jsonl 第 {line_no} 行缺少非空字符串 prompt", file=sys.stderr)
        sys.exit(1)
    sys.stdout.buffer.write(prompt.encode('utf-8'))
    sys.stdout.buffer.write(b'\0')
PY
)

if [ "${#base_prompts[@]}" -eq 0 ]; then
  echo "错误: jsonl 中没有可执行 prompt" >&2
  exit 1
fi

extract_key() {
  local log="$1"
  local key
  key="$(rg -m1 -o '现在我们处理[^，,。[:space:]]+' "$log" 2>/dev/null | sed 's/^现在我们处理//' | head -n1 || true)"
  key="$(trim "$key")"

  if [ -n "$key" ]; then
    printf '%s' "$key"
    return 0
  fi

  local base guess
  base="$(basename "$log" .log)"
  guess="${base#*_}"
  if [ -n "$guess" ]; then
    guess="${guess/_//}"
    printf '%s' "$guess"
    return 0
  fi

  return 1
}

extract_last_session() {
  local log="$1"
  rg -o 'session id: [0-9a-fA-F-]{36}' "$log" 2>/dev/null | awk '{print $3}' | tail -n1
}

extract_last_progress() {
  local log="$1"
  local line
  line="$(rg '\[queue\] \[[0-9]+/[0-9]+\] (exec|resume) 尝试' "$log" 2>/dev/null | tail -n1 || true)"
  if [ -z "$line" ]; then
    echo "0 0"
    return 0
  fi
  printf '%s\n' "$line" | sed -E 's/.*\[([0-9]+)\/([0-9]+)\].*/\1 \2/'
}

build_prompts_for_key() {
  local key="$1"
  local prompt
  prompts_for_key=()

  for prompt in "${base_prompts[@]}"; do
    if [[ "$prompt" == *'$$$KEY$$$'* ]]; then
      if [ -z "$key" ]; then
        echo "错误: prompt 使用了 $$$KEY$$$ 占位符，但日志中无法提取 key" >&2
        return 1
      fi
      prompts_for_key+=("${prompt//\$\$\$KEY\$\$\$/$key}")
    else
      prompts_for_key+=("$prompt")
    fi
  done
}

run_with_retry_capture() {
  local log_file="$1"
  local phase="$2"
  local idx="$3"
  local total="$4"
  local prompt="$5"
  shift 5

  local attempt=1
  local rc=0
  local tmp_out cmd_desc
  cmd_desc="$(format_cmd "$@")"

  while [ "$attempt" -le "$max_retries" ]; do
    tmp_out="$(mktemp)"
    printf '[queue] [%s/%s] %s 尝试 %s/%s: %s\n' "$idx" "$total" "$phase" "$attempt" "$max_retries" "$(prompt_preview "$prompt")" | tee -a "$log_file"

    if "$@" >"$tmp_out" 2>&1; then
      last_cmd_output="$(cat "$tmp_out")"
      cat "$tmp_out" | tee -a "$log_file"
      rm -f "$tmp_out"
      return 0
    else
      rc=$?
      printf '[queue] [%s/%s] %s 失败: exit=%s\n' "$idx" "$total" "$phase" "$rc" | tee -a "$log_file"
      printf '[queue] 命令: %s\n' "$cmd_desc" | tee -a "$log_file"
      echo '----- 失败日志 (tail -n 60) -----' | tee -a "$log_file"
      tail -n 60 "$tmp_out" | tee -a "$log_file"
      echo '----- 失败日志结束 -----' | tee -a "$log_file"
      rm -f "$tmp_out"

      if [ "$attempt" -lt "$max_retries" ]; then
        printf '[queue] [%s/%s] 将在 %ss 后重试\n' "$idx" "$total" "$retry_delay_sec" | tee -a "$log_file"
        sleep "$retry_delay_sec"
      fi
    fi

    ((attempt += 1))
  done

  return 1
}

codex_base=(codex)
if [ -n "$workdir" ]; then
  codex_base+=(-C "$workdir")
fi

mapfile -t stale_logs < <(find "$logs_dir" -maxdepth 1 -type f -name '*.log' -mmin "+$stale_minutes" | sort)

if [ "${#stale_logs[@]}" -eq 0 ]; then
  echo "未发现超过 ${stale_minutes} 分钟未更新的日志文件。"
  exit 0
fi

printf '发现 %s 个超过 %s 分钟未更新的日志，开始检查恢复。\n' "${#stale_logs[@]}" "$stale_minutes"

process_one_log() {
  local log="$1"
  key="$(extract_key "$log" || true)"
  sid="$(extract_last_session "$log" || true)"
  read -r last_idx last_total <<<"$(extract_last_progress "$log")"

  if ! build_prompts_for_key "$key"; then
    return 1
  fi
  total_prompts="${#prompts_for_key[@]}"

  append_to_log=1
  if [ "$dry_run" -eq 1 ]; then
    append_to_log=0
  fi

  if [ "$last_total" -ne 0 ] && [ "$last_total" -ne "$total_prompts" ]; then
    if [ "$append_to_log" -eq 1 ]; then
      printf '[recover] 警告: %s 记录总数=%s, 当前 jsonl 总数=%s\n' "$log" "$last_total" "$total_prompts" | tee -a "$log"
    else
      printf '[recover] 警告: %s 记录总数=%s, 当前 jsonl 总数=%s\n' "$log" "$last_total" "$total_prompts"
    fi
  fi

  if [ "$last_idx" -le 0 ]; then
    start_idx=1
  else
    start_idx="$last_idx"
  fi

  if [ "$start_idx" -gt "$total_prompts" ]; then
    if [ "$append_to_log" -eq 1 ]; then
      printf '[recover] 跳过: %s 进度 %s/%s 已超出当前 prompts 总数 %s\n' "$log" "$start_idx" "$last_total" "$total_prompts" | tee -a "$log"
    else
      printf '[recover] 跳过: %s 进度 %s/%s 已超出当前 prompts 总数 %s\n' "$log" "$start_idx" "$last_total" "$total_prompts"
    fi
    return 0
  fi

  if [ "$start_idx" -eq 1 ] && [ -n "$sid" ]; then
    # 通常意味着第一条 exec 已经成功，可从第二条继续。
    if [ "$total_prompts" -ge 2 ]; then
      start_idx=2
    fi
  fi

  printf '[recover] 目标日志: %s\n' "$log"
  printf '[recover] key=%s, session=%s, 从第 %s/%s 条继续\n' "${key:-<unknown>}" "${sid:-<none>}" "$start_idx" "$total_prompts"
  if [ "$append_to_log" -eq 1 ]; then
    printf '[recover] key=%s, session=%s, 从第 %s/%s 条继续\n' "${key:-<unknown>}" "${sid:-<none>}" "$start_idx" "$total_prompts" >>"$log"
  fi

  if [ "$dry_run" -eq 1 ]; then
    return 0
  fi

  last_cmd_output=""

  if [ "$start_idx" -eq 1 ]; then
    first_prompt="${prompts_for_key[0]}"
    if ! run_with_retry_capture "$log" "exec" 1 "$total_prompts" "$first_prompt" "${codex_base[@]}" exec --skip-git-repo-check --color never -- "$first_prompt"; then
      echo "[recover] 失败: 第 1 条执行失败" | tee -a "$log"
      return 1
    fi

    sid="$(printf '%s\n' "$last_cmd_output" | rg '^session id:' | awk '{print $3}' | tail -n1)"
    if [ -z "$sid" ]; then
      echo "[recover] 失败: 无法从第 1 条输出解析 session id" | tee -a "$log"
      return 1
    fi

    start_idx=2
  fi

  if [ -z "$sid" ]; then
    echo "[recover] 失败: 缺少 session id，无法 resume。" | tee -a "$log"
    return 1
  fi

  for ((order=start_idx; order<=total_prompts; order++)); do
    prompt="${prompts_for_key[$((order-1))]}"
    if ! run_with_retry_capture "$log" "resume" "$order" "$total_prompts" "$prompt" "${codex_base[@]}" exec resume --skip-git-repo-check "$sid" -- "$prompt"; then
      echo "[recover] 失败: 第 $order 条恢复失败，session id: $sid" | tee -a "$log"
      return 1
    fi
  done

  return 0
}

any_failed=0
pids=()
pid_logs=()
for log in "${stale_logs[@]}"; do
  process_one_log "$log" &
  pids+=("$!")
  pid_logs+=("$log")
done

for idx in "${!pids[@]}"; do
  pid="${pids[$idx]}"
  log="${pid_logs[$idx]}"
  if ! wait "$pid"; then
    echo "[recover] 失败日志: $log" >&2
    any_failed=1
  fi
done

if [ "$any_failed" -ne 0 ]; then
  exit 1
fi

echo "恢复任务执行完成。"
