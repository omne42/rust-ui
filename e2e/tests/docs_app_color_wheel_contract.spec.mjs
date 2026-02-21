import { expect, test } from "@playwright/test";

const COLOR_WHEEL_PAGE = "/#/components/color-wheel";
const CONTROLLED_ROOT =
  '[data-component="color-wheel"] #docs-color-wheel-hue[data-slot="color-wheel"][data-control-mode="controlled"]';

async function gotoColorWheel(page) {
  await page.goto(COLOR_WHEEL_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

async function resolveControlledRoot(page) {
  const root = page.locator(CONTROLLED_ROOT).first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-ui-schema", "ui.color-wheel.agent-contract.v1");
  await expect(root).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");
  return root;
}

test("docs-app color-wheel contract uses semantic selectors with wasm-safe stable waits", async ({
  page,
}) => {
  await gotoColorWheel(page);
  const root = await resolveControlledRoot(page);

  const input = root.locator('[data-slot="color-wheel-input"]').first();
  const label = root.locator('[data-slot="color-wheel-label"]').first();
  const track = root.locator('[data-slot="color-wheel-track"]').first();

  await expect(track).toBeVisible();
  await expect(input).toHaveAttribute("role", "slider");
  await expect(input).toHaveAttribute("aria-labelledby", "docs-color-wheel-hue-label");
  await expect(label).toHaveAttribute("id", "docs-color-wheel-hue-label");

  await expect(root).toHaveAttribute("data-state", "ready");
  await expect(root).toHaveAttribute("data-control-mode", "controlled");
  await expect(root).toHaveAttribute("data-value-source", "external");
  await expect(root).toHaveAttribute("data-ui-intent", "select-hue-angle");
  await expect(root).toHaveAttribute("data-ui-action", "idle");
  await expect(root).toHaveAttribute("data-ui-source", "on_value_change");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");
  await expect(root).toHaveAttribute("data-ui-state", "active");
});

test("docs-app color-wheel interaction path covers ready and settled semantic breakpoints", async ({
  page,
}) => {
  await gotoColorWheel(page);
  const root = await resolveControlledRoot(page);
  const input = root.locator('[data-slot="color-wheel-input"]').first();
  const track = root.locator('[data-slot="color-wheel-track"]').first();

  const before = Number((await root.getAttribute("data-value")) ?? "0");

  await input.focus();
  await expect(input).toBeFocused();
  await page.keyboard.press("ArrowRight");

  const after = Number((await root.getAttribute("data-value")) ?? "0");
  expect(after).toBeGreaterThan(before);
  await expect(root).toHaveAttribute("data-interaction-source", "keyboard");
  await expect(root).toHaveAttribute("data-ui-action", "keyboard");
  await expect(root).toHaveAttribute("data-ui-source", "on_value_change");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");

  await track.dispatchEvent("pointerdown", {
    bubbles: true,
    pointerId: 1,
    clientX: 12,
    clientY: 12,
  });
  await track.dispatchEvent("pointermove", {
    bubbles: true,
    pointerId: 1,
    clientX: 18,
    clientY: 14,
  });
  await track.dispatchEvent("pointerup", {
    bubbles: true,
    pointerId: 1,
    clientX: 18,
    clientY: 14,
  });
  await expect(root).toHaveAttribute("data-interaction-source", "pointer");
  await expect(root).toHaveAttribute("data-ui-action", "pointer");

  const disabledRoot = page
    .locator('[data-component="color-wheel"] #docs-color-wheel-disabled[data-slot="color-wheel"]')
    .first();
  const disabledInput = disabledRoot.locator('[data-slot="color-wheel-input"]').first();
  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("data-ui-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-ui-output-status", "submittable");
  await expect(disabledInput).toHaveAttribute("aria-disabled", "true");
  await expect(disabledInput).toBeDisabled();

  const customRoot = page
    .locator('[data-component="color-wheel"] #docs-color-wheel-custom[data-slot="color-wheel"]')
    .first();
  await expect(customRoot).toHaveAttribute("data-motion-source", "custom");
  await expect(customRoot).toHaveAttribute("data-ui-output-status", "verified");
  await expect(customRoot).toHaveAttribute("data-ui-source", "none");
});

test("docs-app color-wheel key flow is repeatable and failures map to semantic breakpoints", async ({
  page,
}) => {
  await gotoColorWheel(page);
  let root = await resolveControlledRoot(page);
  let input = root.locator('[data-slot="color-wheel-input"]').first();

  await input.focus();
  await expect(input).toBeFocused();
  const before = Number((await root.getAttribute("data-value")) ?? "0");

  await page.keyboard.press("ArrowRight");
  const after = Number((await root.getAttribute("data-value")) ?? "0");
  expect(after).toBeGreaterThan(before);
  await expect(root).toHaveAttribute("data-interaction-source", "keyboard");
  await expect(root).toHaveAttribute("data-ui-action", "keyboard");
  await expect(root).toHaveAttribute("data-ui-source", "on_value_change");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  root = await resolveControlledRoot(page);
  input = root.locator('[data-slot="color-wheel-input"]').first();
  await expect(root).toHaveAttribute("data-ui-action", "idle");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");

  await input.focus();
  await page.keyboard.press("ArrowRight");
  const replayed = Number((await root.getAttribute("data-value")) ?? "0");
  expect(replayed).toBeGreaterThan(before);
  await expect(root).toHaveAttribute("data-interaction-source", "keyboard");
  await expect(root).toHaveAttribute("data-ui-action", "keyboard");
});

test("docs-app color-wheel high-risk paths keep keyboard and disabled branches semantically explicit", async ({
  page,
}) => {
  await gotoColorWheel(page);
  const root = await resolveControlledRoot(page);
  const input = root.locator('[data-slot="color-wheel-input"]').first();

  await input.focus();
  await expect(input).toBeFocused();
  await page.keyboard.press("ArrowLeft");
  await expect(root).toHaveAttribute("data-interaction-source", "keyboard");
  await expect(root).toHaveAttribute("data-ui-action", "keyboard");
  await expect(root).toHaveAttribute("data-ui-source", "on_value_change");
  await expect(root).toHaveAttribute("data-ui-state", "active");

  const disabledRoot = page
    .locator('[data-component="color-wheel"] #docs-color-wheel-disabled[data-slot="color-wheel"]')
    .first();
  const disabledInput = disabledRoot.locator('[data-slot="color-wheel-input"]').first();
  await expect(disabledRoot).toBeVisible();
  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("data-ui-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-ui-output-status", "submittable");
  await expect(disabledInput).toHaveAttribute("aria-disabled", "true");
  await expect(disabledInput).toBeDisabled();
});

test("docs-app color-wheel interactive playground updates props and preview with semantic markers", async ({
  page,
}) => {
  await gotoColorWheel(page);

  const controls = page.locator('[data-slot="color-wheel-workbench-controls"]').first();
  const workbench = page
    .locator('[data-component="color-wheel"] #docs-color-wheel-workbench[data-slot="color-wheel"]')
    .first();
  const state = page.locator('[data-slot="color-wheel-workbench-state"]').first();
  const input = workbench.locator('[data-slot="color-wheel-input"]').first();

  await expect(controls).toBeVisible();
  await expect(workbench).toBeVisible();
  await expect(state).toContainText("preset:");
  await expect(state).toContainText("value:");
  await expect(workbench).toHaveAttribute("data-state", "ready");

  const before = Number((await workbench.getAttribute("data-value")) ?? "0");
  await input.focus();
  await page.keyboard.press("ArrowRight");
  const after = Number((await workbench.getAttribute("data-value")) ?? "0");
  expect(after).toBeGreaterThan(before);
  await expect(workbench).toHaveAttribute("data-interaction-source", "keyboard");
  await expect(workbench).toHaveAttribute("data-ui-action", "keyboard");

  const disabledToggle = controls.getByRole("checkbox", { name: "Disabled" });
  await disabledToggle.check();
  await expect(workbench).toHaveAttribute("data-state", "disabled");
  await expect(workbench).toHaveAttribute("data-ui-state", "disabled");

  await disabledToggle.uncheck();
  await expect(workbench).toHaveAttribute("data-state", "ready");
  await expect(workbench).toHaveAttribute("data-ui-state", "active");

  const customClassToggle = controls.getByRole("checkbox", { name: "Custom class" });
  await customClassToggle.check();
  await expect(workbench).toHaveAttribute("data-class-source", "custom");

  const reducedMotionToggle = controls.getByRole("checkbox", { name: "Reduced motion" });
  await reducedMotionToggle.check();
  await expect(workbench).toHaveAttribute("data-motion-source", "custom");
});
