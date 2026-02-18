import { expect, test } from "@playwright/test";

test("docs-app fieldset contract uses semantic selectors with settled waits", async ({ page }) => {
  await page.goto("/#/components/fieldset");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("Fieldset");

  const fieldset = page.locator('[data-component="fieldset"] [data-slot="fieldset"]').first();
  await expect(fieldset).toBeVisible();
  await expect(fieldset).toHaveAttribute("data-ui-schema", "ui.fieldset.agent-contract");
  await expect(fieldset).toHaveAttribute("data-ui-schema-version", "1");
  await expect(fieldset).toHaveAttribute("data-ui-intent", "form-grouping");
  await expect(fieldset).toHaveAttribute("data-ui-action", "initialize");
  await expect(fieldset).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(fieldset).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(fieldset).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(fieldset).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app fieldset key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/fieldset");
  await page.locator("body:not(:has(#boot))").waitFor();

  const requiredFieldset = page
    .locator('[data-component="fieldset"] [data-slot="fieldset"][data-required="true"]')
    .first();
  await expect(requiredFieldset).toBeVisible();
  await expect(requiredFieldset).toHaveAttribute("data-required-source", "required");
  await expect(requiredFieldset).toHaveAttribute("data-invalid-source", "default");

  const invalidFieldset = page
    .locator('[data-component="fieldset"] [data-slot="fieldset"][data-invalid="true"]')
    .first();
  await expect(invalidFieldset).toBeVisible();
  await expect(invalidFieldset).toHaveAttribute("data-invalid-source", "is_invalid");
  await expect(invalidFieldset).toHaveAttribute("data-error-source", "custom");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  const reloaded = page
    .locator('[data-component="fieldset"] [data-slot="fieldset"][data-invalid="true"]')
    .first();
  await expect(reloaded).toHaveAttribute("data-invalid-source", "is_invalid");
});

test("docs-app fieldset playground source is copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/fieldset");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page.locator('[data-component="fieldset"] section.playground').first();
  await expect(playground).toBeVisible();

  const codeToggle = playground.getByRole("button", { name: /Show code|Hide code/ }).first();
  await expect(codeToggle).toBeVisible();

  const codeBlock = playground.locator('[data-slot="code-block"]');
  if ((await codeBlock.count()) === 0) {
    await codeToggle.click();
  }

  await expect(codeBlock.first()).toBeVisible();
  await expect(codeBlock.first()).toHaveAttribute("data-copyable", "true");
  await expect(playground.locator('[data-slot="code-block-code"]').first()).toContainText(
    "<Fieldset"
  );
});
