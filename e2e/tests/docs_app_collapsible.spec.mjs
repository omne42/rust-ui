import { expect, test } from "@playwright/test";

const COLLAPSIBLE_PAGE = '[data-component="collapsible"]';
const COLLAPSIBLE_ROOT = '[data-slot="collapsible"]';
const COLLAPSIBLE_TRIGGER = '[data-slot="collapsible-trigger"]';
const COLLAPSIBLE_PANEL = '[data-slot="collapsible-panel"]';

async function waitForWasmReady(page) {
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(COLLAPSIBLE_PAGE)).toBeVisible();
}

async function expectCollapsibleReady(root, state) {
  await expect(root).toHaveAttribute("data-state", state);
  await expect(root).toHaveAttribute(
    "data-open-change-source",
    /(initial|interaction|external-sync)/,
  );
}

async function expectCollapsibleSettledOpen(root, trigger, panel) {
  await expectCollapsibleReady(root, "open");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await expect(panel).toBeVisible();
}

async function expectCollapsibleSettledClosed(root, trigger, panel) {
  await expectCollapsibleReady(root, "closed");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(panel).toBeHidden();
}

test("docs-app: collapsible keeps headless interaction contracts stable", async ({
  page,
}) => {
  await page.goto("/#/components/collapsible");
  await waitForWasmReady(page);

  const controlledRoot = page
    .locator(
      `${COLLAPSIBLE_PAGE} ${COLLAPSIBLE_ROOT}[data-open-mode="controlled"]`,
    )
    .first();
  const controlledTrigger = controlledRoot.locator(COLLAPSIBLE_TRIGGER).first();
  const controlledPanel = controlledRoot.locator(COLLAPSIBLE_PANEL).first();

  await expect(controlledRoot).toHaveAttribute("data-open-mode", "controlled");
  await expectCollapsibleSettledOpen(
    controlledRoot,
    controlledTrigger,
    controlledPanel,
  );
  await expect(controlledRoot).toHaveAttribute("data-label-source", "title");
  await expect(controlledRoot).toHaveAttribute("data-class-source", "default");
  await expect(controlledRoot).toHaveAttribute("data-motion-source", "default");
  await expect(controlledRoot).toHaveAttribute("data-open-value-source", "external");
  await expect(controlledTrigger).toHaveAttribute("data-open-value-source", "external");
  await expect(controlledTrigger).toHaveAttribute(
    "data-open-change-source",
    /(initial|interaction|external-sync)/,
  );

  await controlledTrigger.click();
  await expectCollapsibleSettledClosed(
    controlledRoot,
    controlledTrigger,
    controlledPanel,
  );

  await controlledTrigger.focus();
  await expect(controlledTrigger).toBeFocused();
  await page.keyboard.press("Enter");
  await expectCollapsibleSettledOpen(
    controlledRoot,
    controlledTrigger,
    controlledPanel,
  );

  const disabledRoot = page
    .locator(`${COLLAPSIBLE_PAGE} ${COLLAPSIBLE_ROOT}[data-state="disabled"]`)
    .first();
  const disabledTrigger = disabledRoot.locator(COLLAPSIBLE_TRIGGER).first();
  const disabledPanel = disabledRoot.locator(COLLAPSIBLE_PANEL).first();

  await expect(disabledRoot).toHaveAttribute("data-open-mode", "uncontrolled");
  await expect(disabledRoot).toHaveAttribute("data-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-motion-source", "custom");
  await expect(disabledRoot).toHaveAttribute("data-custom-motion", "true");
  await expect(disabledRoot).toHaveAttribute("data-open-value-source", "primitive");
  await expect(disabledRoot).toHaveAttribute(
    "data-open-change-source",
    /(initial|interaction|external-sync)/,
  );
  await expect(disabledTrigger).toBeDisabled();
  await expect(disabledTrigger).toHaveAttribute("aria-expanded", "false");
  await expect(disabledPanel).toBeHidden();
});

test("docs-app: collapsible interactive playground updates props/state and replays deterministically", async ({
  page,
}) => {
  await page.goto("/#/components/collapsible");
  await waitForWasmReady(page);

  const interactivePlayground = page
    .locator("section.playground")
    .filter({
      has: page.getByRole("heading", {
        name: "Interactive Playground (Display + Config + Code + CSS Test)",
      }),
    })
    .first();
  await expect(interactivePlayground).toBeVisible();

  await interactivePlayground
    .locator('[data-slot="playground-toggle-settings"]')
    .first()
    .click();

  const controls = interactivePlayground
    .locator(
      '[data-slot="playground-controls"] [data-slot="collapsible-workbench-controls"]',
    )
    .first();
  await expect(controls).toBeVisible();

  const controlledMode = controls
    .locator('[data-slot="segmented-control-option"]', { hasText: "Controlled" })
    .first();
  await controlledMode.click();

  const preview = interactivePlayground
    .locator('[data-slot="collapsible-workbench-preview"]')
    .first();
  const root = preview.locator(COLLAPSIBLE_ROOT).first();
  const trigger = root.locator(COLLAPSIBLE_TRIGGER).first();
  const panel = root.locator(COLLAPSIBLE_PANEL).first();

  await expect(root).toHaveAttribute("data-open-mode", "controlled");
  await expectCollapsibleSettledOpen(root, trigger, panel);

  const controlledOpenSwitch = controls
    .locator('[data-slot="switch"]', { hasText: "Controlled open" })
    .first();
  await controlledOpenSwitch.click();
  await expectCollapsibleSettledClosed(root, trigger, panel);
  await expect(
    preview.locator('[data-slot="collapsible-workbench-controlled-state"]').first(),
  ).toContainText("controlled open: false");

  await controlledOpenSwitch.focus();
  await page.keyboard.press("Space");
  await expectCollapsibleSettledOpen(root, trigger, panel);
  await expect(
    preview.locator('[data-slot="collapsible-workbench-controlled-state"]').first(),
  ).toContainText("controlled open: true");

  const disabledSwitch = controls
    .locator('[data-slot="switch"]', { hasText: "Disabled" })
    .first();
  await disabledSwitch.click();
  await expect(root).toHaveAttribute("data-state", "disabled");
  await expect(trigger).toBeDisabled();
  await expect(panel).toBeHidden();

  await interactivePlayground
    .locator('[data-slot="playground-toggle-code"]')
    .first()
    .click();
  const codeBlock = interactivePlayground
    .locator('[data-slot="playground-code"] [data-slot="code-block-code"]')
    .first();
  await expect(codeBlock).toBeVisible();
  await expect(codeBlock).toContainText("is_disabled=true");

  await disabledSwitch.click();
  await expect(root).not.toHaveAttribute("data-state", "disabled");
  await expect(trigger).not.toBeDisabled();
  await expect(codeBlock).not.toContainText("is_disabled=true");

  await page.reload();
  await waitForWasmReady(page);
  await expect(interactivePlayground).toBeVisible();
});
