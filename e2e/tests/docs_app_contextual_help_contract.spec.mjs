import { expect, test } from "@playwright/test";

async function openContextualHelpDocs(page) {
  await page.goto("/#/components/contextual-help");
  await page.locator("body:not(:has(#boot))").waitFor();
  const docsRoot = page.locator('[data-component="contextual-help"]').first();
  await expect(docsRoot).toBeVisible();
  return docsRoot;
}

test("docs-app contextual-help controlled flow keeps headless a11y contracts stable", async ({
  page,
}) => {
  const docsRoot = await openContextualHelpDocs(page);
  const controlledRoot = docsRoot
    .locator(
      '[data-slot="contextual-help"][data-open-mode="controlled"][data-class-source="custom"]',
    )
    .first();
  const trigger = controlledRoot.locator('button[aria-haspopup="dialog"]').first();

  await expect(controlledRoot).toHaveAttribute("data-open-source", "custom");
  await expect(controlledRoot).toHaveAttribute("data-open-change-source", "provided");
  await expect(controlledRoot).toHaveAttribute("data-closed", "true");
  await expect(trigger).toHaveAttribute("aria-haspopup", "dialog");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");

  await trigger.click();

  const panel = page
    .locator('[data-slot="contextual-help-panel"][data-open-mode="controlled"]')
    .first();
  await expect(controlledRoot).toHaveAttribute("data-open", "true");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");
  await expect(panel).toBeVisible();
  await expect(panel).toHaveAttribute("data-open", "true");
  await expect(panel).toHaveAttribute("role", "dialog");
  await expect(panel).toHaveAttribute("aria-modal", "false");

  await page.keyboard.press("Escape");
  await expect(trigger).toHaveAttribute("aria-expanded", "false");
  await expect(controlledRoot).toHaveAttribute("data-closed", "true");
  await expect(
    page.locator('[data-slot="contextual-help-panel"][data-open-mode="controlled"]'),
  ).toHaveCount(0, { timeout: 6000 });
});

test("docs-app contextual-help key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  const docsRoot = await openContextualHelpDocs(page);
  const controlledRoot = docsRoot
    .locator(
      '[data-slot="contextual-help"][data-open-mode="controlled"][data-class-source="custom"]',
    )
    .first();
  const trigger = controlledRoot.locator('button[aria-haspopup="dialog"]').first();
  const controlledPanels = page.locator(
    '[data-slot="contextual-help-panel"][data-open-mode="controlled"]',
  );

  await expect(controlledRoot).toHaveAttribute("data-closed", "true");

  await trigger.click();
  await expect(controlledRoot).toHaveAttribute("data-open", "true");
  await expect(controlledPanels.first()).toHaveAttribute("role", "dialog");
  await expect(controlledPanels.first()).toHaveAttribute("data-open", "true");

  await page.keyboard.press("Escape");
  await expect(controlledRoot).toHaveAttribute("data-closed", "true");
  await expect(controlledPanels).toHaveCount(0, { timeout: 6000 });
  await expect(trigger).toBeFocused();

  await trigger.click();
  await expect(controlledRoot).toHaveAttribute("data-open", "true");
  await expect(controlledPanels.first()).toHaveAttribute("data-open", "true");
  await expect(trigger).toHaveAttribute("aria-expanded", "true");

  await page.keyboard.press("Escape");
  await expect(controlledRoot).toHaveAttribute("data-closed", "true");
  await expect(controlledPanels).toHaveCount(0, { timeout: 6000 });
});

test("docs-app contextual-help keeps disabled trigger non-interactive", async ({ page }) => {
  const docsRoot = await openContextualHelpDocs(page);
  const disabledRoot = docsRoot
    .locator('[data-slot="contextual-help"][data-state="disabled"]')
    .first();
  const disabledTrigger = disabledRoot.locator('button[aria-haspopup="dialog"]').first();

  await expect(disabledRoot).toHaveAttribute("data-open-mode", "uncontrolled");
  await expect(disabledRoot).toHaveAttribute("data-open-source", "default");
  await expect(disabledTrigger).toBeDisabled();
  await expect(disabledTrigger).toHaveAttribute("aria-expanded", "false");

  await disabledTrigger.click({ force: true });
  await expect(disabledRoot).toHaveAttribute("data-closed", "true");
  await expect(
    page.locator('[data-slot="contextual-help-panel"][data-state="disabled"]'),
  ).toHaveCount(0);
});
