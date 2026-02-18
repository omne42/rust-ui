import { expect, test } from "@playwright/test";

test("docs-app alert-banner exposes semantic state/source markers", async ({ page }) => {
  await page.goto("/#/components/alert-banner");
  await page.locator("body:not(:has(#boot))").waitFor();

  const banners = page.locator('section[data-slot="alert-banner"]');
  await expect(banners).toHaveCount(4);

  const info = banners.nth(0);
  await expect(info).toHaveAttribute("data-tone", "info");
  await expect(info).toHaveAttribute("data-fill", "border");
  await expect(info).toHaveAttribute("role", "status");
  await expect(info).toHaveAttribute("aria-live", "polite");
  await expect(info).toHaveAttribute("data-hide-icon-source", "default");
  await expect(info).toHaveAttribute("data-icon-visible", "true");

  const negative = banners.nth(1);
  await expect(negative).toHaveAttribute("data-tone", "negative");
  await expect(negative).toHaveAttribute("role", "alert");
  await expect(negative).toHaveAttribute("aria-live", "assertive");

  const hiddenIcon = banners.nth(2);
  await expect(hiddenIcon).toHaveAttribute("data-hide-icon-source", "is-hide-icon");
  await expect(hiddenIcon).toHaveAttribute("data-hide-icon", "true");
  await expect(hiddenIcon).toHaveAttribute("data-icon-visible", "false");

  const customMotion = banners.nth(3);
  await expect(customMotion).toHaveAttribute("data-motion-source", "custom");
  await expect(customMotion).toHaveAttribute("data-custom-motion", "true");
});

test("docs-app alert-banner semantics stay stable after reload", async ({ page }) => {
  await page.goto("/#/components/alert-banner");
  await page.locator("body:not(:has(#boot))").waitFor();

  const first = page.locator('section[data-slot="alert-banner"]').first();
  await expect(first).toHaveAttribute("data-tone", "info");
  await expect(first).toHaveAttribute("data-motion-source", "default");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const afterReload = page.locator('section[data-slot="alert-banner"]').first();
  await expect(afterReload).toHaveAttribute("data-tone", "info");
  await expect(afterReload).toHaveAttribute("data-motion-source", "default");
});
