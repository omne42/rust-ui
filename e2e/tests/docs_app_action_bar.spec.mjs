import { expect, test } from "@playwright/test";

async function openActionBarDocs(page) {
  await page.goto("/#/components/action-bar");
  await page.locator("body:not(:has(#boot))").waitFor();
  const docsRoot = page.locator('[data-component="action-bar"]').first();
  await expect(docsRoot).toBeVisible();
  return docsRoot;
}

function actionBarLocators(docsRoot) {
  const actionBar = docsRoot
    .locator('[data-slot="action-bar"][data-control-mode="controlled"][data-has-clear="true"]')
    .first();
  const clearButton = actionBar
    .locator('[data-slot="action-bar-clear"] [data-slot="button"]')
    .first();
  const incrementButton = docsRoot
    .locator('[data-slot="button"][aria-label="Increase selected count"]')
    .first();
  const selectedCount = actionBar.locator('[data-slot="action-bar-selection-count"]').first();

  return { actionBar, clearButton, incrementButton, selectedCount };
}

async function runActionBarCriticalFlow(page, docsRoot) {
  const { actionBar, clearButton, incrementButton } = actionBarLocators(docsRoot);

  await expect(actionBar).toHaveAttribute("data-state", "visible");
  await expect(actionBar).toHaveAttribute("data-selected-count", "2");

  // High-risk focus + keyboard path (no fixed sleep, semantic settled checkpoints only).
  await clearButton.focus();
  await expect(clearButton).toBeFocused();
  await page.keyboard.press("Space");

  await expect(actionBar).toHaveAttribute("data-selected-count", "0");
  await expect(actionBar).toHaveAttribute("data-state", "hidden");
  await expect(actionBar).toHaveAttribute("aria-hidden", "true");

  await incrementButton.click();
  await expect(actionBar).toHaveAttribute("data-selected-count", "1");
  await expect(actionBar).toHaveAttribute("data-state", "visible");
  await expect(actionBar).not.toHaveAttribute("aria-hidden", "true");
}

test("docs-app action-bar uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const docsRoot = await openActionBarDocs(page);
  const { actionBar, selectedCount, clearButton } = actionBarLocators(docsRoot);

  await actionBar.scrollIntoViewIfNeeded();
  await expect(actionBar).toHaveAttribute("role", "toolbar");
  await expect(actionBar).toHaveAttribute("data-state", "visible");
  await expect(actionBar).toHaveAttribute("data-selection", /(single|multiple)/);
  await expect(actionBar).toHaveAttribute("data-control-mode", "controlled");
  await expect(actionBar).toHaveAttribute("data-selected-count", "2");
  await expect(actionBar).toHaveAttribute("data-selected-count-source", "external");
  await expect(selectedCount).toContainText("2");
  await expect(clearButton).toBeVisible();
});

test("docs-app action-bar motion path uses semantic ready and settled breakpoints", async ({
  page,
}) => {
  const docsRoot = await openActionBarDocs(page);
  await runActionBarCriticalFlow(page, docsRoot);
});

test("docs-app action-bar key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  const docsRoot = await openActionBarDocs(page);
  await runActionBarCriticalFlow(page, docsRoot);

  await page.reload();
  const reloadedDocsRoot = await openActionBarDocs(page);
  const { actionBar: reloadedActionBar } = actionBarLocators(reloadedDocsRoot);
  await expect(reloadedActionBar).toHaveAttribute("data-selected-count", "2");

  await runActionBarCriticalFlow(page, reloadedDocsRoot);
});
