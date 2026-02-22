import { expect, test } from "@playwright/test";

const COLOR_HANDLE_PAGE = "/#/components/color-handle";

async function gotoColorHandle(page) {
  await page.goto(COLOR_HANDLE_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

async function resolveWorkbenchHandle(page) {
  const component = page.locator('[data-component="color-handle"]').first();
  await expect(component).toBeVisible();

  const root = component
    .locator('#docs-color-handle-workbench[data-slot="color-handle"]')
    .first();

  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-ui-schema", "ui.color-handle.agent-contract");
  await expect(root).toHaveAttribute("data-ui-stream-support", "optional");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");

  return { component, root };
}

test("docs-app color-handle contract uses semantic selectors with wasm-safe ready waits", async ({
  page,
}) => {
  await gotoColorHandle(page);
  const { root } = await resolveWorkbenchHandle(page);

  await expect(root).toHaveAttribute("data-state", "focused");
  await expect(root).toHaveAttribute("data-focused", "true");
  await expect(root).toHaveAttribute("data-ui-intent", "color-selection");
  await expect(root).toHaveAttribute("data-ui-action", "focus");
  await expect(root).toHaveAttribute("data-ui-state", "focused");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");
  await expect(root).toHaveAttribute("data-motion-source", /default|custom/);
});

test("docs-app color-handle flow uses semantic settled conditions for motion/drag state", async ({
  page,
}) => {
  await gotoColorHandle(page);
  const { component, root } = await resolveWorkbenchHandle(page);

  const controls = component.locator('[data-slot="color-handle-workbench-controls"]').first();
  const draggingToggle = controls.locator('[data-slot="color-handle-workbench-dragging"]').first();
  const showLoupeToggle = controls
    .locator('[data-slot="color-handle-workbench-show-loupe"]')
    .first();
  const focusedToggle = controls.locator('[data-slot="color-handle-workbench-focused"]').first();
  const disabledToggle = controls.locator('[data-slot="color-handle-workbench-disabled"]').first();
  const motionRange = controls.locator('[data-slot="color-handle-workbench-motion"]').first();

  await draggingToggle.check();
  await expect(root).toHaveAttribute("data-state", "dragging");
  await expect(root).toHaveAttribute("data-dragging", "true");
  await expect(root).toHaveAttribute("data-loupe-visible", "true");
  await expect(root).toHaveAttribute("data-ui-action", "drag-update");
  await expect(root).toHaveAttribute("data-ui-source", "drag-interaction");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");

  await motionRange.fill("420");
  await expect(root).toHaveAttribute("data-motion-source", "custom");
  await expect(root).toHaveAttribute("style", /--ui-color-handle-motion-duration:\s*420ms;/);

  await showLoupeToggle.uncheck();
  await expect(root).not.toHaveAttribute("data-loupe-visible", "true");

  await draggingToggle.uncheck();
  await expect(root).toHaveAttribute("data-state", "focused");
  await expect(root).toHaveAttribute("data-ui-action", "focus");

  await focusedToggle.uncheck();
  await expect(root).toHaveAttribute("data-state", "color");
  await expect(root).toHaveAttribute("data-ui-action", "initialize");

  await disabledToggle.check();
  await expect(root).toHaveAttribute("data-state", "disabled");
  await expect(root).toHaveAttribute("data-disabled", "true");
  await expect(root).toHaveAttribute("data-ui-state", "disabled");
  await expect(root).toHaveAttribute("data-ui-action", "initialize");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");
  await expect(root).not.toHaveAttribute("data-ui-capability-drag", "true");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedRoot = page
    .locator('[data-component="color-handle"] #docs-color-handle-workbench[data-slot="color-handle"]')
    .first();
  await expect(reloadedRoot).toHaveAttribute("data-state", "focused");
  await expect(reloadedRoot).toHaveAttribute("data-ui-action", "focus");
});

test("docs-app color-handle playground source is copy-paste ready", async ({ page }) => {
  await gotoColorHandle(page);

  const component = page.locator('[data-component="color-handle"]').first();
  await expect(component).toBeVisible();

  const playground = component
    .locator("section.playground")
    .filter({ has: page.locator("#docs-color-handle-workbench") })
    .first();
  await expect(playground).toBeVisible();

  const codeToggle = playground.getByRole("button", { name: /Show code|Hide code/ }).first();
  await expect(codeToggle).toBeVisible();

  const codeBlock = playground.locator('[data-slot="code-block"]');
  if ((await codeBlock.count()) === 0) {
    await codeToggle.click();
  }

  await expect(codeBlock.first()).toBeVisible();
  await expect(codeBlock.first()).toHaveAttribute("data-copyable", "true");

  const code = playground.locator('[data-slot="code-block-code"]').first();
  await expect(code).toContainText("use leptos::prelude::*;");
  await expect(code).toContainText("use ui::*;");
  await expect(code).toContainText("<ColorHandle");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);

  const copyReadyCard = component.locator('[data-slot="color-handle-copy-ready"]').first();
  await expect(copyReadyCard).toContainText("apps/docs-app/src/playground.rs::compose_copy_ready_code");
  await expect(copyReadyCard).toContainText("components/color-handle/src/view.rs");
  await expect(copyReadyCard).toContainText("components/color-handle/src/logic.rs");
  await expect(copyReadyCard).toContainText("components/color-handle/src/styles.rs");
});
