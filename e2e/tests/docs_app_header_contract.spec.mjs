import { expect, test } from "@playwright/test";

test("docs-app header uses semantic selectors with wasm-stable waits", async ({ page }) => {
  await page.goto("/#/components/header");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("Header");

  const header = page.locator('[data-component="header"] [data-slot="header"]').first();
  await expect(header).toBeVisible();
  await expect(header).toHaveAttribute("data-ui-schema", "ui.header");
  await expect(header).toHaveAttribute("data-ui-intent", "section-heading");
  await expect(header).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(header).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(header).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(header).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app header key flow is repeatable with semantic contract breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/header");
  await page.locator("body:not(:has(#boot))").waitFor();

  const interactive = page.locator(
    '[data-component="header"] [data-slot="header"].docs-header-interactive',
  );
  const summary = page.locator('[data-component="header"] [data-slot="header-interactive-summary"]');
  await expect(interactive).toBeVisible();
  await expect(interactive).toHaveAttribute("data-ui-state", "default");
  await expect(interactive).toHaveAttribute("data-ui-source", "implicit-default");
  await expect(summary).toContainText("tone=default bordered=false");

  await page.locator('[data-action="toggle-tone"]').click();
  await expect(interactive).toHaveAttribute("data-ui-state", "strong");
  await expect(interactive).toHaveAttribute("data-ui-source", "props-strong");
  await expect(summary).toContainText("tone=strong bordered=false");

  await page.locator('[data-action="toggle-bordered"]').click();
  await expect(interactive).toHaveAttribute("data-ui-state", "strong-bordered");
  await expect(summary).toContainText("tone=strong bordered=true");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  const interactiveAfterReload = page.locator(
    '[data-component="header"] [data-slot="header"].docs-header-interactive',
  );
  const summaryAfterReload = page.locator(
    '[data-component="header"] [data-slot="header-interactive-summary"]',
  );
  await expect(interactiveAfterReload).toHaveAttribute("data-ui-state", "default");
  await expect(interactiveAfterReload).toHaveAttribute("data-ui-source", "implicit-default");
  await expect(summaryAfterReload).toContainText("tone=default bordered=false");
});
