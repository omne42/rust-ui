import { expect, test } from "@playwright/test";

test("docs-app field-button keeps stable semantic selectors and settled contract states", async ({
  page,
}) => {
  await page.goto("/#/components/field-button");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-slot="field-button"]').first();
  await expect(root).toBeVisible();

  const playgrounds = root.locator("section.playground");
  await expect(playgrounds).toHaveCount(2);

  const defaultButton = playgrounds.nth(0).locator('[data-slot="button"]').first();
  const quietButton = playgrounds.nth(0).locator('[data-slot="button"]').nth(1);
  const invalidActiveButton = playgrounds.nth(1).locator('[data-slot="button"]').first();
  const disabledButton = playgrounds.nth(1).locator('[data-slot="button"]').nth(1);

  await expect(defaultButton).toHaveAttribute("aria-label", "Open options");
  await expect(quietButton).toHaveAttribute("aria-label", "Open calendar");
  await expect(quietButton).toHaveClass(/ui-field-button--quiet/);

  await expect(invalidActiveButton).toHaveAttribute("aria-label", "Invalid trigger");
  await expect(invalidActiveButton).toHaveClass(/ui-field-button--invalid/);
  await expect(invalidActiveButton).toHaveClass(/ui-field-button--active/);

  await expect(disabledButton).toHaveAttribute("aria-label", "Disabled trigger");
  await expect(disabledButton).toHaveAttribute("data-disabled", "true");
  await expect(disabledButton).toBeDisabled();
});

test("docs-app field-button key interaction flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/field-button");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-slot="field-button"]').first();
  await expect(root).toBeVisible();

  const playground = root.locator("section.playground").first();
  const defaultButton = playground.locator('[data-slot="button"]').first();
  const quietButton = playground.locator('[data-slot="button"]').nth(1);

  await expect(defaultButton).toHaveAttribute("aria-label", "Open options");
  await defaultButton.click();
  await expect(defaultButton).toBeFocused();

  await expect(quietButton).toHaveAttribute("aria-label", "Open calendar");
  await quietButton.click();
  await expect(quietButton).toBeFocused();

  await expect(root).toHaveAttribute("data-slot", "field-button");
});

test("docs-app field-button playground source is copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/field-button");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-slot="field-button"] section.playground')
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
  await expect(code).toContainText("use ui::*;");
  await expect(code).toContainText("<FieldButton");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);
});
