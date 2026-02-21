import { expect, test } from "@playwright/test";

async function gotoComboBoxDocsAndWaitSettled(page) {
  await page.goto("/#/components/combo-box");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="combo-box"]').first();
  await expect(docsRoot).toBeVisible();

  const showcase = docsRoot.locator('[data-slot="combo-box-showcase"]').first();
  await expect(showcase).toBeVisible();

  const controlledRoot = showcase
    .locator('[data-slot="combo-box"][data-controlled="true"]')
    .first();
  await expect(controlledRoot).toBeVisible();
  await expect(controlledRoot).toHaveAttribute("data-state", "closed");
  await expect(controlledRoot).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(controlledRoot).toHaveAttribute("data-ui-output-status", "verified");

  const controlledInput = controlledRoot
    .locator('[data-slot="combo-box-input"][role="combobox"]')
    .first();
  await expect(controlledInput).toBeVisible();
  await expect(controlledInput).toHaveAttribute("aria-expanded", "false");

  return { docsRoot, controlledRoot, controlledInput };
}

test("docs-app combo-box uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const { docsRoot, controlledRoot, controlledInput } = await gotoComboBoxDocsAndWaitSettled(page);

  await expect(
    docsRoot.locator('[data-slot="combo-box-streaming-snapshot"] [data-ui-output-state="snapshot"]')
  ).toBeVisible();
  await expect(
    docsRoot.locator('[data-slot="combo-box-streaming-snapshot"] [data-ui-output-state="streaming"]')
  ).toBeVisible();

  await expect(controlledRoot).toHaveAttribute("data-id-source", "custom");
  await expect(controlledRoot).toHaveAttribute("data-label-source", "custom");
  await expect(controlledRoot).toHaveAttribute("data-motion-source", "default");
  await expect(controlledInput).toHaveAttribute("aria-controls", /docs-combo-box-controlled-listbox/);
});

test("docs-app combo-box flow is repeatable with semantic ready/settled breakpoints", async ({
  page,
}) => {
  const { controlledRoot, controlledInput } = await gotoComboBoxDocsAndWaitSettled(page);

  const trigger = controlledRoot.locator('[data-slot="combo-box-trigger"]').first();
  await expect(trigger).toBeVisible();

  await trigger.click();
  await expect(controlledRoot).toHaveAttribute("data-state", "open");
  await expect(controlledInput).toHaveAttribute("aria-expanded", "true");
  await expect(controlledRoot).toHaveAttribute("data-ui-action", "navigate-options");
  await expect(
    page.locator('[data-slot="combo-box-panel"] [data-slot="combo-box-listbox"]').first()
  ).toBeVisible();

  await page.keyboard.press("Escape");
  await expect(controlledRoot).toHaveAttribute("data-state", "closed");
  await expect(controlledInput).toHaveAttribute("aria-expanded", "false");
  await expect(page.locator('[data-slot="combo-box-panel"]')).toHaveCount(0);

  await controlledInput.focus();
  await controlledInput.fill("Ru");
  await expect(controlledRoot).toHaveAttribute("data-typed", "true");
  await expect(controlledRoot).toHaveAttribute("data-ui-action", "filter-query");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(
    page
      .locator('[data-component="combo-box"] [data-slot="combo-box"][data-controlled="true"]')
      .first()
  ).toHaveAttribute("data-state", "closed");
});
