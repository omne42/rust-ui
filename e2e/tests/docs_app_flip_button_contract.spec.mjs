import { expect, test } from "@playwright/test";

test("docs-app flip-button keeps stable semantic selectors and settled contract states", async ({
  page,
}) => {
  await page.goto("/#/components/flip-button");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-slot="flip-button"]').first();
  await expect(root).toBeVisible();

  const playgrounds = root.locator("section.playground");
  await expect(playgrounds).toHaveCount(4);

  const topFlip = playgrounds.nth(0).locator('[data-slot="flip-button"]').first();
  const topButton = topFlip.locator('[data-slot="button"]').first();
  await expect(topFlip).toBeVisible();
  await expect(topButton).toBeVisible();

  await expect(topFlip).toHaveAttribute("data-from", "top");
  await expect(topFlip).toHaveAttribute("data-state", /(active|inactive)/);
  await expect(topFlip).toHaveAttribute("data-hover", /(hovered|not-hovered)/);
  await expect(topFlip).toHaveAttribute(
    "data-focus-within-state",
    /(focus-within|no-focus-within)/,
  );

  await topFlip.hover();
  await expect(topFlip).toHaveAttribute("data-hover", "hovered");

  await topButton.focus();
  await expect(topFlip).toHaveAttribute("data-focus-within-state", "focus-within");
});

test("docs-app flip-button key interaction flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/flip-button");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-slot="flip-button"]').first();
  await expect(root).toBeVisible();

  const topFlip = root
    .locator("section.playground")
    .nth(0)
    .locator('[data-slot="flip-button"]')
    .first();
  const topButton = topFlip.locator('[data-slot="button"]').first();

  await expect(topFlip).toHaveAttribute("data-state", "inactive");
  await expect(topFlip).toHaveAttribute("data-hover", "not-hovered");
  await expect(topFlip).toHaveAttribute("data-focus-within-state", "no-focus-within");

  await topFlip.hover();
  await expect(topFlip).toHaveAttribute("data-hover", "hovered");
  await expect(topFlip).toHaveAttribute("data-state", "active");

  await topButton.focus();
  await expect(topFlip).toHaveAttribute("data-focus-within-state", "focus-within");

  await topButton.evaluate((el) => el.blur());
  await page.locator("body").hover();
  await expect(topFlip).toHaveAttribute("data-focus-within-state", "no-focus-within");
  await expect(topFlip).toHaveAttribute("data-hover", "not-hovered");
  await expect(topFlip).toHaveAttribute("data-state", "inactive");
});
