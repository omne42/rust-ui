import { expect, test } from "@playwright/test";

test("docs-app pagination exposes semantic state markers", async ({ page }) => {
  await page.goto("/#/components/pagination");
  await page.locator("body:not(:has(#boot))").waitFor();

  const display = page.locator('[data-slot="pagination-display-playground"]').first();
  const controlled = display.locator('[data-slot="pagination"]').first();
  await expect(display).toBeVisible();
  await expect(controlled).toHaveAttribute("data-page", "1");
  await expect(controlled).toHaveAttribute("data-total-pages", "12");
  await expect(controlled.locator('[data-slot="pagination-prev"]').first()).toHaveAttribute(
    "data-disabled",
    "true"
  );

  const activePage = controlled.locator('[data-slot="pagination-page"][data-current="true"]');
  await expect(activePage).toHaveAttribute("data-page", "1");

  const nextButton = controlled
    .locator('[data-slot="pagination-next"] button')
    .first();
  await nextButton.click();

  await expect(controlled).toHaveAttribute("data-page", "2");
  await expect(page.locator("text=last change: 2").first()).toBeVisible();
});

test("docs-app pagination disabled/empty matrix is stable across reload", async ({
  page,
}) => {
  await page.goto("/#/components/pagination");
  await page.locator("body:not(:has(#boot))").waitFor();

  const disabled = page
    .locator('[data-slot="pagination-state-disabled"]')
    .locator('[data-slot="pagination"]')
    .first();
  await expect(disabled).toHaveAttribute("data-disabled", "true");
  await expect(disabled).toHaveAttribute("data-single-page", "true");

  const empty = page
    .locator('[data-slot="pagination-state-empty"]')
    .locator('[data-slot="pagination"]')
    .first();
  await expect(empty).toHaveAttribute("data-empty", "true");
  await expect(empty).toHaveAttribute("data-total-pages", "0");
  await expect(empty).toHaveAttribute("data-page", "1");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const emptyAfterReload = page
    .locator('[data-slot="pagination-state-empty"]')
    .locator('[data-slot="pagination"]')
    .first();
  await expect(emptyAfterReload).toHaveAttribute("data-empty", "true");
  await expect(emptyAfterReload).toHaveAttribute("data-page", "1");
});

test("docs-app pagination css test exposes custom class contract", async ({ page }) => {
  await page.goto("/#/components/pagination");
  await page.locator("body:not(:has(#boot))").waitFor();

  const custom = page.locator('[data-slot="pagination-css-test-custom"]').first();
  const root = custom.locator('[data-slot="pagination"]').first();

  await expect(custom).toBeVisible();
  await expect(root).toHaveClass(/docs-pagination-custom/);
});
