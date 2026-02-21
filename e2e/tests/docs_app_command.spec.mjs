import { expect, test } from "@playwright/test";

async function waitForCommandReady(root) {
  await expect(root).toHaveAttribute("data-ui-schema", "ui.command.agent-contract");
  await expect(root).toHaveAttribute(
    "data-state",
    /default|query-results|query-empty|empty|disabled|disabled-empty/,
  );
}

async function runDefaultSubmitFlow(page, defaultScope, commandRoot) {
  const input = commandRoot.locator('[data-slot="command-input"]').first();
  await input.fill("cal");
  await expect(commandRoot).toHaveAttribute("data-query", "present");
  await expect(commandRoot).toHaveAttribute("data-state", "query-results");
  await expect(commandRoot).toHaveAttribute("data-query-control", "uncontrolled");

  await input.focus();
  await page.keyboard.press("ArrowDown");

  const focusedOption = commandRoot
    .locator('[data-slot="command-item"][data-focused="true"]')
    .first();
  await expect(focusedOption).toBeVisible();
  await expect(focusedOption).toHaveAttribute("role", "option");
  await focusedOption.click();

  const lastAction = defaultScope
    .locator('[data-slot="command-last-action"][data-scenario="default"]')
    .first();
  await expect(lastAction).toHaveAttribute("data-last-action", "calendar");
  await expect(commandRoot).toHaveAttribute("data-state", "query-results");
}

test("docs-app command e2e uses semantic selectors and stable readiness waits", async ({ page }) => {
  await page.goto("/#/components/command");

  const defaultScope = page.locator('[data-slot="command-e2e-default"]').first();
  const commandRoot = defaultScope
    .locator('[data-slot="command"][data-ui-schema="ui.command.agent-contract"]')
    .first();

  await waitForCommandReady(commandRoot);
  await expect(commandRoot).toHaveAttribute("data-query-control", "uncontrolled");
  await expect(commandRoot).toHaveAttribute("data-query-default-source", "empty");
  await expect(commandRoot).toHaveAttribute("data-action-source", "custom");

  const input = commandRoot.locator('[data-slot="command-input"]').first();
  await expect(input).toHaveAttribute("role", "combobox");
  await expect(input).toHaveAttribute("aria-autocomplete", "list");
  await expect(commandRoot.locator('[data-slot="command-list"]').first()).toHaveAttribute(
    "role",
    "listbox",
  );
});

test("docs-app command e2e covers ready and settled interaction without fragile waits", async ({
  page,
}) => {
  await page.goto("/#/components/command");

  const defaultScope = page.locator('[data-slot="command-e2e-default"]').first();
  const commandRoot = defaultScope
    .locator('[data-slot="command"][data-ui-schema="ui.command.agent-contract"]')
    .first();
  await waitForCommandReady(commandRoot);

  const input = commandRoot.locator('[data-slot="command-input"]').first();
  await input.fill("cal");
  await expect(commandRoot).toHaveAttribute("data-query", "present");
  await expect(commandRoot).toHaveAttribute("data-state", "query-results");

  await input.focus();
  await page.keyboard.press("ArrowDown");

  const focusedOption = commandRoot.locator('[data-slot="command-item"][data-focused="true"]').first();
  await expect(focusedOption).toBeVisible();
  await expect(focusedOption).toHaveAttribute("role", "option");
  await focusedOption.click();

  const lastAction = defaultScope
    .locator('[data-slot="command-last-action"][data-scenario="default"]')
    .first();
  await expect(lastAction).toHaveAttribute("data-last-action", "calendar");

  await expect(commandRoot).toHaveAttribute("data-query", "present");
  await expect(commandRoot).toHaveAttribute("data-state", "query-results");
});

test("docs-app command key flow is repeatable and failures map to semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/command");

  let defaultScope = page.locator('[data-slot="command-e2e-default"]').first();
  let commandRoot = defaultScope
    .locator('[data-slot="command"][data-ui-schema="ui.command.agent-contract"]')
    .first();
  await waitForCommandReady(commandRoot);
  await runDefaultSubmitFlow(page, defaultScope, commandRoot);

  await page.reload();

  defaultScope = page.locator('[data-slot="command-e2e-default"]').first();
  commandRoot = defaultScope
    .locator('[data-slot="command"][data-ui-schema="ui.command.agent-contract"]')
    .first();
  await waitForCommandReady(commandRoot);
  await runDefaultSubmitFlow(page, defaultScope, commandRoot);
});

test("docs-app command high-risk paths keep focus keyboard and settled semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/command");

  const defaultScope = page.locator('[data-slot="command-e2e-default"]').first();
  const commandRoot = defaultScope
    .locator('[data-slot="command"][data-ui-schema="ui.command.agent-contract"]')
    .first();
  await waitForCommandReady(commandRoot);

  const input = commandRoot.locator('[data-slot="command-input"]').first();
  await input.fill("cal");
  await input.focus();
  await expect(input).toBeFocused();

  await page.keyboard.press("ArrowDown");
  const focusedOption = commandRoot
    .locator('[data-slot="command-item"][data-focused="true"]')
    .first();
  await expect(focusedOption).toHaveAttribute("data-focused", "true");
  await expect(focusedOption).toHaveAttribute("role", "option");
  await focusedOption.click();

  const lastAction = defaultScope
    .locator('[data-slot="command-last-action"][data-scenario="default"]')
    .first();
  await expect(lastAction).toHaveAttribute("data-last-action", "calendar");
  await expect(commandRoot).toHaveAttribute("data-state", "query-results");
});

test("docs-app command interactive playground key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/command");

  let workbenchScope = page.locator('[data-slot="command-workbench"]').first();
  const workbenchControls = page.locator('[data-slot="command-workbench-controls"]').first();
  await expect(workbenchControls).toBeVisible();

  let workbenchRoot = workbenchScope
    .locator('[data-slot="command"][data-ui-schema="ui.command.agent-contract"]')
    .first();
  await waitForCommandReady(workbenchRoot);
  await expect(workbenchRoot).toHaveAttribute("data-disabled", "false");
  await expect(workbenchRoot).toHaveAttribute("data-action-source", "custom");

  let input = workbenchRoot.locator('[data-slot="command-input"]').first();
  await input.fill("cal");
  await expect(workbenchRoot).toHaveAttribute("data-query", "present");
  await expect(workbenchRoot).toHaveAttribute("data-state", "query-results");
  await input.focus();
  await page.keyboard.press("ArrowDown");

  let focusedOption = workbenchRoot
    .locator('[data-slot="command-item"][data-focused="true"]')
    .first();
  await expect(focusedOption).toHaveAttribute("role", "option");
  await focusedOption.click();

  let lastAction = workbenchScope.locator('[data-slot="command-workbench-last-action"]').first();
  await expect(lastAction).toHaveAttribute("data-last-action", "calendar");

  await page.reload();

  workbenchScope = page.locator('[data-slot="command-workbench"]').first();
  workbenchRoot = workbenchScope
    .locator('[data-slot="command"][data-ui-schema="ui.command.agent-contract"]')
    .first();
  await waitForCommandReady(workbenchRoot);

  input = workbenchRoot.locator('[data-slot="command-input"]').first();
  await input.fill("cal");
  await page.keyboard.press("ArrowDown");
  focusedOption = workbenchRoot
    .locator('[data-slot="command-item"][data-focused="true"]')
    .first();
  await focusedOption.click();

  lastAction = workbenchScope.locator('[data-slot="command-workbench-last-action"]').first();
  await expect(lastAction).toHaveAttribute("data-last-action", "calendar");
  await expect(workbenchRoot).toHaveAttribute("data-state", "query-results");
});

test("docs-app command marker scenario keeps semantic settled state under custom motion", async ({
  page,
}) => {
  await page.goto("/#/components/command");

  const markerScope = page.locator('[data-slot="command-e2e-markers"]').first();
  const markerRoot = markerScope
    .locator('[data-slot="command"][data-ui-schema="ui.command.agent-contract"]')
    .first();
  await waitForCommandReady(markerRoot);
  await expect(markerRoot).toHaveAttribute("data-motion-source", "custom");

  const firstOption = markerRoot.locator('[data-slot="command-item"][data-index="0"]').first();
  await expect(firstOption).toHaveAttribute("role", "option");
  await firstOption.click();

  const lastAction = markerScope
    .locator('[data-slot="command-last-action"][data-scenario="markers"]')
    .first();
  await expect(lastAction).toHaveAttribute("data-last-action", "open-recent");
  await expect(markerRoot).toHaveAttribute("data-state", "default");
});
