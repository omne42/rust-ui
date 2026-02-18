import { expect, test } from "@playwright/test";

test("docs-app search-field uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  await page.goto("/#/components/search-field");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page
    .locator('[data-slot="search-field"]')
    .filter({ has: page.locator("#docs-search-field-markers") })
    .first();
  const input = root.locator("#docs-search-field-markers");
  const controls = page.locator('[data-slot="search-field-marker-controls"]').first();
  const toggleInvalid = controls
    .locator('[data-slot="search-field-toggle-invalid"] [data-slot="button"]')
    .first();
  const toggleReadOnly = controls
    .locator('[data-slot="search-field-toggle-readonly"] [data-slot="button"]')
    .first();
  const toggleDisabled = controls
    .locator('[data-slot="search-field-toggle-disabled"] [data-slot="button"]')
    .first();

  await expect(root).toBeVisible();
  await expect(input).toBeVisible();
  await expect(toggleInvalid).toBeVisible();
  await expect(toggleReadOnly).toBeVisible();
  await expect(toggleDisabled).toBeVisible();

  await expect(root).toHaveAttribute("data-ui-schema", "ui.search-field");
  await expect(root).toHaveAttribute("data-ui-intent", "form-search-input");
  await expect(root).toHaveAttribute("data-value-control-mode", "controlled");
  await expect(root).toHaveAttribute("data-default-value-source", "custom");
  await expect(root).toHaveAttribute("data-value-change-source", "on_value_change");
  await expect(root).toHaveAttribute("data-requirement", "required");
  await expect(root).toHaveAttribute("data-state", "ready");
  await expect(root).toHaveAttribute("data-value", "filled");
  await expect(input).toHaveAttribute("aria-required", "true");
  await expect(input).toHaveAttribute("aria-keyshortcuts", "Escape");
});

test("docs-app search-field covers ready-settled keyboard and pointer flow via semantic markers", async ({
  page,
}) => {
  await page.goto("/#/components/search-field");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page
    .locator('[data-slot="search-field"]')
    .filter({ has: page.locator("#docs-search-field-markers") })
    .first();
  const input = root.locator("#docs-search-field-markers");
  const controls = page.locator('[data-slot="search-field-marker-controls"]').first();
  const toggleInvalid = controls
    .locator('[data-slot="search-field-toggle-invalid"] [data-slot="button"]')
    .first();
  const toggleReadOnly = controls
    .locator('[data-slot="search-field-toggle-readonly"] [data-slot="button"]')
    .first();
  const toggleDisabled = controls
    .locator('[data-slot="search-field-toggle-disabled"] [data-slot="button"]')
    .first();

  await input.fill("release");
  await expect(input).toHaveValue("release");
  await expect(root).toHaveAttribute("data-value", "filled");

  await input.press("Escape");
  await expect(input).toHaveValue("");
  await expect(root).toHaveAttribute("data-value", "empty");

  await toggleInvalid.click();
  await expect(root).toHaveAttribute("data-state", "invalid");
  await expect(root).toHaveAttribute("data-invalid", "true");
  await expect(input).toHaveAttribute("aria-invalid", "true");

  await toggleInvalid.focus();
  await expect(toggleInvalid).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(root).toHaveAttribute("data-state", "ready");
  await expect(root).not.toHaveAttribute("data-invalid", "true");

  await toggleReadOnly.click();
  await expect(root).toHaveAttribute("data-state", "readonly");
  await expect(root).toHaveAttribute("data-read-only", "true");
  await expect(input).toHaveAttribute("readonly", "");

  await toggleReadOnly.focus();
  await expect(toggleReadOnly).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(root).toHaveAttribute("data-state", "ready");
  await expect(root).not.toHaveAttribute("data-read-only", "true");

  await toggleDisabled.click();
  await expect(root).toHaveAttribute("data-state", "disabled");
  await expect(root).toHaveAttribute("data-disabled", "true");
  await expect(input).toHaveAttribute("disabled", "");

  await toggleDisabled.focus();
  await expect(toggleDisabled).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(root).toHaveAttribute("data-state", "ready");
  await expect(root).not.toHaveAttribute("data-disabled", "true");
});
