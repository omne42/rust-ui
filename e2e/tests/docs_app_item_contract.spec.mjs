import { expect, test } from "@playwright/test";

async function gotoItemDocsAndWaitSettled(page) {
  await page.goto("/#/components/item");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="item"]').first();
  await expect(docsRoot).toBeVisible();

  const settledItem = docsRoot
    .locator(
      '[data-slot="item"][data-ui-schema="ui.item.agent-contract.v1"][data-ui-action="render"][data-ui-streaming-fallback="snapshot"][data-ui-output-mode="snapshot"][data-ui-output-status="validated"]'
    )
    .first();
  await expect(settledItem).toBeVisible();

  return docsRoot;
}

test("docs-app item uses semantic selectors with wasm-stable ready waits", async ({ page }) => {
  const docsRoot = await gotoItemDocsAndWaitSettled(page);

  const group = docsRoot.locator('[data-slot="item-group"][role="list"]').first();
  const baselineItem = docsRoot
    .locator(
      '[data-slot="item"][role="listitem"][data-variant="default"][data-size="default"][data-variant-source="default"][data-size-source="default"]'
    )
    .first();
  const separator = docsRoot
    .locator('[data-slot="item-separator"][role="separator"][aria-orientation="horizontal"]')
    .first();

  await expect(group).toBeVisible();
  await expect(baselineItem).toBeVisible();
  await expect(baselineItem).toHaveAttribute("data-ui-output-status", "validated");
  await expect(baselineItem).toHaveAttribute("data-ui-streaming-fallback", "snapshot");
  await expect(separator).toBeVisible();
});

async function runItemInteractiveFlow(docsRoot) {
  const playground = docsRoot
    .locator("section.playground")
    .filter({ has: docsRoot.locator('[data-slot="playground-controls"] [data-slot="segmented-control"]') })
    .first();
  await expect(playground).toBeVisible();

  const controls = playground.locator('[data-slot="playground-controls"]').first();
  const variantControl = controls.locator('[data-slot="segmented-control"]').nth(0);
  const sizeControl = controls.locator('[data-slot="segmented-control"]').nth(1);
  const previewItem = playground
    .locator(
      '[data-slot="item"][data-ui-action="render"][data-ui-streaming-fallback="snapshot"][data-ui-output-status="validated"]'
    )
    .first();

  await expect(previewItem).toHaveAttribute("data-variant", "default");
  await expect(previewItem).toHaveAttribute("data-size", "default");
  await expect(previewItem).toHaveAttribute("data-variant-source", "prop");
  await expect(previewItem).toHaveAttribute("data-size-source", "prop");

  await variantControl
    .locator('[data-slot="segmented-control-option"][data-index="1"]')
    .first()
    .click();
  await expect(previewItem).toHaveAttribute("data-variant", "outline");
  await expect(previewItem).toHaveAttribute("data-ui-output-status", "validated");

  await sizeControl
    .locator('[data-slot="segmented-control-option"][data-index="1"]')
    .first()
    .click();
  await expect(previewItem).toHaveAttribute("data-size", "sm");
  await expect(previewItem).toHaveAttribute("data-ui-streaming-fallback", "snapshot");
}

test("docs-app item key flow is repeatable with semantic ready/settled breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoItemDocsAndWaitSettled(page);
  await runItemInteractiveFlow(docsRoot);

  await page.reload();
  const reloadedRoot = await gotoItemDocsAndWaitSettled(page);
  await runItemInteractiveFlow(reloadedRoot);
});

test("docs-app item playground source is copy-paste ready", async ({ page }) => {
  const docsRoot = await gotoItemDocsAndWaitSettled(page);
  const playground = docsRoot
    .locator("section.playground")
    .filter({ has: docsRoot.locator('[data-slot="playground-controls"] [data-slot="segmented-control"]') })
    .first();
  await expect(playground).toBeVisible();

  const codeToggle = playground
    .getByRole("button", { name: /Show code|Hide code/ })
    .first();
  await expect(codeToggle).toBeVisible();

  const codeBlock = playground.locator('[data-slot="code-block"]');
  if ((await codeBlock.count()) === 0) {
    await codeToggle.click();
  }

  await expect(codeBlock.first()).toBeVisible();
  await expect(codeBlock.first()).toHaveAttribute("data-copyable", "true");

  const code = playground.locator('[data-slot="code-block-code"]').first();
  await expect(code).toContainText("use leptos::prelude::*;");
  await expect(code).toContainText("use ui_components::{Item");
  await expect(code).toContainText("<ItemGroup>");
  await expect(code).toContainText("<Item");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);
});
