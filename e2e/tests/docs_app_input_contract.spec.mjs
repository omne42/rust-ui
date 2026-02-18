import { expect, test } from "@playwright/test";

test("docs-app input clears on Escape through semantic headless contract", async ({ page }) => {
  await page.goto("/#/components/input");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-slot="input"]').first();
  const input = root.locator('[data-slot="input-input"]').first();
  const clearButton = root.locator('[data-slot="input-clear"]').first();

  await expect(root).toBeVisible();
  await expect(input).toBeVisible();

  await input.fill("Alpha");
  await expect(input).toHaveValue("Alpha");
  await expect(input).toHaveAttribute("aria-keyshortcuts", "Escape");
  await expect(clearButton).toHaveAttribute("data-visible", "true");

  await input.press("Escape");
  await expect(input).toHaveValue("");
  await expect(input).not.toHaveAttribute("aria-keyshortcuts", "Escape");
  await expect(clearButton).not.toHaveAttribute("data-visible", "true");
});

test("docs-app input-group keeps stable group semantics for attached and detached states", async ({
  page,
}) => {
  await page.goto("/#/components/input-group");
  await page.locator("body:not(:has(#boot))").waitFor();

  const groups = page.locator('[data-slot="input-group"][role="group"]');
  await expect(groups).toHaveCount(3);

  const attached = groups.filter({ has: page.getByLabel("Email user") }).first();
  await expect(attached).toHaveAttribute("role", "group");
  await expect(attached).toHaveAttribute("aria-label", "Email input group");
  await expect(attached).toHaveAttribute("data-attachment", "attached");

  const attachedInput = attached.locator('[data-slot="input-input"]').first();
  await attachedInput.fill("alice");
  await expect(attachedInput).toHaveValue("alice");

  const detached = groups.filter({ has: page.getByLabel("Search query") }).first();
  await expect(detached).toHaveAttribute("aria-label", "Search controls");
  await expect(detached).toHaveAttribute("data-attachment", "detached");
  await expect(detached).toHaveAttribute("data-detached", "true");

  const disabled = groups.filter({ has: page.getByLabel("Disabled field") }).first();
  await expect(disabled).toHaveAttribute("aria-label", "Disabled controls");
  await expect(disabled).toHaveAttribute("data-disabled", "true");
});

test("docs-app input-otp normalizes digits and preserves slot contracts", async ({ page }) => {
  await page.goto("/#/components/input-otp");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-slot="input-otp"]').first();
  const input = root.locator('[data-slot="input-otp-input"]').first();
  const slots = root.locator('[data-slot="input-otp-slot"]');

  await expect(root).toBeVisible();
  await expect(input).toBeVisible();
  await expect(slots).toHaveCount(6);

  await input.fill("12a3");
  await expect(input).toHaveValue("123");
  await expect(slots.first()).toHaveAttribute("data-filled", "true");

  await input.fill("1234567");
  await expect(input).toHaveValue("123456");

  await input.press("Backspace");
  await expect(input).toHaveValue("12345");
});

test("docs-app input-otp comparison playground keeps disabled/invalid/default contracts", async ({
  page,
}) => {
  await page.goto("/#/components/input-otp");
  await page.locator("body:not(:has(#boot))").waitFor();

  const comparison = page.locator('[data-slot="input-otp-state-compare"]').first();
  const defaultInput = comparison.getByLabel("Default OTP");
  await defaultInput.fill("9a8");
  await expect(defaultInput).toHaveValue("98");

  const disabledInput = comparison.getByLabel("Disabled OTP");
  await expect(disabledInput).toBeDisabled();
  await expect(disabledInput).toHaveValue("2468");

  const invalidInput = comparison.getByLabel("Invalid OTP");
  await expect(invalidInput).toHaveAttribute("aria-invalid", "true");
  await expect(comparison.locator('[data-slot="input-otp-error"]')).toBeVisible();
});
