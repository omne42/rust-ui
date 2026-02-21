import { expect, test } from "@playwright/test";

async function gotoAlertDocsAndWaitSettled(page) {
  await page.goto("/#/components/alert");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="alert"]').first();
  await expect(docsRoot).toBeVisible();

  const settledAlert = docsRoot
    .locator(
      '[data-slot="alert"][data-ui-state="snapshot"][data-ui-streaming="optional"][data-ui-fallback="snapshot"][data-ui-output-status="verified"]'
    )
    .first();
  await expect(settledAlert).toBeVisible();

  return docsRoot;
}

test("docs-app alert uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const docsRoot = await gotoAlertDocsAndWaitSettled(page);

  const alert = docsRoot
    .locator(
      '[data-slot="alert"][data-ui-schema="alert.v1"][data-ui-intent="status-region"][data-ui-action="announce"]'
    )
    .first();

  await expect(alert).toBeVisible();
  await expect(alert).toHaveAttribute("data-ui-state", "snapshot");
  await expect(alert).toHaveAttribute("data-ui-source", "default");
  await expect(alert).toHaveAttribute("data-ui-streaming", "optional");
  await expect(alert).toHaveAttribute("data-ui-fallback", "snapshot");
  await expect(alert).toHaveAttribute("data-ui-output-status", "verified");
  await expect(alert).toHaveAttribute("data-variant-source", "default");
  await expect(alert).toHaveAttribute("data-motion-source", "default");
  await expect(alert).toHaveAttribute("role", /(status|alert)/);
  await expect(alert).toHaveAttribute("aria-live", /(polite|assertive)/);
});

test("docs-app alert flow is repeatable via semantic breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoAlertDocsAndWaitSettled(page);

  const matrixNegative = docsRoot
    .locator('[data-slot="alert"][data-tone="negative"][data-fill="subtle"][data-layout="banner"]')
    .first();
  await expect(matrixNegative).toBeVisible();
  await expect(matrixNegative).toHaveAttribute("data-title", "present");
  await expect(matrixNegative).toHaveAttribute("data-description", "present");
  await expect(matrixNegative).toHaveAttribute("data-icon", "visible");

  const hiddenIconCustom = docsRoot
    .locator(
      '[data-slot="alert"][data-tone="notice"][data-fill="bold"][data-icon="hidden"][data-custom-class="true"]'
    )
    .first();
  await expect(hiddenIconCustom).toBeVisible();
  await expect(hiddenIconCustom).toHaveAttribute("data-hide-icon", "true");
  await expect(hiddenIconCustom).toHaveAttribute("data-ui-output-status", "verified");

  const inlineSnapshot = docsRoot
    .locator('[data-slot="alert"][data-layout="inline"][data-tone="info"][data-ui-state="snapshot"]')
    .first();
  await expect(inlineSnapshot).toBeVisible();
  await expect(inlineSnapshot).toHaveAttribute("data-ui-streaming", "optional");
  await expect(inlineSnapshot).toHaveAttribute("data-ui-fallback", "snapshot");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(
    page
      .locator('[data-component="alert"] [data-slot="alert"][data-layout="inline"][data-tone="info"]')
      .first()
  ).toBeVisible();
});

test("docs-app alert key flow is repeatable with focus+keyboard semantic breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoAlertDocsAndWaitSettled(page);

  const interactiveAlert = docsRoot
    .locator('[data-slot="alert"][data-custom-class="true"][data-ui-output-status="verified"]')
    .first();
  await expect(interactiveAlert).toBeVisible();
  await expect(interactiveAlert).toHaveAttribute("data-icon", "hidden");
  await expect(interactiveAlert).toHaveAttribute("data-ui-state", "snapshot");
  await expect(interactiveAlert).toHaveAttribute("data-ui-streaming", "optional");

  const actionButton = interactiveAlert.locator('[data-slot="button"]').first();
  await expect(actionButton).toBeVisible();
  await actionButton.focus();
  await expect(actionButton).toBeFocused();

  await page.keyboard.press("Enter");
  await expect(interactiveAlert).toHaveAttribute("data-ui-output-status", "verified");
  await expect(interactiveAlert).toHaveAttribute("data-hide-icon", "true");
  await expect(interactiveAlert).toHaveAttribute("data-hide-icon-source", /.+/);

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedInteractiveAlert = page
    .locator(
      '[data-component="alert"] [data-slot="alert"][data-custom-class="true"][data-ui-output-status="verified"]'
    )
    .first();
  await expect(reloadedInteractiveAlert).toBeVisible();
  const reloadedButton = reloadedInteractiveAlert.locator('[data-slot="button"]').first();
  await reloadedButton.focus();
  await expect(reloadedButton).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(reloadedInteractiveAlert).toHaveAttribute("data-ui-output-status", "verified");
});
