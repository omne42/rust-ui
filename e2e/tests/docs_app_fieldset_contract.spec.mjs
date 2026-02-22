import { expect, test } from "@playwright/test";

test("docs-app fieldset contract uses semantic selectors with settled waits", async ({ page }) => {
  await page.goto("/#/components/fieldset");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("Fieldset");

  const fieldset = page.locator('[data-component="fieldset"] [data-slot="fieldset"]').first();
  await expect(fieldset).toBeVisible();
  await expect(fieldset).toHaveAttribute("data-ui-schema", "ui.fieldset.agent-contract");
  await expect(fieldset).toHaveAttribute("data-ui-schema-version", "1");
  await expect(fieldset).toHaveAttribute("data-ui-intent", "form-grouping");
  await expect(fieldset).toHaveAttribute("data-ui-action", "initialize");
  await expect(fieldset).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(fieldset).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(fieldset).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(fieldset).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app fieldset perf probe keeps budget observability markers", async ({ page }) => {
  await page.goto("/#/components/fieldset");
  await page.locator("body:not(:has(#boot))").waitFor();

  const perfProbe = page.locator('[data-slot="ui-perf-probe"]').first();
  await expect(perfProbe).toBeVisible();
  await expect(perfProbe).toHaveAttribute("data-perf-mount-ms", /[0-9]/);
  await expect(perfProbe).toHaveAttribute("data-perf-budget-ms", /[0-9]/);
  await expect(perfProbe).toHaveAttribute("data-perf-budget-update-ms", /[0-9]/);
  await expect(perfProbe).toHaveAttribute("data-perf-budget-heap-kb", /[0-9]/);
  await expect(perfProbe).toHaveAttribute("data-perf-observability", "mount-plus-budget");
  await expect(perfProbe).not.toHaveAttribute("data-perf-violation", "true");
});

test("docs-app fieldset key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/fieldset");
  await page.locator("body:not(:has(#boot))").waitFor();

  const requiredFieldset = page
    .locator('[data-component="fieldset"] [data-slot="fieldset"][data-required="true"]')
    .first();
  await expect(requiredFieldset).toBeVisible();
  await expect(requiredFieldset).toHaveAttribute("data-required-source", "required");
  await expect(requiredFieldset).toHaveAttribute("data-invalid-source", "default");

  const invalidFieldset = page
    .locator('[data-component="fieldset"] [data-slot="fieldset"][data-invalid="true"]')
    .first();
  await expect(invalidFieldset).toBeVisible();
  await expect(invalidFieldset).toHaveAttribute("data-invalid-source", "is_invalid");
  await expect(invalidFieldset).toHaveAttribute("data-error-source", "custom");
  const actionButton = invalidFieldset.locator('[data-slot="button"]').first();
  await expect(actionButton).toBeVisible();
  await actionButton.focus();
  await expect(actionButton).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(invalidFieldset).toHaveAttribute("data-invalid-source", "is_invalid");
  await actionButton.click();
  await expect(invalidFieldset).toHaveAttribute("data-error-source", "custom");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  const reloaded = page
    .locator('[data-component="fieldset"] [data-slot="fieldset"][data-invalid="true"]')
    .first();
  await expect(reloaded).toHaveAttribute("data-invalid-source", "is_invalid");
});

test("docs-app fieldset docs product surface covers hello/state/controlled/streaming playgrounds", async ({ page }) => {
  await page.goto("/#/components/fieldset");
  await page.locator("body:not(:has(#boot))").waitFor();

  await expect(
    page.locator('[data-component="fieldset"] [data-slot="playground"] h2:has-text("Hello World")')
  ).toBeVisible();
  await expect(
    page.locator(
      '[data-component="fieldset"] [data-slot="playground"] h2:has-text("Controlled vs Uncontrolled (Snapshot Contrast)")'
    )
  ).toBeVisible();
  await expect(
    page.locator(
      '[data-component="fieldset"] [data-slot="playground"] h2:has-text("Streaming Optional (fallback=snapshot)")'
    )
  ).toBeVisible();
  await expect(page.locator('[data-slot="fieldset-workbench-compare"]')).toBeVisible();
});

test("docs-app fieldset playground source is copy-paste ready", async ({ page }) => {
  await page.goto("/#/components/fieldset");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page.locator('[data-component="fieldset"] section.playground').first();
  await expect(playground).toBeVisible();

  const codeToggle = playground.getByRole("button", { name: /Show code|Hide code/ }).first();
  await expect(codeToggle).toBeVisible();

  const codeBlock = playground.locator('[data-slot="code-block"]');
  if ((await codeBlock.count()) === 0) {
    await codeToggle.click();
  }

  await expect(codeBlock.first()).toBeVisible();
  await expect(codeBlock.first()).toHaveAttribute("data-copyable", "true");
  await expect(playground.locator('[data-slot="code-block-code"]').first()).toContainText(
    "use leptos::prelude::*;"
  );
  await expect(playground.locator('[data-slot="code-block-code"]').first()).toContainText(
    "use ui::*;"
  );
  await expect(playground.locator('[data-slot="code-block-code"]').first()).toContainText(
    "<Fieldset"
  );
});

test("docs-app fieldset source-first section is copy-paste ready and traceable", async ({ page }) => {
  await page.goto("/#/components/fieldset");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="fieldset"]').first();
  await expect(docsRoot).toBeVisible();

  const sourceFirst = docsRoot.locator('[data-slot="fieldset-source-first"]').first();
  await expect(sourceFirst).toBeVisible();
  await expect(sourceFirst).toContainText("Show code");
  await expect(sourceFirst).toContainText("compose_copy_ready_code");
  await expect(sourceFirst).toContainText("components/fieldset/src/mod.rs");
  await expect(sourceFirst).toContainText("components/fieldset/src/logic.rs");
  await expect(sourceFirst).toContainText("components/fieldset/src/view.rs");
  await expect(sourceFirst).toContainText("components/fieldset/src/styles.rs");
  await expect(sourceFirst).toContainText("components/fieldset/src/motion.rs");
  await expect(sourceFirst).toContainText("component-fieldset");
  await expect(sourceFirst).toContainText("inject-css");

  const sourceSnippet = sourceFirst.locator('[data-slot="code-block"]').first();
  await expect(sourceSnippet).toHaveAttribute("data-copyable", "true");
});
