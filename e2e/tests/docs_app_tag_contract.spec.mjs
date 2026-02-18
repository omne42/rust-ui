import { expect, test } from "@playwright/test";

test("docs-app tag contract uses semantic selectors with settled waits", async ({ page }) => {
  await page.goto("/#/components/tag");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-component="tag"] [data-slot="tag"]').first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-ui-schema", "ui.tag.agent-contract");
  await expect(root).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "full-snapshot");
  await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(root).toHaveAttribute("data-ui-output-status", /verified|submittable/);
});

test("docs-app tag key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/tag");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-component="tag"]').first();
  await expect(root).toBeVisible();

  const removableTag = root.locator('[data-slot="tag"][data-removable="true"]').first();
  await expect(removableTag).toHaveAttribute("data-ui-action", "initialize");

  const removeButton = removableTag.locator('[data-slot="tag-remove-button"]').first();
  await removeButton.focus();
  await page.keyboard.press("Enter");

  await expect(removableTag).toHaveAttribute("data-ui-action", "remove-pointer");
  await expect(removableTag).toHaveAttribute("data-ui-source", "remove-pointer");
  await expect(removableTag).toHaveAttribute("data-ui-output-status", "submittable");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedTag = page
    .locator('[data-component="tag"] [data-slot="tag"][data-removable="true"]')
    .first();
  await expect(reloadedTag).toHaveAttribute("data-ui-action", "initialize");
  await expect(reloadedTag).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app tag playground source is copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/tag");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-component="tag"] section.playground')
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
  await expect(code).toContainText("<Tag");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);
});
