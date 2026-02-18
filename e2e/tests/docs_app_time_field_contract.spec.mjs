import { expect, test } from "@playwright/test";

test("docs-app time-field exposes stable semantics and controlled clear flow", async ({ page }) => {
  await page.goto("/#/components/time-field");
  await page.locator("body:not(:has(#boot))").waitFor();

  const controlled = page.locator('[data-slot="time-field"]#docs-time-field-controlled');
  await expect(controlled).toBeVisible();
  await expect(controlled).toHaveAttribute("role", "group");
  await expect(controlled).toHaveAttribute("data-control-mode", "controlled");
  await expect(controlled).toHaveAttribute("data-value-source", "external");
  await expect(controlled).toHaveAttribute("data-default-value-source", "implicit");
  await expect(controlled).toHaveAttribute("data-value-change-source", "provided");
  await expect(controlled).toHaveAttribute("data-state", "value");
  await expect(controlled).toHaveAttribute("data-ui-schema", "ui.time-field.agent-contract");
  await expect(controlled).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(controlled).toHaveAttribute("data-ui-stream-fallback", "full-snapshot");
  await expect(controlled).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(controlled).toHaveAttribute("data-ui-output-status", "verified");

  const hourInput = controlled.locator('[data-slot="time-field-hour"]');
  const minuteInput = controlled.locator('[data-slot="time-field-minute"]');
  const clearButton = controlled.locator('[data-slot="time-field-clear"]');

  await expect(controlled).toHaveAttribute("aria-labelledby", "docs-time-field-controlled-label");
  await expect(hourInput).toHaveAttribute("aria-label", /.+/);
  await expect(minuteInput).toHaveAttribute("aria-label", /.+/);

  await expect(hourInput).toHaveValue("09");
  await expect(minuteInput).toHaveValue("30");
  await expect(clearButton).toHaveAttribute("data-visible", "true");

  // Pointer path
  await hourInput.fill("18");
  await expect(hourInput).toHaveValue("18");
  await expect(controlled).toHaveAttribute("data-ui-action", "edit-hour");
  await expect(controlled).toHaveAttribute("data-ui-source", "hour-input");
  await expect(controlled).toHaveAttribute("data-ui-output-status", "submittable");

  await clearButton.click();
  await expect(hourInput).toHaveValue("");
  await expect(minuteInput).toHaveValue("");
  await expect(controlled).toHaveAttribute("data-state", "empty");
  await expect(controlled).toHaveAttribute("data-ui-action", "clear");
  await expect(controlled).toHaveAttribute("data-ui-source", "clear-press");
  await expect(controlled).toHaveAttribute("data-ui-output-status", "submittable");
  await expect(clearButton).not.toHaveAttribute("data-visible", "true");

  // Keyboard path
  await hourInput.fill("07");
  await minuteInput.fill("15");
  await expect(controlled).toHaveAttribute("data-state", "value");
  await expect(controlled).toHaveAttribute("data-ui-action", "edit-minute");
  await expect(controlled).toHaveAttribute("data-ui-source", "minute-input");
  await expect(clearButton).toHaveAttribute("data-visible", "true");
  await clearButton.focus();
  await page.keyboard.press("Enter");
  await expect(hourInput).toHaveValue("");
  await expect(minuteInput).toHaveValue("");
  await expect(controlled).toHaveAttribute("data-state", "empty");
  await expect(controlled).toHaveAttribute("data-ui-action", "clear");
  await expect(controlled).toHaveAttribute("data-ui-source", "clear-press");

  const uncontrolled = page.locator('[data-slot="time-field"]#docs-time-field-strong');
  await expect(uncontrolled).toBeVisible();
  await expect(uncontrolled).toHaveAttribute("data-tone", "strong");
  await expect(uncontrolled).toHaveAttribute("data-control-mode", "uncontrolled");
  await expect(uncontrolled).toHaveAttribute("data-default-value-source", "provided");
  await expect(uncontrolled).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(uncontrolled).toHaveAttribute("data-ui-stream-fallback", "full-snapshot");
  await expect(uncontrolled).toHaveAttribute("data-ui-stream-mode", "snapshot");
});

test("docs-app time-field key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/time-field");
  await page.locator("body:not(:has(#boot))").waitFor();

  const controlled = page.locator('[data-slot="time-field"]#docs-time-field-controlled');
  const hourInput = controlled.locator('[data-slot="time-field-hour"]');
  const minuteInput = controlled.locator('[data-slot="time-field-minute"]');
  const clearButton = controlled.locator('[data-slot="time-field-clear"]');

  await hourInput.fill("11");
  await minuteInput.fill("45");
  await expect(controlled).toHaveAttribute("data-state", "value");
  await expect(controlled).toHaveAttribute("data-ui-action", "edit-minute");
  await expect(controlled).toHaveAttribute("data-ui-source", "minute-input");
  await expect(controlled).toHaveAttribute("data-ui-output-status", "submittable");

  await clearButton.click();
  await expect(controlled).toHaveAttribute("data-state", "empty");
  await expect(controlled).toHaveAttribute("data-ui-action", "clear");
  await expect(controlled).toHaveAttribute("data-ui-source", "clear-press");
  await expect(controlled).toHaveAttribute("data-ui-output-status", "submittable");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const controlledAfterReload = page.locator('[data-slot="time-field"]#docs-time-field-controlled');
  await expect(controlledAfterReload).toHaveAttribute("data-state", "value");
  await expect(controlledAfterReload).toHaveAttribute("data-ui-action", "initialize");
  await expect(controlledAfterReload).toHaveAttribute("data-ui-source", "init");
  await expect(controlledAfterReload).toHaveAttribute("data-ui-output-status", "verified");
  await expect(controlledAfterReload).toHaveAttribute("data-ui-stream-mode", "snapshot");
});
