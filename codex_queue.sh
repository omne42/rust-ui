#!/usr/bin/env bash
set -euo pipefail

replace_key=""
key_provided=0
workdir=""
tmpdir=""
max_retries="${CODEX_QUEUE_RETRIES:-3}"
retry_delay_sec="${CODEX_QUEUE_RETRY_DELAY_SEC:-2}"
last_cmd_output=""

usage() {
  echo "用法:"
  echo "  $0 <item1> [item2 ...]"
  echo "  $0 <item1> [item2 ...] [--workdir <dir>] [--tmpdir <dir>] [--retries <n>] [--retry-delay <sec>]"
  echo "  $0 --jsonl <file.jsonl> [--key <key>] [--workdir <dir>] [--tmpdir <dir>] [--retries <n>] [--retry-delay <sec>]"
  echo "示例:"
  echo "  $0 \"只回复一个字符：1\" \"只回复一个字符：2\" \"只回复一个字符：3\" --workdir /path/to/repo --tmpdir /tmp/mytmp"
  echo "  $0 --jsonl prompts.jsonl --key abc123 --workdir /path/to/repo --tmpdir /tmp/mytmp --retries 3 --retry-delay 2"
  echo "说明:"
  echo "  jsonl 中可使用占位符 \$\$\$KEY\$\$\$，通过 --key 传入替换值。"
}

is_positive_int() {
  [[ "$1" =~ ^[1-9][0-9]*$ ]]
}

is_nonnegative_int() {
  [[ "$1" =~ ^[0-9]+$ ]]
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

format_cmd() {
  local part out=""
  for part in "$@"; do
    out+=$(printf '%q ' "$part")
  done
  printf '%s' "${out% }"
}

run_with_retry_capture() {
  local phase="$1"
  local idx="$2"
  local total="$3"
  local prompt="$4"
  shift 4

  local attempt=1
  local rc=0
  local tmp_out cmd_desc
  cmd_desc="$(format_cmd "$@")"

  while [ "$attempt" -le "$max_retries" ]; do
    tmp_out="$(mktemp)"
    echo "[queue] [$idx/$total] $phase 尝试 $attempt/$max_retries: $(prompt_preview "$prompt")" >&2

    if "$@" >"$tmp_out" 2>&1; then
      last_cmd_output="$(cat "$tmp_out")"
      printf '%s\n' "$last_cmd_output"
      rm -f "$tmp_out"
      return 0
    else
      rc=$?
      echo "[queue] [$idx/$total] $phase 失败: exit=$rc" >&2
      echo "[queue] 命令: $cmd_desc" >&2
      echo "----- 失败日志 (tail -n 60) -----" >&2
      tail -n 60 "$tmp_out" >&2 || true
      echo "----- 失败日志结束 -----" >&2
      rm -f "$tmp_out"

      if [ "$attempt" -lt "$max_retries" ]; then
        echo "[queue] [$idx/$total] 将在 ${retry_delay_sec}s 后重试" >&2
        sleep "$retry_delay_sec"
      fi
    fi
    ((attempt += 1))
  done

  return 1
}

load_prompts_from_jsonl() {
  local file="$1"
  local line_no=0
  local line prompt placeholder
  local tmp_prompts
  placeholder='$$$KEY$$$'

  if [ ! -f "$file" ]; then
    echo "错误: jsonl 文件不存在: $file" >&2
    exit 1
  fi

  if command -v jq >/dev/null 2>&1; then
    while IFS= read -r line || [ -n "$line" ]; do
      ((line_no += 1))

      # 跳过空行
      if [ -z "${line//[[:space:]]/}" ]; then
        continue
      fi

      if ! prompt="$(printf '%s' "$line" | jq -er '.prompt | strings')" ; then
        echo "错误: 第 $line_no 行不是合法的 {\"prompt\":\"...\"} JSON 对象" >&2
        exit 1
      fi

      if [ -z "$prompt" ]; then
        echo "错误: 第 $line_no 行 prompt 为空字符串" >&2
        exit 1
      fi

      if [[ "$prompt" == *"$placeholder"* ]]; then
        if [ "$key_provided" -ne 1 ]; then
          echo "错误: 第 $line_no 行包含占位符 \$\$\$KEY\$\$\$，但未传 --key" >&2
          exit 1
        fi
        prompt="${prompt//\$\$\$KEY\$\$\$/$replace_key}"
      fi

      items+=("$prompt")
    done < "$file"
    return
  fi

  if ! command -v python3 >/dev/null 2>&1; then
    echo "错误: 解析 jsonl 需要 jq 或 python3" >&2
    exit 1
  fi

  tmp_prompts="$(mktemp)"
  if ! python3 - "$file" "$replace_key" "$key_provided" > "$tmp_prompts" <<'PY'
import json
import sys

path = sys.argv[1]
replace_key = sys.argv[2]
key_provided = sys.argv[3] == "1"
placeholder = "$$$KEY$$$"
with open(path, "r", encoding="utf-8") as f:
    for line_no, raw in enumerate(f, 1):
        if not raw.strip():
            continue
        try:
            data = json.loads(raw)
        except json.JSONDecodeError:
            print(f"错误: 第 {line_no} 行不是合法的 JSON", file=sys.stderr)
            sys.exit(1)

        prompt = data.get("prompt")
        if not isinstance(prompt, str):
            print(f"错误: 第 {line_no} 行缺少字符串类型 prompt", file=sys.stderr)
            sys.exit(1)
        if prompt == "":
            print(f"错误: 第 {line_no} 行 prompt 为空字符串", file=sys.stderr)
            sys.exit(1)

        if placeholder in prompt:
            if not key_provided:
                print(f"错误: 第 {line_no} 行包含占位符 $$$KEY$$$，但未传 --key", file=sys.stderr)
                sys.exit(1)
            prompt = prompt.replace(placeholder, replace_key)

        print(prompt)
PY
  then
    rm -f "$tmp_prompts"
    exit 1
  fi

  while IFS= read -r prompt || [ -n "$prompt" ]; do
    items+=("$prompt")
  done < "$tmp_prompts"
  rm -f "$tmp_prompts"
}

sid=""
jsonl_file=""

if [ "$#" -eq 0 ]; then
  usage
  exit 1
fi

items=()
while [ "$#" -gt 0 ]; do
  case "$1" in
    --jsonl)
      if [ "$#" -lt 2 ]; then
        echo "错误: --jsonl 需要文件参数" >&2
        exit 1
      fi
      jsonl_file="$2"
      shift 2
      ;;
    --key)
      if [ "$#" -lt 2 ]; then
        echo "错误: --key 需要 key 参数" >&2
        exit 1
      fi
      replace_key="$2"
      key_provided=1
      shift 2
      ;;
    --workdir|-C|--cd)
      if [ "$#" -lt 2 ]; then
        echo "错误: --workdir 需要目录参数" >&2
        exit 1
      fi
      workdir="$2"
      shift 2
      ;;
    --tmpdir|--tmp-dir)
      if [ "$#" -lt 2 ]; then
        echo "错误: --tmpdir 需要目录参数" >&2
        exit 1
      fi
      tmpdir="$2"
      shift 2
      ;;
    --retries)
      if [ "$#" -lt 2 ]; then
        echo "错误: --retries 需要数字参数" >&2
        exit 1
      fi
      max_retries="$2"
      shift 2
      ;;
    --retry-delay)
      if [ "$#" -lt 2 ]; then
        echo "错误: --retry-delay 需要秒数参数" >&2
        exit 1
      fi
      retry_delay_sec="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      items+=("$1")
      shift
      ;;
  esac
done

if [ -n "$jsonl_file" ] && [ "${#items[@]}" -gt 0 ]; then
  echo "错误: 使用 --jsonl 时不要再传入额外的 prompt 参数" >&2
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

if [ -n "$jsonl_file" ]; then
  items=()
  load_prompts_from_jsonl "$jsonl_file"
elif [ "${#items[@]}" -eq 1 ] && [ -f "${items[0]}" ] && [[ "${items[0]}" == *.jsonl ]]; then
  jsonl_file="${items[0]}"
  items=()
  load_prompts_from_jsonl "$jsonl_file"
fi

if [ "${#items[@]}" -eq 0 ]; then
  echo "错误: 没有可执行的 prompt" >&2
  exit 1
fi

if ! is_positive_int "$max_retries"; then
  echo "错误: --retries 必须是 >=1 的整数，当前值: $max_retries" >&2
  exit 1
fi

if ! is_nonnegative_int "$retry_delay_sec"; then
  echo "错误: --retry-delay 必须是 >=0 的整数秒，当前值: $retry_delay_sec" >&2
  exit 1
fi

if [ -n "$workdir" ] && [ ! -d "$workdir" ]; then
  echo "错误: workdir 不存在或不是目录: $workdir" >&2
  exit 1
fi

codex_base=(codex)
if [ -n "$workdir" ]; then
  codex_base+=(-C "$workdir")
fi

total_items="${#items[@]}"
for idx in "${!items[@]}"; do
  item="${items[$idx]}"
  order="$((idx + 1))"

  if [ "$idx" -eq 0 ]; then
    if ! run_with_retry_capture "exec" "$order" "$total_items" "$item" "${codex_base[@]}" exec --skip-git-repo-check --color never -- "$item"; then
      echo "错误: 第 $order 条 prompt 执行失败，重试 $max_retries 次后仍失败" >&2
      exit 1
    fi

    first_output="$last_cmd_output"
    sid="$(printf '%s\n' "$first_output" | rg '^session id:' | awk '{print $3}' | tail -n1)"

    if [ -z "$sid" ]; then
      echo "错误: 第 $order 条 prompt 成功返回但未解析到 session id" >&2
      echo "提示: 可能是 codex 输出格式变化，检查该条输出中的 'session id:' 行" >&2
      exit 1
    fi
  else
    if ! run_with_retry_capture "resume" "$order" "$total_items" "$item" "${codex_base[@]}" exec resume --skip-git-repo-check "$sid" -- "$item"; then
      echo "错误: 第 $order 条 prompt 执行失败，session id: $sid" >&2
      exit 1
    fi
  fi
done
