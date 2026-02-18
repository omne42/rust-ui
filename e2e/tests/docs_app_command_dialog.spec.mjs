import { expect, test } from "@playwright/test";

test("docs-app command-dialog controlled playground closes on action", async ({ page }) => {
  await page.goto("/#/components/command-dialog");
  await page.locator("body:not(:has(#boot))").waitFor();

  await page.getByRole("button", { name: "Open CommandDialog" }).click();

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
  await expect(page.getByText("last action: calendar")).toBeVisible();
});

test("docs-app command-dialog marker playground stays open when close_on_action=false", async ({
  page,
}) => {
  await page.goto("/#/components/command-dialog");
  await page.locator("body:not(:has(#boot))").waitFor();

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
  await expect(markerDialog).toHaveAttribute("data-stream-fallback", "snapshot");
  await expect(page.getByText("last action: new-file")).toBeVisible();
});
