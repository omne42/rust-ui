import { expect, test } from "@playwright/test";

test("docs-app badge contract uses semantic selectors with settled waits", async ({ page }) => {
  await page.goto("/#/components/badge");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("Badge");

  const badge = page.locator('[data-component="badge"] [data-slot="badge"]').first();
  await expect(badge).toBeVisible();
  await expect(badge).toHaveAttribute("data-ui-schema", "ui.badge.agent-contract");
  await expect(badge).toHaveAttribute("data-ui-schema-version", "1");
  await expect(badge).toHaveAttribute("data-ui-intent", "status-display");
  await expect(badge).toHaveAttribute("data-ui-action", "initialize");
  await expect(badge).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(badge).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(badge).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(badge).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app badge key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/badge");
  await page.locator("body:not(:has(#boot))").waitFor();

  const defaultBadge = page
    .locator('[data-component="badge"] [data-slot="badge"][data-variant="default"]')
    .first();
  await expect(defaultBadge).toHaveAttribute("data-fill", "solid");
  await expect(defaultBadge).toHaveAttribute("data-class-source", "default");

  const customBadge = page
    .locator('[data-component="badge"] [data-slot="badge"][data-class-source="custom"]')
    .first();
  await expect(customBadge).toBeVisible();
  await expect(customBadge).toHaveAttribute("data-custom-class", "true");
  await expect(customBadge).toHaveAttribute("data-ui-source", "custom");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  const reloaded = page
    .locator('[data-component="badge"] [data-slot="badge"][data-variant="default"]')
    .first();
  await expect(reloaded).toHaveAttribute("data-fill", "solid");
  await expect(reloaded).toHaveAttribute("data-class-source", "default");
});

test("docs-app badge playground source is copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/badge");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page.locator('[data-component="badge"] section.playground').first();
  await expect(playground).toBeVisible();

  const codeToggle = playground.getByRole("button", { name: /Show code|Hide code/ }).first();
  await expect(codeToggle).toBeVisible();

  const codeBlock = playground.locator('[data-slot="code-block"]');
  if ((await codeBlock.count()) === 0) {
    await codeToggle.click();
  }

  await expect(codeBlock.first()).toBeVisible();
  await expect(codeBlock.first()).toHaveAttribute("data-copyable", "true");
  await expect(playground.locator('[data-slot="code-block-code"]').first()).toContainText(
    "<Badge>"
  );
});
