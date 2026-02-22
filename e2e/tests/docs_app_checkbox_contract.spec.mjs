import { expect, test } from "@playwright/test";

const CHECKBOX_PAGE = "/#/components/checkbox";

async function waitForWasmReady(page) {
  await page.locator("body:not(:has(#boot))").waitFor();
}

async function expectCheckboxReady(surface, checkbox) {
  await expect(surface).toHaveAttribute("data-e2e-ready", "true");
  await expect(checkbox).toBeVisible();
  await expect(checkbox).toHaveAttribute("role", "checkbox");
  await expect(checkbox).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(checkbox).toHaveAttribute("data-ui-output-status", "verified");
}

async function expectCheckboxSettled(checkbox, expected) {
  await expect(checkbox).toHaveAttribute("aria-checked", expected.ariaChecked);
  await expect(checkbox).toHaveAttribute("data-state", expected.dataState);
  await expect(checkbox).toHaveAttribute("data-ui-action", "press.toggle");
  await expect(checkbox).toHaveAttribute("data-ui-state", expected.uiState);
  await expect(checkbox).toHaveAttribute("data-ui-source", "state-primitives");
}

function resolveControlledFlow(page) {
  const controlled = page.locator('[data-slot="checkbox-e2e-controlled-surface"]').first();
  const checkbox = controlled
    .locator('[data-slot="checkbox-e2e-controlled-target"] [data-slot="checkbox"][role="checkbox"]')
    .first();
  const checkedState = controlled.locator('[data-slot="checkbox-e2e-controlled-checked"]').first();
  const changeState = controlled.locator('[data-slot="checkbox-e2e-controlled-last-change"]').first();
  return { controlled, checkbox, checkedState, changeState };
}

test("docs-app checkbox exposes headless semantic contract and keyboard flow", async ({ page }) => {
  await page.goto(CHECKBOX_PAGE);
  await waitForWasmReady(page);

  const { controlled, checkbox, checkedState, changeState } = resolveControlledFlow(page);

  await expectCheckboxReady(controlled, checkbox);
  await expectCheckboxSettled(checkbox, {
    ariaChecked: "false",
    dataState: "unchecked",
    uiState: "unchecked",
  });

  await checkbox.focus();
  await expect(checkbox).toBeFocused();
  await page.keyboard.press("Space");

  await expectCheckboxSettled(checkbox, {
    ariaChecked: "true",
    dataState: "checked",
    uiState: "checked",
  });
  await expect(checkedState).toContainText("true");
  await expect(changeState).toContainText("true");

  await page.keyboard.press("Space");
  await expectCheckboxSettled(checkbox, {
    ariaChecked: "false",
    dataState: "unchecked",
    uiState: "unchecked",
  });
  await expect(checkedState).toContainText("false");
  await expect(changeState).toContainText("false");
});

test("docs-app checkbox key flow is repeatable and failures map to semantic breakpoints", async ({ page }) => {
  await page.goto(CHECKBOX_PAGE);
  await waitForWasmReady(page);

  let { controlled, checkbox, checkedState, changeState } = resolveControlledFlow(page);
  await expectCheckboxReady(controlled, checkbox);
  await expectCheckboxSettled(checkbox, {
    ariaChecked: "false",
    dataState: "unchecked",
    uiState: "unchecked",
  });
  await expect(checkbox).toHaveAttribute("data-state-source", "controlled");
  await expect(checkbox).toHaveAttribute("data-checked-source", "is_checked");
  await expect(checkbox).toHaveAttribute("data-handler-source", "on_checked_change");

  await checkbox.focus();
  await expect(checkbox).toBeFocused();
  await page.keyboard.press("Space");
  await expectCheckboxSettled(checkbox, {
    ariaChecked: "true",
    dataState: "checked",
    uiState: "checked",
  });
  await expect(checkedState).toContainText("true");
  await expect(changeState).toContainText("true");

  await page.reload();
  await waitForWasmReady(page);

  ({ controlled, checkbox, checkedState, changeState } = resolveControlledFlow(page));
  await expectCheckboxReady(controlled, checkbox);
  await expectCheckboxSettled(checkbox, {
    ariaChecked: "false",
    dataState: "unchecked",
    uiState: "unchecked",
  });
  await expect(checkedState).toContainText("false");
  await expect(changeState).toContainText("none");

  await checkbox.focus();
  await expect(checkbox).toBeFocused();
  await page.keyboard.press("Space");
  await expectCheckboxSettled(checkbox, {
    ariaChecked: "true",
    dataState: "checked",
    uiState: "checked",
  });
  await expect(checkedState).toContainText("true");
  await expect(changeState).toContainText("true");
});

test("docs-app checkbox keeps disabled items non-interactive", async ({ page }) => {
  await page.goto(CHECKBOX_PAGE);
  await waitForWasmReady(page);

  const matrix = page.locator('[data-slot="checkbox-e2e-matrix-surface"]').first();
  const disabledOn = matrix
    .locator('[data-slot="checkbox-e2e-disabled-on"] [data-slot="checkbox"][role="checkbox"]')
    .first();
  const disabledOff = matrix
    .locator('[data-slot="checkbox-e2e-disabled-off"] [data-slot="checkbox"][role="checkbox"]')
    .first();

  await expectCheckboxReady(matrix, disabledOn);
  await expectCheckboxReady(matrix, disabledOff);

  await expect(disabledOn).toBeDisabled();
  await expect(disabledOn).toHaveAttribute("aria-disabled", "true");
  await expect(disabledOn).toHaveAttribute("tabindex", "-1");
  await expect(disabledOn).toHaveAttribute("data-disabled", "true");
  await expectCheckboxSettled(disabledOn, {
    ariaChecked: "true",
    dataState: "checked",
    uiState: "disabled",
  });

  await expect(disabledOff).toBeDisabled();
  await expect(disabledOff).toHaveAttribute("aria-disabled", "true");
  await expect(disabledOff).toHaveAttribute("tabindex", "-1");
  await expect(disabledOff).toHaveAttribute("data-disabled", "true");
  await expectCheckboxSettled(disabledOff, {
    ariaChecked: "false",
    dataState: "unchecked",
    uiState: "disabled",
  });

  await disabledOn.click({ force: true });
  await disabledOff.click({ force: true });
  await expectCheckboxSettled(disabledOn, {
    ariaChecked: "true",
    dataState: "checked",
    uiState: "disabled",
  });
  await expectCheckboxSettled(disabledOff, {
    ariaChecked: "false",
    dataState: "unchecked",
    uiState: "disabled",
  });
});

test("docs-app checkbox high-risk paths keep focus keyboard and disabled branches semantically explicit", async ({
  page,
}) => {
  await page.goto(CHECKBOX_PAGE);
  await waitForWasmReady(page);

  const { controlled, checkbox } = resolveControlledFlow(page);
  const matrix = page.locator('[data-slot="checkbox-e2e-matrix-surface"]').first();
  const disabledOn = matrix
    .locator('[data-slot="checkbox-e2e-disabled-on"] [data-slot="checkbox"][role="checkbox"]')
    .first();
  const disabledOff = matrix
    .locator('[data-slot="checkbox-e2e-disabled-off"] [data-slot="checkbox"][role="checkbox"]')
    .first();

  await expectCheckboxReady(controlled, checkbox);
  await checkbox.focus();
  await expect(checkbox).toBeFocused();
  await page.keyboard.press("Space");
  await expectCheckboxSettled(checkbox, {
    ariaChecked: "true",
    dataState: "checked",
    uiState: "checked",
  });

  await expectCheckboxReady(matrix, disabledOn);
  await expectCheckboxReady(matrix, disabledOff);
  await expect(disabledOn).toBeDisabled();
  await expect(disabledOn).toHaveAttribute("aria-disabled", "true");
  await expect(disabledOff).toBeDisabled();
  await expect(disabledOff).toHaveAttribute("aria-disabled", "true");

  await disabledOn.click({ force: true });
  await disabledOff.click({ force: true });
  await expectCheckboxSettled(disabledOn, {
    ariaChecked: "true",
    dataState: "checked",
    uiState: "disabled",
  });
  await expectCheckboxSettled(disabledOff, {
    ariaChecked: "false",
    dataState: "unchecked",
    uiState: "disabled",
  });
});

test("docs-app checkbox playground source is copy-paste ready", async ({ page }) => {
  await page.goto(CHECKBOX_PAGE);
  await waitForWasmReady(page);

  const interactive = page.locator('[data-slot="checkbox-e2e-interactive-surface"]').first();
  await expect(interactive).toBeVisible();

  await interactive.locator('[data-slot="playground-toggle-code"]').click();

  const codeBlock = interactive
    .locator('[data-slot="playground-code"] [data-slot="code-block"]')
    .first();
  await expect(codeBlock).toBeVisible();
  await expect(codeBlock).toHaveAttribute("data-copyable", "true");
  await expect(codeBlock).toContainText("use leptos::prelude::*;");
  await expect(codeBlock).toContainText("use ui::*;");
  await expect(codeBlock).toContainText("<Checkbox");

  const streamingPolicy = page.locator('[data-slot="checkbox-streaming-policy"]').first();
  await expect(streamingPolicy).toContainText("Streaming Optional; fallback=snapshot.");
});
