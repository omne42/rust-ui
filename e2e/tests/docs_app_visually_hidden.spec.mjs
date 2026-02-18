import { expect, test } from "@playwright/test";

test("docs-app: visually-hidden semantic markers are stable", async ({ page }) => {
  await page.goto("/#/components/visually-hidden");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toBeVisible();

  const nodes = page.locator('.ui-visually-hidden[data-slot="visually-hidden"]');
  await expect(nodes).toHaveCount(3);

  const defaultNode = nodes.nth(0);
  await expect(defaultNode).toHaveAttribute("data-focus-mode", "hidden");
  await expect(defaultNode).toHaveAttribute("data-focus-source", "default");
  await expect(defaultNode).toHaveAttribute("data-class-source", "default");

  const focusableNode = nodes.nth(2);
  await expect(focusableNode).toHaveAttribute("data-focus-mode", "focusable");
  await expect(focusableNode).toHaveAttribute("data-focus-source", "is_focusable");
});

test("docs-app: visually-hidden focusable skip-link path remains keyboard reachable", async ({
  page,
}) => {
  await page.goto("/#/components/visually-hidden");
  await page.locator("body:not(:has(#boot))").waitFor();

  const skipLink = page.locator('a[href="#docs-visually-hidden-target"]');
  const target = page.locator("#docs-visually-hidden-target");
  await expect(target).toHaveCount(1);
  await skipLink.focus();
  await expect(skipLink).toBeFocused();
  await expect(skipLink).toHaveAttribute("href", "#docs-visually-hidden-target");
});
