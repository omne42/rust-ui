import { expect, test } from "@playwright/test";

test("docs-app separator uses semantic selectors with wasm-stable waits", async ({ page }) => {
  await page.goto("/#/components/separator");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("Separator");

  const semantic = page
    .locator('[data-component="separator"] [data-slot="separator"][data-state="semantic"]')
    .first();
  const decorative = page
    .locator('[data-component="separator"] [data-slot="separator"][data-state="decorative"]')
    .first();

  await expect(semantic).toBeVisible();
  await expect(semantic).toHaveAttribute("role", "separator");
  await expect(semantic).toHaveAttribute("data-state-source", "props-static");
  await expect(semantic).toHaveAttribute("data-output-mode", "snapshot");
  await expect(semantic).toHaveAttribute("data-streaming-fallback", "snapshot");
  await expect(semantic).toHaveAttribute("data-output-status", "verified");

  await expect(decorative).toBeVisible();
  await expect(decorative).toHaveAttribute("aria-hidden", "true");
  await expect(decorative).toHaveAttribute("data-state-source", "props-static");
  await expect(decorative).toHaveAttribute("data-output-mode", "snapshot");
  await expect(decorative).toHaveAttribute("data-streaming-fallback", "snapshot");
  await expect(decorative).toHaveAttribute("data-output-status", "verified");
});

test("docs-app separator key flow is repeatable with semantic contract breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/separator");
  await page.locator("body:not(:has(#boot))").waitFor();

  const separatorNodes = page.locator('[data-component="separator"] [data-slot="separator"]');
  await expect(separatorNodes.first()).toBeVisible();
  await expect(separatorNodes).toHaveCount(5);
  await expect(separatorNodes.first()).toHaveAttribute("data-state-source", "props-static");
  await expect(separatorNodes.first()).toHaveAttribute("data-output-status", "verified");

  await page.goto("/#/components/spacer");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("Spacer");

  await page.goto("/#/components/separator");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(separatorNodes.first()).toHaveAttribute("data-state-source", "props-static");
  await expect(separatorNodes.first()).toHaveAttribute("data-output-status", "verified");
});
