import { expect, test } from "@playwright/test";

test("docs-app color-picker contract uses semantic selectors with settled waits", async ({
  page,
}) => {
  await page.goto("/#/components/color-picker");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-component="color-picker"] section.playground')
    .filter({ has: page.locator("#docs-color-picker-basic") })
    .first();
  await expect(playground).toBeVisible();

  const root = playground.locator('#docs-color-picker-basic[data-slot="color-picker"]');
  const trigger = root.locator('[data-slot="color-picker-trigger"]').first();

  await expect(root).toHaveAttribute("data-open-mode", "controlled");
  await expect(root).toHaveAttribute("data-label-source", "custom");
  await expect(root).toHaveAttribute("data-aria-source", "default");
  await expect(root).toHaveAttribute("data-state", "selected");
  await expect(root).toHaveAttribute("data-has-selection", "true");

  await trigger.click();
  await expect(root).toHaveAttribute("data-state", "open");
  await expect(root).toHaveAttribute("data-open", "true");

  const panel = page.locator('#docs-color-picker-basic-panel[data-slot="color-picker-panel"]');
  await expect(panel).toBeVisible();
  await expect(panel).toHaveAttribute("role", "dialog");

  const blueOption = panel
    .locator('[data-slot="color-swatch-picker-option"][data-color="#3b82f6"]')
    .first();
  await blueOption.click();

  await expect(root.locator('[data-slot="color-picker-value"]').first()).toContainText("#3b82f6");

  await trigger.click();
  await expect(root).toHaveAttribute("data-state", "selected");
});

test("docs-app color-picker key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/color-picker");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page
    .locator('[data-component="color-picker"] #docs-color-picker-basic[data-slot="color-picker"]')
    .first();
  const trigger = root.locator('[data-slot="color-picker-trigger"]').first();

  await trigger.focus();
  await expect(trigger).toBeFocused();
  await page.keyboard.press("Enter");

  await expect(root).toHaveAttribute("data-state", "open");
  await expect(root).toHaveAttribute("data-open", "true");

  const blueOption = page
    .locator('#docs-color-picker-basic-panel [data-slot="color-swatch-picker-option"][data-color="#3b82f6"]')
    .first();
  await blueOption.focus();
  await expect(blueOption).toBeFocused();
  await page.keyboard.press("Enter");

  await expect(root.locator('[data-slot="color-picker-value"]').first()).toContainText("#3b82f6");

  await page.keyboard.press("Escape");
  await expect(root).toHaveAttribute("data-state", "selected");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedRoot = page
    .locator('[data-component="color-picker"] #docs-color-picker-basic[data-slot="color-picker"]')
    .first();
  await expect(reloadedRoot).toHaveAttribute("data-state", "selected");
  await expect(reloadedRoot.locator('[data-slot="color-picker-value"]').first()).toContainText(
    "#ef4444",
  );
});

test("docs-app color-picker playground source is copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/color-picker");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-component="color-picker"] section.playground')
    .filter({ has: page.locator("#docs-color-picker-basic") })
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
  await expect(code).toContainText("<ColorPicker");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);
});
