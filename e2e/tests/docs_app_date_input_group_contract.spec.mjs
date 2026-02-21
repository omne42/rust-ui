import { expect, test } from "@playwright/test";

async function gotoDateInputGroupDocsAndWaitSettled(page) {
  await page.goto("/#/components/date-input-group");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="date-input-group"]').first();
  await expect(docsRoot).toBeVisible();

  const settledRoot = docsRoot
    .locator(
      '[data-slot="date-input-group"][data-ui-schema="ui.date-input-group.agent-contract"][data-ui-schema-version="v1"][data-ui-stream-mode="snapshot"][data-ui-output-status="verified"]',
    )
    .first();
  await expect(settledRoot).toBeVisible();

  return docsRoot;
}

test("docs-app date-input-group uses semantic selectors with wasm-stable wait strategy", async ({
  page,
}) => {
  const docsRoot = await gotoDateInputGroupDocsAndWaitSettled(page);
  const stateMatrix = docsRoot.locator('[data-slot="date-input-group-state-matrix"]').first();
  await expect(stateMatrix).toBeVisible();

  const defaultGroup = stateMatrix
    .locator(
      '[data-slot="date-input-group"][data-state="default"][data-variant="primary"][data-width="fit"][data-ui-source="state-primitives"]',
    )
    .first();
  await expect(defaultGroup).toBeVisible();
  await expect(defaultGroup).toHaveAttribute("data-aria-source", "default");
  await expect(defaultGroup).toHaveAttribute("data-motion-source", "default");

  const segmentedGroup = stateMatrix
    .locator(
      '[data-slot="date-input-group"][data-state="segmented"][data-segmented="true"][data-has-prefix="true"][data-has-suffix="true"][data-aria-source="custom"]',
    )
    .first();
  await expect(segmentedGroup).toBeVisible();
  await expect(segmentedGroup).toHaveAttribute("data-ui-state", "segmented");
  await expect(segmentedGroup).toHaveAttribute("data-ui-output-status", "verified");

  const disabledInvalidGroup = stateMatrix
    .locator(
      '[data-slot="date-input-group"][data-state="disabled-invalid"][data-variant="secondary"][data-width="full"][data-disabled="true"][data-invalid="true"][data-custom-class="true"][data-class-source="custom"]',
    )
    .first();
  await expect(disabledInvalidGroup).toBeVisible();
  await expect(disabledInvalidGroup).toHaveAttribute("data-ui-state", "disabled");
  await expect(disabledInvalidGroup).toHaveAttribute("data-ui-motion-source", "default");
});

test("docs-app date-input-group motion/stream path uses semantic ready/settled breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoDateInputGroupDocsAndWaitSettled(page);
  const streamingContract = docsRoot
    .locator('[data-slot="date-input-group-streaming-contract"]')
    .first();
  await expect(streamingContract).toBeVisible();

  const streamSettled = streamingContract
    .locator(
      '[data-slot="date-input-group"][data-ui-stream-support="unsupported"][data-ui-stream-fallback="snapshot"][data-ui-stream-mode="snapshot"][data-ui-output-status="verified"]',
    )
    .first();
  await expect(streamSettled).toBeVisible();
  await expect(streamSettled).toHaveAttribute("data-motion-source", "default");
  await expect(streamSettled).toHaveAttribute("data-ui-action", "snapshot-render");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const streamSettledAfterReload = page
    .locator('[data-component="date-input-group"]')
    .first()
    .locator(
      '[data-slot="date-input-group"][data-ui-stream-support="unsupported"][data-ui-stream-fallback="snapshot"][data-ui-stream-mode="snapshot"][data-ui-output-status="verified"]',
    )
    .first();
  await expect(streamSettledAfterReload).toBeVisible();
  await expect(streamSettledAfterReload).toHaveAttribute("data-motion-source", "default");
  await expect(streamSettledAfterReload).toHaveAttribute("data-ui-action", "snapshot-render");
});

test("docs-app date-input-group key flow is repeatable with semantic failure breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoDateInputGroupDocsAndWaitSettled(page);
  const streamingContract = docsRoot
    .locator('[data-slot="date-input-group-streaming-contract"]')
    .first();
  await expect(streamingContract).toBeVisible();

  const modeSelect = streamingContract
    .locator('[data-slot="date-input-group-requested-stream-mode"]')
    .first();
  const outputSelect = streamingContract
    .locator('[data-slot="date-input-group-requested-output-status"]')
    .first();
  const requestedState = streamingContract
    .locator('[data-slot="date-input-group-streaming-requested-state"]')
    .first();
  const settledGroup = streamingContract
    .locator(
      '[data-slot="date-input-group"][data-ui-stream-support="unsupported"][data-ui-stream-fallback="snapshot"][data-ui-stream-mode="snapshot"][data-ui-output-status="verified"]',
    )
    .first();

  await expect(modeSelect).toBeVisible();
  await expect(outputSelect).toBeVisible();
  await expect(requestedState).toBeVisible();
  await expect(settledGroup).toBeVisible();

  // Pointer path
  await modeSelect.selectOption("snapshot");
  await outputSelect.selectOption("verified");
  await expect(requestedState).toContainText("requested mode: snapshot");
  await expect(requestedState).toContainText("requested output status: verified");
  await expect(settledGroup).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(settledGroup).toHaveAttribute("data-ui-output-status", "verified");
  await expect(settledGroup).toHaveAttribute("data-ui-action", "snapshot-render");

  // Keyboard path
  await modeSelect.focus();
  await expect(modeSelect).toBeFocused();
  await page.keyboard.press("ArrowUp");
  await expect(requestedState).toContainText("requested mode: streaming");
  await expect(settledGroup).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(settledGroup).toHaveAttribute("data-ui-output-status", "verified");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const streamingAfterReload = page
    .locator('[data-component="date-input-group"]')
    .first()
    .locator('[data-slot="date-input-group-streaming-contract"]')
    .first();
  const requestedAfterReload = streamingAfterReload
    .locator('[data-slot="date-input-group-streaming-requested-state"]')
    .first();
  const settledAfterReload = streamingAfterReload
    .locator(
      '[data-slot="date-input-group"][data-ui-stream-support="unsupported"][data-ui-stream-fallback="snapshot"][data-ui-stream-mode="snapshot"][data-ui-output-status="verified"]',
    )
    .first();

  await expect(requestedAfterReload).toContainText("requested mode: streaming");
  await expect(requestedAfterReload).toContainText("requested output status: draft");
  await expect(settledAfterReload).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(settledAfterReload).toHaveAttribute("data-ui-output-status", "verified");
  await expect(settledAfterReload).toHaveAttribute("data-ui-action", "snapshot-render");
});
