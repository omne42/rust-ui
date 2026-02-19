import { expect, test } from "@playwright/test";

test("docs-app resizable contract exposes semantic markers with stable selectors", async ({
  page,
}) => {
  await page.goto("/#/components/resizable");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-component="resizable"]').first();
  await expect(root).toBeVisible();

  const splitter = root.locator('[data-slot="resizable"]').first();
  await expect(splitter).toBeVisible();
  await expect(splitter).toHaveAttribute("data-ui-schema", "ui.resizable.agent-contract.v1");
  await expect(splitter).toHaveAttribute("data-ui-intent", "adjust-split");
  await expect(splitter).toHaveAttribute("data-ui-action-model", "pointer+keyboard");
  await expect(splitter).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(splitter).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(splitter).toHaveAttribute("data-ui-stream-mode", "snapshot");
});

test("docs-app resizable key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/resizable");
  await page.locator("body:not(:has(#boot))").waitFor();

  const controlledRoot = page
    .locator('[data-slot="resizable"][data-control-mode="controlled"]')
    .first();
  await expect(controlledRoot).toBeVisible();
  const controlledHandle = controlledRoot.locator('[data-slot="resizable-handle"]').first();

  await expect(controlledRoot).toHaveAttribute("data-control-mode", "controlled");
  await expect(controlledRoot).toHaveAttribute("data-state", "idle");
  await expect(controlledRoot).toHaveAttribute("data-idle", "true");
  await expect(controlledRoot).not.toHaveAttribute("data-dragging", "true");
  await expect(controlledRoot).toHaveAttribute("data-orientation", "vertical");
  await expect(controlledHandle).toHaveAttribute("aria-valuenow", "58.00");

  await controlledHandle.focus();
  await expect(controlledHandle).toBeFocused();
  await page.keyboard.press("ArrowDown");

  await expect(controlledHandle).toHaveAttribute("aria-valuenow", "60.00");
  await expect(controlledRoot).toHaveAttribute("data-value-change-source", "on_value_change");
  await expect(controlledRoot).toHaveAttribute("data-state", "idle");
  await expect(controlledRoot).toHaveAttribute("data-idle", "true");
  await expect(controlledRoot).not.toHaveAttribute("data-dragging", "true");
});

test("docs-app resizable playground source remains copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/resizable");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="resizable"]').first();
  await expect(docsRoot).toBeVisible();

  const codeToggle = docsRoot.getByRole("button", { name: /Show code|Hide code/ }).first();
  await expect(codeToggle).toBeVisible();

  const codeBlock = docsRoot.locator('[data-slot="code-block"]');
  if ((await codeBlock.count()) === 0) {
    await codeToggle.click();
  }

  await expect(codeBlock.first()).toBeVisible();
  await expect(codeBlock.first()).toHaveAttribute("data-copyable", "true");

  const code = docsRoot.locator('[data-slot="code-block-code"]').first();
  await expect(code).toContainText("use leptos::prelude::*;");
  await expect(code).toContainText("use ui_layout::{Resizable, ResizableOrientation};");
  await expect(code).toContainText("<Resizable");
});
