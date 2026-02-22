import { expect, test } from "@playwright/test";

test("docs-app tag-group contract uses semantic selectors with settled waits", async ({ page }) => {
  await page.goto("/#/components/tag-group");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-component="tag-group"] [data-slot="tag-group"]').first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("role", "group");
  await expect(root).toHaveAttribute("data-ui-schema", "ui.tag-group.agent-contract");
  await expect(root).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "full-snapshot");
  await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(root).toHaveAttribute("data-count", /[0-9]+/);
});

test("docs-app tag-group key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/tag-group");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page
    .locator('[data-component="tag-group"] [data-slot="tag-group"]')
    .first();
  await expect(root).toHaveAttribute("data-count", "3");
  await expect(root).toHaveAttribute("data-ui-action", "initialize");

  const removeButton = root.locator('[data-slot="tag-remove-button"]').first();
  await removeButton.focus();
  await page.keyboard.press("Enter");

  await expect(root).toHaveAttribute("data-count", "2");
  await expect(root).toHaveAttribute("data-ui-action", "remove-pointer");
  await expect(root).toHaveAttribute("data-ui-source", "remove-pointer");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedRoot = page
    .locator('[data-component="tag-group"] [data-slot="tag-group"]')
    .first();
  await expect(reloadedRoot).toHaveAttribute("data-count", "3");
  await expect(reloadedRoot).toHaveAttribute("data-ui-action", "initialize");
  await expect(reloadedRoot).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app tag-group playground source is copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/tag-group");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-component="tag-group"] section.playground')
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
  await expect(code).toContainText("<TagGroup");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);
});
