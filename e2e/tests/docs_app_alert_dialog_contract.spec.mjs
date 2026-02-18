import { expect, test } from "@playwright/test";

test("docs-app alert-dialog exposes stable role/source markers", async ({ page }) => {
  await page.goto("/#/components/alert-dialog");
  await page.locator("body:not(:has(#boot))").waitFor();

  await page.getByRole("button", { name: "Open marker alert" }).click();

  const overlayPanel = page.locator('[data-slot="overlay-panel"][role="alertdialog"]').first();
  const alertDialog = page.locator('[data-slot="alert-dialog"]').first();

  await expect(overlayPanel).toBeVisible();
  await expect(overlayPanel).toHaveAttribute("aria-modal", "true");
  await expect(alertDialog).toHaveAttribute("data-state", "open");
  await expect(alertDialog).toHaveAttribute("data-id-source", "custom");
  await expect(alertDialog).toHaveAttribute("data-title-source", "custom");
  await expect(alertDialog).toHaveAttribute("data-description-source", "custom");
  await expect(alertDialog).toHaveAttribute("data-cancel-source", "custom");
  await expect(alertDialog).toHaveAttribute("data-secondary-source", "custom");
  await expect(alertDialog).toHaveAttribute("data-motion-source", "custom");
  await expect(alertDialog).toHaveAttribute("data-auto-focus", "secondary");

  const secondaryButton = page.getByRole("button", { name: "Save draft" });
  await expect(secondaryButton).toBeFocused();
});

test("docs-app alert-dialog closes via escape", async ({ page }) => {
  await page.goto("/#/components/alert-dialog");
  await page.locator("body:not(:has(#boot))").waitFor();

  await page.getByRole("button", { name: "Open destructive" }).click();

  const overlayPanel = page.locator('[data-slot="overlay-panel"][role="alertdialog"]');
  await expect(overlayPanel.first()).toBeVisible();

  await page.keyboard.press("Escape");
  await expect(overlayPanel).toHaveCount(0);
});
