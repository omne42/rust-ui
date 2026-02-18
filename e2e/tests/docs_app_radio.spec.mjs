import { expect, test } from "@playwright/test";

test("docs-app radio-group exposes headless semantic contract and keyboard flow", async ({ page }) => {
  await page.goto("/#/components/radio-group");
  await page.locator("body:not(:has(#boot))").waitFor();

  const group = page
    .locator('[data-slot="radio-group"][role="radiogroup"]')
    .filter({ has: page.locator('#docs-radio-group-radio-0') })
    .first();

  const radio1 = page.locator("#docs-radio-group-radio-1");
  const radio2 = page.locator("#docs-radio-group-radio-2");

  await expect(group).toBeVisible();
  await expect(group).toHaveAttribute("role", "radiogroup");
  await expect(group).toHaveAttribute("aria-orientation", "vertical");
  await expect(group).toHaveAttribute("data-has-selection", "true");
  await expect(group).toHaveAttribute("data-selected-index", "1");

  await radio1.focus();
  await expect(radio1).toBeFocused();
  await page.keyboard.press("ArrowDown");

  await expect(group).toHaveAttribute("data-selected-index", "2");
  await expect(radio2).toHaveAttribute("aria-checked", "true");
  await expect(radio2).toHaveAttribute("data-active", "true");
});

test("docs-app radio-group disabled option remains non-interactive and state is repeatable", async ({
  page,
}) => {
  await page.goto("/#/components/radio-group");
  await page.locator("body:not(:has(#boot))").waitFor();

  const billingGroup = page
    .locator('[data-slot="radio-group"][role="radiogroup"]')
    .filter({ has: page.locator('#docs-radio-group-billing-radio-0') })
    .first();

  const disabledOption = page.locator("#docs-radio-group-billing-radio-1");
  const option0 = page.locator("#docs-radio-group-billing-radio-0");

  await expect(billingGroup).toBeVisible();
  await expect(billingGroup).toHaveAttribute("aria-orientation", "horizontal");
  await expect(billingGroup).toHaveAttribute("data-disabled-option-count", "1");
  await expect(disabledOption).toHaveAttribute("disabled", "");
  await expect(disabledOption).toHaveAttribute("aria-disabled", "true");

  await option0.click();
  await expect(billingGroup).toHaveAttribute("data-selected-index", "0");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedBillingGroup = page
    .locator('[data-slot="radio-group"][role="radiogroup"]')
    .filter({ has: page.locator('#docs-radio-group-billing-radio-0') })
    .first();

  await expect(reloadedBillingGroup).toHaveAttribute("data-selected-index", "2");
});
