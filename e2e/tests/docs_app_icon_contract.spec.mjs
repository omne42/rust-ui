import { expect, test } from "@playwright/test";

async function gotoIconDocsAndWaitSettled(page) {
  await page.goto("/#/components/icon");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="icon"]').first();
  await expect(docsRoot).toBeVisible();

  const settledIcon = docsRoot
    .locator(
      '[data-slot="icon"][data-ui-schema="ui.icon.agent-contract.v1"][data-ui-output-status="verified"]'
    )
    .first();
  await expect(settledIcon).toBeVisible();
  await expect(settledIcon).toHaveAttribute("data-ui-streaming", "optional");
  await expect(settledIcon).toHaveAttribute("data-ui-streaming-fallback", "snapshot");
  await expect(settledIcon).toHaveAttribute("data-ui-output-mode", "snapshot");

  return docsRoot;
}

async function runIconCriticalFlow(docsRoot) {
  const workbench = docsRoot
    .locator('[data-slot="playground"]')
    .filter({ has: docsRoot.locator('[data-slot="icon-workbench-controls"]') })
    .first();
  await expect(workbench).toBeVisible();

  const controls = workbench.locator('[data-slot="icon-workbench-controls"]').first();
  const sizeSelect = controls.locator("select").nth(0);
  const toneSelect = controls.locator("select").nth(1);
  const disabledCheckbox = controls.locator('input[type="checkbox"]').nth(0);
  const decorativeCheckbox = controls.locator('input[type="checkbox"]').nth(1);
  const customClassCheckbox = controls.locator('input[type="checkbox"]').nth(2);
  const ariaLabelInput = controls.locator('input[type="text"]').first();

  await sizeSelect.selectOption("lg");
  await toneSelect.selectOption("accent");
  await decorativeCheckbox.uncheck();
  await customClassCheckbox.check();
  await ariaLabelInput.fill("Workbench critical icon");

  const configuredIcon = workbench
    .locator(
      '[data-slot="icon"][data-size="lg"][data-tone="accent"][data-class-source="custom"][data-state="labeled"]'
    )
    .first();
  await expect(configuredIcon).toBeVisible();
  await expect(configuredIcon).toHaveAttribute("role", "img");
  await expect(configuredIcon).toHaveAttribute("aria-label", "Workbench critical icon");
  await expect(configuredIcon).toHaveAttribute("data-aria-source", "custom");
  await expect(configuredIcon).toHaveAttribute("data-ui-source", "custom");
  await expect(configuredIcon).toHaveAttribute("data-ui-state", "labeled");

  await disabledCheckbox.check();
  await expect(configuredIcon).toHaveAttribute("data-disabled", "true");
  await expect(configuredIcon).toHaveAttribute("data-state", "disabled");
  await expect(configuredIcon).toHaveAttribute("data-ui-state", "disabled");

  await disabledCheckbox.uncheck();
  await customClassCheckbox.uncheck();
  await decorativeCheckbox.check();
  await sizeSelect.selectOption("md");
  await toneSelect.selectOption("default");

  await expect(
    workbench.locator(
      '[data-slot="icon"][data-size="lg"][data-tone="accent"][data-class-source="custom"]'
    )
  ).toHaveCount(0);
}

test("docs-app icon uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const docsRoot = await gotoIconDocsAndWaitSettled(page);

  const decorativeIcon = docsRoot
    .locator('[data-slot="icon"][data-state="decorative"][data-aria-source="default"]')
    .first();
  await expect(decorativeIcon).toBeVisible();
  await expect(decorativeIcon).toHaveAttribute("aria-hidden", "true");
  await expect(decorativeIcon).not.toHaveAttribute("role", "img");

  const labeledIcon = docsRoot
    .locator('[data-slot="icon"][data-state="labeled"][data-aria-source="custom"]')
    .first();
  await expect(labeledIcon).toBeVisible();
  await expect(labeledIcon).toHaveAttribute("role", "img");
  await expect(labeledIcon).toHaveAttribute("aria-label", /.+/);
  await expect(labeledIcon).toHaveAttribute("data-ui-source", "custom");

  const disabledIcon = docsRoot
    .locator('[data-slot="icon"][data-state="disabled"][data-disabled="true"]')
    .first();
  await expect(disabledIcon).toBeVisible();
  await expect(disabledIcon).toHaveAttribute("data-ui-state", "disabled");
});

test("docs-app icon critical flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoIconDocsAndWaitSettled(page);
  await runIconCriticalFlow(docsRoot);

  await page.reload();
  const reloadedRoot = await gotoIconDocsAndWaitSettled(page);
  await runIconCriticalFlow(reloadedRoot);
});

test("docs-app icon source-first snippet is copy-paste ready with imports", async ({
  page,
}) => {
  const docsRoot = await gotoIconDocsAndWaitSettled(page);
  const sourceFirst = docsRoot
    .locator('[data-slot="playground"]')
    .filter({ hasText: "Source-first Starter (Copy-Paste Ready)" })
    .first();
  await expect(sourceFirst).toBeVisible();

  const codeBlock = sourceFirst.locator('[data-slot="code-block"]');
  if ((await codeBlock.count()) === 0) {
    const toggle = sourceFirst.getByRole("button", { name: /Show code|Hide code/ }).first();
    await toggle.click();
  }

  const visibleCode = sourceFirst.locator('[data-slot="code-block"]').first();
  await expect(visibleCode).toBeVisible();
  await expect(visibleCode).toHaveAttribute("data-copyable", "true");

  const codeText = sourceFirst.locator('[data-slot="code-block-code"]').first();
  await expect(codeText).toContainText("use leptos::prelude::*;");
  await expect(codeText).toContainText("use ui::{Icon, IconSize, IconTone};");
  await expect(codeText).toContainText("<Icon size=IconSize::Sm tone=IconTone::Accent is_decorative=true>");
});
