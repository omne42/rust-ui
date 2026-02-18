import { expect, test } from "@playwright/test";

test("docs-app dialog exposes stable role/source markers", async ({ page }) => {
  await page.goto("/#/components/dialog");
  await page.locator("body:not(:has(#boot))").waitFor();

  await page.getByRole("button", { name: "Open marker dialog" }).click();

  const overlayPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-marker-title"]',
  );
  const dialogRoot = overlayPanel.locator('[data-slot="dialog"]').first();

  await expect(overlayPanel).toBeVisible();
  await expect(overlayPanel).toHaveAttribute("aria-modal", "true");
  await expect(dialogRoot).toHaveAttribute("data-state", "with-description");
  await expect(dialogRoot).toHaveAttribute("data-size", "lg");
  await expect(dialogRoot).toHaveAttribute("data-id-source", "custom");
  await expect(dialogRoot).toHaveAttribute("data-title-source", "custom");
  await expect(dialogRoot).toHaveAttribute("data-description-source", "custom");
  await expect(dialogRoot).toHaveAttribute("data-close-source", "custom");
  await expect(dialogRoot).toHaveAttribute("data-motion-source", "custom");

  await expect(page.getByRole("button", { name: "Dismiss dialog" })).toBeVisible();
});

test("docs-app dialog closes via escape", async ({ page }) => {
  await page.goto("/#/components/dialog");
  await page.locator("body:not(:has(#boot))").waitFor();

  await page.getByRole("button", { name: "Open dialog" }).click();

  const overlayPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-title"]',
  );
  await expect(overlayPanel).toBeVisible();

  await overlayPanel.press("Escape");
  await expect(overlayPanel).toHaveCount(0);
});

test("docs-app dialog interactive + comparison playgrounds stay contract-stable", async ({ page }) => {
  await page.goto("/#/components/dialog");
  await page.locator("body:not(:has(#boot))").waitFor();

  const workbench = page.locator('[data-slot="dialog-workbench"]').first();
  await workbench.getByRole("button", { name: "Open workbench dialog" }).click();
  const workbenchPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-workbench-title"]',
  );
  const workbenchDialog = workbenchPanel.locator('[data-slot="dialog"]').first();
  await expect(workbenchDialog).toHaveAttribute("data-state", "with-description");
  await expect(workbenchDialog).toHaveAttribute("data-close-button", "shown");
  await workbenchPanel.getByRole("button", { name: "Cancel" }).click();
  await expect(workbenchPanel).toHaveCount(0);

  const comparison = page.locator('[data-slot="dialog-scenario-compare"]').first();

  await comparison.getByRole("button", { name: "Open default comparison" }).click();
  const defaultPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-compare-default-title"]',
  );
  const defaultDialog = defaultPanel.locator('[data-slot="dialog"]').first();
  await expect(defaultDialog).toHaveAttribute("data-state", "with-description");
  await expect(defaultDialog).toHaveAttribute("data-size", "md");
  await defaultPanel.getByRole("button", { name: "Close" }).click();
  await expect(defaultPanel).toHaveCount(0);

  await comparison.getByRole("button", { name: "Open compact comparison" }).click();
  const compactPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-compare-compact-title"]',
  );
  const compactDialog = compactPanel.locator('[data-slot="dialog"]').first();
  await expect(compactDialog).toHaveAttribute("data-state", "title-only");
  await expect(compactDialog).toHaveAttribute("data-close-button", "hidden");
  await compactPanel.getByRole("button", { name: "Dismiss" }).click();
  await expect(compactPanel).toHaveCount(0);

  await comparison.getByRole("button", { name: "Open motion comparison" }).click();
  const motionPanel = page.locator(
    '[data-slot="overlay-panel"][role="dialog"][aria-labelledby="docs-dialog-compare-motion-title"]',
  );
  const motionDialog = motionPanel.locator('[data-slot="dialog"]').first();
  await expect(motionDialog).toHaveAttribute("data-motion-source", "custom");
  await motionPanel.getByRole("button", { name: "Close" }).click();
  await expect(motionPanel).toHaveCount(0);
});
