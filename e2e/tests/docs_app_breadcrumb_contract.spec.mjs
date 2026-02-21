import { expect, test } from "@playwright/test";

const BREADCRUMB_PAGE = "/#/components/breadcrumb";

async function openBreadcrumbDocs(page) {
  await page.goto(BREADCRUMB_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="breadcrumb"]').first();
  await expect(docsRoot).toBeVisible();
  return docsRoot;
}

test("docs-app breadcrumb uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const docsRoot = await openBreadcrumbDocs(page);

  const linkedTrail = docsRoot
    .locator('[data-slot="breadcrumb-state-linked"] [data-slot="breadcrumb"]')
    .first();
  await expect(linkedTrail).toBeVisible();
  await expect(linkedTrail).toHaveAttribute("data-ui-schema", "ui.breadcrumb.agent-contract");
  await expect(linkedTrail).toHaveAttribute("data-ui-schema-version", "v1");
  await expect(linkedTrail).toHaveAttribute("data-ui-render-mode", "snapshot");
  await expect(linkedTrail).toHaveAttribute("data-ui-stream-support", "optional");
  await expect(linkedTrail).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(linkedTrail).toHaveAttribute("data-ui-output-status", "verified");
  await expect(linkedTrail).toHaveAttribute("data-has-items", "true");
  await expect(linkedTrail).toHaveAttribute("data-has-current-page", "true");

  await expect(
    linkedTrail.locator('[data-slot="breadcrumb-page"][aria-current="page"]').first(),
  ).toBeVisible();
  await expect(linkedTrail.locator('[data-slot="breadcrumb-item"]').first()).toHaveAttribute(
    "data-index",
    "0",
  );
});

test("docs-app breadcrumb key flow is repeatable with semantic breakpoints", async ({ page }) => {
  const docsRoot = await openBreadcrumbDocs(page);

  const contractPanel = docsRoot.locator('[data-slot="breadcrumb-streaming-contract"]').first();
  const contractBreadcrumb = contractPanel.locator('[data-slot="breadcrumb"]').first();
  const streamModeControl = contractPanel.locator('[data-slot="segmented-control"]').first();
  const snapshotOption = streamModeControl
    .locator('[data-slot="segmented-control-option"][data-index="0"]')
    .first();
  const streamingOption = streamModeControl
    .locator('[data-slot="segmented-control-option"][data-index="1"]')
    .first();

  for (const cycle of [1, 2]) {
    await test.step(`breadcrumb repeatable key flow cycle ${cycle}`, async () => {
      await streamingOption.focus();
      await expect(streamingOption).toBeFocused();
      await page.keyboard.press("Enter");
      await expect(contractPanel).toHaveAttribute("data-requested-stream-mode", "streaming");
      await expect(contractBreadcrumb).toHaveAttribute("data-ui-render-mode", "snapshot");
      await expect(contractBreadcrumb).toHaveAttribute("data-ui-stream-fallback", "snapshot");
      await expect(contractBreadcrumb).toHaveAttribute("data-ui-output-status", "verified");
      await expect(contractBreadcrumb).toHaveAttribute("data-ui-state", "linked-trail");

      await snapshotOption.focus();
      await expect(snapshotOption).toBeFocused();
      await page.keyboard.press("Enter");
      await expect(contractPanel).toHaveAttribute("data-requested-stream-mode", "snapshot");
      await expect(contractBreadcrumb).toHaveAttribute("data-ui-render-mode", "snapshot");
      await expect(contractBreadcrumb).toHaveAttribute("data-ui-stream-fallback", "snapshot");
      await expect(contractBreadcrumb).toHaveAttribute("data-ui-output-status", "verified");
      await expect(contractBreadcrumb).toHaveAttribute("data-ui-state", "linked-trail");
    });
  }

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  const reloadedPanel = page.locator('[data-slot="breadcrumb-streaming-contract"]').first();
  const reloadedBreadcrumb = reloadedPanel.locator('[data-slot="breadcrumb"]').first();
  await expect(reloadedPanel).toHaveAttribute("data-requested-stream-mode", "snapshot");
  await expect(reloadedBreadcrumb).toHaveAttribute("data-ui-render-mode", "snapshot");
  await expect(reloadedBreadcrumb).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app breadcrumb high-risk path covers focus keyboard and settled semantic breakpoints", async ({
  page,
}) => {
  const docsRoot = await openBreadcrumbDocs(page);

  const contractPanel = docsRoot.locator('[data-slot="breadcrumb-streaming-contract"]').first();
  const contractBreadcrumb = contractPanel.locator('[data-slot="breadcrumb"]').first();
  const streamModeControl = contractPanel.locator('[data-slot="segmented-control"]').first();
  const streamingOption = streamModeControl
    .locator('[data-slot="segmented-control-option"][data-index="1"]')
    .first();

  await streamingOption.focus();
  await expect(streamingOption).toBeFocused();
  await page.keyboard.press("Enter");

  await expect(contractPanel).toHaveAttribute("data-requested-stream-mode", "streaming");
  await expect(contractBreadcrumb).toHaveAttribute("data-ui-state", "linked-trail");
  await expect(contractBreadcrumb).toHaveAttribute("data-ui-output-status", "verified");
  await expect(
    contractBreadcrumb.locator('[data-slot="breadcrumb-page"][aria-current="page"]').first(),
  ).toBeVisible();
  await expect(contractBreadcrumb).not.toHaveAttribute("aria-busy", /.+/);
  await expect(contractBreadcrumb).not.toHaveAttribute("data-loading", /.+/);
});

test("docs-app breadcrumb streaming fallback stays semantically settled without async-ready loops", async ({
  page,
}) => {
  const docsRoot = await openBreadcrumbDocs(page);

  const contractPanel = docsRoot.locator('[data-slot="breadcrumb-streaming-contract"]').first();
  await expect(contractPanel).toBeVisible();
  await expect(contractPanel).toHaveAttribute("data-requested-stream-mode", "snapshot");

  const contractBreadcrumb = contractPanel.locator('[data-slot="breadcrumb"]').first();
  await expect(contractBreadcrumb).toHaveAttribute("data-ui-render-mode", "snapshot");
  await expect(contractBreadcrumb).toHaveAttribute("data-ui-stream-support", "optional");
  await expect(contractBreadcrumb).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(contractBreadcrumb).toHaveAttribute("data-ui-output-status", "verified");

  const streamModeControl = contractPanel.locator('[data-slot="segmented-control"]').first();
  await streamModeControl
    .locator('[data-slot="segmented-control-option"][data-index="1"]')
    .click();

  await expect(contractPanel).toHaveAttribute("data-requested-stream-mode", "streaming");
  await expect(contractBreadcrumb).toHaveAttribute("data-ui-render-mode", "snapshot");
  await expect(contractBreadcrumb).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(contractBreadcrumb).toHaveAttribute("data-ui-output-status", "verified");

  await expect(contractBreadcrumb).not.toHaveAttribute("aria-busy", /.+/);
  await expect(contractBreadcrumb).not.toHaveAttribute("data-loading", /.+/);
});
