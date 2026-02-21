import { expect, test } from "@playwright/test";

test("docs-app error-view exposes semantic state/source markers", async ({ page }) => {
  await page.goto("/#/components/error-view");
  await page.locator("body:not(:has(#boot))").waitFor();

  const visible = page.locator(
    '[data-slot="error-view"][data-state="visible"][data-tone="negative"]',
  );
  await expect(visible).toHaveCount(1);
  await expect(visible).toHaveAttribute("data-state", "visible");
  await expect(visible).toHaveAttribute("data-invalid", "true");
  await expect(visible).toHaveAttribute("data-message-source", "custom");
  await expect(visible).toHaveAttribute("data-tone", "negative");

  const hidden = page.locator(
    '[data-slot="error-view"][data-state="hidden"][data-hidden="true"]',
  );
  await expect(hidden).toHaveCount(1);
  await expect(hidden).toHaveAttribute("data-state", "hidden");
  await expect(hidden).toHaveAttribute("data-hidden", "true");
  await expect(hidden).toHaveAttribute("aria-hidden", "true");

  const custom = page.locator(
    '[data-slot="error-view"][data-motion-source="custom"][data-actions="true"]',
  );
  await expect(custom).toHaveCount(1);
  await expect(custom).toHaveAttribute("data-state", "visible");
  await expect(custom).toHaveAttribute("data-tone", "neutral");
  await expect(custom).toHaveAttribute("data-compact", "true");
  await expect(custom).toHaveAttribute("data-compact-source", "is-prop");
  await expect(custom).toHaveAttribute("data-bordered", "true");
  await expect(custom).toHaveAttribute("data-bordered-source", "is-prop");
  await expect(custom).toHaveAttribute("data-actions", "true");
  await expect(custom).toHaveAttribute("data-content", "children");
  await expect(custom).toHaveAttribute("data-motion-source", "custom");

  const retryButton = custom.getByRole("button", { name: "Retry" });
  await expect(retryButton).toBeVisible();
  await retryButton.focus();
  await expect(retryButton).toBeFocused();
});

test("docs-app error-view semantics remain stable after reload", async ({ page }) => {
  await page.goto("/#/components/error-view");
  await page.locator("body:not(:has(#boot))").waitFor();

  const custom = page.locator(
    '[data-slot="error-view"][data-motion-source="custom"][data-actions="true"]',
  );
  await expect(custom).toHaveAttribute("data-tone", "neutral");
  await expect(custom).toHaveAttribute("data-state", "visible");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const afterReload = page.locator(
    '[data-slot="error-view"][data-motion-source="custom"][data-actions="true"]',
  );
  await expect(afterReload).toHaveAttribute("data-tone", "neutral");
  await expect(afterReload).toHaveAttribute("data-state", "visible");
});

test("docs-app error-view motion path uses semantic ready/settled breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/error-view");
  await page.locator("body:not(:has(#boot))").waitFor();

  const hiddenSettled = page.locator(
    '[data-slot="error-view"][data-state="hidden"][aria-hidden="true"]',
  );
  await expect(hiddenSettled).toHaveCount(1);

  const visibleSettled = page.locator(
    '[data-slot="error-view"][data-motion-source="custom"][data-state="visible"]',
  );
  await expect(visibleSettled).toHaveCount(1);
  await expect(visibleSettled).toHaveAttribute("data-actions", "true");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(
    page.locator('[data-slot="error-view"][data-state="hidden"][aria-hidden="true"]'),
  ).toHaveCount(1);
  await expect(
    page.locator(
      '[data-slot="error-view"][data-motion-source="custom"][data-state="visible"]',
    ),
  ).toHaveCount(1);
});

test("docs-app error-view flow is repeatable with semantic failure breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/error-view");
  await page.locator("body:not(:has(#boot))").waitFor();

  const custom = page.locator(
    '[data-slot="error-view"][data-motion-source="custom"][data-actions="true"]',
  );
  await expect(custom).toHaveCount(1);
  await expect(custom).toHaveAttribute("data-state", "visible");

  const retryButton = custom.getByRole("button", { name: "Retry" });
  await expect(retryButton).toBeVisible();
  await retryButton.focus();
  await expect(retryButton).toBeFocused();
  await page.keyboard.press("Enter");
  await expect(custom).toHaveAttribute("data-state", "visible");
  await expect(custom).toHaveAttribute("data-actions", "true");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const customAfterReload = page.locator(
    '[data-slot="error-view"][data-motion-source="custom"][data-actions="true"]',
  );
  await expect(customAfterReload).toHaveCount(1);
  await expect(customAfterReload).toHaveAttribute("data-state", "visible");

  const retryAfterReload = customAfterReload.getByRole("button", { name: "Retry" });
  await retryAfterReload.focus();
  await expect(retryAfterReload).toBeFocused();
});

test("docs-app error-view interactive playground key flow is repeatable with semantic breakpoints", async ({
  page,
}) => {
  await page.goto("/#/components/error-view");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="error-view"]').first();
  await expect(docsRoot).toBeVisible();

  const interactivePlayground = docsRoot
    .locator("section.playground")
    .filter({ has: docsRoot.getByRole("heading", { name: "Interactive Playground", exact: true }) })
    .first();
  await expect(interactivePlayground).toBeVisible();

  const settingsButton = interactivePlayground
    .getByRole("button", { name: /Show settings|Hide settings/ })
    .first();
  await expect(settingsButton).toBeVisible();

  if (
    (await interactivePlayground.locator('[data-slot="error-view-workbench-controls"]').count()) === 0
  ) {
    await settingsButton.click();
  }

  const controls = interactivePlayground
    .locator('[data-slot="error-view-workbench-controls"]')
    .first();
  await expect(controls).toBeVisible();

  const workbenchRoot = interactivePlayground
    .locator('[data-slot="error-view-workbench"] [data-slot="error-view"]')
    .first();
  await expect(workbenchRoot).toBeVisible();
  await expect(workbenchRoot).toHaveAttribute("data-state", "visible");
  await expect(workbenchRoot).toHaveAttribute("data-tone", "negative");

  await controls
    .locator(
      '[data-slot="error-view-workbench-tone"] [data-slot="segmented-control-option"][data-index="1"]',
    )
    .first()
    .click();
  await expect(workbenchRoot).toHaveAttribute("data-tone", "neutral");

  await controls
    .locator(
      '[data-slot="error-view-workbench-message"] [data-slot="segmented-control-option"][data-index="1"]',
    )
    .first()
    .click();

  await controls
    .locator('[data-slot="error-view-workbench-toggle-compact"] input[type="checkbox"]')
    .first()
    .setChecked(true);
  await expect(workbenchRoot).toHaveAttribute("data-compact", "true");

  await controls
    .locator('[data-slot="error-view-workbench-toggle-bordered"] input[type="checkbox"]')
    .first()
    .setChecked(true);
  await expect(workbenchRoot).toHaveAttribute("data-bordered", "true");

  await controls
    .locator('[data-slot="error-view-workbench-toggle-invalid"] input[type="checkbox"]')
    .first()
    .setChecked(false);
  await expect(workbenchRoot).toHaveAttribute("data-state", "hidden");
  await expect(workbenchRoot).toHaveAttribute("aria-hidden", "true");

  await controls
    .locator('[data-slot="error-view-workbench-toggle-invalid"] input[type="checkbox"]')
    .first()
    .setChecked(true);
  await expect(workbenchRoot).toHaveAttribute("data-state", "visible");
  await expect(workbenchRoot).toHaveAttribute("data-message-source", "custom");

  const feedback = interactivePlayground
    .locator('[data-slot="error-view-workbench-feedback"]')
    .first();
  await expect(feedback).toContainText("invalid=true");
  await expect(feedback).toContainText("tone=neutral");
  await expect(feedback).toContainText("compact=true");
  await expect(feedback).toContainText("bordered=true");
  await expect(feedback).toContainText("message=retry");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const controlsAfterReload = page
    .locator('[data-slot="error-view-workbench-controls"]')
    .first();
  const rootAfterReload = page
    .locator('[data-slot="error-view-workbench"] [data-slot="error-view"]')
    .first();
  await expect(controlsAfterReload).toBeVisible();
  await expect(rootAfterReload).toHaveAttribute("data-state", "visible");
  await expect(rootAfterReload).toHaveAttribute("data-tone", "negative");
});
