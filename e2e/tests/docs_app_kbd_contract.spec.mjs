import { expect, test } from "@playwright/test";

async function gotoKbdDocsAndWaitSettled(page) {
  await page.goto("/#/components/kbd");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="kbd"]').first();
  await expect(docsRoot).toBeVisible();

  const settledKbd = docsRoot.locator('[data-slot="kbd"][data-size][data-state]').first();
  await expect(settledKbd).toBeVisible();
  await expect(settledKbd).toHaveAttribute("data-size", /(sm|md)/);
  await expect(settledKbd).toHaveAttribute("data-state", /(with-keys|label-only)/);

  return docsRoot;
}

test("docs-app kbd uses semantic selectors with wasm-stable ready waits", async ({ page }) => {
  const docsRoot = await gotoKbdDocsAndWaitSettled(page);

  const withKeys = docsRoot
    .locator('[data-slot="kbd"][data-size="md"][data-state="with-keys"][data-keys="true"]')
    .first();
  await expect(withKeys).toBeVisible();

  const labelOnly = docsRoot
    .locator('[data-slot="kbd"][data-size="md"][data-state="label-only"]')
    .first();
  await expect(labelOnly).toBeVisible();

  const customClass = docsRoot
    .locator(
      '[data-slot="kbd"][data-size="sm"][data-state="with-keys"][data-keys="true"][data-custom-class="true"]'
    )
    .first();
  await expect(customClass).toBeVisible();
});

async function runKbdWorkbenchFlow(docsRoot) {
  const workbench = docsRoot
    .locator('[data-slot="playground"]')
    .filter({ has: docsRoot.locator('[data-slot="kbd-workbench-controls"]') })
    .first();
  await expect(workbench).toBeVisible();

  const controls = workbench.locator('[data-slot="kbd-workbench-controls"]').first();
  const sizeSelect = controls.locator("select").first();
  const keysInput = controls.locator('input[type="text"]').nth(0);
  const labelInput = controls.locator('input[type="text"]').nth(1);
  const customClassCheckbox = controls.locator('input[type="checkbox"]').first();
  const preview = workbench.locator('[data-slot="kbd"]').first();

  await expect(preview).toHaveAttribute("data-size", "md");
  await expect(preview).toHaveAttribute("data-state", "with-keys");

  await sizeSelect.selectOption("sm");
  await expect(preview).toHaveAttribute("data-size", "sm");

  await keysInput.fill("");
  await expect(preview).toHaveAttribute("data-state", "label-only");
  await expect(preview).not.toHaveAttribute("data-keys", "true");

  await labelInput.fill("Esc");
  await expect(preview).toContainText("Esc");

  await customClassCheckbox.check();
  await expect(preview).toHaveAttribute("data-custom-class", "true");

  await keysInput.fill("Shift");
  await expect(preview).toHaveAttribute("data-state", "with-keys");
  await expect(preview).toHaveAttribute("data-keys", "true");
}

test("docs-app kbd workbench flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoKbdDocsAndWaitSettled(page);
  await runKbdWorkbenchFlow(docsRoot);

  await page.reload();
  const reloadedRoot = await gotoKbdDocsAndWaitSettled(page);
  await runKbdWorkbenchFlow(reloadedRoot);
});
