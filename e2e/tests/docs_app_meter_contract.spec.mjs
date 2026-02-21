import { expect, test } from "@playwright/test";

const METER_PAGE = "/#/components/meter";
const METER_ROOT = '[data-component="meter"]';
const WORKBENCH_METER = '#docs-meter-workbench[data-slot="meter"]';

async function gotoMeterDocsAndWaitSettled(page) {
  await page.goto(METER_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator(METER_ROOT).first();
  await expect(docsRoot).toBeVisible();

  const meter = docsRoot.locator(WORKBENCH_METER).first();
  await expect(meter).toBeVisible();
  await expect(meter).toHaveAttribute("role", "meter");
  await expect(meter).toHaveAttribute("data-ui-schema", "ui.meter.agent-contract.v1");
  await expect(meter).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(meter).toHaveAttribute("data-ui-output-mode", "snapshot");
  await expect(meter).toHaveAttribute("data-ui-output-status", "validated");
  await expect(meter).toHaveAttribute("data-state", "determinate");
  await expect(meter).toHaveAttribute("data-ui-state-phase", "determinate");

  return { docsRoot, meter };
}

async function runMeterRepeatableKeyFlow(page, docsRoot, meter) {
  const incrementAction = docsRoot
    .locator('[data-action="meter-workbench-increment"] [data-slot="button"]')
    .first();
  const toggleIndeterminateAction = docsRoot
    .locator('[data-action="meter-workbench-toggle-indeterminate"] [data-slot="button"]')
    .first();

  await expect(incrementAction).toBeVisible();
  await expect(toggleIndeterminateAction).toBeVisible();

  const before = Number((await meter.getAttribute("aria-valuenow")) ?? "0");

  await incrementAction.focus();
  await expect(incrementAction).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(meter).toHaveAttribute("data-state", "determinate");
  await expect(meter).toHaveAttribute("data-ui-state-phase", "determinate");
  const after = Number((await meter.getAttribute("aria-valuenow")) ?? "0");
  expect(after).toBeGreaterThan(before);

  await toggleIndeterminateAction.focus();
  await expect(toggleIndeterminateAction).toBeFocused();
  await page.keyboard.press("Space");
  await expect(meter).toHaveAttribute("data-state", "indeterminate");
  await expect(meter).toHaveAttribute("data-ui-state-phase", "indeterminate");
  await expect(meter).toHaveAttribute("data-indeterminate", "true");
  await expect(meter).not.toHaveAttribute("data-determinate", "true");

  await page.keyboard.press("Space");
  await expect(meter).toHaveAttribute("data-state", "determinate");
  await expect(meter).toHaveAttribute("data-ui-state-phase", "determinate");
  await expect(meter).toHaveAttribute("data-determinate", "true");
}

test("docs-app meter e2e selectors use semantic data markers with wasm-safe settled waits", async ({
  page,
}) => {
  const { docsRoot, meter } = await gotoMeterDocsAndWaitSettled(page);

  const streamingPolicy = docsRoot.locator('[data-slot="meter-streaming-policy"]').first();
  await expect(streamingPolicy).toBeVisible();

  const incrementAction = docsRoot
    .locator('[data-action="meter-workbench-increment"] [data-slot="button"]')
    .first();
  const toggleIndeterminateAction = docsRoot
    .locator('[data-action="meter-workbench-toggle-indeterminate"] [data-slot="button"]')
    .first();

  await expect(incrementAction).toBeVisible();
  await expect(toggleIndeterminateAction).toBeVisible();
  await expect(meter).toHaveAttribute("data-label-source", /default|custom/);
  await expect(meter).toHaveAttribute("data-motion-source", /default|custom/);
  await expect(meter).toHaveAttribute("data-class-source", /default|custom/);
});

test("docs-app meter animation path uses semantic ready and settled breakpoints", async ({
  page,
}) => {
  const { docsRoot, meter } = await gotoMeterDocsAndWaitSettled(page);
  const incrementAction = docsRoot
    .locator('[data-action="meter-workbench-increment"] [data-slot="button"]')
    .first();
  const toggleIndeterminateAction = docsRoot
    .locator('[data-action="meter-workbench-toggle-indeterminate"] [data-slot="button"]')
    .first();

  const before = Number((await meter.getAttribute("aria-valuenow")) ?? "0");
  await incrementAction.click();
  await expect(meter).toHaveAttribute("data-state", "determinate");
  const after = Number((await meter.getAttribute("aria-valuenow")) ?? "0");
  expect(after).toBeGreaterThan(before);

  await toggleIndeterminateAction.click();
  await expect(meter).toHaveAttribute("data-state", "indeterminate");
  await expect(meter).toHaveAttribute("data-ui-state-phase", "indeterminate");
  await expect(meter).toHaveAttribute("data-indeterminate", "true");
  await expect(meter).not.toHaveAttribute("data-determinate", "true");

  await toggleIndeterminateAction.click();
  await expect(meter).toHaveAttribute("data-state", "determinate");
  await expect(meter).toHaveAttribute("data-ui-state-phase", "determinate");
  await expect(meter).toHaveAttribute("data-determinate", "true");
});

test("docs-app meter key flow is repeatable and maps failures to semantic breakpoints", async ({
  page,
}) => {
  let settled = await gotoMeterDocsAndWaitSettled(page);
  await runMeterRepeatableKeyFlow(page, settled.docsRoot, settled.meter);

  await page.reload();
  settled = await gotoMeterDocsAndWaitSettled(page);
  await runMeterRepeatableKeyFlow(page, settled.docsRoot, settled.meter);
});
