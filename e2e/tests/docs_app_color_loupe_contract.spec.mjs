import { expect, test } from "@playwright/test";

const COLOR_LOUPE_PAGE = "/#/components/color-loupe";

async function gotoColorLoupe(page) {
  await page.goto(COLOR_LOUPE_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

test("docs-app color-loupe uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  await gotoColorLoupe(page);

  const component = page.locator('[data-component="color-loupe"]').first();
  await expect(component).toBeVisible();

  const openRoot = component
    .locator('#docs-color-loupe-matrix-open[data-slot="color-loupe"]')
    .first();
  await expect(openRoot).toBeVisible();
  await expect(openRoot).toHaveAttribute("role", "img");
  await expect(openRoot).toHaveAttribute("data-state", "open");
  await expect(openRoot).toHaveAttribute("data-open", "true");
  await expect(openRoot).toHaveAttribute("data-x-bucket", "start");
  await expect(openRoot).toHaveAttribute("data-y-bucket", "end");
  await expect(openRoot).toHaveAttribute("data-aria-source", "default");
  await expect(openRoot).toHaveAttribute("data-class-source", "default");

  const disabledRoot = component
    .locator('#docs-color-loupe-matrix-disabled[data-slot="color-loupe"]')
    .first();
  await expect(disabledRoot).toBeVisible();
  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");

  const outputMode = component.locator('[data-slot="color-loupe-output-mode"]').first();
  await expect(outputMode).toHaveAttribute("data-ui-streaming", "optional");
  await expect(outputMode).toHaveAttribute("data-ui-fallback", "snapshot");
  await expect(outputMode).toHaveAttribute("data-ui-output-state", "snapshot");
  await expect(
    outputMode.locator('#docs-color-loupe-snapshot-draft[data-slot="color-loupe"]').first()
  ).toHaveAttribute("data-output-state", "draft");
  await expect(
    outputMode.locator('#docs-color-loupe-snapshot-verified[data-slot="color-loupe"]').first()
  ).toHaveAttribute("data-output-state", "verified");
  await expect(
    outputMode
      .locator('#docs-color-loupe-snapshot-committable[data-slot="color-loupe"]')
      .first()
  ).toHaveAttribute("data-output-state", "committable");
});

test("docs-app color-loupe workbench flow uses semantic ready/settled breakpoints", async ({
  page,
}) => {
  await gotoColorLoupe(page);

  const playground = page
    .locator('[data-component="color-loupe"] section.playground')
    .filter({ has: page.locator("#docs-color-loupe-workbench-main") })
    .first();
  await expect(playground).toBeVisible();

  const settingsToggle = playground
    .getByRole("button", { name: /Show settings|Hide settings/ })
    .first();
  const controlsHost = playground.locator('[data-slot="playground-controls"]').first();
  if ((await controlsHost.count()) === 0) {
    await settingsToggle.click();
  }
  await expect(controlsHost).toBeVisible();

  const controls = controlsHost.locator('[data-slot="color-loupe-workbench-controls"]').first();
  await expect(controls).toBeVisible();

  const main = playground.locator('#docs-color-loupe-workbench-main[data-slot="color-loupe"]').first();
  await expect(main).toHaveAttribute("data-state", "open");
  await expect(main).toHaveAttribute("data-open", "true");
  await expect(main).toHaveAttribute("data-x-bucket", "center");
  await expect(main).toHaveAttribute("data-y-bucket", "center");

  const positionControl = controls
    .locator('[data-slot="color-loupe-workbench-position"] [data-slot="segmented-control"]')
    .first();
  await positionControl
    .locator('[data-slot="segmented-control-option"][data-index="2"]')
    .click();
  await expect(main).toHaveAttribute("data-x-bucket", "end");
  await expect(main).toHaveAttribute("data-y-bucket", "start");

  const customAriaSwitch = controls
    .locator('[data-slot="color-loupe-workbench-custom-aria"] [data-slot="switch"]')
    .first();
  const customClassSwitch = controls
    .locator('[data-slot="color-loupe-workbench-custom-class"] [data-slot="switch"]')
    .first();
  await customAriaSwitch.click();
  await customClassSwitch.click();
  await expect(main).toHaveAttribute("data-aria-source", "custom");
  await expect(main).toHaveAttribute("data-class-source", "custom");
  await expect(main).toHaveAttribute("data-custom-class", "true");

  const disabledSwitch = controls
    .locator('[data-slot="color-loupe-workbench-disabled"] [data-slot="switch"]')
    .first();
  await disabledSwitch.click();
  await expect(main).toHaveAttribute("data-state", "disabled");
  await expect(main).toHaveAttribute("data-disabled", "true");

  const openSwitch = controls
    .locator('[data-slot="color-loupe-workbench-open"] [data-slot="switch"]')
    .first();
  await openSwitch.click();
  await expect(main).toHaveAttribute("data-state", "disabled");

  await disabledSwitch.click();
  await expect(main).not.toHaveAttribute("data-disabled", "true");
  await expect(main).toHaveAttribute("data-state", "color");
  await expect(main).not.toHaveAttribute("data-open", "true");

  await openSwitch.click();
  await expect(main).toHaveAttribute("data-state", "open");
  await expect(main).toHaveAttribute("data-open", "true");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  const reloadedMain = page
    .locator('[data-component="color-loupe"] #docs-color-loupe-workbench-main[data-slot="color-loupe"]')
    .first();
  await expect(reloadedMain).toHaveAttribute("data-state", "open");
  await expect(reloadedMain).toHaveAttribute("data-open", "true");
});
