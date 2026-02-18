import { expect, test } from "@playwright/test";

test("docs-app dropdown uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  await page.goto("/#/components/dropdown");
  await page.locator("body:not(:has(#boot))").waitFor();

  const defaultPlayground = page
    .locator('[data-slot="dropdown-default-playground"]')
    .first();
  const trigger = page.locator("#docs-dropdown-controlled-trigger").first();
  const root = trigger
    .locator('xpath=ancestor::*[@data-slot="dropdown"][1]')
    .first();
  const openMarker = page.locator('[data-slot="dropdown-controlled-open"]').first();

  await expect(defaultPlayground).toBeVisible();
  await expect(root).toBeVisible();
  await expect(trigger).toBeVisible();
  await expect(root).toHaveAttribute("data-controlled", "true");
  await expect(root).toHaveAttribute("data-closed", "true");
  await expect(openMarker).toHaveText("open: false");

  await trigger.click();
  await expect(openMarker).toHaveText("open: true");
  await expect(root).toHaveAttribute("data-open", "true");
});

test("docs-app dropdown key flow is repeatable with semantic contract breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/dropdown");
  await page.locator("body:not(:has(#boot))").waitFor();

  const trigger = page.locator("#docs-dropdown-default-trigger").first();
  const root = trigger
    .locator('xpath=ancestor::*[@data-slot="dropdown"][1]')
    .first();
  const lastAction = page.locator('[data-slot="dropdown-last-action"]').first();

  await expect(lastAction).toHaveText("last action: None");

  await trigger.focus();
  await expect(trigger).toBeFocused();
  await page.keyboard.press("ArrowDown");
  await expect(root).toHaveAttribute("data-open", "true");

  const option = page
    .locator('[data-slot="menu-item"]')
    .filter({ hasText: "Profile" })
    .first();
  await expect(option).toBeVisible();
  await option.click();

  await expect(lastAction).toHaveText("last action: 0");
  await expect(root).toHaveAttribute("data-closed", "true");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator('[data-slot="dropdown-last-action"]').first()).toHaveText(
    "last action: None"
  );
});

test("docs-app dropdown playground code panel exposes copy-ready snippet", async ({ page }) => {
  await page.goto("/#/components/dropdown");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator("section.playground")
    .filter({ has: page.locator("#docs-dropdown-default-trigger") })
    .first();
  await expect(playground).toBeVisible();

  const showCode = playground.getByRole("button", { name: "Show code" }).first();
  await showCode.click();

  const codeBlock = playground.locator('[data-slot="code-block"]').first();
  await expect(codeBlock).toBeVisible();
  await expect(codeBlock).toHaveAttribute("data-copyable", "true");
  await expect(playground.locator(".ui-code-block__copy-button").first()).toBeVisible();
});

test("docs-app dropdown interactive playground exposes display config code and css-test panels", async ({
  page,
}) => {
  await page.goto("/#/components/dropdown");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator("section.playground")
    .filter({
      has: page.getByRole("heading", { name: "Interactive Playground", exact: true }),
    })
    .first();
  await expect(playground).toBeVisible();
  await expect(playground.locator('[data-slot="dropdown-workbench-preview"]').first()).toBeVisible();

  await playground.getByRole("button", { name: "Show settings", exact: true }).click();
  await expect(playground.locator('[data-slot="playground-controls"]').first()).toBeVisible();
  await expect(playground.locator("#docs-dropdown-workbench-placement-trigger").first()).toBeVisible();

  await playground.getByRole("button", { name: "Show code", exact: true }).click();
  await expect(playground.locator('[data-slot="playground-code"]').first()).toBeVisible();
  await expect(playground.locator('[data-slot="code-block"]').first()).toBeVisible();

  await playground.getByRole("button", { name: "Show test", exact: true }).click();
  await expect(playground.locator('[data-slot="playground-test"]').first()).toBeVisible();
  await expect(playground.locator(".playground__test-editor").first()).toBeVisible();
  await expect(playground.getByText("Actual config", { exact: true })).toBeVisible();
});

test("docs-app dropdown state matrix compare keeps key variants visible", async ({ page }) => {
  await page.goto("/#/components/dropdown");
  await page.locator("body:not(:has(#boot))").waitFor();

  const matrix = page.locator('[data-slot="dropdown-state-matrix"]').first();
  await expect(matrix).toBeVisible();
  await expect(page.locator("#docs-dropdown-compare-default-trigger").first()).toBeVisible();
  await expect(page.locator("#docs-dropdown-compare-controlled-trigger").first()).toBeVisible();
  await expect(page.locator("#docs-dropdown-compare-disabled-trigger").first()).toBeVisible();
  await expect(page.locator("#docs-dropdown-compare-empty-trigger").first()).toBeVisible();

  const controlledRoot = page
    .locator("#docs-dropdown-compare-controlled-trigger")
    .first()
    .locator('xpath=ancestor::*[@data-slot="dropdown"][1]')
    .first();
  await expect(controlledRoot).toHaveAttribute("data-controlled", "true");

  const disabledRoot = page
    .locator("#docs-dropdown-compare-disabled-trigger")
    .first()
    .locator('xpath=ancestor::*[@data-slot="dropdown"][1]')
    .first();
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");

  const emptyRoot = page
    .locator("#docs-dropdown-compare-empty-trigger")
    .first()
    .locator('xpath=ancestor::*[@data-slot="dropdown"][1]')
    .first();
  await expect(emptyRoot).toHaveAttribute("data-empty", "true");
});
