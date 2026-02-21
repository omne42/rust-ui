import { expect, test } from "@playwright/test";

async function gotoLegendDocsAndWaitSettled(page) {
  await page.goto("/#/components/legend");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="legend"]').first();
  await expect(docsRoot).toBeVisible();

  const settledLegend = docsRoot
    .locator(
      '[data-slot="legend"][data-ui-output-status="verified"][data-ui-stream-fallback="snapshot"][data-ui-stream-mode="snapshot"]'
    )
    .first();
  await expect(settledLegend).toBeVisible();

  return docsRoot;
}

async function runLegendControlledRequiredFlow(page, docsRoot) {
  const playground = docsRoot
    .locator('[data-slot="playground"]')
    .filter({ has: docsRoot.locator('[data-slot="switch"]') })
    .first();
  await expect(playground).toBeVisible();

  const controlledLegend = playground
    .locator('[data-slot="legend"][data-tone="muted"][data-required-source="is_required"]')
    .first();
  const controlledSwitch = playground.locator('[data-slot="switch"]').first();

  await expect(controlledLegend).toBeVisible();
  await expect(controlledLegend).toHaveAttribute("data-state", "required");
  await expect(controlledLegend).toHaveAttribute("data-required", "true");

  await controlledSwitch.focus();
  await page.keyboard.press("Space");
  await expect(controlledLegend).toHaveAttribute("data-state", "optional");
  await expect(controlledLegend).not.toHaveAttribute("data-required", "true");
  await expect(controlledLegend).toHaveAttribute("data-required-source", "is_required");

  await page.keyboard.press("Space");
  await expect(controlledLegend).toHaveAttribute("data-state", "required");
  await expect(controlledLegend).toHaveAttribute("data-required", "true");
}

test("docs-app legend contract uses semantic selectors with wasm-stable waits", async ({
  page,
}) => {
  const docsRoot = await gotoLegendDocsAndWaitSettled(page);

  const defaultLegend = docsRoot
    .locator(
      '[data-slot="legend"][data-tone="default"][data-state="optional"][data-required-source="default"][data-disabled-source="default"]'
    )
    .first();
  await expect(defaultLegend).toBeVisible();

  const requiredLegend = docsRoot
    .locator('[data-slot="legend"][data-tone="default"][data-state="required"][data-required="true"]')
    .first();
  await expect(requiredLegend).toBeVisible();
  await expect(requiredLegend).toHaveAttribute("data-required-source", "is_required");

  const customLegend = docsRoot
    .locator('[data-slot="legend"][data-tone="muted"][data-indicator-source="custom"][data-class-source="custom"]')
    .first();
  await expect(customLegend).toBeVisible();

  const disabledLegend = docsRoot
    .locator('[data-slot="legend"][data-tone="strong"][data-disabled="true"][aria-disabled="true"]')
    .first();
  await expect(disabledLegend).toBeVisible();
  await expect(disabledLegend).toHaveAttribute("data-disabled-source", "is_disabled");
});

test("docs-app legend key flow is repeatable with semantic breakpoints", async ({ page }) => {
  const docsRoot = await gotoLegendDocsAndWaitSettled(page);
  await runLegendControlledRequiredFlow(page, docsRoot);

  await page.reload();
  const reloadedRoot = await gotoLegendDocsAndWaitSettled(page);
  await runLegendControlledRequiredFlow(page, reloadedRoot);
});

test("docs-app legend source-first snippets are copy-paste ready and traceable", async ({
  page,
}) => {
  const docsRoot = await gotoLegendDocsAndWaitSettled(page);

  const playground = docsRoot.locator('[data-slot="playground"]').first();
  await expect(playground).toBeVisible();

  const codeToggle = playground
    .getByRole("button", { name: /Show code|Hide code/ })
    .first();
  await expect(codeToggle).toBeVisible();

  const codeBlock = playground.locator('[data-slot="code-block"]').first();
  if ((await codeBlock.count()) === 0) {
    await codeToggle.click();
  }

  await expect(codeBlock).toBeVisible();
  await expect(codeBlock).toHaveAttribute("data-copyable", "true");

  const code = playground.locator('[data-slot="code-block-code"]').first();
  await expect(code).toContainText("use leptos::prelude::*;");
  await expect(code).toContainText("use ui_components::*;");
  await expect(code).toContainText("<Legend");
  await expect(code).toContainText("text=\"Notification settings\"");

  const sourcePaths = docsRoot.locator('[data-slot="legend-source-paths"]').first();
  await expect(sourcePaths).toBeVisible();
  await expect(sourcePaths).toContainText("components/legend/src/logic.rs");
  await expect(sourcePaths).toContainText("components/legend/src/view.rs");

  const prerequisites = docsRoot.locator('[data-slot="legend-source-prerequisites"]').first();
  await expect(prerequisites).toBeVisible();
  await expect(prerequisites).toContainText("component-legend");
  await expect(prerequisites).toContainText("inject-css");
});
