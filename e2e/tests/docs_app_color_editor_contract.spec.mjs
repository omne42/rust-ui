import { expect, test } from "@playwright/test";

const COLOR_EDITOR_PAGE = "/#/components/color-editor";
const CONTROLLED_ROOT =
  '[data-component="color-editor"] #docs-color-editor-controlled[data-slot="color-editor"]';

async function gotoColorEditor(page) {
  await page.goto(COLOR_EDITOR_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

async function resolveControlledRoot(page) {
  const root = page.locator(CONTROLLED_ROOT).first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-ui-schema", "ui.color-editor.agent-contract.v1");
  await expect(root).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "full-snapshot");
  await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");
  return root;
}

test("docs-app color-editor contract uses semantic selectors with wasm-safe stable waits", async ({
  page,
}) => {
  await gotoColorEditor(page);
  const root = await resolveControlledRoot(page);

  const tabs = root.locator('[data-slot="color-editor-formats"]').first();
  const hexTab = root
    .locator('[data-slot="color-editor-format-button"][data-format="hex"]')
    .first();
  const channels = root.locator('[data-slot="color-editor-channels"]').first();

  await expect(tabs).toHaveAttribute("role", "tablist");
  await expect(hexTab).toHaveAttribute("role", "tab");
  await expect(hexTab).toHaveAttribute("aria-selected", "true");
  await expect(channels).toHaveAttribute("role", "tabpanel");

  await expect(root).toHaveAttribute("data-state", "ready");
  await expect(root).toHaveAttribute("data-format", "hex");
  await expect(root).toHaveAttribute("data-ui-selection-source", "external");
  await expect(root).toHaveAttribute("data-ui-format-source", "external");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app color-editor motion path uses ready/settled semantic breakpoints", async ({
  page,
}) => {
  await gotoColorEditor(page);
  const root = await resolveControlledRoot(page);

  const hslTab = root
    .locator('[data-slot="color-editor-format-button"][data-format="hsl"]')
    .first();
  await hslTab.focus();
  await expect(hslTab).toBeFocused();
  await hslTab.click();

  await expect(root).toHaveAttribute("data-format", "hsl");
  await expect(root).toHaveAttribute("data-ui-action", "format-change");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");

  const reducedMotionRoot = page
    .locator('[data-component="color-editor"] #docs-color-editor-disabled[data-slot="color-editor"]')
    .first();
  await expect(reducedMotionRoot).toBeVisible();
  await expect(reducedMotionRoot).toHaveAttribute("data-motion-source", "custom");
  await expect(reducedMotionRoot).toHaveAttribute("data-ui-output-status", "verified");
  await expect(reducedMotionRoot).toHaveAttribute("data-state", "disabled");
});

test("docs-app color-editor key flow is repeatable and failures map to semantic breakpoints", async ({
  page,
}) => {
  await gotoColorEditor(page);
  let root = await resolveControlledRoot(page);

  const hslTab = root
    .locator('[data-slot="color-editor-format-button"][data-format="hsl"]')
    .first();
  await hslTab.focus();
  await expect(hslTab).toBeFocused();
  await page.keyboard.press("ArrowRight");

  const hsbTab = root
    .locator('[data-slot="color-editor-format-button"][data-format="hsb"]')
    .first();
  await expect(hsbTab).toBeFocused();
  await expect(root).toHaveAttribute("data-format", "hsb");
  await expect(root).toHaveAttribute("data-ui-action", "format-change");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  root = await resolveControlledRoot(page);
  await expect(root).toHaveAttribute("data-format", "hex");
  await expect(root).toHaveAttribute("data-ui-action", "snapshot-render");
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app color-editor high-risk paths keep focus keyboard and disabled branches semantically explicit", async ({
  page,
}) => {
  await gotoColorEditor(page);
  const root = await resolveControlledRoot(page);

  const rgbTab = root
    .locator('[data-slot="color-editor-format-button"][data-format="rgb"]')
    .first();
  await rgbTab.focus();
  await expect(rgbTab).toBeFocused();
  await page.keyboard.press("ArrowLeft");

  const hexTab = root
    .locator('[data-slot="color-editor-format-button"][data-format="hex"]')
    .first();
  await expect(hexTab).toBeFocused();
  await expect(root).toHaveAttribute("data-format", "hex");
  await expect(root).toHaveAttribute("data-ui-action", "format-change");
  await expect(root).toHaveAttribute("data-ui-output-status", "submittable");

  const disabledRoot = page
    .locator('[data-component="color-editor"] #docs-color-editor-disabled[data-slot="color-editor"]')
    .first();
  await expect(disabledRoot).toBeVisible();
  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("data-ui-action", "snapshot-render");

  const disabledRgbTab = disabledRoot
    .locator('[data-slot="color-editor-format-button"][data-format="rgb"]')
    .first();
  await expect(disabledRgbTab).toHaveAttribute("aria-disabled", "true");
  await expect(disabledRgbTab).toBeDisabled();
});
