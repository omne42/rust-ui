import { expect, test } from "@playwright/test";

async function gotoKeyboardDocsAndWaitSettled(page) {
  await page.goto("/#/components/keyboard");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="keyboard"]').first();
  await expect(docsRoot).toBeVisible();

  const settledKeyboard = docsRoot
    .locator(
      '[data-slot="keyboard"][data-ui-schema="ui.keyboard.agent-contract/v1"][data-ui-output-status="verified"]'
    )
    .first();
  await expect(settledKeyboard).toBeVisible();
  await expect(settledKeyboard).toHaveAttribute("data-ui-action", "render");
  await expect(settledKeyboard).toHaveAttribute("data-ui-state", /(default|muted|compact)/);

  return docsRoot;
}

test("docs-app keyboard uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const docsRoot = await gotoKeyboardDocsAndWaitSettled(page);

  const baseline = docsRoot
    .locator(
      '[data-slot="keyboard"][data-state="default"][data-aria-source="default"][data-class-source="default"]'
    )
    .first();
  await expect(baseline).toBeVisible();
  await expect(baseline).toHaveAttribute("aria-label", "Keyboard");

  const compact = docsRoot
    .locator('[data-slot="keyboard"][data-state="compact"][data-compact="true"]')
    .first();
  await expect(compact).toBeVisible();
  await expect(compact).toHaveAttribute("data-ui-state", "compact");

  const custom = docsRoot
    .locator('[data-slot="keyboard"][data-aria-source="custom"][data-class-source="custom"]')
    .first();
  await expect(custom).toBeVisible();
  await expect(custom).toHaveAttribute("aria-label", "Open command palette");
  await expect(custom).toHaveAttribute("data-ui-source", "custom");
});

async function runKeyboardWorkbenchFlow(page, docsRoot) {
  const workbench = docsRoot
    .locator('[data-slot="playground"]')
    .filter({ has: docsRoot.locator('[data-slot="segmented-control"]').first() })
    .first();
  await expect(workbench).toBeVisible();

  const toneControl = workbench.locator('[data-slot="segmented-control"]').nth(0);
  const keyTextControl = workbench.locator('[data-slot="segmented-control"]').nth(1);
  const compactSwitch = workbench.locator('[data-slot="switch"]').nth(0);
  const customAriaSwitch = workbench.locator('[data-slot="switch"]').nth(1);
  const customClassSwitch = workbench.locator('[data-slot="switch"]').nth(2);
  const preview = workbench.locator('[data-slot="keyboard"]').first();

  await expect(preview).toHaveAttribute("data-state", "default");
  await expect(preview).toHaveAttribute("data-ui-output-status", "verified");

  await toneControl
    .locator('[data-slot="segmented-control-option"][data-index="1"]')
    .first()
    .click();
  await expect(preview).toHaveAttribute("data-tone", "muted");
  await expect(preview).toHaveAttribute("data-state", "muted");

  await keyTextControl
    .locator('[data-slot="segmented-control-option"][data-index="1"]')
    .first()
    .click();
  await expect(preview).toContainText("Ctrl+Shift+P");

  // High-risk keyboard path: focus + Space instead of pointer-only toggles.
  await compactSwitch.focus();
  await expect(compactSwitch).toBeFocused();
  await page.keyboard.press("Space");
  await expect(preview).toHaveAttribute("data-state", "compact");
  await expect(preview).toHaveAttribute("data-compact", "true");

  await customAriaSwitch.focus();
  await expect(customAriaSwitch).toBeFocused();
  await page.keyboard.press("Space");
  await expect(preview).toHaveAttribute("data-aria-source", "custom");
  await expect(preview).toHaveAttribute("aria-label", "Open command palette");

  await customClassSwitch.focus();
  await expect(customClassSwitch).toBeFocused();
  await page.keyboard.press("Space");
  await expect(preview).toHaveAttribute("data-class-source", "custom");
  await expect(preview).toHaveAttribute("data-custom-class", "true");
  await expect(preview).toHaveAttribute("data-state", "compact");
}

test("docs-app keyboard workbench flow is repeatable with semantic ready/settled breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoKeyboardDocsAndWaitSettled(page);
  await runKeyboardWorkbenchFlow(page, docsRoot);

  await page.reload();
  const reloadedRoot = await gotoKeyboardDocsAndWaitSettled(page);
  await runKeyboardWorkbenchFlow(page, reloadedRoot);
});

test("docs-app keyboard source-first snippet is copy-paste ready with imports", async ({
  page,
}) => {
  const docsRoot = await gotoKeyboardDocsAndWaitSettled(page);
  const sourceFirst = docsRoot
    .locator('[data-slot="playground"]')
    .filter({ hasText: "Source-first Starter (Copy-Paste Ready)" })
    .first();
  await expect(sourceFirst).toBeVisible();
  await expect(sourceFirst).toContainText("Copy action auto-injects missing imports for direct run.");

  const codeBlock = sourceFirst.locator('[data-slot="code-block"]');
  if ((await codeBlock.count()) === 0) {
    const toggle = sourceFirst.getByRole("button", { name: /Show code|Hide code/ }).first();
    await toggle.click();
  }

  const visibleCode = sourceFirst.locator('[data-slot="code-block"]').first();
  await expect(visibleCode).toBeVisible();
  await expect(visibleCode).toHaveAttribute("data-copyable", "true");

  const codeText = sourceFirst.locator('[data-slot="code-block-code"]').first();
  await expect(codeText).toContainText("use leptos::prelude::*;");
  await expect(codeText).toContainText("use ui_components::{Keyboard, KeyboardTone};");
  await expect(codeText).toContainText("<Keyboard tone=KeyboardTone::Muted>\"⌥⇧P\"</Keyboard>");
});
