import { expect, test } from "@playwright/test";

const CHECKBOX_FIELD_PAGE = "/#/components/checkbox-field";
const WASM_READY = "body:not(:has(#boot))";

test("docs-app checkbox-field contract uses semantic selectors with settled waits", async ({
  page,
}) => {
  await page.goto(CHECKBOX_FIELD_PAGE);
  await page.locator(WASM_READY).waitFor();

  const newsletter = page
    .locator('#docs-checkbox-field-newsletter[data-slot="checkbox-field"]')
    .first();
  const newsletterCheckbox = newsletter.locator('[data-slot="checkbox"][role="checkbox"]').first();

  await expect(newsletter).toBeVisible();
  await expect(newsletterCheckbox).toBeVisible();
  await expect(newsletter).toHaveAttribute("data-state", "checked");
  await expect(newsletter).toHaveAttribute("data-checked", "true");
  await expect(newsletter).toHaveAttribute("data-checked-mode", "controlled");
  await expect(newsletter).toHaveAttribute("data-checked-prop-source", "is_checked");
  await expect(newsletter).toHaveAttribute("data-checked-change-source", "on_checked_change");

  const terms = page
    .locator('#docs-checkbox-field-terms[data-slot="checkbox-field"]')
    .first();
  await expect(terms).toBeVisible();
  await expect(terms).toHaveAttribute("data-state", "invalid");
  await expect(terms).toHaveAttribute("data-invalid", "true");
  await expect(terms).toHaveAttribute("data-tone", "quiet");
  await expect(terms).toHaveAttribute("data-indicator-placement", "end");

  const readOnly = page
    .locator('#docs-checkbox-field-read-only[data-slot="checkbox-field"]')
    .first();
  await expect(readOnly).toBeVisible();
  await expect(readOnly).toHaveAttribute("data-state", "disabled");
  await expect(readOnly).toHaveAttribute("data-disabled", "true");
});

test("docs-app checkbox-field covers ready/settled semantic breakpoints for controlled and uncontrolled paths", async ({
  page,
}) => {
  await page.goto(CHECKBOX_FIELD_PAGE);
  await page.locator(WASM_READY).waitFor();

  const controlled = page
    .locator('#docs-checkbox-field-controlled[data-slot="checkbox-field"]')
    .first();
  const controlledCheckbox = controlled.locator('[data-slot="checkbox"][role="checkbox"]').first();

  await expect(controlled).toBeVisible();
  await expect(controlled).toHaveAttribute("data-checked-mode", "controlled");
  await expect(controlled).toHaveAttribute("data-state", "checked");
  await controlledCheckbox.click();
  await expect(controlled).toHaveAttribute("data-state", "unchecked");
  await controlledCheckbox.click();
  await expect(controlled).toHaveAttribute("data-state", "checked");

  const uncontrolled = page
    .locator('#docs-checkbox-field-uncontrolled[data-slot="checkbox-field"]')
    .first();
  const uncontrolledCheckbox = uncontrolled
    .locator('[data-slot="checkbox"][role="checkbox"]')
    .first();

  await expect(uncontrolled).toBeVisible();
  await expect(uncontrolled).toHaveAttribute("data-checked-mode", "uncontrolled");
  await expect(uncontrolled).toHaveAttribute("data-state", "checked");
  await uncontrolledCheckbox.click();
  await expect(uncontrolled).toHaveAttribute("data-state", "unchecked");

  await page.reload();
  await page.locator(WASM_READY).waitFor();

  const reloadedUncontrolled = page
    .locator('#docs-checkbox-field-uncontrolled[data-slot="checkbox-field"]')
    .first();
  await expect(reloadedUncontrolled).toHaveAttribute("data-checked-mode", "uncontrolled");
  await expect(reloadedUncontrolled).toHaveAttribute("data-state", "checked");
});

test("docs-app checkbox-field key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto(CHECKBOX_FIELD_PAGE);
  await page.locator(WASM_READY).waitFor();

  const controlled = page
    .locator('#docs-checkbox-field-controlled[data-slot="checkbox-field"]')
    .first();
  const controlledCheckbox = controlled.locator('[data-slot="checkbox"][role="checkbox"]').first();

  await expect(controlled).toHaveAttribute("data-state", "checked");
  await controlledCheckbox.focus();
  await expect(controlledCheckbox).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(controlled).toHaveAttribute("data-state", "unchecked");
  await page.keyboard.press("Enter");
  await expect(controlled).toHaveAttribute("data-state", "checked");

  const uncontrolled = page
    .locator('#docs-checkbox-field-uncontrolled[data-slot="checkbox-field"]')
    .first();
  const uncontrolledCheckbox = uncontrolled
    .locator('[data-slot="checkbox"][role="checkbox"]')
    .first();

  await expect(uncontrolled).toHaveAttribute("data-state", "checked");
  await uncontrolledCheckbox.focus();
  await expect(uncontrolledCheckbox).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(uncontrolled).toHaveAttribute("data-state", "unchecked");

  await page.reload();
  await page.locator(WASM_READY).waitFor();

  const reloadedUncontrolled = page
    .locator('#docs-checkbox-field-uncontrolled[data-slot="checkbox-field"]')
    .first();
  await expect(reloadedUncontrolled).toHaveAttribute("data-state", "checked");
});

test("docs-app checkbox-field playground source is copy-paste ready", async ({ page }) => {
  await page.goto(CHECKBOX_FIELD_PAGE);
  await page.locator(WASM_READY).waitFor();

  const playground = page
    .locator('[data-component="checkbox-field"] section.playground')
    .filter({ has: page.locator("#docs-checkbox-field-newsletter") })
    .first();
  await expect(playground).toBeVisible();

  const codeToggle = playground
    .getByRole("button", { name: /Show code|Hide code/ })
    .first();
  await expect(codeToggle).toBeVisible();

  const codeBlock = playground.locator('[data-slot="code-block"]');
  if ((await codeBlock.count()) === 0) {
    await codeToggle.click();
  }

  await expect(codeBlock.first()).toBeVisible();
  await expect(codeBlock.first()).toHaveAttribute("data-copyable", "true");

  const code = playground.locator('[data-slot="code-block-code"]').first();
  await expect(code).toContainText("use leptos::prelude::*;");
  await expect(code).toContainText("use ui_components::*;");
  await expect(code).toContainText("<CheckboxField");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);

  const copyReady = page.locator('[data-slot="checkbox-field-copy-ready"]').first();
  await expect(copyReady).toContainText("use leptos::prelude::*;");
  await expect(copyReady).toContainText("use ui_components::*;");

  const sourcePaths = page.locator('[data-slot="checkbox-field-source-paths"]').first();
  await expect(sourcePaths).toContainText("components/checkbox-field/src/mod.rs");
  await expect(sourcePaths).toContainText("components/checkbox-field/src/logic.rs");
  await expect(sourcePaths).toContainText("components/checkbox-field/src/view.rs");
  await expect(sourcePaths).toContainText("components/checkbox-field/src/styles.rs");

  const sourcePrereq = page
    .locator('[data-slot="checkbox-field-source-prerequisites"]')
    .first();
  await expect(sourcePrereq).toContainText("component-checkbox_field");
  await expect(sourcePrereq).toContainText("inject-css");
});
