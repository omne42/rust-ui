#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

if ! command -v trunk >/dev/null 2>&1; then
  echo "ensure-trunk-error-details: trunk not found in PATH" >&2
  exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "ensure-trunk-error-details: cargo not found in PATH" >&2
  exit 1
fi

if command -v strings >/dev/null 2>&1; then
  if strings "$(command -v trunk)" | grep -F "cargo build stderr (tail):" >/dev/null 2>&1 \
    && strings "$(command -v trunk)" | grep -F "cargo artifacts stderr (tail):" >/dev/null 2>&1 \
    && strings "$(command -v trunk)" | grep -F "trunk-overlay-shell" >/dev/null 2>&1; then
    exit 0
  fi
fi

version="$(trunk --version | awk '{print $2}')"
source_dir=""
for candidate in "$HOME"/.cargo/registry/src/*/"trunk-$version"; do
  if [[ -d "$candidate" ]]; then
    source_dir="$candidate"
    break
  fi
done

if [[ -z "$source_dir" ]]; then
  cat >&2 <<EOF
ensure-trunk-error-details: cannot find trunk source for version $version.
Expected path pattern: \$HOME/.cargo/registry/src/*/trunk-$version
EOF
  exit 1
fi

target_file="$source_dir/src/pipelines/rust/mod.rs"
if [[ ! -f "$target_file" ]]; then
  echo "ensure-trunk-error-details: missing file $target_file" >&2
  exit 1
fi
autoreload_file="$source_dir/src/autoreload.js"
if [[ ! -f "$autoreload_file" ]]; then
  echo "ensure-trunk-error-details: missing file $autoreload_file" >&2
  exit 1
fi

if ! grep -Fq "cargo build stderr (tail):" "$target_file" \
  || ! grep -Fq "cargo artifacts stderr (tail):" "$target_file"; then
  python3 - "$target_file" <<'PY'
from pathlib import Path
import sys

path = Path(sys.argv[1])
source = path.read_text(encoding="utf-8")
updated = source

build_runner_old = """        let build_res = common::run_command("cargo", "cargo", &args, &self.cfg.working_directory)
            .await
            .context("error during cargo build execution");
"""
build_runner_new = """        let build_out = Command::new("cargo")
            .current_dir(&self.cfg.working_directory)
            .args(args.as_slice())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .context("error spawning cargo build task")?
            .wait_with_output()
            .await
            .context("error during cargo build execution")?;
"""

build_error_old = """        // Now propagate any errors which came from the cargo build.
        build_res?;
"""
build_error_new = """        // Now propagate any errors which came from the cargo build.
        if !build_out.status.success() {
            let stderr = String::from_utf8_lossy(&build_out.stderr);
            let stdout = String::from_utf8_lossy(&build_out.stdout);

            let stderr_tail = stderr
                .lines()
                .rev()
                .take(120)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\\n");
            let stdout_tail = stdout
                .lines()
                .rev()
                .take(40)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\\n");

            let mut details = String::new();
            if !stderr_tail.trim().is_empty() {
                details.push_str("cargo build stderr (tail):\\n");
                details.push_str(&stderr_tail);
            }
            if !stdout_tail.trim().is_empty() {
                if !details.is_empty() {
                    details.push_str("\\n\\n");
                }
                details.push_str("cargo build stdout (tail):\\n");
                details.push_str(&stdout_tail);
            }
            if details.is_empty() {
                details.push_str("cargo build failed with empty stdout/stderr");
            }

            bail!("cargo build returned a bad status: {}\\n\\n{details}", build_out.status);
        }
"""

artifacts_error_old = """        if !artifacts_out.status.success() {
            eprintln!("{}", String::from_utf8_lossy(&artifacts_out.stderr));
            bail!("bad status returned from cargo artifacts request");
        }
"""
artifacts_error_new = """        if !artifacts_out.status.success() {
            let stderr = String::from_utf8_lossy(&artifacts_out.stderr);
            let stdout = String::from_utf8_lossy(&artifacts_out.stdout);

            let stderr_tail = stderr
                .lines()
                .rev()
                .take(80)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\\n");
            let stdout_tail = stdout
                .lines()
                .rev()
                .take(40)
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .join("\\n");

            let mut details = String::new();
            if !stderr_tail.trim().is_empty() {
                details.push_str("cargo artifacts stderr (tail):\\n");
                details.push_str(&stderr_tail);
            }
            if !stdout_tail.trim().is_empty() {
                if !details.is_empty() {
                    details.push_str("\\n\\n");
                }
                details.push_str("cargo artifacts stdout (tail):\\n");
                details.push_str(&stdout_tail);
            }
            if details.is_empty() {
                details.push_str("cargo artifacts request failed with empty stdout/stderr");
            }

            bail!("bad status returned from cargo artifacts request\\n\\n{details}");
        }
"""

if "cargo build stderr (tail):" not in updated:
    if build_runner_old in updated:
        updated = updated.replace(build_runner_old, build_runner_new, 1)
    else:
        print(
            "ensure-trunk-error-details: warning: cannot find cargo build runner block",
            file=sys.stderr,
        )
    if build_error_old in updated:
        updated = updated.replace(build_error_old, build_error_new, 1)
    else:
        print(
            "ensure-trunk-error-details: warning: cannot find cargo build error block",
            file=sys.stderr,
        )

if "cargo artifacts stderr (tail):" not in updated:
    if artifacts_error_old in updated:
        updated = updated.replace(artifacts_error_old, artifacts_error_new, 1)
    else:
        print(
            "ensure-trunk-error-details: warning: cannot find cargo artifacts error block",
            file=sys.stderr,
        )

if updated != source:
    path.write_text(updated, encoding="utf-8")
PY
fi

if ! grep -Fq "trunk-overlay-shell" "$autoreload_file"; then
  python3 - "$autoreload_file" <<'PY'
import re
import sys
from pathlib import Path

path = Path(sys.argv[1])
source = path.read_text(encoding="utf-8")
overlay = """    class Overlay {
        constructor() {
            this._overlay = document.createElement("div");
            this._overlay.id = "trunk-build-failure-overlay";

            const style = this._overlay.style;
            style.position = "fixed";
            style.top = "0";
            style.left = "0";
            style.right = "0";
            style.bottom = "0";
            style.padding = "24px";
            style.display = "flex";
            style.alignItems = "center";
            style.justifyContent = "center";
            style.backgroundColor = "rgba(17, 24, 39, 0.72)";
            style.backdropFilter = "blur(6px)";
            style.fontFamily = "Inter, ui-sans-serif, system-ui, sans-serif";
            style.zIndex = "2147483647";
            style.pointerEvents = "auto";
            style.userSelect = "none";

            const shell = document.createElement("section");
            shell.className = "trunk-overlay-shell";
            shell.setAttribute("role", "dialog");
            shell.setAttribute("aria-live", "assertive");
            shell.style.width = "min(1100px, 96vw)";
            shell.style.maxHeight = "88vh";
            shell.style.display = "flex";
            shell.style.flexDirection = "column";
            shell.style.gap = "12px";
            shell.style.padding = "16px";
            shell.style.border = "1px solid #334155";
            shell.style.borderRadius = "12px";
            shell.style.background = "#0b1220";
            shell.style.color = "#e5e7eb";
            shell.style.boxShadow = "0 20px 80px rgba(2, 6, 23, 0.7)";

            const titleRow = document.createElement("div");
            titleRow.style.display = "flex";
            titleRow.style.alignItems = "center";
            titleRow.style.gap = "10px";

            this._title = document.createElement("div");
            this._title.innerText = "Build failure";
            this._title.style.fontSize = "20px";
            this._title.style.fontWeight = "700";
            this._title.style.letterSpacing = "0.1px";

            const icon = document.createElement("div");
            icon.innerHTML = '<svg xmlns="http://www.w3.org/2000/svg" width="22" height="22" fill="#ef4444" viewBox="0 0 16 16"><path d="M8.982 1.566a1.13 1.13 0 0 0-1.96 0L.165 13.233c-.457.778.091 1.767.98 1.767h13.713c.889 0 1.438-.99.98-1.767L8.982 1.566zM8 5c.535 0 .954.462.9.995l-.35 3.507a.552.552 0 0 1-1.1 0L7.1 5.995A.905.905 0 0 1 8 5zm.002 6a1 1 0 1 1 0 2 1 1 0 0 1 0-2z"/></svg>';
            titleRow.append(icon, this._title);

            const actions = document.createElement("div");
            actions.style.display = "flex";
            actions.style.alignItems = "center";
            actions.style.justifyContent = "space-between";
            actions.style.gap = "12px";

            const hint = document.createElement("div");
            hint.textContent = "Ctrl/Cmd + A selects only this error message.";
            hint.style.color = "#94a3b8";
            hint.style.fontSize = "12px";

            this._copyButton = document.createElement("button");
            this._copyButton.type = "button";
            this._copyButton.textContent = "Copy";
            this._copyButton.style.fontSize = "12px";
            this._copyButton.style.fontWeight = "600";
            this._copyButton.style.padding = "6px 10px";
            this._copyButton.style.borderRadius = "8px";
            this._copyButton.style.border = "1px solid #475569";
            this._copyButton.style.background = "#111827";
            this._copyButton.style.color = "#e5e7eb";
            this._copyButton.style.cursor = "pointer";
            this._copyButton.onclick = () => this._copyReason();
            actions.append(hint, this._copyButton);

            this._message = document.createElement("textarea");
            this._message.readOnly = true;
            this._message.wrap = "off";
            this._message.spellcheck = false;
            this._message.autocomplete = "off";
            this._message.style.width = "100%";
            this._message.style.flex = "1 1 auto";
            this._message.style.minHeight = "260px";
            this._message.style.maxHeight = "70vh";
            this._message.style.overflow = "auto";
            this._message.style.resize = "none";
            this._message.style.padding = "12px";
            this._message.style.border = "1px solid #334155";
            this._message.style.borderRadius = "10px";
            this._message.style.background = "#020617";
            this._message.style.color = "#e2e8f0";
            this._message.style.userSelect = "text";
            this._message.style.fontFamily = "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, Liberation Mono, monospace";
            this._message.style.fontSize = "12px";
            this._message.style.lineHeight = "1.45";

            shell.append(titleRow, actions, this._message);
            this._overlay.append(shell);

            this._inject();
            this._lockDocument();

            this._onKeyDown = (ev) => {
                const key = (ev.key || "").toLowerCase();
                if ((ev.ctrlKey || ev.metaKey) && key === "a") {
                    ev.preventDefault();
                    ev.stopPropagation();
                    this._message.focus();
                    this._message.select();
                }
            };
            window.addEventListener("keydown", this._onKeyDown, true);

            window.setInterval(() => {
                this._inject();
                this._lockDocument();
            }, 250);
        }

        set reason(reason) {
            this._message.value = reason || "";
            this._message.scrollTop = 0;
            this._message.focus();
            this._message.select();
        }

        _inject() {
            if (!this._overlay.isConnected) {
                document.body?.prepend(this._overlay);
            }
        }

        _lockDocument() {
            if (document.documentElement) {
                document.documentElement.style.overflow = "hidden";
                document.documentElement.style.userSelect = "none";
            }
            if (document.body) {
                document.body.style.overflow = "hidden";
                document.body.style.userSelect = "none";
            }
        }

        _copyReason() {
            const value = this._message.value || "";
            if (!value) {
                return;
            }

            const setCopied = () => {
                this._copyButton.textContent = "Copied";
                window.setTimeout(() => {
                    this._copyButton.textContent = "Copy";
                }, 1200);
            };

            if (navigator.clipboard && navigator.clipboard.writeText) {
                navigator.clipboard.writeText(value).then(setCopied).catch(() => {
                    this._message.focus();
                    this._message.select();
                    document.execCommand("copy");
                    setCopied();
                });
            } else {
                this._message.focus();
                this._message.select();
                document.execCommand("copy");
                setCopied();
            }
        }

    }"""

pattern = r"    class Overlay \{[\s\S]*?\n    \}\n\n    class Client \{"
replacement = overlay + "\n\n    class Client {"
patched, count = re.subn(pattern, replacement, source, count=1)
if count != 1:
    raise SystemExit("ensure-trunk-error-details: failed to locate Overlay block in autoreload.js")
path.write_text(patched, encoding="utf-8")
PY
fi

cargo install --path "$source_dir" --locked --force >/dev/null

if command -v strings >/dev/null 2>&1; then
  if ! strings "$(command -v trunk)" | grep -F "cargo build stderr (tail):" >/dev/null 2>&1 \
    || ! strings "$(command -v trunk)" | grep -F "cargo artifacts stderr (tail):" >/dev/null 2>&1 \
    || ! strings "$(command -v trunk)" | grep -F "trunk-overlay-shell" >/dev/null 2>&1; then
    echo "ensure-trunk-error-details: trunk reinstall finished but marker check failed" >&2
    exit 1
  fi
fi

echo "ensure-trunk-error-details: trunk patched for detailed buildFailure output." >&2
