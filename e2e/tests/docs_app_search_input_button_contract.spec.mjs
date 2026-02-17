import { expect, test } from "@playwright/test";

test("docs-app search-input-button flow uses semantic selectors and stable readiness", async ({
  page,
}) => {
  await page.goto("/#/components/search-input-button");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page.locator("section.playground").first();
  await expect(playground).toBeVisible();

  const button = playground.locator('[data-slot="search-input-button"]').first();
  await expect(button).toBeVisible();
  await expect(button).toHaveAttribute("data-state", "enabled");
  await expect(button).toHaveAttribute("data-shortcut", "hidden");
  await expect(button).toHaveAttribute("data-placeholder", "custom");
  await expect(button).toHaveAttribute("data-compact-placeholder", "custom");
  await expect(button).toHaveAttribute("data-aria-label-source", "placeholder");

  const fullPlaceholder = button.locator(
    '[data-slot="search-input-button-placeholder-full"]',
  );
  await expect(fullPlaceholder).toContainText("Search");

  const shortcut = button.locator('[data-slot="search-input-button-shortcut"]');
  await expect(shortcut).toHaveCount(0);

  await button.hover();
  await expect(button).toHaveAttribute("data-hovered", "true");

  const matrix = page.locator("section.playground").nth(1);
  await expect(matrix).toBeVisible();

  const disabledButton = matrix
    .locator('[data-slot="search-input-button"][data-state="disabled"]')
    .first();
  await expect(disabledButton).toBeVisible();
  await expect(disabledButton).toBeDisabled();
  await expect(disabledButton).toHaveAttribute("data-disabled", "true");
});
