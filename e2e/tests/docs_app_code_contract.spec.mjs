import { expect, test } from "@playwright/test";

async function gotoCodeDocsAndWaitSettled(page) {
  await page.goto("/#/components/code");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="code"]').first();
  await expect(docsRoot).toBeVisible();

  const settledCode = docsRoot
    .locator(
      '[data-slot="code"][data-ui-streaming="optional"][data-ui-fallback="snapshot"][data-ui-output-state="verified"][aria-live="off"][aria-busy="false"]'
    )
    .first();
  await expect(settledCode).toBeVisible();
  await expect(settledCode).toHaveAttribute("data-state", /(inline|block)/);

  return docsRoot;
}

async function runCodeCriticalFlow(page, docsRoot) {
  const playground = docsRoot
    .locator('[data-slot="playground"]')
    .filter({ has: docsRoot.locator('[data-slot="code-workbench-controls"]') })
    .first();
  await expect(playground).toBeVisible();

  const controls = playground.locator('[data-slot="code-workbench-controls"]').first();
  const primaryCode = playground
    .locator('[data-slot="code-workbench-primary"] [data-slot="code"]')
    .first();

  await expect(primaryCode).toBeVisible();
  await expect(primaryCode).toHaveAttribute("data-variant", "inline");
  await expect(primaryCode).toHaveAttribute("data-state", "inline");
  await expect(primaryCode).toHaveAttribute("data-ui-output-state", "verified");
  await expect(primaryCode).toHaveAttribute("aria-busy", "false");

  const variantControl = controls.locator('[data-slot="segmented-control"]').first();
  await variantControl.locator('[data-slot="segmented-control-option"][data-index="1"]').click();
  await expect(primaryCode).toHaveAttribute("data-variant", "block");
  await expect(primaryCode).toHaveAttribute("data-state", "block");

  // Code has no overlay/async path; prioritize focus + keyboard regression branch.
  const customClassSwitch = controls.locator('[data-slot="switch"]').first();
  await customClassSwitch.focus();
  await expect(customClassSwitch).toBeFocused();
  await page.keyboard.press("Space");
  await expect(primaryCode).toHaveAttribute("data-custom-class", "true");
  await expect(primaryCode).toHaveAttribute("data-ui-fallback", "snapshot");

  const compareSwitch = controls.locator('[data-slot="switch"]').nth(2);
  await compareSwitch.click();
  await expect(playground.locator('[data-slot="code-workbench-compare"]')).toHaveCount(0);

  await compareSwitch.focus();
  await expect(compareSwitch).toBeFocused();
  await page.keyboard.press("Space");

  const compareArea = playground.locator('[data-slot="code-workbench-compare"]').first();
  await expect(compareArea).toBeVisible();
  await expect(compareArea.locator('[data-slot="code"][data-variant="inline"]').first()).toBeVisible();
  await expect(compareArea.locator('[data-slot="code"][data-variant="block"]').first()).toBeVisible();
}

test("docs-app code uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const docsRoot = await gotoCodeDocsAndWaitSettled(page);

  const inlineCode = docsRoot
    .locator('[data-slot="code"][data-variant="inline"][data-state="inline"]')
    .first();
  await expect(inlineCode).toBeVisible();

  const blockCode = docsRoot
    .locator('[data-slot="code"][data-variant="block"][data-state="block"]')
    .first();
  await expect(blockCode).toBeVisible();

  const customClassCode = docsRoot
    .locator('[data-slot="code"][data-variant="block"][data-custom-class="true"]')
    .first();
  await expect(customClassCode).toBeVisible();
  await expect(customClassCode).toHaveAttribute("data-ui-fallback", "snapshot");

  const sourceFirst = docsRoot.locator('[data-slot="code-source-first"]').first();
  await expect(sourceFirst).toBeVisible();
  await expect(sourceFirst.locator('[data-slot="snippet"]').first()).toHaveAttribute(
    "data-copyable",
    "true"
  );
});

test("docs-app code critical flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoCodeDocsAndWaitSettled(page);

  await runCodeCriticalFlow(page, docsRoot);

  const stateMatrix = docsRoot.locator('[data-slot="code-state-matrix"]').first();
  const streamingModes = docsRoot.locator('[data-slot="code-streaming-modes"]').first();
  await expect(stateMatrix).toBeVisible();
  await expect(streamingModes).toBeVisible();

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedDocsRoot = page.locator('[data-component="code"]').first();
  await expect(reloadedDocsRoot).toBeVisible();
  await runCodeCriticalFlow(page, reloadedDocsRoot);
});
