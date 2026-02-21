import { expect, test } from "@playwright/test";

const COACHMARK_PAGE = "/#/components/coachmark";
const WASM_READY_SELECTOR = "body:not(:has(#boot))";
const COACHMARK_CONTENT_SELECTOR =
  '[data-component="coachmark"] [data-slot="coachmark-content"]';
const COACHMARK_CONTROLLED_TOGGLE_SELECTOR = '[data-slot="coachmark-controlled-toggle"]';
const COACHMARK_CONTROLLED_CONTEXTUAL_HELP_SELECTOR =
  '[data-slot="contextual-help"][data-open-mode="controlled"]';
const COACHMARK_POPOVER_ROOT_OPEN_SELECTOR = '[data-slot="popover"][data-state="open"]';
const COACHMARK_POPOVER_PANEL_SELECTOR = '[data-slot="popover-panel"][data-state="panel"]';
const COACHMARK_DIALOG_PANEL_SELECTOR = '[data-slot="contextual-help-panel"][role="dialog"]';

async function gotoCoachmark(page) {
  await page.goto(COACHMARK_PAGE);
  await page.locator(WASM_READY_SELECTOR).waitFor();
}

async function waitForCoachmarkReady(page) {
  const readyRoot = page
    .locator(
      `${COACHMARK_CONTENT_SELECTOR}[data-ui-output-status="verified"][data-ui-stream-mode="snapshot"]`,
    )
    .first();
  await expect(readyRoot).toBeVisible();
  return readyRoot;
}

async function waitForCoachmarkSettled(root) {
  await expect(root).toHaveAttribute("data-ui-output-status", "verified");
  await expect(root).toHaveAttribute("data-output-status", "verified");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(root).toHaveAttribute("data-stream-fallback", "snapshot");
}

async function resolveOpenCoachmarkContent(page, mode = "uncontrolled") {
  const root = page
    .locator(
      `${COACHMARK_CONTENT_SELECTOR}[data-open-mode="${mode}"][data-ui-output-status="verified"]`,
    )
    .first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-ui-schema", "ui.coachmark.agent-contract.v1");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await waitForCoachmarkSettled(root);
  return root;
}

test("docs-app coachmark uses semantic selectors with wasm-stable ready waits", async ({ page }) => {
  await gotoCoachmark(page);
  await waitForCoachmarkReady(page);
  const content = await resolveOpenCoachmarkContent(page, "uncontrolled");

  await expect(content).toHaveAttribute("data-state", "enabled");
  await expect(content).toHaveAttribute("data-asset", "present");
  await expect(content).toHaveAttribute("data-asset-source", "variant");
  await expect(content).toHaveAttribute("data-cta", "dual");
  await expect(content).toHaveAttribute("data-steps", "present");
  await expect(content).toHaveAttribute("data-ui-intent", "guided-tour");
  await expect(content).toHaveAttribute("data-ui-action", "navigate-step");
  await expect(content).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app coachmark key flow keeps controlled markers stable", async ({ page }) => {
  await gotoCoachmark(page);
  await waitForCoachmarkReady(page);

  const toggle = page.locator(COACHMARK_CONTROLLED_TOGGLE_SELECTOR).first();
  await expect(toggle).toBeVisible();
  await toggle.click();

  const content = await resolveOpenCoachmarkContent(page, "controlled");
  await expect(content).toHaveAttribute("data-asset-source", "image");
  await expect(content).toHaveAttribute("data-actions", "present");
  await expect(content).toHaveAttribute("data-ui-source", "external");
  await expect(content).toHaveAttribute("data-stream-mode", "snapshot");
  await expect(content).toHaveAttribute("data-output-status", "verified");
});

test("docs-app coachmark key flow is repeatable with overlay focus and keyboard dismissal", async ({ page }) => {
  await gotoCoachmark(page);
  await waitForCoachmarkReady(page);

  const toggle = page.locator(COACHMARK_CONTROLLED_TOGGLE_SELECTOR).first();
  const controlledContextualHelp = page.locator(COACHMARK_CONTROLLED_CONTEXTUAL_HELP_SELECTOR).first();
  await toggle.click();

  await resolveOpenCoachmarkContent(page, "controlled");
  await expect(controlledContextualHelp).toHaveAttribute("data-open", "true");
  await expect(controlledContextualHelp).toHaveAttribute("data-open-interaction-source", "trigger-press");
  await expect(controlledContextualHelp).toHaveAttribute("data-ui-action", "toggle-open");

  const popoverRoot = page.locator(COACHMARK_POPOVER_ROOT_OPEN_SELECTOR).first();
  const popoverPanel = page.locator(COACHMARK_POPOVER_PANEL_SELECTOR).first();
  const dialogPanel = page.locator(COACHMARK_DIALOG_PANEL_SELECTOR).first();
  await expect(popoverRoot).toHaveAttribute("data-ui-overlay-portal", "");
  await expect(popoverPanel).toBeVisible();
  await expect(dialogPanel).toHaveAttribute("data-ui-action", "toggle-open");

  await popoverPanel.focus();
  await page.keyboard.press("Escape");

  await expect(controlledContextualHelp).toHaveAttribute("data-closed", "true");
  await expect(controlledContextualHelp).toHaveAttribute("data-open-interaction-source", "dismiss-press");
  await expect(controlledContextualHelp).toHaveAttribute("data-ui-action", "dismiss");
  await expect(page.locator(COACHMARK_POPOVER_ROOT_OPEN_SELECTOR)).toHaveCount(0);

  await toggle.focus();
  await page.keyboard.press("Enter");

  const reopenedContent = await resolveOpenCoachmarkContent(page, "controlled");
  await waitForCoachmarkSettled(reopenedContent);
  await expect(controlledContextualHelp).toHaveAttribute("data-open-interaction-source", "trigger-press");
  await expect(controlledContextualHelp).toHaveAttribute("data-ui-action", "toggle-open");
});

test("docs-app coachmark key flow is repeatable after reload", async ({ page }) => {
  await gotoCoachmark(page);
  await waitForCoachmarkReady(page);

  const toggle = page.locator(COACHMARK_CONTROLLED_TOGGLE_SELECTOR).first();
  await toggle.click();
  let content = await resolveOpenCoachmarkContent(page, "controlled");
  await expect(content).toHaveAttribute("data-ui-source", "external");

  await page.reload();
  await page.locator(WASM_READY_SELECTOR).waitFor();
  await waitForCoachmarkReady(page);

  content = await resolveOpenCoachmarkContent(page, "uncontrolled");
  await expect(content).toHaveAttribute("data-open-mode", "uncontrolled");
  await expect(content).toHaveAttribute("data-ui-source", "internal");
});
