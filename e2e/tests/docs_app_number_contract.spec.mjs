import { expect, test } from "@playwright/test";

test("docs-app static-number contract uses semantic selectors with settled waits", async ({
  page,
}) => {
  await page.goto("/#/components/static-number");
  await page.locator("body:not(:has(#boot))").waitFor();

  const component = page.locator('[data-component="static-number"]').first();
  await expect(component).toBeVisible();

  const numbers = component.locator('[data-slot="static-number"]');
  await expect(numbers.first()).toBeVisible();
  await expect(numbers.first()).toHaveAttribute("data-sign", "positive");
  await expect(numbers.first()).toHaveAttribute("data-decimal-separator-source", "default");
  await expect(numbers.first()).toHaveAttribute("data-decimal-places-source", "custom");
  await expect(numbers.first()).toHaveAttribute("data-thousand-separator-source", "custom");

  await expect(numbers.nth(1)).toHaveAttribute("data-sign", "negative");
  await expect(numbers.nth(2)).toHaveAttribute("data-thousand-separator-source", "none");

  const custom = component.locator(".docs-static-number-custom").first();
  await expect(custom).toHaveAttribute("data-class-source", "custom");
});

test("docs-app sliding-number key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/sliding-number");
  await page.locator("body:not(:has(#boot))").waitFor();

  const component = page.locator('[data-component="sliding-number"]').first();
  const sliding = component.locator('[data-slot="sliding-number"]').first();
  const a11yValue = sliding.locator('[data-slot="sliding-number-a11y-value"]').first();

  await expect(sliding).toBeVisible();
  await expect(sliding).toHaveAttribute("data-state", "animated");
  await expect(sliding).toHaveAttribute("data-motion-source", "default");
  await expect(a11yValue).toContainText("12,345.67");

  await page.keyboard.press("Tab");
  await page.keyboard.press("Tab");
  await page.keyboard.press("Enter");
  await expect(a11yValue).toContainText("12,595.67");

  await page.keyboard.press("Tab");
  await page.keyboard.press("Enter");
  await expect(a11yValue).toContainText("12,495.67");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedValue = page
    .locator('[data-component="sliding-number"] [data-slot="sliding-number-a11y-value"]')
    .first();
  await expect(reloadedValue).toContainText("12,345.67");
});

test("docs-app number playground source is copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/static-number");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-component="static-number"] section.playground')
    .filter({ has: page.locator('[data-slot="static-number"]') })
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
  await expect(code).toContainText("<StaticNumber");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);
});
