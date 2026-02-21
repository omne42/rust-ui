import { expect, test } from "@playwright/test";

async function gotoLabelDocsAndWaitSettled(page) {
  await page.goto("/#/components/label");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="label"]').first();
  await expect(docsRoot).toBeVisible();

  const settledLabel = docsRoot
    .locator(
      '[data-slot="label"][data-ui-schema="ui.label.agent-contract.v1"][data-ui-stream-support="optional"][data-ui-stream-fallback="snapshot"][data-ui-output-status="verified"]',
    )
    .first();
  await expect(settledLabel).toBeVisible();

  return docsRoot;
}

test("docs-app label uses semantic selectors with wasm-stable readiness waits", async ({
  page,
}) => {
  const docsRoot = await gotoLabelDocsAndWaitSettled(page);

  const requiredLabel = docsRoot
    .locator(
      '[data-slot="label"][for="docs-label-name"][data-state="required"][data-required="true"][data-has-for="true"]',
    )
    .first();
  await expect(requiredLabel).toBeVisible();
  await expect(requiredLabel).toHaveAttribute("data-ui-action", "render-snapshot");
  await expect(requiredLabel).toHaveAttribute("data-ui-output-status", "verified");
  await expect(requiredLabel).toHaveAttribute("data-ui-source", "custom");
  await expect(requiredLabel).toHaveAttribute("data-label-source", "custom");
  await expect(requiredLabel).toHaveAttribute("data-indicator-source", "default");
  await expect(requiredLabel).toHaveAttribute("data-motion-source", "default");
});

test("docs-app label key flow is repeatable with semantic focus breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoLabelDocsAndWaitSettled(page);

  const focusLabelSelector =
    '[data-slot="label"][for="docs-label-workbench-compare"][data-state="required"][data-required="true"][data-has-for="true"][data-ui-output-status="verified"]';
  const focusLabel = docsRoot.locator(focusLabelSelector).first();
  const focusInput = page.locator("#docs-label-workbench-compare").first();

  await expect(focusLabel).toHaveAttribute("data-indicator-source", "custom");
  await expect(focusLabel).toHaveAttribute("data-class-source", "custom");
  await focusLabel.click();
  await expect(focusInput).toBeFocused();
  await page.keyboard.type("owner-one");
  await expect(focusInput).toHaveValue("owner-one");
  await expect(focusLabel).toHaveAttribute("data-ui-output-status", "verified");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedLabel = page
    .locator(`[data-component="label"] ${focusLabelSelector}`)
    .first();
  const reloadedInput = page.locator("#docs-label-workbench-compare").first();
  await expect(reloadedLabel).toHaveAttribute("data-has-for", "true");
  await reloadedLabel.click();
  await expect(reloadedInput).toBeFocused();
  await page.keyboard.type("owner-two");
  await expect(reloadedInput).toHaveValue("owner-two");
});

test("docs-app label streaming/snapshot markers stay settled without fixed sleeps", async ({
  page,
}) => {
  const docsRoot = await gotoLabelDocsAndWaitSettled(page);

  const streamPlayground = docsRoot
    .locator("section.playground")
    .filter({ has: page.locator("#docs-label-streaming") })
    .first();
  await expect(streamPlayground).toBeVisible();

  const streamLabel = streamPlayground
    .locator(
      '[data-slot="label"][for="docs-label-streaming"][data-state="required"][data-ui-stream-support="optional"][data-ui-stream-fallback="snapshot"][data-ui-output-status="verified"]',
    )
    .first();
  await expect(streamLabel).toBeVisible();
  await expect(streamLabel).toHaveAttribute("data-ui-action", "render-snapshot");

  const fallbackLabel = streamPlayground
    .locator(
      '[data-slot="label"]:not([for])[data-state="optional"][data-emphasis="subtle"][data-ui-stream-support="optional"][data-ui-stream-fallback="snapshot"][data-ui-output-status="verified"]',
    )
    .first();
  await expect(fallbackLabel).toBeVisible();
  await expect(fallbackLabel).toHaveAttribute("data-motion-source", "default");
});
