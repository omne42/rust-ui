import { expect, test } from "@playwright/test";

async function gotoFieldErrorDocsAndWaitSettled(page) {
  await page.goto("/#/components/field-error");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="field-error"]').first();
  await expect(docsRoot).toBeVisible();

  const settledRoot = docsRoot
    .locator(
      '[data-slot="field-error"][data-state="visible"][data-message-source="custom"]',
    )
    .first();
  await expect(settledRoot).toBeVisible();
  await expect(settledRoot).toHaveAttribute("role", "alert");
  await expect(settledRoot).toHaveAttribute("aria-live", "assertive");

  return docsRoot;
}

test("docs-app field-error uses semantic selectors with wasm-stable wait strategy", async ({
  page,
}) => {
  const docsRoot = await gotoFieldErrorDocsAndWaitSettled(page);

  const visible = docsRoot
    .locator(
      '[data-slot="field-error"][data-state="visible"][data-tone="negative"][data-message-source="custom"][data-aria-source="custom"]',
    )
    .first();
  await expect(visible).toBeVisible();
  await expect(visible).toHaveAttribute("data-disabled", "false");

  const disabled = docsRoot
    .locator('[data-slot="field-error"][data-state="disabled"][data-disabled="true"]')
    .first();
  await expect(disabled).toBeVisible();
  await expect(disabled).toHaveAttribute("data-class-source", "custom");

  const hidden = docsRoot
    .locator('[data-slot="field-error"][data-state="hidden"][aria-hidden="true"]')
    .first();
  await expect(hidden).toBeVisible();
});

test("docs-app field-error key flow remains repeatable via semantic breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoFieldErrorDocsAndWaitSettled(page);

  const baseSelector =
    '[data-slot="field-error"][data-state="visible"][data-tone="negative"][data-message-source="custom"]';
  await expect(docsRoot.locator(baseSelector).first()).toBeVisible();

  await page.goto("/#/components/error-message");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("ErrorMessage");

  await page.goto("/#/components/field-error");
  await page.locator("body:not(:has(#boot))").waitFor();
  const reloadedRoot = page
    .locator(`[data-component="field-error"] ${baseSelector}`)
    .first();
  await expect(reloadedRoot).toBeVisible();
  await expect(reloadedRoot).toHaveAttribute("role", "alert");
  await expect(reloadedRoot).toHaveAttribute("aria-live", "assertive");
});

test("docs-app field-error interactive playground supports realtime props/state preview", async ({
  page,
}) => {
  const docsRoot = await gotoFieldErrorDocsAndWaitSettled(page);

  const playground = docsRoot
    .locator("section.playground")
    .filter({ has: docsRoot.locator('[data-slot="field-error-config-controls"]') })
    .first();
  await expect(playground).toBeVisible();

  const toggleSettings = playground
    .getByRole("button", { name: /Show settings|Hide settings/ })
    .first();
  const controls = playground.locator('[data-slot="field-error-config-controls"]').first();
  if (!(await controls.isVisible())) {
    await toggleSettings.click();
  }
  await expect(controls).toBeVisible();

  const root = playground.locator('[data-slot="field-error"]').first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-state", "visible");
  await expect(root).toHaveAttribute("data-message-source", "default");

  await controls.locator('[data-action="cycle-tone-config"]').selectOption("1");
  await expect(root).toHaveAttribute("data-tone", "neutral");

  await controls.locator('[data-action="toggle-message-config"]').check();
  await expect(root).toHaveAttribute("data-message-source", "custom");

  await controls.locator('[data-action="toggle-aria-config"]').check();
  await expect(root).toHaveAttribute("data-aria-source", "custom");

  await controls.locator('[data-action="toggle-class-config"]').check();
  await expect(root).toHaveAttribute("data-class-source", "custom");

  await controls.locator('[data-action="toggle-icon-config"]').check();
  await expect(root).toHaveAttribute("data-show-icon", "true");

  await controls.locator('[data-action="toggle-disabled-config"]').check();
  await expect(root).toHaveAttribute("data-state", "disabled");

  await controls.locator('[data-action="toggle-visible-config"]').uncheck();
  await expect(root).toHaveAttribute("data-state", "hidden");
  await expect(root).toHaveAttribute("aria-hidden", "true");

  const summary = controls.locator('[data-slot="field-error-config-summary"]').first();
  await expect(summary).toContainText("visible=false");
  await expect(summary).toContainText("disabled=true");
  await expect(summary).toContainText("message_source=custom");
});

test("docs-app field-error source-first docs are copy-paste ready", async ({ page }) => {
  const docsRoot = await gotoFieldErrorDocsAndWaitSettled(page);

  const playground = docsRoot.locator("section.playground").first();
  await expect(playground).toBeVisible();

  const toggleCode = playground
    .getByRole("button", { name: /Show code|Hide code/ })
    .first();
  await expect(toggleCode).toBeVisible();

  const codeBlock = playground.locator('[data-slot="code-block"]');
  if ((await codeBlock.count()) === 0) {
    await toggleCode.click();
  }

  await expect(codeBlock.first()).toBeVisible();
  await expect(codeBlock.first()).toHaveAttribute("data-copyable", "true");
  const code = codeBlock.first().locator('[data-slot="code-block-code"]').first();
  await expect(code).toContainText("use leptos::prelude::*;");
  await expect(code).toContainText("use ui::{FieldError, FieldErrorTone};");
  await expect(code).toContainText("<FieldError");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);

  const sourceFirst = docsRoot.locator('[data-slot="field-error-source-first"]').first();
  await expect(sourceFirst).toBeVisible();
  await expect(sourceFirst).toContainText("components/field-error/src/mod.rs");
  await expect(sourceFirst).toContainText("components/field-error/src/logic.rs");
  await expect(sourceFirst).toContainText("components/field-error/src/view.rs");
  await expect(sourceFirst).toContainText("components/field-error/src/styles.rs");
  await expect(sourceFirst).toContainText("component-field_error");
  await expect(sourceFirst).toContainText("inject-css");
});
