import { expect, test } from "@playwright/test";

test("docs-app drop-zone uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  await page.goto("/#/components/drop-zone");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="drop-zone"]').first();
  await expect(docsRoot).toBeVisible();

  const quickStartRoot = docsRoot
    .locator('[data-slot="drop-zone-e2e-quick-start"] [data-slot="drop-zone"]')
    .first();
  await expect(quickStartRoot).toBeVisible();
  await expect(quickStartRoot).toHaveAttribute("data-motion-source", "default");
  await expect(quickStartRoot).toHaveAttribute("data-disabled-source", "default");

  const disabledRoot = docsRoot
    .locator('[data-slot="drop-zone-e2e-state-disabled"] [data-slot="drop-zone"]')
    .first();
  await expect(disabledRoot).toBeVisible();
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("data-disabled-source", "is_disabled");

  const customMotionRoot = docsRoot
    .locator('[data-slot="drop-zone-e2e-state-custom-motion"] [data-slot="drop-zone"]')
    .first();
  await expect(customMotionRoot).toBeVisible();
  await expect(customMotionRoot).toHaveAttribute("data-motion-source", "custom");
});

test("docs-app drop-zone motion interaction uses semantic ready and settled breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/drop-zone");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="drop-zone"]').first();
  await expect(docsRoot).toBeVisible();

  const workbench = docsRoot.locator('[data-slot="drop-zone-workbench"]').first();
  const workbenchRoot = workbench
    .locator('[data-slot="drop-zone-workbench-surface"] [data-slot="drop-zone"]')
    .first();
  await expect(workbenchRoot).toBeVisible();
  await expect(workbenchRoot).toHaveAttribute("data-drag-phase", "idle");
  await expect(workbenchRoot).toHaveAttribute("data-motion-source", "default");

  const customMotionToggle = workbench
    .locator('[data-slot="drop-zone-workbench-toggle-custom-motion"] [data-slot="switch"]')
    .first();
  await expect(customMotionToggle).toBeVisible();

  await customMotionToggle.click();
  await expect(workbenchRoot).toHaveAttribute("data-motion-source", "custom");
  await expect(workbenchRoot).toHaveAttribute("data-drag-phase", "idle");

  await customMotionToggle.click();
  await expect(workbenchRoot).toHaveAttribute("data-motion-source", "default");
  await expect(workbenchRoot).toHaveAttribute("data-drag-phase", "idle");
});

test("docs-app drop-zone key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/drop-zone");
  await page.locator("body:not(:has(#boot))").waitFor();

  await page.evaluate(() => {
    window.localStorage.removeItem("docs:drop-zone:workbench:state");
  });
  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="drop-zone"]').first();
  await expect(docsRoot).toBeVisible();

  const workbench = docsRoot.locator('[data-slot="drop-zone-workbench"]').first();
  const workbenchRoot = workbench
    .locator('[data-slot="drop-zone-workbench-surface"] [data-slot="drop-zone"]')
    .first();
  const zone = workbenchRoot.locator('[data-slot="drop-zone-zone"]').first();
  const zoneButton = zone.locator('[data-slot="drop-zone-button"]').first();
  const disabledToggle = workbench
    .locator('[data-slot="drop-zone-workbench-toggle-disabled"] [data-slot="switch"]')
    .first();

  await expect(workbenchRoot).toHaveAttribute("data-disabled", "false");
  await expect(zone).toHaveAttribute("data-drag-phase", "idle");

  for (const cycle of [1, 2]) {
    await test.step(`drop-zone key flow cycle ${cycle}`, async () => {
      await disabledToggle.focus();
      await expect(disabledToggle).toBeFocused();
      await page.keyboard.press("Enter");

      await expect(workbenchRoot).toHaveAttribute("data-disabled", "true");
      await expect(workbenchRoot).toHaveAttribute("data-disabled-source", "is_disabled");
      await expect(zone).toHaveAttribute("aria-disabled", "true");

      await zoneButton.focus();
      await expect(zoneButton).toBeFocused();
      await expect(zone).toHaveAttribute("data-focused", "true");

      await disabledToggle.focus();
      await expect(disabledToggle).toBeFocused();
      await page.keyboard.press("Enter");

      await expect(workbenchRoot).toHaveAttribute("data-disabled", "false");
      await expect(workbenchRoot).toHaveAttribute("data-disabled-source", "is_disabled");
      await expect(zone).toHaveAttribute("aria-disabled", "false");
      await expect(zone).toHaveAttribute("data-focused", "false");
      await expect(zone).toHaveAttribute("data-drag-phase", "idle");
    });
  }

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  const reloadedWorkbenchRoot = page
    .locator('[data-slot="drop-zone-workbench-surface"] [data-slot="drop-zone"]')
    .first();
  await expect(reloadedWorkbenchRoot).toHaveAttribute("data-disabled", "false");
  await expect(reloadedWorkbenchRoot).toHaveAttribute("data-drag-phase", "idle");
});

test("docs-app drop-zone source-first docs are copy-paste ready and traceable", async ({
  page,
}) => {
  await page.goto("/#/components/drop-zone");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="drop-zone"]').first();
  await expect(docsRoot).toBeVisible();

  const sourceFirst = docsRoot.locator('[data-slot="drop-zone-source-first"]').first();
  await expect(sourceFirst).toBeVisible();
  await expect(sourceFirst).toContainText("component-drop_zone");
  await expect(sourceFirst).toContainText("inject-css");
  await expect(sourceFirst).toContainText("components/drop-zone/src/mod.rs");
  await expect(sourceFirst).toContainText("components/drop-zone/src/logic.rs");
  await expect(sourceFirst).toContainText("components/drop-zone/src/view.rs");
  await expect(sourceFirst).toContainText("components/drop-zone/src/styles.rs");
  await expect(sourceFirst).toContainText("components/drop-zone/src/motion.rs");

  const sourceFirstPlayground = docsRoot
    .locator('[data-slot="drop-zone-source-first"]')
    .first()
    .locator("xpath=ancestor::section[@data-slot='playground']")
    .first();
  const showCodeButton = sourceFirstPlayground
    .locator('[data-slot="playground-toggle-code"]')
    .first();
  await showCodeButton.click();

  const codePanel = sourceFirstPlayground.locator('[data-slot="playground-code"]').first();
  await expect(codePanel).toBeVisible();
  await expect(codePanel).toContainText("use leptos::prelude::*;");
  await expect(codePanel).toContainText(
    "use ui_components::{DropZone, DropZoneMotion, DroppedFile};",
  );
  await expect(codePanel.locator(".ui-code-block__copy-button").first()).toBeVisible();
});
