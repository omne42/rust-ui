import { expect, test } from "@playwright/test";

test("docs-app select contract uses semantic selectors with settled waits", async ({
  page,
}) => {
  await page.goto("/#/components/select");
  await page.locator("body:not(:has(#boot))").waitFor();

  const trigger = page.locator("#docs-select-controlled-trigger").first();
  const root = trigger
    .locator('xpath=ancestor::*[@data-slot="select" and @data-has-items][1]')
    .first();

  await expect(root).toBeVisible();
  await expect(trigger).toBeVisible();
  await expect(root).toHaveAttribute("data-selected-index", "1");
  await expect(root).toHaveAttribute("data-has-disabled-options", "true");
  await expect(root).toHaveAttribute("data-closed", "true");

  await trigger.focus();
  await expect(trigger).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(root).toHaveAttribute("data-selected-index", "2");
  await expect(root).toHaveAttribute("data-closed", "true");
});

test("docs-app select key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/select");
  await page.locator("body:not(:has(#boot))").waitFor();

  const trigger = page.locator("#docs-select-controlled-trigger").first();
  const root = trigger
    .locator('xpath=ancestor::*[@data-slot="select" and @data-has-items][1]')
    .first();

  await expect(root).toHaveAttribute("data-selected-index", "1");
  await trigger.focus();
  await expect(trigger).toBeFocused();

  await page.keyboard.press("ArrowRight");
  await expect(root).toHaveAttribute("data-selected-index", "2");
  await page.keyboard.press("ArrowLeft");
  await expect(root).toHaveAttribute("data-selected-index", "1");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  const reloadedTrigger = page.locator("#docs-select-controlled-trigger").first();
  const reloadedRoot = reloadedTrigger
    .locator('xpath=ancestor::*[@data-slot="select" and @data-has-items][1]')
    .first();
  await expect(reloadedRoot).toHaveAttribute("data-selected-index", "1");
  await expect(reloadedRoot).toHaveAttribute("data-closed", "true");
});

test("docs-app select controlled playground code panel exposes copy-ready snippet", async ({
  page,
}) => {
  await page.goto("/#/components/select");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator("section.playground")
    .filter({ has: page.locator("#docs-select-controlled-trigger") })
    .first();
  await expect(playground).toBeVisible();

  const showCode = playground.getByRole("button", { name: "Show code" }).first();
  await expect(showCode).toBeVisible();
  await showCode.click();

  const codeBlock = playground.locator('[data-slot="code-block"]').first();
  await expect(codeBlock).toBeVisible();
  await expect(codeBlock).toHaveAttribute("data-copyable", "true");
  await expect(playground.locator(".ui-code-block__copy-button").first()).toBeVisible();
});
