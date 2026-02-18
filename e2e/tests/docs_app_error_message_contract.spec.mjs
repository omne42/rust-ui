import { expect, test } from "@playwright/test";

const ERROR_MESSAGE_PAGE = "/#/components/error-message";

test("docs-app error-message semantic markers are stable", async ({ page }) => {
  await page.goto(ERROR_MESSAGE_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("ErrorMessage");

  const root = page
    .locator('[data-component="error-message"] [data-slot="error-message"]')
    .first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-tone", "negative");
  await expect(root).toHaveAttribute("data-state", "default");
  await expect(root).toHaveAttribute("data-message-source", "custom");
  await expect(root).toHaveAttribute("data-aria-source", "custom");
  await expect(root).toHaveAttribute(
    "data-ui-schema",
    "ui.error-message.agent-contract.v1",
  );
  await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");
  await expect(root).toHaveAttribute("role", "alert");
  await expect(root).toHaveAttribute("aria-live", "assertive");

  const disabled = page
    .locator(
      '[data-component="error-message"] [data-slot="error-message"][data-disabled="true"]',
    )
    .first();
  await expect(disabled).toBeVisible();
  await expect(disabled).toHaveAttribute("data-state", "disabled");
  await expect(disabled).toHaveAttribute("data-ui-action", "read-only");
  await expect(disabled).toHaveAttribute("data-output-status", "draft");
});

test("docs-app error-message key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto(ERROR_MESSAGE_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page
    .locator('[data-component="error-message"] [data-slot="error-message"]')
    .first();
  await expect(root).toHaveAttribute("data-state", "default");
  await expect(root).toHaveAttribute("data-ui-action", "announce-error");

  await page.goto("/#/components/description");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("Description");

  await page.goto(ERROR_MESSAGE_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
  const reloaded = page
    .locator('[data-component="error-message"] [data-slot="error-message"]')
    .first();
  await expect(reloaded).toHaveAttribute("data-state", "default");
  await expect(reloaded).toHaveAttribute("data-ui-stream-fallback", "snapshot");
});

test("docs-app error-message playground code path remains copy-paste ready", async ({
  page,
}) => {
  await page.goto(ERROR_MESSAGE_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-component="error-message"] section.playground')
    .first();
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
  await expect(
    codeBlock.first().locator('[data-slot="code-block-code"]').first(),
  ).toContainText("<ErrorMessage text=\"Invalid email address\".to_string() />");
  await expect(codeBlock.first().locator('[data-slot="button"]').first()).toBeVisible();
});
