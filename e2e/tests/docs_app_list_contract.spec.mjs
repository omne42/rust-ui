import { expect, test } from "@playwright/test";

const WASM_READY_SELECTOR = "body:not(:has(#boot))";

async function gotoListDocsAndWaitSettled(page) {
  await page.goto("/#/components/list");
  await page.locator(WASM_READY_SELECTOR).waitFor();

  const docsRoot = page.locator('[data-component="list"]').first();
  await expect(docsRoot).toBeVisible();

  const showcase = docsRoot.locator('[data-slot="list-showcase"]').first();
  await expect(showcase).toBeVisible();

  const controlledRoot = showcase.locator('[data-slot="listbox"][aria-label="Default list"]').first();
  await expect(controlledRoot).toBeVisible();

  return { docsRoot, controlledRoot };
}

async function expectListReady(root) {
  await expect(root).toHaveAttribute("role", "listbox");
  await expect(root).toHaveAttribute("data-selection-mode", "controlled");
  await expect(root).toHaveAttribute("data-selection-value-source", "external");
  await expect(root).toHaveAttribute("data-default-selection-source", "none");
  await expect(root).toHaveAttribute("data-selection-change-source", "provided");
  await expect(root).toHaveAttribute("data-interaction-source", "none");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");
}

async function expectListPointerSettled(root) {
  await expect(root).toHaveAttribute("data-interaction-source", "pointer");
  await expect(root).toHaveAttribute("data-has-selection", "true");
  await expect(root).toHaveAttribute("data-ui-state", "has-selection");
  await expect(root.locator('[data-slot="listbox-option"][data-selected="true"]')).toHaveCount(1);
}

async function expectListKeyboardSettled(root, previousActiveDescendant) {
  await expect(root).toHaveAttribute("data-interaction-source", "keyboard");
  const activeDescendant = await root.getAttribute("aria-activedescendant");
  expect(activeDescendant).not.toBeNull();
  expect(activeDescendant).not.toEqual(previousActiveDescendant);
  await expect(root.locator('[data-slot="listbox-option"][data-focused="true"]')).toHaveCount(1);
}

async function runListReadySettledFlow(page, root) {
  const pointerTarget = root.locator('[data-slot="listbox-option"][data-index="3"]').first();
  await pointerTarget.click();
  await expectListPointerSettled(root);

  const activeAfterPointer = await root.getAttribute("aria-activedescendant");
  expect(activeAfterPointer).not.toBeNull();

  await root.focus();
  await page.keyboard.press("ArrowDown");
  await expectListKeyboardSettled(root, activeAfterPointer);

  await page.keyboard.press("Enter");
  await expect(root).toHaveAttribute("data-interaction-source", "keyboard");
  await expect(root.locator('[data-slot="listbox-option"][data-selected="true"]')).toHaveCount(1);
}

test("docs-app list uses semantic selectors with wasm-stable ready waits", async ({ page }) => {
  const { docsRoot, controlledRoot } = await gotoListDocsAndWaitSettled(page);
  await expectListReady(controlledRoot);

  await expect(
    controlledRoot.locator('[data-slot="listbox-option"][data-index="2"][data-disabled="true"]').first()
  ).toBeVisible();

  await expect(
    docsRoot.locator('[data-slot="list-streaming-snapshot"] [data-ui-output-state="snapshot"]').first()
  ).toBeVisible();
  await expect(
    docsRoot.locator('[data-slot="list-streaming-snapshot"] [data-ui-output-state="streaming"]').first()
  ).toBeVisible();
});

test("docs-app list flow covers ready and settled semantic breakpoints", async ({ page }) => {
  const { controlledRoot } = await gotoListDocsAndWaitSettled(page);
  await expectListReady(controlledRoot);
  await runListReadySettledFlow(page, controlledRoot);

  await page.reload();
  const { controlledRoot: reloadedRoot } = await gotoListDocsAndWaitSettled(page);
  await expectListReady(reloadedRoot);
  await runListReadySettledFlow(page, reloadedRoot);
});
