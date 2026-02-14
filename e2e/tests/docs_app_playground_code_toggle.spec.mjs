import { expect, test } from "@playwright/test";

test("docs-app component playground can toggle code visibility", async ({ page }) => {
  await page.goto("/#/components/button");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page.locator("section.playground").first();
  await expect(playground).toBeVisible();

  const toggle = playground.getByRole("button", { name: /Hide code|Show code/ });
  await expect(toggle).toBeVisible();

  const codeBlock = playground.locator('[data-slot="code-block"]');

  const wasVisible = await codeBlock.count().then((c) => c > 0);
  await toggle.click();

  if (wasVisible) {
    await expect(codeBlock).toHaveCount(0);
  } else {
    await expect(codeBlock.first()).toBeVisible();
  }
});
