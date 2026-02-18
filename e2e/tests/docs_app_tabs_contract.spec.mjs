import { expect, test } from "@playwright/test";

test("docs-app tabs workbench uses semantic selectors and settled marker waits", async ({ page }) => {
  await page.goto("/#/components/tabs");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator("section.playground")
    .filter({ has: page.locator('[data-slot="tabs-workbench"]') })
    .first();
  await expect(playground).toBeVisible();

  await playground.getByRole("button", { name: /Show settings|Hide settings/ }).click();

  const controls = playground.locator('[data-slot="playground-controls"]');
  const workbench = playground.locator('[data-slot="tabs-workbench"]').first();
  const tabsRoot = playground
    .locator('[data-slot="tabs-workbench-canvas"] [data-slot="tabs"]')
    .first();
  const tabsList = tabsRoot.locator('[data-slot="tabs-list"]').first();
  const indicator = tabsList.locator('[data-slot="tabs-indicator"]').first();
  const detailsTab = tabsRoot.locator('[data-slot="tabs-tab"][data-index="1"]').first();

  await expect(controls).toBeVisible();
  await expect(workbench).toBeVisible();
  await expect(tabsRoot).toHaveAttribute("data-control-mode", "controlled");
  await expect(tabsRoot).toHaveAttribute("data-keyboard-activation", "automatic");
  await expect(tabsRoot).toHaveAttribute("data-selected-index", "0");

  await detailsTab.click();
  await expect(tabsRoot).toHaveAttribute("data-selected-index", "1");
  await expect(detailsTab).toHaveAttribute("data-selected", "true");
  await expect(indicator).toHaveCSS("opacity", "1");

  const persistCheckbox = controls
    .locator('label:has-text("Persist selected index (optional)") input[type="checkbox"]')
    .first();
  await expect(persistCheckbox).toBeVisible();
  await persistCheckbox.check();
  await expect(playground.locator("span.ui-muted", { hasText: "persist selected: on" })).toBeVisible();
});

test("docs-app tabs keyboard focus path is repeatable and semantic", async ({ page }) => {
  await page.goto("/#/components/tabs");
  await page.locator("body:not(:has(#boot))").waitFor();

  const controlledPlayground = page
    .locator("section.playground")
    .filter({ has: page.locator('#docs-tabs-tab-0') })
    .first();
  await expect(controlledPlayground).toBeVisible();

  const tabsRoot = controlledPlayground.locator('[data-slot="tabs"]').first();
  const tab0 = controlledPlayground.locator("#docs-tabs-tab-0");
  const tab1 = controlledPlayground.locator("#docs-tabs-tab-1");
  const tab2 = controlledPlayground.locator("#docs-tabs-tab-2");
  const panel1 = controlledPlayground.locator("#docs-tabs-panel-1");

  await tab0.focus();
  await expect(tab0).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(tab1).toBeFocused();
  await expect(tabsRoot).toHaveAttribute("data-selected-index", "1");
  await expect(tab1).toHaveAttribute("aria-selected", "true");
  await expect(panel1).toBeVisible();

  await page.keyboard.press("ArrowRight");
  await expect(tab2).toBeFocused();
  await expect(tabsRoot).toHaveAttribute("data-selected-index", "2");
});
