import { expect, test } from "@playwright/test";

test("docs-app: accordion interaction + keyboard roving", async ({ page }) => {
  await page.goto("/#/components/accordion");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toBeVisible();

  const accordions = page.locator("[data-slot=\"accordion\"]");
  const multiAccordion = accordions.nth(0);
  const singleAccordion = accordions.nth(1);
  await expect(multiAccordion).toHaveAttribute("data-has-items", "true");
  await expect(singleAccordion).toHaveAttribute("data-has-items", "true");
  await expect(multiAccordion).toHaveAttribute(
    "data-ui-schema",
    "ui.accordion.agent-contract",
  );
  await expect(multiAccordion).toHaveAttribute("data-ui-schema-version", "1");
  await expect(multiAccordion).toHaveAttribute("data-ui-intent", "disclosure");
  await expect(multiAccordion).toHaveAttribute("data-ui-stream-support", "unsupported");
  await expect(multiAccordion).toHaveAttribute("data-ui-stream-fallback", "full-snapshot");
  await expect(multiAccordion).toHaveAttribute(
    "data-ui-output-status",
    /(draft|verified|submittable)/,
  );

  const openIndices = page.locator("span.ui-muted", { hasText: "open indices:" });

  const multiTrigger0 = multiAccordion.locator(
    "[data-slot=\"accordion-trigger\"][data-index=\"0\"]",
  );
  const multiTrigger1 = multiAccordion.locator(
    "[data-slot=\"accordion-trigger\"][data-index=\"1\"]",
  );
  const multiPanel1 = multiAccordion.locator(
    "[data-slot=\"accordion-panel\"][data-index=\"1\"]",
  );
  await expect(multiPanel1).toHaveAttribute("data-ui-fragment-kind", "accordion-panel");

  await multiTrigger0.scrollIntoViewIfNeeded();
  await expect(openIndices).toContainText("[0]");
  await expect(multiTrigger0).toHaveAttribute("aria-expanded", "true");
  await expect(multiTrigger1).toHaveAttribute("aria-expanded", "false");
  await expect(multiPanel1).toBeHidden();

  await multiTrigger1.click();
  await expect(openIndices).toContainText("[0, 1]");
  await expect(multiTrigger1).toHaveAttribute("aria-expanded", "true");
  await expect(multiTrigger1).toHaveAttribute("data-open", "true");
  await expect(multiPanel1).toHaveAttribute("data-open", "true");
  await expect(multiPanel1).toBeVisible();
  await expect(multiAccordion).toHaveAttribute(
    "data-ui-action",
    /(toggle-pointer|external-sync)/,
  );
  await expect(multiAccordion).toHaveAttribute("data-ui-output-status", /(draft|verified)/);

  await multiTrigger0.focus();
  await expect(multiTrigger0).toBeFocused();
  await page.keyboard.press("ArrowDown");
  await expect(multiTrigger1).toBeFocused();
  await page.keyboard.press("Space");

  await expect(openIndices).toContainText("[0]");
  await expect(multiTrigger1).toHaveAttribute("aria-expanded", "false");
  await expect(multiTrigger1).not.toHaveAttribute("data-open", "true");
  await expect(multiPanel1).not.toHaveAttribute("data-open", "true");
  await expect(multiPanel1).toBeHidden();
  await expect(multiAccordion).toHaveAttribute(
    "data-ui-action",
    /(toggle-keyboard|external-sync)/,
  );
  await expect(multiAccordion).toHaveAttribute("data-ui-output-status", /(draft|verified)/);

  const singleOpen = page.locator("span.ui-muted", { hasText: "single open:" });
  const singleTrigger0 = singleAccordion.locator(
    "[data-slot=\"accordion-trigger\"][data-index=\"0\"]",
  );
  const singleTrigger1 = singleAccordion.locator(
    "[data-slot=\"accordion-trigger\"][data-index=\"1\"]",
  );
  const singleTrigger2 = singleAccordion.locator(
    "[data-slot=\"accordion-trigger\"][data-index=\"2\"]",
  );

  await singleTrigger0.scrollIntoViewIfNeeded();
  await expect(singleOpen).toContainText("[1]");
  await expect(singleTrigger2).toBeDisabled();

  await singleTrigger0.click();
  await expect(singleOpen).toContainText("[0]");
  await expect(singleTrigger0).toHaveAttribute("aria-expanded", "true");
  await expect(singleTrigger1).toHaveAttribute("aria-expanded", "false");

  await singleTrigger1.focus();
  await expect(singleTrigger1).toBeFocused();
  await page.keyboard.press("ArrowDown");
  await expect(singleTrigger0).toBeFocused();
});
