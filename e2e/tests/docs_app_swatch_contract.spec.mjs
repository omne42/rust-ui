import { expect, test } from "@playwright/test";

test("docs-app swatch contract uses semantic selectors with settled waits", async ({ page }) => {
  await page.goto("/#/components/swatch");
  await page.locator("body:not(:has(#boot))").waitFor();

  const swatch = page.locator('[data-component="swatch"] [data-slot="swatch"]').first();
  await expect(swatch).toBeVisible();
  await expect(swatch).toHaveAttribute("data-ui-schema", "ui.swatch.agent-contract");
  await expect(swatch).toHaveAttribute("data-ui-schema-version", "1");
  await expect(swatch).toHaveAttribute("data-ui-intent", "color-selection");
  await expect(swatch).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(swatch).toHaveAttribute("data-ui-stream-fallback", "full-snapshot");
  await expect(swatch).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(swatch).toHaveAttribute("data-ui-output-status", /verified|submittable/);
});

test("docs-app swatch key flow is repeatable and fails at semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/swatch");
  await page.locator("body:not(:has(#boot))").waitFor();

  const controlledSwatch = page
    .locator('[data-component="swatch"] [data-slot="swatch"][aria-label="Brand blue"]')
    .first();
  await expect(controlledSwatch).toBeVisible();

  await expect(controlledSwatch).toHaveAttribute("data-ui-action", "initialize");
  await expect(controlledSwatch).toHaveAttribute("data-ui-output-status", "verified");

  await controlledSwatch.focus();
  await page.keyboard.press("Enter");

  await expect(controlledSwatch).toHaveAttribute("data-ui-action", "toggle-press");
  await expect(controlledSwatch).toHaveAttribute("data-ui-source", "toggle-press");
  await expect(controlledSwatch).toHaveAttribute("data-ui-output-status", "submittable");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedSwatch = page
    .locator('[data-component="swatch"] [data-slot="swatch"][aria-label="Brand blue"]')
    .first();
  await expect(reloadedSwatch).toHaveAttribute("data-ui-action", "initialize");
  await expect(reloadedSwatch).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app swatch playground source is copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/swatch");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-component="swatch"] section.playground')
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
  await expect(code).toContainText("<Swatch");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);
});
