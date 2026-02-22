import { expect, test } from "@playwright/test";

async function gotoColorSwatchDocs(page) {
  await page.goto("/#/components/color-swatch");
  await page.locator("body:not(:has(#boot))").waitFor();

  const component = page.locator('[data-component="color-swatch"]').first();
  await expect(component).toBeVisible();
  await expect(
    component.locator('[data-slot="color-swatch"][data-ui-output-status="verified"]').first()
  ).toBeVisible();
  return component;
}

test("docs-app color-swatch contract uses semantic selectors with settled waits", async ({
  page,
}) => {
  const component = await gotoColorSwatchDocs(page);
  const root = component.locator('[data-slot="color-swatch"]').first();

  await expect(root).toHaveAttribute("data-ui-schema", "ui.color-swatch.agent-contract");
  await expect(root).toHaveAttribute("data-ui-schema-version", "1");
  await expect(root).toHaveAttribute("data-ui-intent", "color-preview");
  await expect(root).toHaveAttribute("data-ui-action", "render");
  await expect(root).toHaveAttribute("data-ui-stream-support", "optional");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");
  await expect(root).toHaveAttribute("role", "img");
  await expect(root).toHaveAttribute("data-aria-source", "default");
});

test("docs-app color-swatch key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  const component = await gotoColorSwatchDocs(page);
  const controls = component.locator('[data-slot="color-swatch-workbench-controls"]').first();
  const canvas = component.locator('[data-slot="color-swatch-workbench-canvas"]').first();
  const root = canvas.locator('[data-slot="color-swatch"]').first();

  await expect(controls).toBeVisible();
  await expect(canvas).toBeVisible();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-aria-source", "default");
  await expect(root).toHaveAttribute("data-motion-source", "default");

  const alphaControl = controls
    .locator('[data-slot="color-swatch-workbench-alpha-control"] [data-slot="segmented-control"]')
    .first();
  await alphaControl
    .locator('[data-slot="segmented-control-option"][data-index="2"]')
    .click();
  await expect(root).toHaveAttribute("data-alpha", "transparent");
  await expect(root).toHaveAttribute("data-state", "transparent");

  const decorativeSwitch = controls
    .locator('[data-slot="color-swatch-workbench-decorative-switch"] [data-slot="switch"]')
    .first();
  await decorativeSwitch.focus();
  await expect(decorativeSwitch).toBeFocused();
  await decorativeSwitch.press("Space");
  await expect(root).toHaveAttribute("data-decorative", "true");
  await expect(root).toHaveAttribute("aria-hidden", "true");
  await expect(root).not.toHaveAttribute("role", "img");

  await decorativeSwitch.press("Space");
  await expect(root).not.toHaveAttribute("aria-hidden", "true");
  await expect(root).toHaveAttribute("role", "img");

  const customAriaSwitch = controls
    .locator('[data-slot="color-swatch-workbench-custom-aria-switch"] [data-slot="switch"]')
    .first();
  await customAriaSwitch.click();
  await expect(root).toHaveAttribute("data-aria-source", "custom");
  await expect(root).toHaveAttribute("aria-label", /Background color/);

  const langSwitch = controls
    .locator('[data-slot="color-swatch-workbench-lang-switch"] [data-slot="switch"]')
    .first();
  await langSwitch.focus();
  await expect(langSwitch).toBeFocused();
  await langSwitch.press("Enter");
  await expect(root).toHaveAttribute("lang", "zh-CN");
  await expect(root).toHaveAttribute("data-motion-source", "default");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  const reloadedRoot = page
    .locator(
      '[data-component="color-swatch"] [data-slot="color-swatch-workbench-canvas"] [data-slot="color-swatch"]'
    )
    .first();
  await expect(reloadedRoot).toHaveAttribute("data-aria-source", "default");
  await expect(reloadedRoot).not.toHaveAttribute("lang", "zh-CN");
  await expect(reloadedRoot).toHaveAttribute("data-motion-source", "default");
});

test("docs-app color-swatch playground source is copy-paste ready", async ({ page }) => {
  const component = await gotoColorSwatchDocs(page);
  const playground = component.locator("section.playground").first();
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
  await expect(code).toContainText("use ui::*;");
  await expect(code).toContainText("<ColorSwatch");
});
