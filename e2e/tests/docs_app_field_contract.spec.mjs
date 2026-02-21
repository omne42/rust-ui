import { expect, test } from "@playwright/test";

async function gotoFieldDocsAndWaitSettled(page) {
  await page.goto("/#/components/field");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="field"]').first();
  await expect(docsRoot).toBeVisible();

  const settledRoot = docsRoot
    .locator(
      '[data-slot="field"][data-ui-schema="ui.field.agent-contract/v1"][data-ui-stream-mode="snapshot"][data-ui-output-status="verified"]'
    )
    .first();
  await expect(settledRoot).toBeVisible();

  return docsRoot;
}

async function resolveFieldWorkbench(docsRoot) {
  const playground = docsRoot
    .locator('section.playground')
    .filter({ has: docsRoot.locator('[data-slot="field-workbench-controls"]') })
    .first();
  const controls = playground.locator('[data-slot="field-workbench-controls"]').first();
  const preview = playground.locator('[data-slot="field"]').first();
  const fieldInput = preview.locator('[data-slot="field-control"] input').first();
  const orientationSelect = controls
    .locator('[data-action="field-workbench-orientation"]')
    .first();
  const toneSelect = controls.locator('[data-action="field-workbench-tone"]').first();
  const invalidToggle = controls
    .locator('[data-action="field-workbench-toggle-invalid"]')
    .first();
  const disabledToggle = controls
    .locator('[data-action="field-workbench-toggle-disabled"]')
    .first();
  const customClassToggle = controls
    .locator('[data-action="field-workbench-toggle-custom-class"]')
    .first();
  const summary = controls.locator('[data-slot="field-workbench-summary"]').first();

  await expect(playground).toBeVisible();
  await expect(controls).toBeVisible();
  await expect(preview).toBeVisible();
  await expect(fieldInput).toBeVisible();
  await expect(orientationSelect).toBeVisible();
  await expect(toneSelect).toBeVisible();
  await expect(invalidToggle).toBeVisible();
  await expect(disabledToggle).toBeVisible();
  await expect(customClassToggle).toBeVisible();
  await expect(summary).toBeVisible();

  return {
    preview,
    fieldInput,
    orientationSelect,
    toneSelect,
    invalidToggle,
    disabledToggle,
    customClassToggle,
    summary,
  };
}

async function runRepeatableFieldKeyboardFlow(page, docsRoot) {
  const { preview, fieldInput, invalidToggle, disabledToggle } =
    await resolveFieldWorkbench(docsRoot);

  await expect(preview).toHaveAttribute("data-state", "required");
  await expect(preview).toHaveAttribute("data-required", "true");

  await fieldInput.focus();
  await expect(fieldInput).toBeFocused();

  await invalidToggle.focus();
  await expect(invalidToggle).toBeFocused();
  await page.keyboard.press("Space");
  await expect(preview).toHaveAttribute("data-state", "invalid");
  await expect(preview).toHaveAttribute("data-invalid", "true");

  await disabledToggle.focus();
  await expect(disabledToggle).toBeFocused();
  await page.keyboard.press("Space");
  await expect(preview).toHaveAttribute("data-state", "invalid-disabled");
  await expect(preview).toHaveAttribute("data-disabled", "true");

  await invalidToggle.focus();
  await expect(invalidToggle).toBeFocused();
  await page.keyboard.press("Space");
  await expect(preview).toHaveAttribute("data-state", "disabled");
  await expect(preview).toHaveAttribute("data-invalid", "false");

  await disabledToggle.focus();
  await expect(disabledToggle).toBeFocused();
  await page.keyboard.press("Space");
  await expect(preview).toHaveAttribute("data-state", "required");
  await expect(preview).toHaveAttribute("data-disabled", "false");
}

test("docs-app field uses semantic selectors with wasm-stable wait strategy", async ({
  page,
}) => {
  const docsRoot = await gotoFieldDocsAndWaitSettled(page);

  const stateMatrix = docsRoot.locator('[data-slot="field-state-matrix"]').first();
  await expect(stateMatrix).toBeVisible();

  await expect(
    stateMatrix
      .locator(
        '[data-slot="field"][data-required="true"][data-required-source="legacy-prop"][data-message-kind="description"]'
      )
      .first()
  ).toBeVisible();

  await expect(
    stateMatrix
      .locator(
        '[data-slot="field"][data-invalid="true"][data-invalid-source="legacy-prop"][data-message-kind="error"]'
      )
      .first()
  ).toBeVisible();

  await expect(
    stateMatrix
      .locator('[data-slot="field"][data-disabled="true"][data-disabled-source="legacy-prop"]')
      .first()
  ).toBeVisible();
});

test("docs-app field motion path uses semantic ready/settled breakpoints", async ({ page }) => {
  const docsRoot = await gotoFieldDocsAndWaitSettled(page);

  const playground = docsRoot
    .locator('section.playground')
    .filter({ has: docsRoot.locator('[data-slot="field-workbench-controls"]') })
    .first();
  const controls = playground.locator('[data-slot="field-workbench-controls"]').first();
  const preview = playground.locator('[data-slot="field"]').first();

  await expect(controls).toBeVisible();
  await expect(preview).toHaveAttribute("data-motion-source", "default");

  await controls.locator('[data-action="field-workbench-toggle-invalid"]').check();
  await expect(preview).toHaveAttribute("data-state", "invalid");

  await controls.locator('[data-action="field-workbench-toggle-disabled"]').check();
  await expect(preview).toHaveAttribute("data-state", "invalid-disabled");

  await controls.locator('[data-action="field-workbench-toggle-invalid"]').uncheck();
  await expect(preview).toHaveAttribute("data-state", "disabled");

  await controls.locator('[data-action="field-workbench-toggle-disabled"]').uncheck();
  await expect(preview).toHaveAttribute("data-state", "required");

  await controls.locator('[data-action="field-workbench-motion-ms"]').evaluate((input) => {
    input.value = "240";
    input.dispatchEvent(new Event("input", { bubbles: true }));
  });
  await expect(preview).toHaveAttribute("data-motion-source", "custom");
  await expect(preview).toHaveAttribute("style", /--ui-field-motion-duration:\s*(1|240)ms;/);
});

test("docs-app field key flow is repeatable with semantic failure breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoFieldDocsAndWaitSettled(page);
  await runRepeatableFieldKeyboardFlow(page, docsRoot);

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRootAfterReload = page.locator('[data-component="field"]').first();
  await expect(docsRootAfterReload).toBeVisible();
  await runRepeatableFieldKeyboardFlow(page, docsRootAfterReload);
});

test("docs-app field interactive playground supports realtime props/state preview", async ({
  page,
}) => {
  const docsRoot = await gotoFieldDocsAndWaitSettled(page);
  const { preview, orientationSelect, toneSelect, customClassToggle, summary } =
    await resolveFieldWorkbench(docsRoot);

  await expect(preview).toHaveAttribute("data-orientation", "vertical");
  await expect(preview).toHaveAttribute("data-tone", "default");

  await orientationSelect.selectOption("horizontal");
  await expect(preview).toHaveAttribute("data-orientation", "horizontal");
  await expect(preview).toHaveAttribute("data-state", "required");

  await toneSelect.selectOption("muted");
  await expect(preview).toHaveAttribute("data-tone", "muted");

  await customClassToggle.check();
  await expect(preview).toHaveAttribute("data-custom-class", "true");
  await expect(preview).toHaveAttribute("data-class-source", "custom");

  await expect(summary).toContainText("orientation=horizontal");
  await expect(summary).toContainText("tone=muted");
  await expect(summary).toContainText("custom_class=true");

  await runRepeatableFieldKeyboardFlow(page, docsRoot);
});
