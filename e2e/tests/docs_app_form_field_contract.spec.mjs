import { expect, test } from "@playwright/test";

test("docs-app form-field contract uses semantic selectors with settled waits", async ({
  page,
}) => {
  await page.goto("/#/components/form-field");
  await page.locator("body:not(:has(#boot))").waitFor();

  const marketing = page.locator('#docs-form-field-marketing[data-slot="form-field"]').first();
  await expect(marketing).toBeVisible();
  await expect(marketing).toHaveAttribute("data-indicator-placement", "start");
  await expect(marketing).toHaveAttribute("data-indicator-variant", "switch");
  await expect(marketing).toHaveAttribute("data-message-kind", "description");
  await expect(marketing).toHaveAttribute("data-state", "selected");

  const marketingSwitch = marketing.locator('[data-slot="switch"]').first();
  await marketingSwitch.click();
  await expect(marketing).toHaveAttribute("data-state", "unselected");

  const tos = page.locator('#docs-form-field-tos[data-slot="form-field"]').first();
  await expect(tos).toBeVisible();
  await expect(tos).toHaveAttribute("data-indicator-variant", "checkbox");
  await expect(tos).toHaveAttribute("data-tone", "quiet");
  await expect(tos).toHaveAttribute("data-message-kind", "error");
  await expect(tos).toHaveAttribute("data-invalid", "true");
  await expect(tos.locator('[data-slot="form-field-error"]').first()).toBeVisible();

  const readOnly = page.locator('#docs-form-field-read-only[data-slot="form-field"]').first();
  await expect(readOnly).toHaveAttribute("data-disabled", "true");
});

test("docs-app form-field key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/form-field");
  await page.locator("body:not(:has(#boot))").waitFor();

  const tos = page.locator('#docs-form-field-tos[data-slot="form-field"]').first();
  const tosCheckbox = tos.locator('[data-slot="checkbox"]').first();

  await tosCheckbox.focus();
  await expect(tosCheckbox).toBeFocused();
  await page.keyboard.press("Enter");

  await expect(tos).toHaveAttribute("data-state", "selected-invalid");
  await expect(tos).toHaveAttribute("data-selected", "true");

  await page.keyboard.press("Enter");
  await expect(tos).toHaveAttribute("data-state", "invalid");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedTos = page.locator('#docs-form-field-tos[data-slot="form-field"]').first();
  await expect(reloadedTos).toHaveAttribute("data-state", "invalid");
  await expect(reloadedTos).toHaveAttribute("data-selected", null);
});

test("docs-app form-field playground source is copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/form-field");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-component="form-field"] section.playground')
    .filter({ has: page.locator("#docs-form-field-marketing") })
    .first();
  await expect(playground).toBeVisible();

  const codeToggle = playground
    .getByRole("button", { name: /Show code|Hide code/ })
    .first();
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
  await expect(code).toContainText("<FormField");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);
});
