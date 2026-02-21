import { expect, test } from "@playwright/test";

const COLOR_THUMB_PAGE = "/#/components/color-thumb";
const WASM_READY_SELECTOR = "body:not(:has(#boot))";
const IDLE_ROOT_SELECTOR =
  '[data-component="color-thumb"] #docs-color-thumb-idle[data-slot="color-thumb"]';

async function gotoColorThumb(page) {
  await page.goto(COLOR_THUMB_PAGE);
  await page.locator(WASM_READY_SELECTOR).waitFor();
}

async function resolveIdleRoot(page) {
  const root = page.locator(IDLE_ROOT_SELECTOR).first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-ui-schema", "ui.color-thumb.agent-contract.v1");
  await expect(root).toHaveAttribute("data-ui-stream-support", "optional");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");
  return root;
}

test("docs-app color-thumb contract uses semantic selectors with wasm-safe stable waits", async ({
  page,
}) => {
  await gotoColorThumb(page);
  const root = await resolveIdleRoot(page);

  const handle = root.locator('[data-slot="color-thumb-handle"]').first();
  const fill = root.locator('[data-slot="color-thumb-fill"]').first();
  const loupe = root.locator('[data-slot="color-thumb-loupe"]').first();
  const loupeFill = root.locator('[data-slot="color-thumb-loupe-fill"]').first();

  await expect(handle).toBeVisible();
  await expect(fill).toBeVisible();
  await expect(loupe).toBeVisible();
  await expect(loupeFill).toBeVisible();

  await expect(root).toHaveAttribute("role", "slider");
  await expect(root).toHaveAttribute("aria-label", /Color/);
  await expect(root).toHaveAttribute("data-state", "idle");
  await expect(root).toHaveAttribute("data-ui-intent", "pick-color-point");
  await expect(root).toHaveAttribute("data-ui-action", "idle");
  await expect(root).toHaveAttribute("data-interaction-source", "default");
  await expect(root).toHaveAttribute("data-aria-source", "default");
  await expect(root).toHaveAttribute("data-loupe-source", "default");
  await expect(root).toHaveAttribute("data-x-source", "external");
  await expect(root).toHaveAttribute("data-y-source", "external");
});

test("docs-app color-thumb focused/dragging/disabled/custom branches stay on semantic ready and settled breakpoints", async ({
  page,
}) => {
  await gotoColorThumb(page);
  await resolveIdleRoot(page);

  const focusedRoot = page
    .locator('[data-component="color-thumb"] #docs-color-thumb-focused[data-slot="color-thumb"]')
    .first();
  const draggingRoot = page
    .locator('[data-component="color-thumb"] #docs-color-thumb-dragging[data-slot="color-thumb"]')
    .first();
  const disabledRoot = page
    .locator('[data-component="color-thumb"] #docs-color-thumb-disabled[data-slot="color-thumb"]')
    .first();
  const customRoot = page
    .locator('[data-component="color-thumb"] #docs-color-thumb-custom[data-slot="color-thumb"]')
    .first();

  await expect(focusedRoot).toHaveAttribute("data-state", "focused");
  await expect(focusedRoot).toHaveAttribute("data-focused", "true");
  await expect(focusedRoot).toHaveAttribute("data-ui-action", "focus");
  await expect(focusedRoot).toHaveAttribute("data-ui-output-status", "verified");
  await focusedRoot.focus();
  await expect(focusedRoot).toBeFocused();

  await expect(draggingRoot).toHaveAttribute("data-state", "dragging");
  await expect(draggingRoot).toHaveAttribute("data-dragging", "true");
  await expect(draggingRoot).toHaveAttribute("data-ui-action", "drag");
  await expect(draggingRoot).toHaveAttribute("data-ui-output-status", "verified");
  await draggingRoot.dispatchEvent("pointerdown");
  await expect(draggingRoot).toHaveAttribute("data-ui-action", "drag");
  await draggingRoot.dispatchEvent("pointerup");
  await expect(draggingRoot).toHaveAttribute("data-ui-action", "drag");

  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("aria-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("tabindex", "-1");
  await expect(disabledRoot).toHaveAttribute("data-ui-output-status", "verified");

  await expect(customRoot).toHaveAttribute("data-state", "dragging");
  await expect(customRoot).toHaveAttribute("data-custom-class", "true");
  await expect(customRoot).toHaveAttribute("data-class-source", "custom");
  await expect(customRoot).toHaveAttribute("data-loupe-source", "external");
  await expect(customRoot).toHaveAttribute("data-ui-action", "drag");
  await expect(customRoot).toHaveAttribute("data-ui-output-status", "verified");
  await expect(customRoot).not.toHaveAttribute("data-loupe-visible", "true");

  await page.reload();
  await page.locator(WASM_READY_SELECTOR).waitFor();

  const reloadedIdleRoot = page.locator(IDLE_ROOT_SELECTOR).first();
  await expect(reloadedIdleRoot).toHaveAttribute("data-state", "idle");
  await expect(reloadedIdleRoot).toHaveAttribute("data-ui-action", "idle");
});

test("docs-app color-thumb key flow is repeatable and failures map to semantic breakpoints", async ({
  page,
}) => {
  await gotoColorThumb(page);
  const root = await resolveIdleRoot(page);

  await root.focus();
  await expect(root).toBeFocused();
  await expect(root).toHaveAttribute("data-state", "idle");
  await expect(root).toHaveAttribute("data-ui-action", "idle");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");

  await page.keyboard.press("ArrowRight");
  await expect(root).toHaveAttribute("data-ui-action", "idle");
  await expect(root).toHaveAttribute("data-ui-state", "idle");
  await expect(root).toHaveAttribute("data-ui-source", "default");

  await page.reload();
  await page.locator(WASM_READY_SELECTOR).waitFor();

  const replayedRoot = await resolveIdleRoot(page);
  await replayedRoot.focus();
  await expect(replayedRoot).toBeFocused();
  await page.keyboard.press("ArrowLeft");
  await expect(replayedRoot).toHaveAttribute("data-state", "idle");
  await expect(replayedRoot).toHaveAttribute("data-ui-action", "idle");
  await expect(replayedRoot).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app color-thumb interactive playground updates props/state and links spec input to preview", async ({
  page,
}) => {
  await gotoColorThumb(page);

  const controls = page
    .locator('[data-slot="color-thumb-workbench-controls"]')
    .first();
  await expect(controls).toBeVisible();

  const workbenchRoot = page
    .locator(
      '[data-component="color-thumb"] #docs-color-thumb-workbench[data-slot="color-thumb"]',
    )
    .first();
  const specRoot = page
    .locator(
      '[data-component="color-thumb"] #docs-color-thumb-workbench-spec[data-slot="color-thumb"]',
    )
    .first();
  const stateText = page
    .locator('[data-slot="color-thumb-workbench-state"]')
    .first();
  const specStateText = page
    .locator('[data-slot="color-thumb-workbench-spec-state"]')
    .first();

  await expect(workbenchRoot).toBeVisible();
  await expect(specRoot).toBeVisible();
  await expect(stateText).toContainText("state: focused");

  const xInput = controls
    .locator('[data-slot="color-thumb-workbench-input-x"]')
    .first();
  const yInput = controls
    .locator('[data-slot="color-thumb-workbench-input-y"]')
    .first();

  await xInput.evaluate((node) => {
    node.value = "82";
    node.dispatchEvent(new Event("input", { bubbles: true }));
  });
  await yInput.evaluate((node) => {
    node.value = "28";
    node.dispatchEvent(new Event("input", { bubbles: true }));
  });
  await expect(stateText).toContainText("x: 82.0");
  await expect(stateText).toContainText("y: 28.0");

  await controls.getByRole("checkbox", { name: "Disabled" }).check();
  await expect(workbenchRoot).toHaveAttribute("data-state", "disabled");

  await controls
    .locator('[data-slot="color-thumb-workbench-replay-drag"]')
    .click();
  await expect(workbenchRoot).toHaveAttribute("data-state", "dragging");
  await expect(stateText).toContainText("state: dragging");
  await expect(stateText).toContainText("replay: 1");

  const specInput = controls
    .locator('[data-slot="color-thumb-workbench-spec-input"]')
    .first();
  await specInput.fill(
    '{"color":"#22c55e","x_percent":64.0,"y_percent":36.0,"is_focused":true,"is_dragging":false}',
  );
  await expect(specRoot).toHaveAttribute("data-state", "focused");
  await expect(specStateText).toContainText("spec: ok");

  await page.reload();
  await page.locator(WASM_READY_SELECTOR).waitFor();
  await expect(
    page.locator('[data-slot="color-thumb-workbench-controls"]').first(),
  ).toBeVisible();
});

test("docs-app color-thumb high-risk paths keep focus keyboard and disabled branches semantically explicit", async ({
  page,
}) => {
  await gotoColorThumb(page);
  await resolveIdleRoot(page);

  const focusedRoot = page
    .locator('[data-component="color-thumb"] #docs-color-thumb-focused[data-slot="color-thumb"]')
    .first();
  const draggingRoot = page
    .locator('[data-component="color-thumb"] #docs-color-thumb-dragging[data-slot="color-thumb"]')
    .first();
  const disabledRoot = page
    .locator('[data-component="color-thumb"] #docs-color-thumb-disabled[data-slot="color-thumb"]')
    .first();

  await focusedRoot.focus();
  await expect(focusedRoot).toBeFocused();
  await page.keyboard.press("ArrowRight");
  await expect(focusedRoot).toHaveAttribute("data-state", "focused");
  await expect(focusedRoot).toHaveAttribute("data-focused", "true");
  await expect(focusedRoot).toHaveAttribute("data-ui-action", "focus");

  await draggingRoot.dispatchEvent("pointerdown");
  await expect(draggingRoot).toHaveAttribute("data-state", "dragging");
  await expect(draggingRoot).toHaveAttribute("data-dragging", "true");
  await expect(draggingRoot).toHaveAttribute("data-ui-action", "drag");
  await draggingRoot.dispatchEvent("pointerup");
  await expect(draggingRoot).toHaveAttribute("data-ui-action", "drag");

  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("aria-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("tabindex", "-1");
  await expect(disabledRoot).toHaveAttribute("data-ui-output-status", "verified");
});
