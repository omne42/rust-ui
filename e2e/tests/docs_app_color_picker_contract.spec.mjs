import { expect, test } from "@playwright/test";

const COLOR_PICKER_PAGE = "/#/components/color-picker";
const CONTROLLED_ROOT =
  '[data-component="color-picker"] #docs-color-picker-basic[data-slot="color-picker"][data-open-mode="controlled"]';

async function gotoColorPicker(page) {
  await page.goto(COLOR_PICKER_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

async function resolveControlledRoot(page) {
  const root = page.locator(CONTROLLED_ROOT).first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-ui-schema", "ui.color-picker.agent-contract");
  await expect(root).toHaveAttribute("data-ui-schema-version", "v1");
  await expect(root).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");
  return root;
}

test("docs-app color-picker contract uses semantic selectors with wasm-stable waits", async ({
  page,
}) => {
  await gotoColorPicker(page);
  const root = await resolveControlledRoot(page);

  const trigger = root.locator('[data-slot="color-picker-trigger"]').first();
  const label = root.locator('[data-slot="color-picker-label"]').first();

  await expect(trigger).toHaveAttribute("role", "button");
  await expect(trigger).toHaveAttribute("aria-haspopup", "dialog");
  await expect(label).toBeVisible();

  await expect(root).toHaveAttribute("data-open-mode", "controlled");
  await expect(root).toHaveAttribute("data-state", /(selected|open)/);
  await expect(root).toHaveAttribute("data-ui-intent", "color.selection");
  await expect(root).toHaveAttribute("data-ui-selection-source", "controlled");
  await expect(root).toHaveAttribute("data-ui-open-source", "controlled");
  await expect(root).toHaveAttribute("data-ui-output-status", /(verified|submittable)/);
});

test("docs-app color-picker interaction path covers ready/settled semantic breakpoints", async ({
  page,
}) => {
  await gotoColorPicker(page);
  const root = await resolveControlledRoot(page);
  const trigger = root.locator('[data-slot="color-picker-trigger"]').first();

  await trigger.focus();
  await expect(trigger).toBeFocused();
  await trigger.click();

  const panel = root.locator('[data-slot="color-picker-panel"][role="dialog"]').first();
  await expect(panel).toBeVisible();
  await expect(root).toHaveAttribute("data-open", "true");
  await expect(root).toHaveAttribute("data-state", "open");
  await expect(root).toHaveAttribute("data-ui-action", "toggle-open");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");

  await trigger.click();

  await expect(root.locator('[data-slot="color-picker-panel"][role="dialog"]')).toHaveCount(0);
  await expect(root).not.toHaveAttribute("data-open", "true");
  await expect(root).toHaveAttribute("data-ui-action", "toggle-close");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");

  const disabledRoot = page
    .locator('[data-component="color-picker"] #docs-color-picker-disabled[data-slot="color-picker"]')
    .first();
  await expect(disabledRoot).toBeVisible();
  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("data-ui-output-status", "verified");

  const disabledTrigger = disabledRoot.locator('[data-slot="color-picker-trigger"]').first();
  await expect(disabledTrigger).toHaveAttribute("aria-disabled", "true");
  await expect(disabledTrigger).toBeDisabled();
});

test("docs-app color-picker key flow is repeatable and failures map to semantic breakpoints", async ({
  page,
}) => {
  await gotoColorPicker(page);
  let root = await resolveControlledRoot(page);
  let trigger = root.locator('[data-slot="color-picker-trigger"]').first();

  await trigger.focus();
  await expect(trigger).toBeFocused();
  await page.keyboard.press("Enter");

  await expect(root).toHaveAttribute("data-open", "true");
  await expect(root).toHaveAttribute("data-state", "open");
  await expect(root).toHaveAttribute("data-ui-action", "toggle-open");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");

  await page.keyboard.press("Enter");

  await expect(root.locator('[data-slot="color-picker-panel"][role="dialog"]')).toHaveCount(0);
  await expect(root).not.toHaveAttribute("data-open", "true");
  await expect(root).toHaveAttribute("data-ui-action", "toggle-close");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  root = await resolveControlledRoot(page);
  trigger = root.locator('[data-slot="color-picker-trigger"]').first();
  await trigger.focus();
  await expect(trigger).toBeFocused();
  await expect(root).toHaveAttribute("data-ui-action", "snapshot-render");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app color-picker high-risk paths keep overlay focus keyboard and async boundaries semantically explicit", async ({
  page,
}) => {
  await gotoColorPicker(page);
  const root = await resolveControlledRoot(page);
  const trigger = root.locator('[data-slot="color-picker-trigger"]').first();

  await trigger.focus();
  await expect(trigger).toBeFocused();
  await page.keyboard.press("Space");

  const panel = root.locator('[data-slot="color-picker-panel"][role="dialog"]').first();
  await expect(panel).toBeVisible();
  await expect(root).toHaveAttribute("data-open", "true");
  await expect(root).toHaveAttribute("data-ui-action", "toggle-open");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");
  await expect(root).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(root).not.toHaveAttribute("aria-busy", "true");

  await page.keyboard.press("Escape");

  await expect(root.locator('[data-slot="color-picker-panel"][role="dialog"]')).toHaveCount(0);
  await expect(root).not.toHaveAttribute("data-open", "true");
  await expect(root).toHaveAttribute("data-ui-action", "toggle-close");

  const disabledRoot = page
    .locator('[data-component="color-picker"] #docs-color-picker-disabled[data-slot="color-picker"]')
    .first();
  await expect(disabledRoot).toBeVisible();
  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("data-ui-output-status", "verified");

  const disabledTrigger = disabledRoot.locator('[data-slot="color-picker-trigger"]').first();
  await expect(disabledTrigger).toHaveAttribute("aria-disabled", "true");
  await expect(disabledTrigger).toBeDisabled();
});
