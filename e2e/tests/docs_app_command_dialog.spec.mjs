import { expect, test } from "@playwright/test";

test("docs-app command-dialog controlled playground closes on action", async ({ page }) => {
  await page.goto("/#/components/command-dialog");
  const dialogs = page.locator('[data-slot="command-dialog"][data-ui-schema="command-dialog"]');
  await expect(dialogs.first()).toHaveAttribute("data-output-status", "verified");

  await page.getByRole("button", { name: "Open CommandDialog" }).focus();
  await page.keyboard.press("Enter");

  const controlledDialog = page
    .locator('[data-slot="command-dialog"][data-open-mode="controlled"]')
    .filter({ has: page.locator("#docs-command-dialog-controlled-command-option-0") })
    .first();

  await expect(controlledDialog).toBeVisible();
  await expect(controlledDialog).toHaveAttribute("data-state", "open");
  await expect(controlledDialog).toHaveAttribute("data-ui-schema", "command-dialog");
  await expect(controlledDialog).toHaveAttribute("data-stream-mode", "snapshot");
  await expect(controlledDialog).toHaveAttribute("data-output-status", "verified");

  await page.locator("#docs-command-dialog-controlled-command-option-0").click();

  await expect(
    page.locator('[data-slot="command-dialog"][data-open-mode="controlled"]'),
  ).toHaveCount(0);
  await expect(
    page.locator('[data-slot="command-dialog-last-action"][data-open-mode="controlled"]'),
  ).toHaveAttribute("data-last-action", "calendar");
});

test("docs-app command-dialog marker playground stays open when close_on_action=false", async ({
  page,
}) => {
  await page.goto("/#/components/command-dialog");
  const dialogs = page.locator('[data-slot="command-dialog"][data-ui-schema="command-dialog"]');
  await expect(dialogs.first()).toHaveAttribute("data-output-status", "verified");

  const markerDialog = page
    .locator('[data-slot="command-dialog"][data-open-mode="uncontrolled"]')
    .filter({ has: page.locator("#docs-command-dialog-marker-command-option-0") })
    .first();

  await expect(markerDialog).toBeVisible();
  await expect(markerDialog).toHaveAttribute("data-state", "open");
  await expect(markerDialog).toHaveAttribute("data-close-on-action", "false");
  await expect(markerDialog).toHaveAttribute("data-overlay-motion-source", "custom");
  await expect(markerDialog).toHaveAttribute("data-ui-action", "keep-open");

  await page.locator("#docs-command-dialog-marker-command-option-0").click();

  await expect(markerDialog).toHaveAttribute("data-state", "open");
  await expect(markerDialog).toHaveAttribute("data-output-status", "verified");
  await expect(markerDialog).toHaveAttribute("data-stream-fallback", "snapshot");
  await expect(
    page.locator('[data-slot="command-dialog-last-action"][data-open-mode="uncontrolled"]'),
  ).toHaveAttribute("data-last-action", "new-file");
});

test("docs-app command-dialog key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/command-dialog");
  const dialogs = page.locator('[data-slot="command-dialog"][data-ui-schema="command-dialog"]');
  await expect(dialogs.first()).toHaveAttribute("data-output-status", "verified");

  const openButton = page.getByRole("button", { name: "Open CommandDialog" });
  await openButton.focus();
  await expect(openButton).toBeFocused();
  await page.keyboard.press("Enter");

  const controlledDialog = page
    .locator('[data-slot="command-dialog"][data-open-mode="controlled"]')
    .filter({ has: page.locator("#docs-command-dialog-controlled-command-option-0") })
    .first();

  await expect(controlledDialog).toHaveAttribute("data-state", "open");
  await expect(controlledDialog).toHaveAttribute("data-ui-schema", "command-dialog");
  await expect(controlledDialog).toHaveAttribute("data-stream-mode", "snapshot");

  const controlledFirstOption = page.locator("#docs-command-dialog-controlled-command-option-0");
  await controlledFirstOption.focus();
  await expect(controlledFirstOption).toBeFocused();
  await page.keyboard.press("Enter");

  await expect(
    page.locator('[data-slot="command-dialog"][data-open-mode="controlled"]'),
  ).toHaveCount(0);
  await expect(
    page.locator('[data-slot="command-dialog-last-action"][data-open-mode="controlled"]'),
  ).toHaveAttribute("data-last-action", "calendar");

  await openButton.focus();
  await expect(openButton).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(controlledDialog).toHaveAttribute("data-state", "open");
  await page.keyboard.press("Escape");
  await expect(
    page.locator('[data-slot="command-dialog"][data-open-mode="controlled"]'),
  ).toHaveCount(0);

  await page.reload();

  const dialogsAfterReload = page.locator(
    '[data-slot="command-dialog"][data-ui-schema="command-dialog"]',
  );
  await expect(dialogsAfterReload.first()).toHaveAttribute("data-output-status", "verified");
  await expect(
    page.locator('[data-slot="command-dialog"][data-open-mode="uncontrolled"]')
      .filter({ has: page.locator("#docs-command-dialog-marker-command-option-0") })
      .first(),
  ).toHaveAttribute("data-state", "open");
});
