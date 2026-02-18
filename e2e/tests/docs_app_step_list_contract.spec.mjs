import { expect, test } from "@playwright/test";

test("docs-app step-list contract uses semantic selectors with settled waits", async ({ page }) => {
  await page.goto("/#/components/step-list");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-component="step-list"]').first();
  await expect(root).toBeVisible();

  const controlledPlayground = root
    .locator("section.playground")
    .filter({ has: page.locator("span.ui-muted", { hasText: "selected index:" }) })
    .first();
  await expect(controlledPlayground).toBeVisible();

  const stepListRoot = controlledPlayground.locator('[data-slot="step-list"]').first();
  await expect(stepListRoot).toHaveAttribute("data-orientation", "horizontal");
  await expect(stepListRoot).toHaveAttribute("data-size", "m");
  await expect(stepListRoot).toHaveAttribute("data-state", "active");
  await expect(stepListRoot).toHaveAttribute("data-selected-index", "1");
  await expect(stepListRoot).toHaveAttribute("data-has-selection", "true");

  const currentItem = stepListRoot.locator('[data-slot="step-list-item"][data-index="1"]').first();
  const currentButton = currentItem.locator('[data-slot="step-list-button"]').first();
  await expect(currentItem).toHaveAttribute("data-status", "current");
  await expect(currentButton).toHaveAttribute("aria-current", "step");
  await expect(currentButton).toHaveAttribute("tabindex", "0");
});

test("docs-app step-list key flow is repeatable with semantic state breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/step-list");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-component="step-list"]').first();
  const controlledPlayground = root
    .locator("section.playground")
    .filter({ has: page.locator("span.ui-muted", { hasText: "selected index:" }) })
    .first();
  const stepListRoot = controlledPlayground.locator('[data-slot="step-list"]').first();

  const currentButton = stepListRoot
    .locator('[data-slot="step-list-item"][data-index="1"] [data-slot="step-list-button"]')
    .first();

  await currentButton.focus();
  await expect(currentButton).toBeFocused();
  await page.keyboard.press("ArrowRight");

  await expect(stepListRoot).toHaveAttribute("data-selected-index", "2");
  await expect(
    stepListRoot.locator('[data-slot="step-list-item"][data-index="2"]').first()
  ).toHaveAttribute("data-status", "current");

  await page.keyboard.press("ArrowLeft");
  await expect(stepListRoot).toHaveAttribute("data-selected-index", "1");

  const verticalPlayground = root
    .locator("section.playground")
    .filter({ has: page.locator('[data-slot="step-list"][data-orientation="vertical"]') })
    .first();
  const disabledButton = verticalPlayground
    .locator('[data-slot="step-list-item"][data-status="disabled"] [data-slot="step-list-button"]')
    .first();

  await expect(disabledButton).toHaveAttribute("aria-disabled", "true");
  await expect(disabledButton).toBeDisabled();
});

test("docs-app step-list playground source is copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/step-list");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-component="step-list"] section.playground')
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
  await expect(code).toContainText("use ui_components::");
  await expect(code).toContainText("<StepList");
});
