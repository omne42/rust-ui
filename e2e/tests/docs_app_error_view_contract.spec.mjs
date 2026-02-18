import { expect, test } from "@playwright/test";

test("docs-app error-view exposes semantic state/source markers", async ({ page }) => {
  await page.goto("/#/components/error-view");
  await page.locator("body:not(:has(#boot))").waitFor();

  const blocks = page.locator('[data-slot="error-view"]');
  await expect(blocks).toHaveCount(3);

  const visible = blocks.nth(0);
  await expect(visible).toHaveAttribute("data-state", "visible");
  await expect(visible).toHaveAttribute("data-invalid", "true");
  await expect(visible).toHaveAttribute("data-message-source", "custom");
  await expect(visible).toHaveAttribute("data-tone", "negative");

  const hidden = blocks.nth(1);
  await expect(hidden).toHaveAttribute("data-state", "hidden");
  await expect(hidden).toHaveAttribute("data-hidden", "true");
  await expect(hidden).toHaveAttribute("aria-hidden", "true");

  const custom = blocks.nth(2);
  await expect(custom).toHaveAttribute("data-tone", "neutral");
  await expect(custom).toHaveAttribute("data-compact", "true");
  await expect(custom).toHaveAttribute("data-compact-source", "is-prop");
  await expect(custom).toHaveAttribute("data-bordered", "true");
  await expect(custom).toHaveAttribute("data-bordered-source", "is-prop");
  await expect(custom).toHaveAttribute("data-actions", "true");
  await expect(custom).toHaveAttribute("data-content", "children");
  await expect(custom).toHaveAttribute("data-motion-source", "custom");
});

test("docs-app error-view semantics remain stable after reload", async ({ page }) => {
  await page.goto("/#/components/error-view");
  await page.locator("body:not(:has(#boot))").waitFor();

  const custom = page.locator('[data-slot="error-view"]').nth(2);
  await expect(custom).toHaveAttribute("data-tone", "neutral");
  await expect(custom).toHaveAttribute("data-state", "visible");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const afterReload = page.locator('[data-slot="error-view"]').nth(2);
  await expect(afterReload).toHaveAttribute("data-tone", "neutral");
  await expect(afterReload).toHaveAttribute("data-state", "visible");
});
