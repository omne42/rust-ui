import { expect, test } from "@playwright/test";

test("docs-app skeleton contract uses semantic selectors with settled waits", async ({ page }) => {
  await page.goto("/#/components/skeleton");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-component="skeleton"] [data-slot="skeleton"]').first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-state", "shimmer");
  await expect(root).toHaveAttribute("data-variant", /rect|circle/);
  await expect(root).toHaveAttribute("aria-hidden", "true");
});

test("docs-app skeleton-group key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/skeleton-group");
  await page.locator("body:not(:has(#boot))").waitFor();

  const pageRoot = page.locator('[data-component="skeleton-group"]').first();
  await expect(pageRoot).toBeVisible();

  const loadingGroup = pageRoot
    .locator('[data-slot="skeleton-group"][data-state="loading"][data-visibility="visible"]')
    .first();
  await expect(loadingGroup).toBeVisible();
  await expect(loadingGroup).toHaveAttribute("aria-busy", "true");

  const hiddenGroup = pageRoot
    .locator('[data-slot="skeleton-group"][data-loading-mode="skeleton-only"][data-visibility="hidden"]')
    .first();
  await expect(hiddenGroup).toHaveAttribute("hidden", "");
  await expect(hiddenGroup).toHaveAttribute("aria-busy", "");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedHiddenGroup = page
    .locator(
      '[data-component="skeleton-group"] [data-slot="skeleton-group"][data-loading-mode="skeleton-only"][data-visibility="hidden"]'
    )
    .first();
  await expect(reloadedHiddenGroup).toHaveAttribute("hidden", "");
  await expect(reloadedHiddenGroup).toHaveAttribute("data-state", "loaded");
});

test("docs-app skeleton playground source is copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/skeleton");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page.locator('[data-component="skeleton"] section.playground').first();
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
  await expect(code).toContainText("<Skeleton");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);
});
