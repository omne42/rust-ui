import { expect, test } from "@playwright/test";

async function openHelpTextDocs(page) {
  await page.goto("/#/components/help-text");
  await page.locator("body:not(:has(#boot))").waitFor();
  const docsRoot = page.locator('[data-component="help-text"]').first();
  await expect(docsRoot).toBeVisible();
  return docsRoot;
}

function helpTextWorkbench(docsRoot) {
  const controls = docsRoot.locator('[data-slot="help-text-workbench-controls"]').first();
  const primaryRoot = docsRoot
    .locator(
      '[data-slot="help-text-workbench-canvas"] [data-slot="help-text-primary-card"] [data-slot="help-text"]',
    )
    .first();
  const toggleInvalid = controls
    .locator('[data-slot="help-text-toggle-invalid"] [data-slot="switch"]')
    .first();
  const toggleDisabled = controls
    .locator('[data-slot="help-text-toggle-disabled"] [data-slot="switch"]')
    .first();

  return { controls, primaryRoot, toggleInvalid, toggleDisabled };
}

async function runRepeatableStateCycle({ primaryRoot, toggleInvalid, toggleDisabled }) {
  await expect(primaryRoot).toHaveAttribute("data-state", "description");
  await expect(primaryRoot).toHaveAttribute("data-message-kind", "description");
  await expect(primaryRoot).toHaveAttribute("data-ui-action", "read-only");
  await expect(primaryRoot).toHaveAttribute("data-ui-output-status", "verified");

  await toggleInvalid.click();
  await expect(primaryRoot).toHaveAttribute("data-state", "error");
  await expect(primaryRoot).toHaveAttribute("data-message-kind", "error");
  await expect(primaryRoot).toHaveAttribute("data-ui-action", "announce-error");
  await expect(primaryRoot.locator('[data-slot="help-text-error"]').first()).toHaveAttribute(
    "aria-live",
    "assertive",
  );

  await toggleInvalid.focus();
  await expect(toggleInvalid).toBeFocused();
  await toggleInvalid.press("Enter");
  await expect(primaryRoot).toHaveAttribute("data-state", "description");
  await expect(primaryRoot).toHaveAttribute("data-message-kind", "description");
  await expect(primaryRoot).toHaveAttribute("data-ui-action", "read-only");

  await toggleDisabled.click();
  await expect(primaryRoot).toHaveAttribute("data-state", "disabled");
  await expect(primaryRoot).toHaveAttribute("aria-disabled", "true");

  await toggleInvalid.click();
  await expect(primaryRoot).toHaveAttribute("data-state", "error-disabled");
  await expect(primaryRoot).toHaveAttribute("data-message-kind", "error");
  await expect(primaryRoot).toHaveAttribute("aria-disabled", "true");
  await expect(primaryRoot).toHaveAttribute("aria-invalid", "true");

  await toggleInvalid.focus();
  await expect(toggleInvalid).toBeFocused();
  await toggleInvalid.press("Enter");
  await expect(primaryRoot).toHaveAttribute("data-state", "disabled");
  await expect(primaryRoot).not.toHaveAttribute("aria-invalid", "true");

  await toggleDisabled.focus();
  await expect(toggleDisabled).toBeFocused();
  await toggleDisabled.press("Enter");
  await expect(primaryRoot).toHaveAttribute("data-state", "description");
  await expect(primaryRoot).not.toHaveAttribute("aria-disabled", "true");
}

test("docs-app help-text uses semantic selectors with wasm-stable readiness waits", async ({
  page,
}) => {
  const docsRoot = await openHelpTextDocs(page);

  const descriptionRoot = docsRoot
    .locator(
      '[data-slot="help-text"][data-message-kind="description"][data-state="description"]',
    )
    .first();

  await expect(descriptionRoot).toBeVisible();
  await expect(descriptionRoot).toHaveAttribute("data-tone", "neutral");
  await expect(descriptionRoot).toHaveAttribute("data-aria-source", "default");
  await expect(descriptionRoot).toHaveAttribute("data-error-source", "none");
  await expect(descriptionRoot).toHaveAttribute("data-ui-stream-support", "optional");
  await expect(descriptionRoot).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(descriptionRoot).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(descriptionRoot).toHaveAttribute("data-ui-output-status", "verified");
  await expect(
    descriptionRoot.locator('[data-slot="help-text-description"]').first(),
  ).toBeVisible();
});

test("docs-app help-text error path keeps semantic settled markers stable", async ({ page }) => {
  const docsRoot = await openHelpTextDocs(page);

  const errorRoot = docsRoot
    .locator(
      '[data-slot="help-text"][data-message-kind="error"][data-state="error"][data-show-error-icon="true"]',
    )
    .first();

  await expect(errorRoot).toBeVisible();
  await expect(errorRoot).toHaveAttribute("data-ui-action", "announce-error");
  await expect(errorRoot).toHaveAttribute("data-ui-output-status", "verified");
  await expect(errorRoot).toHaveAttribute("data-error-source", "custom");
  await expect(errorRoot.locator('[data-slot="help-text-icon"]').first()).toBeVisible();

  const errorText = errorRoot.locator('[data-slot="help-text-error"]').first();
  await expect(errorText).toHaveAttribute("role", "alert");
  await expect(errorText).toHaveAttribute("aria-live", "assertive");
});

test("docs-app help-text key flow is repeatable with semantic contract breakpoints", async ({
  page,
}) => {
  let docsRoot = await openHelpTextDocs(page);
  await runRepeatableStateCycle(helpTextWorkbench(docsRoot));

  await page.goto("/#/components/badge");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toHaveText("Badge");

  docsRoot = await openHelpTextDocs(page);
  await runRepeatableStateCycle(helpTextWorkbench(docsRoot));
});
