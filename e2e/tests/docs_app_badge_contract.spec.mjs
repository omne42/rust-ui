import { expect, test } from "@playwright/test";

async function gotoBadgeDocsAndWaitSettled(page) {
  await page.goto("/#/components/badge");
  await page.locator("body:not(:has(#boot))").waitFor();
  const settledBadge = page.locator(
    '[data-component="badge"] [data-slot="badge"][data-ui-output-status="verified"]'
  );
  await expect(settledBadge.first()).toBeVisible();
  return settledBadge.first();
}

async function ensureWorkbenchControlsVisible(playground) {
  const controls = playground.locator('[data-slot="badge-workbench-controls"]');
  if ((await controls.count()) > 0) {
    await expect(controls.first()).toBeVisible();
    return controls.first();
  }

  const actionButtons = playground.locator(
    'button[data-slot="button"]:not([data-icon-only="true"])'
  );
  const actionCount = await actionButtons.count();
  for (let index = 0; index < actionCount; index += 1) {
    await actionButtons.nth(index).click();
    if ((await controls.count()) > 0) {
      await expect(controls.first()).toBeVisible();
      return controls.first();
    }
  }

  throw new Error("badge workbench controls should be reachable from playground actions");
}

test("docs-app badge contract uses semantic selectors with settled waits", async ({ page }) => {
  const badge = await gotoBadgeDocsAndWaitSettled(page);
  await expect(badge).toBeVisible();
  await expect(badge).toHaveAttribute("data-ui-schema", "ui.badge.agent-contract");
  await expect(badge).toHaveAttribute("data-ui-schema-version", "1");
  await expect(badge).toHaveAttribute("data-ui-intent", "status-display");
  await expect(badge).toHaveAttribute("data-ui-action", "initialize");
  await expect(badge).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(badge).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(badge).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(badge).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app badge key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await gotoBadgeDocsAndWaitSettled(page);

  const defaultBadge = page
    .locator('[data-component="badge"] [data-slot="badge"][data-variant="default"]')
    .first();
  await expect(defaultBadge).toHaveAttribute("data-fill", "solid");
  await expect(defaultBadge).toHaveAttribute("data-class-source", "default");

  const customBadge = page
    .locator('[data-component="badge"] [data-slot="badge"][data-class-source="custom"]')
    .first();
  await expect(customBadge).toBeVisible();
  await expect(customBadge).toHaveAttribute("data-custom-class", "true");
  await expect(customBadge).toHaveAttribute("data-ui-source", "custom");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  const reloaded = page
    .locator('[data-component="badge"] [data-slot="badge"][data-variant="default"]')
    .first();
  await expect(reloaded).toHaveAttribute("data-fill", "solid");
  await expect(reloaded).toHaveAttribute("data-class-source", "default");
});

test("docs-app badge playground source is copy-paste ready", async ({ page }) => {
  await gotoBadgeDocsAndWaitSettled(page);

  const playground = page.locator('[data-component="badge"] [data-slot="playground"]').first();
  await expect(playground).toBeVisible();

  const codeBlock = playground.locator('[data-slot="code-block"]');
  if ((await codeBlock.count()) === 0) {
    const actionButtons = playground.locator(
      'button[data-slot="button"]:not([data-icon-only="true"])'
    );
    const actionCount = await actionButtons.count();
    for (let index = 0; index < actionCount; index += 1) {
      await actionButtons.nth(index).click();
      if ((await codeBlock.count()) > 0) {
        break;
      }
    }
  }

  await expect(codeBlock.first()).toBeVisible();
  await expect(codeBlock.first()).toHaveAttribute("data-copyable", "true");
  const codeText = playground.locator('[data-slot="code-block-code"]').first();
  await expect(codeText).toContainText("use leptos::prelude::*;");
  await expect(codeText).toContainText("use ui_components::*;");
  await expect(codeText).toContainText("<Badge>");
});

test("docs-app badge workbench is interactive and updates preview semantics", async ({ page }) => {
  await gotoBadgeDocsAndWaitSettled(page);

  const playground = page
    .locator('[data-component="badge"] [data-slot="playground"]')
    .filter({ has: page.locator('[data-slot="badge-workbench-compare"]') })
    .first();
  await expect(playground).toBeVisible();

  const controls = await ensureWorkbenchControlsVisible(playground);
  const comparePrimaryRow = playground.locator('[data-slot="badge-workbench-compare"] .docs-row').first();
  const configuredBadge = comparePrimaryRow.locator('[data-slot="badge"]').nth(1);
  await expect(configuredBadge).toBeVisible();

  const segmentedControls = controls.locator('[data-slot="segmented-control"]');
  const variantControl = segmentedControls.nth(0);
  const localeControl = segmentedControls.nth(1);
  const customClassSwitch = controls.locator('[data-slot="switch"]').nth(0);
  const rtlSwitch = controls.locator('[data-slot="switch"]').nth(1);

  await variantControl.locator('[data-slot="segmented-control-option"][data-index="3"]').click();
  await expect(configuredBadge).toHaveAttribute("data-variant", "outline");
  await expect(configuredBadge).toHaveAttribute("data-fill", "outline");

  await customClassSwitch.click();
  await expect(configuredBadge).toHaveAttribute("data-class-source", "custom");
  await expect(configuredBadge).toHaveAttribute("data-ui-source", "custom");

  await localeControl.locator('[data-slot="segmented-control-option"][data-index="2"]').click();
  await expect(configuredBadge).toHaveAttribute("lang", "ar");

  await rtlSwitch.click();
  await expect(configuredBadge).toHaveAttribute("dir", "rtl");
});
