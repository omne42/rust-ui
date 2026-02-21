import { expect, test } from "@playwright/test";

async function gotoFieldLabelDocsAndWaitSettled(page) {
  await page.goto("/#/components/field-label");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="field-label"]').first();
  await expect(docsRoot).toBeVisible();

  const settledLabel = docsRoot
    .locator(
      '#docs-field-label-hello[data-slot="field-label"][data-ui-schema="field_label.v1"][data-ui-streaming="optional"][data-ui-fallback="snapshot"][data-ui-output-state="verified"]'
    )
    .first();
  await expect(settledLabel).toBeVisible();
  await expect(settledLabel).toHaveAttribute("data-state", "required");

  return docsRoot;
}

test("docs-app field-label uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const docsRoot = await gotoFieldLabelDocsAndWaitSettled(page);

  const helperLabel = docsRoot
    .locator('#docs-field-label-email[data-slot="field-label"][data-tone="default"]')
    .first();
  await expect(helperLabel).toBeVisible();
  await expect(helperLabel).toHaveAttribute("data-required", "true");
  await expect(helperLabel).toHaveAttribute("data-has-for", "true");
  await expect(helperLabel).toHaveAttribute("data-text-source", "custom");
  await expect(helperLabel).toHaveAttribute("data-indicator-source", "default");
  await expect(helperLabel).toHaveAttribute("data-aria-source", "default");
});

test("docs-app field-label key flow is repeatable via semantic focus breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoFieldLabelDocsAndWaitSettled(page);

  const focusLinkSelector =
    '[data-slot="field-label"][for="docs-field-label-compare-default"][data-has-for="true"][data-ui-output-state="verified"]';

  const label = docsRoot.locator(focusLinkSelector).first();
  const input = page.locator("#docs-field-label-compare-default").first();

  await expect(label).toHaveAttribute("data-state", "required");
  await expect(label).toHaveAttribute("data-required", "true");
  await label.click();
  await expect(input).toBeFocused();
  await page.keyboard.type("owner-one");
  await expect(input).toHaveValue("owner-one");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedLabel = page
    .locator(`[data-component="field-label"] ${focusLinkSelector}`)
    .first();
  const reloadedInput = page.locator("#docs-field-label-compare-default").first();

  await expect(reloadedLabel).toHaveAttribute("data-has-for", "true");
  await reloadedLabel.click();
  await expect(reloadedInput).toBeFocused();
  await page.keyboard.type("owner-two");
  await expect(reloadedInput).toHaveValue("owner-two");
});

test("docs-app field-label workbench playground supports interactive props/state preview", async ({
  page,
}) => {
  const docsRoot = await gotoFieldLabelDocsAndWaitSettled(page);

  const controls = docsRoot.locator('[data-slot="field-label-config-controls"]').first();
  const summary = controls.locator('[data-slot="field-label-config-summary"]').first();
  const workbenchLabel = docsRoot
    .locator('[data-slot="field-label"][for="docs-field-label-workbench"]')
    .first();
  const workbenchInput = page.locator("#docs-field-label-workbench").first();

  await expect(controls).toBeVisible();
  await expect(workbenchLabel).toBeVisible();
  await expect(workbenchInput).toBeVisible();

  await expect(workbenchLabel).toHaveAttribute("data-tone", "default");
  await expect(workbenchLabel).toHaveAttribute("data-required", "true");
  await expect(workbenchLabel).toHaveAttribute("data-disabled", "false");
  await expect(summary).toContainText("required=true");
  await expect(summary).toContainText("disabled=false");

  await controls.locator('[data-action="cycle-tone-config"]').click();
  await expect(workbenchLabel).toHaveAttribute("data-tone", "muted");

  await controls.locator('[data-action="toggle-required-config"]').click();
  await expect(workbenchLabel).toHaveAttribute("data-required", "false");

  await controls.locator('[data-action="toggle-disabled-config"]').click();
  await expect(workbenchLabel).toHaveAttribute("data-disabled", "true");
  await expect(workbenchInput).toBeDisabled();

  await controls.locator('[data-action="toggle-for-config"]').click();
  await expect(workbenchLabel).toHaveAttribute("data-has-for", "false");

  await controls.locator('[data-action="toggle-indicator-config"]').click();
  await expect(workbenchLabel).toHaveAttribute("data-indicator-source", "custom");

  await controls.locator('[data-action="toggle-aria-config"]').click();
  await expect(workbenchLabel).toHaveAttribute("data-aria-source", "custom");

  await controls.locator('[data-action="toggle-class-config"]').click();
  await expect(workbenchLabel).toHaveAttribute("data-class-source", "custom");
  await expect(summary).toContainText("required=false");
  await expect(summary).toContainText("disabled=true");
  await expect(summary).toContainText("indicator=custom");
  await expect(summary).toContainText("aria=custom");
  await expect(summary).toContainText("class=custom");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedControls = page
    .locator('[data-component="field-label"] [data-slot="field-label-config-controls"]')
    .first();
  const reloadedWorkbenchLabel = page
    .locator('[data-component="field-label"] [data-slot="field-label"][for="docs-field-label-workbench"]')
    .first();
  await expect(reloadedControls).toBeVisible();
  await reloadedControls.locator('[data-action="cycle-tone-config"]').click();
  await expect(reloadedWorkbenchLabel).toHaveAttribute("data-tone", "muted");
});

test("docs-app field-label playground source is copy-paste ready", async ({ page }) => {
  const docsRoot = await gotoFieldLabelDocsAndWaitSettled(page);

  const playground = docsRoot
    .locator("section.playground")
    .filter({ has: docsRoot.locator('[data-slot="field-label-config-controls"]') })
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
  await expect(code).toContainText("use ui_components::{FieldLabel, FieldLabelTone};");
  await expect(code).toContainText("<FieldLabel");

  const copyButton = codeBlock.first().locator('[data-slot="button"]').first();
  await expect(copyButton).toHaveAttribute("aria-label", /Copy to clipboard/i);
});
