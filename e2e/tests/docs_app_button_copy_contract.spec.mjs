import { expect, test } from "@playwright/test";

test("docs-app button-copy flow uses semantic selectors with settled async copy states", async ({ page }) => {
  await page.addInitScript(() => {
    window.__copiedText = "";
    const clipboard = {
      writeText(value) {
        window.__copiedText = String(value);
        return Promise.resolve();
      },
    };
    Object.defineProperty(navigator, "clipboard", {
      configurable: true,
      value: clipboard,
    });
  });

  await page.goto("/#/components/button-copy");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page.locator("section.playground").first();
  await expect(playground).toBeVisible();

  const copyRoot = playground.locator('[data-slot="button-copy"]').first();
  const copyButton = copyRoot.locator('[data-slot="button"]').first();
  const status = copyRoot.locator('[data-slot="button-copy-status"]').first();

  await expect(copyRoot).toHaveAttribute("data-copy-status", "idle");
  await expect(copyRoot).toHaveAttribute("data-copyable", "true");
  await expect(copyButton).not.toBeDisabled();

  await copyButton.click();

  await expect(copyRoot).toHaveAttribute("data-copy-status", "copied");
  await expect(copyRoot).toHaveAttribute("data-copied", "true");
  await expect(status).toContainText("Copied!");
  await expect
    .poll(() => page.evaluate(() => window.__copiedText))
    .toBe("cargo add ui-components");

  await expect(copyRoot).toHaveAttribute("data-copy-status", "idle", { timeout: 3500 });
  await expect(copyRoot).not.toHaveAttribute("data-copied", "true", { timeout: 3500 });
});
