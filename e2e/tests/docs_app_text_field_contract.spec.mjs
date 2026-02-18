import { expect, test } from "@playwright/test";

test("docs-app text-field uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  await page.goto("/#/components/text-field");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page
    .locator('[data-slot="text-field"]')
    .filter({ has: page.locator("#docs-text-field-markers") })
    .first();
  const input = root.locator("#docs-text-field-markers");
  const controls = page.locator('[data-slot="text-field-marker-controls"]').first();
  const toggleInvalid = controls
    .locator('[data-slot="text-field-toggle-invalid"] [data-slot="button"]')
    .first();
  const toggleReadOnly = controls
    .locator('[data-slot="text-field-toggle-readonly"] [data-slot="button"]')
    .first();
  const toggleDisabled = controls
    .locator('[data-slot="text-field-toggle-disabled"] [data-slot="button"]')
    .first();

  await expect(root).toBeVisible();
  await expect(input).toBeVisible();
  await expect(toggleInvalid).toBeVisible();
  await expect(toggleReadOnly).toBeVisible();
  await expect(toggleDisabled).toBeVisible();

  await expect(root).toHaveAttribute("data-ui-schema", "ui.text-field");
  await expect(root).toHaveAttribute("data-ui-intent", "form-text-input");
  await expect(root).toHaveAttribute("data-motion-source", "default");
  await expect(root).toHaveAttribute("data-value-control-mode", "controlled");
  await expect(root).toHaveAttribute("data-default-value-source", "default");
  await expect(root).toHaveAttribute("data-value-change-source", "on_value_change");
  await expect(root).toHaveAttribute("data-requirement", "required");
  await expect(root).toHaveAttribute("data-label-source", "custom");
  await expect(root).toHaveAttribute("data-description-source", "custom");
  await expect(root).toHaveAttribute("data-error-source", "custom");
  await expect(root).toHaveAttribute("data-placeholder-source", "custom");
  await expect(root).toHaveAttribute("data-type-source", "custom");
  await expect(root).toHaveAttribute("data-state", "ready");
  await expect(root).toHaveAttribute("data-value", "filled");

  await expect(input).toHaveAttribute("aria-required", "true");
  await expect(input).toHaveAttribute("aria-describedby", /docs-text-field-markers-description/);
});

test("docs-app text-field covers ready-settled pointer and keyboard flow via semantic markers", async ({
  page,
}) => {
  await page.goto("/#/components/text-field");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page
    .locator('[data-slot="text-field"]')
    .filter({ has: page.locator("#docs-text-field-markers") })
    .first();
  const input = root.locator("#docs-text-field-markers");
  const controls = page.locator('[data-slot="text-field-marker-controls"]').first();
  const toggleInvalid = controls
    .locator('[data-slot="text-field-toggle-invalid"] [data-slot="button"]')
    .first();
  const toggleReadOnly = controls
    .locator('[data-slot="text-field-toggle-readonly"] [data-slot="button"]')
    .first();
  const toggleDisabled = controls
    .locator('[data-slot="text-field-toggle-disabled"] [data-slot="button"]')
    .first();

  await input.fill("qa@rustui.dev");
  await expect(input).toHaveValue("qa@rustui.dev");
  await expect(root).toHaveAttribute("data-value", "filled");
  await expect(root).toHaveAttribute("data-state", "ready");

  await toggleInvalid.click();
  await expect(root).toHaveAttribute("data-state", "invalid");
  await expect(root).toHaveAttribute("data-invalid", "true");
  await expect(input).toHaveAttribute("aria-invalid", "true");

  await toggleInvalid.focus();
  await expect(toggleInvalid).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(root).toHaveAttribute("data-state", "ready");
  await expect(root).not.toHaveAttribute("data-invalid", "true");
  await expect(input).not.toHaveAttribute("aria-invalid", "true");

  await toggleReadOnly.click();
  await expect(root).toHaveAttribute("data-state", "readonly");
  await expect(root).toHaveAttribute("data-read-only", "true");
  await expect(input).toHaveAttribute("readonly", "");

  await toggleReadOnly.focus();
  await expect(toggleReadOnly).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(root).toHaveAttribute("data-state", "ready");
  await expect(root).not.toHaveAttribute("data-read-only", "true");
  await expect(input).not.toHaveAttribute("readonly", "");

  await toggleDisabled.click();
  await expect(root).toHaveAttribute("data-state", "disabled");
  await expect(root).toHaveAttribute("data-disabled", "true");
  await expect(input).toHaveAttribute("disabled", "");

  await toggleDisabled.focus();
  await expect(toggleDisabled).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(root).toHaveAttribute("data-state", "ready");
  await expect(root).not.toHaveAttribute("data-disabled", "true");
  await expect(input).not.toHaveAttribute("disabled", "");
});
