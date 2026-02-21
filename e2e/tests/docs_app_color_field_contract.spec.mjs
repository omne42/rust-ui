import { expect, test } from "@playwright/test";

test("docs-app color-field contract uses semantic selectors with settled waits", async ({
  page,
}) => {
  await page.goto("/#/components/color-field");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-component="color-field"] section.playground')
    .filter({ has: page.locator("#docs-color-field-basic") })
    .first();
  await expect(playground).toBeVisible();

  const root = playground.locator('#docs-color-field-basic[data-slot="color-field"]').first();
  const input = root.locator('[data-slot="color-field-input"]').first();

  await expect(root).toHaveAttribute("data-state", "valid");
  await expect(root).toHaveAttribute("data-has-value", "true");
  await expect(root).toHaveAttribute("data-valid", "true");
  await expect(root).toHaveAttribute("data-label-source", "custom");
  await expect(root).toHaveAttribute("data-placeholder-source", "default");
  await expect(root).toHaveAttribute("data-aria-source", "default");
  await expect(root).toHaveAttribute("data-has-preview", "true");
  await expect(input).toHaveAttribute("aria-label", /.+/);
  await expect(input).toHaveAttribute("aria-labelledby", /docs-color-field-basic-label/);
  await expect(input).toHaveValue("#4f46e5");
});

test("docs-app color-field flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/color-field");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page
    .locator('[data-component="color-field"] #docs-color-field-basic[data-slot="color-field"]')
    .first();
  const input = root.locator('[data-slot="color-field-input"]').first();

  await input.focus();
  await expect(input).toBeFocused();
  await input.fill("javascript:alert(1)");
  await expect(root).toHaveAttribute("data-state", "invalid");
  await expect(root).toHaveAttribute("data-invalid", "true");
  await expect(root).not.toHaveAttribute("data-valid", "true");
  await expect(root).not.toHaveAttribute("data-has-preview", "true");
  await expect(input).toHaveAttribute("aria-invalid", "true");

  const clear = root.locator('[data-slot="color-field-clear"]').first();
  await clear.focus();
  await expect(clear).toBeFocused();
  await clear.press("Shift+Tab");
  await expect(input).toBeFocused();
  await clear.click();
  await expect(root).toHaveAttribute("data-state", "empty");
  await expect(root).not.toHaveAttribute("data-has-value", "true");
  await expect(root).not.toHaveAttribute("data-invalid", "true");
  await expect(input).toHaveValue("");

  const disabledRoot = page
    .locator('[data-component="color-field"] #docs-color-field-disabled[data-slot="color-field"]')
    .first();
  const disabledInput = disabledRoot.locator('[data-slot="color-field-input"]').first();

  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledInput).toBeDisabled();

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedRoot = page
    .locator('[data-component="color-field"] #docs-color-field-basic[data-slot="color-field"]')
    .first();
  await expect(reloadedRoot).toHaveAttribute("data-state", "valid");
});

test("docs-app color-field playground source is copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/color-field");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-component="color-field"] section.playground')
    .filter({ has: page.locator("#docs-color-field-basic") })
    .first();
  await expect(playground).toBeVisible();

  const codeToggle = playground.getByRole("button", { name: /Show code|Hide code/ }).first();
  await expect(codeToggle).toBeVisible();

  const codeBlock = playground.locator('[data-slot="code-block"]');
  if ((await codeBlock.count()) === 0) {
    await codeToggle.click();
  }

  await expect(codeBlock.first()).toBeVisible();
  await expect(codeBlock.first()).toHaveAttribute("data-copyable", "true");

  const code = playground.locator('[data-slot="code-block-code"]').first();
  await expect(code).toContainText("use leptos::prelude::*;");
  await expect(code).toContainText("use ui_components::*;");
  await expect(code).toContainText("<ColorField");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);
});
