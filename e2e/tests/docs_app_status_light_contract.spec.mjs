import { expect, test } from "@playwright/test";

test("docs-app: status-light semantic markers are stable", async ({ page }) => {
  await page.goto("/#/components/status-light");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("StatusLight");

  const root = page.locator('[data-component="status-light"] [data-slot="status-light"]').first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-variant", "default");
  await expect(root).toHaveAttribute("data-state", "static");
  await expect(root).toHaveAttribute("data-role-source", "none");
  await expect(root).toHaveAttribute("data-class-source", "default");

  const live = page
    .locator('[data-component="status-light"] [data-slot="status-light"][data-role-source="custom"]')
    .first();
  await expect(live).toBeVisible();
  await expect(live).toHaveAttribute("role", "status");
  await expect(live).toHaveAttribute("aria-live", "polite");
  await expect(live).toHaveAttribute("data-state", "live");
});

test("docs-app: status-light key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/status-light");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-component="status-light"] [data-slot="status-light"]').first();
  await expect(root).toHaveAttribute("data-state", "static");
  await expect(root).toHaveAttribute("data-role-source", "none");

  const live = page
    .locator('[data-component="status-light"] [data-slot="status-light"][data-role-source="custom"]')
    .first();
  await expect(live).toHaveAttribute("data-state", "live");
  await expect(live).toHaveAttribute("data-role-source", "custom");

  await page.goto("/#/components/badge");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("Badge");

  await page.goto("/#/components/status-light");
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedRoot = page
    .locator('[data-component="status-light"] [data-slot="status-light"]')
    .first();
  await expect(reloadedRoot).toHaveAttribute("data-state", "static");
  await expect(reloadedRoot).toHaveAttribute("data-role-source", "none");
});

test("docs-app: status-light playground code path remains copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/status-light");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page.locator('[data-component="status-light"] section.playground').first();
  await expect(playground).toBeVisible();

  const toggle = playground.getByRole("button", { name: /Hide code|Show code/ });
  await expect(toggle).toBeVisible();

  const codeBlock = playground.locator('[data-slot="code-block"]');
  const wasVisible = await codeBlock.count().then((count) => count > 0);
  if (!wasVisible) {
    await toggle.click();
  }

  await expect(codeBlock.first()).toBeVisible();
  await expect(codeBlock.first()).toHaveAttribute("data-copyable", "true");
  await expect(codeBlock.first().locator('[data-slot="code-block-code"]').first()).toContainText(
    "<StatusLight variant=StatusLightVariant::Default>"
  );
  await expect(codeBlock.first().locator('[data-slot="button"]').first()).toBeVisible();
});
