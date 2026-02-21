import { expect, test } from "@playwright/test";

async function gotoCodeBlockDocsAndWaitReady(page) {
  await page.goto("/#/components/code-block");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="code-block"]').first();
  await expect(docsRoot).toBeVisible();

  const readySnapshot = docsRoot
    .locator(
      '[data-slot="code-block-streaming-preview"] [data-slot="code-block"][data-ui-output-mode="snapshot"][data-ui-output-status="validated"][data-ui-state="idle"][aria-busy="false"]'
    )
    .first();
  await expect(readySnapshot).toBeVisible();

  return docsRoot;
}

function firstCopyableCodeBlock(docsRoot) {
  return docsRoot
    .locator('section.playground')
    .first()
    .locator('[data-slot="code-block"][data-copyable="true"][data-copied-source="uncontrolled"]')
    .first();
}

async function installClipboardStub(page) {
  await page.addInitScript(() => {
    window.__copiedText = "";
    const clipboard = {
      writeText(value) {
        window.__copiedText = String(value);
        return Promise.resolve();
      },
    };
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: clipboard,
    });
  });
}

async function runCopyFlowAndWaitSettled(page, docsRoot) {
  const copyRoot = firstCopyableCodeBlock(docsRoot);
  const copyButton = copyRoot.locator('button[type="button"]').first();

  await expect(copyRoot).toHaveAttribute("data-ui-state", "idle");
  await expect(copyRoot).toHaveAttribute("data-copied-source", "uncontrolled");
  await expect(copyRoot).toHaveAttribute("aria-busy", "false");
  await expect(copyButton).toBeVisible();
  await copyButton.focus();
  await expect(copyButton).toBeFocused();
  await page.keyboard.press("Space");

  await expect(copyRoot).toHaveAttribute("data-ui-state", "copied");
  await expect(copyRoot).toHaveAttribute("data-copied", "true");
  await expect(copyRoot).toHaveAttribute("aria-busy", "false");
  await expect.poll(() => page.evaluate(() => window.__copiedText)).toBe(
    "cargo check -p ui-components"
  );

  await expect(copyRoot).toHaveAttribute("data-ui-state", "idle", { timeout: 4500 });
  await expect(copyRoot).not.toHaveAttribute("data-copied", "true", { timeout: 4500 });
}

test("docs-app code-block uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const docsRoot = await gotoCodeBlockDocsAndWaitReady(page);

  const stateMatrix = docsRoot.locator('[data-slot="code-block-state-matrix"]').first();
  const streamingModes = docsRoot.locator('[data-slot="code-block-streaming-modes"]').first();
  const sourceFirst = docsRoot.locator('[data-slot="code-block-source-first"]').first();
  await expect(stateMatrix).toBeVisible();
  await expect(streamingModes).toBeVisible();
  await expect(sourceFirst).toBeVisible();

  const controlledPreview = docsRoot.locator('[data-slot="code-block-controlled-preview"]').first();
  await expect(
    controlledPreview.locator('[data-slot="code-block"][data-copied-source="uncontrolled"]').first()
  ).toBeVisible();
  await expect(
    controlledPreview.locator('[data-slot="code-block"][data-copied-source="controlled"]').first()
  ).toBeVisible();

  const snapshotBlock = docsRoot
    .locator(
      '[data-slot="code-block-streaming-preview"] [data-slot="code-block"][data-ui-output-mode="snapshot"][data-ui-output-status="validated"]'
    )
    .first();
  const streamingBlock = docsRoot
    .locator(
      '[data-slot="code-block-streaming-preview"] [data-slot="code-block"][data-ui-output-mode="streaming"][data-ui-output-status="draft"]'
    )
    .first();
  await expect(snapshotBlock).toBeVisible();
  await expect(streamingBlock).toBeVisible();
});

test("docs-app code-block keyboard + async copy path uses semantic ready and settled checkpoints", async ({
  page,
}) => {
  await installClipboardStub(page);
  const docsRoot = await gotoCodeBlockDocsAndWaitReady(page);
  await runCopyFlowAndWaitSettled(page, docsRoot);
});

test("docs-app code-block key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await installClipboardStub(page);
  const docsRoot = await gotoCodeBlockDocsAndWaitReady(page);
  await runCopyFlowAndWaitSettled(page, docsRoot);

  await page.reload();
  const reloadedDocsRoot = await gotoCodeBlockDocsAndWaitReady(page);
  await runCopyFlowAndWaitSettled(page, reloadedDocsRoot);
});
