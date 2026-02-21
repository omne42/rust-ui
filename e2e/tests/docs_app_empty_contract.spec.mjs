import { expect, test } from "@playwright/test";

async function gotoEmptyDocsAndWaitSettled(page) {
  await page.goto("/#/components/empty");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="empty"]').first();
  await expect(docsRoot).toBeVisible();

  const settledRoot = docsRoot
    .locator(
      '[data-slot="empty"][data-state="root"][data-ui-stream-support="optional"][data-ui-stream-fallback="snapshot"][data-ui-output-status="verified"]'
    )
    .first();
  await expect(settledRoot).toBeVisible();
  await expect(settledRoot).toHaveAttribute("data-ui-action", "render-snapshot");

  return docsRoot;
}

async function runEmptyCriticalFlow(docsRoot) {
  const playground = docsRoot
    .locator('[data-slot="playground"]')
    .filter({ has: docsRoot.locator('[data-slot="playground-controls"]') })
    .first();
  await expect(playground).toBeVisible();

  const controls = playground.locator('[data-slot="playground-controls"]').first();
  const variantControl = controls.locator('[data-slot="segmented-control"]').first();
  const root = playground.locator('[data-slot="empty"][data-state="root"]').first();
  const media = playground.locator('[data-slot="empty-icon"][data-state="media"]').first();

  await expect(root).toHaveAttribute("data-ui-action", "render-snapshot");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");
  await expect(media).toHaveAttribute("data-variant", "default");
  await expect(media).toHaveAttribute("data-variant-source", "default");

  await variantControl
    .locator('[data-slot="segmented-control-option"][data-index="1"]')
    .first()
    .click();
  await expect(media).toHaveAttribute("data-variant", "icon");
  await expect(media).toHaveAttribute("data-variant-source", "custom");

  await variantControl
    .locator('[data-slot="segmented-control-option"][data-index="0"]')
    .first()
    .click();
  await expect(media).toHaveAttribute("data-variant", "default");
  await expect(media).toHaveAttribute("data-variant-source", "default");
}

test("docs-app empty uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const docsRoot = await gotoEmptyDocsAndWaitSettled(page);

  const header = docsRoot.locator('[data-slot="empty-header"][data-state="header"]').first();
  const title = docsRoot.locator('[data-slot="empty-title"][data-state="title"]').first();
  const description = docsRoot
    .locator('[data-slot="empty-description"][data-state="description"]')
    .first();
  const iconMedia = docsRoot
    .locator('[data-slot="empty-icon"][data-state="media"][data-variant="icon"]')
    .first();

  await expect(header).toBeVisible();
  await expect(title).toBeVisible();
  await expect(description).toBeVisible();
  await expect(iconMedia).toBeVisible();
  await expect(iconMedia).toHaveAttribute("data-variant-source", "custom");
});

test("docs-app empty workbench flow uses semantic ready/settled breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoEmptyDocsAndWaitSettled(page);
  await runEmptyCriticalFlow(docsRoot);
});

test("docs-app empty key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoEmptyDocsAndWaitSettled(page);
  await runEmptyCriticalFlow(docsRoot);

  await page.reload();
  const reloadedDocsRoot = await gotoEmptyDocsAndWaitSettled(page);
  await runEmptyCriticalFlow(reloadedDocsRoot);
});
