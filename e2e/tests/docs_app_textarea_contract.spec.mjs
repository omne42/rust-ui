import { expect, test } from "@playwright/test";

test("docs-app textarea contract uses semantic selectors with settled waits", async ({
  page,
}) => {
  await page.goto("/#/components/textarea");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-component="textarea"] section.playground')
    .filter({ has: page.locator("#docs-textarea-marker") })
    .first();
  await expect(playground).toBeVisible();

  const root = playground.locator('[data-slot="textarea"]').first();
  const input = root.locator("#docs-textarea-marker");
  const toggleInvalid = playground.locator('[data-slot="button"]').first();

  await expect(root).toHaveAttribute("data-value-control-mode", "controlled");
  await expect(root).toHaveAttribute("data-default-value-source", "default");
  await expect(root).toHaveAttribute("data-value-change-source", "on_value_change");
  await expect(root).toHaveAttribute("data-state", "ready");
  await expect(root).toHaveAttribute("data-value", "filled");
  await expect(root).toHaveAttribute("data-requirement", "required");
  await expect(root).toHaveAttribute("data-label-source", "custom");
  await expect(root).toHaveAttribute("data-description-source", "custom");
  await expect(root).toHaveAttribute("data-error-source", "custom");
  await expect(root).toHaveAttribute("data-placeholder-source", "custom");
  await expect(root).toHaveAttribute("data-rows-source", "custom");

  await input.fill("updated summary for release notes");
  await expect(input).toHaveValue("updated summary for release notes");
  await expect(root).toHaveAttribute("data-value", "filled");

  await toggleInvalid.click();
  await expect(root).toHaveAttribute("data-state", "invalid");
  await expect(root).toHaveAttribute("data-invalid", "true");
  await expect(playground.locator('[data-slot="textarea-error"]')).toBeVisible();

  await toggleInvalid.click();
  await expect(root).toHaveAttribute("data-state", "ready");
});

test("docs-app textarea key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/textarea");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator('[data-component="textarea"] section.playground')
    .filter({ has: page.locator("#docs-textarea-marker") })
    .first();
  await expect(playground).toBeVisible();

  const root = playground.locator('[data-slot="textarea"]').first();
  const input = root.locator("#docs-textarea-marker");
  const toggleInvalid = playground.locator('[data-slot="button"]').first();

  await input.focus();
  await expect(input).toBeFocused();
  await page.keyboard.type(" semantic-key-flow");
  await expect(root).toHaveAttribute("data-value", "filled");

  await toggleInvalid.focus();
  await expect(toggleInvalid).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(root).toHaveAttribute("data-state", "invalid");
  await expect(root).toHaveAttribute("data-invalid", "true");
  await expect(playground.locator('[data-slot="textarea-error"]')).toBeVisible();

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedPlayground = page
    .locator('[data-component="textarea"] section.playground')
    .filter({ has: page.locator("#docs-textarea-marker") })
    .first();
  const reloadedRoot = reloadedPlayground.locator('[data-slot="textarea"]').first();
  await expect(reloadedRoot).toHaveAttribute("data-state", "ready");
  await expect(reloadedRoot).toHaveAttribute("data-value", "filled");
});
