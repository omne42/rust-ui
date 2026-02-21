import { expect, test } from "@playwright/test";

async function gotoEmptyStateDocsAndWaitSettled(page) {
  await page.goto("/#/components/empty-state");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="empty-state"]').first();
  await expect(docsRoot).toBeVisible();

  const settledRoot = docsRoot
    .locator(
      '[data-slot="empty-state"][data-ui-schema="ui-empty-state-agent-contract"][data-ui-schema-version="1"][data-ui-action="render"][data-ui-streaming="optional"][data-ui-render-mode="snapshot"][data-ui-fallback="snapshot"][data-ui-output-status="validated"]'
    )
    .first();
  await expect(settledRoot).toBeVisible();

  return docsRoot;
}

async function assertSemanticReadySettledContracts(docsRoot) {
  const defaultRoot = docsRoot
    .locator(
      '[data-slot="empty-state"][data-state="plain"][data-title-source="default"][data-description-source="default"][data-aria-source="default"][data-ui-intent="informative"][data-ui-source="default"][data-motion-source="default"]'
    )
    .first();
  await expect(defaultRoot).toBeVisible();
  await expect(defaultRoot).toHaveAttribute("role", "status");
  await expect(defaultRoot).toHaveAttribute("aria-live", "polite");

  const actionableRoot = docsRoot
    .locator(
      '[data-slot="empty-state"][data-state="rich"][data-actions="true"][data-icon="true"][data-ui-intent="actionable"][data-ui-source="custom"][data-ui-output-status="validated"]'
    )
    .first();
  await expect(actionableRoot).toBeVisible();
  await expect(actionableRoot).toHaveAttribute("data-ui-render-mode", "snapshot");
  await expect(actionableRoot).toHaveAttribute("data-ui-fallback", "snapshot");

  const compactBorderedRoot = docsRoot
    .locator(
      '[data-slot="empty-state"][data-state="rich"][data-compact="true"][data-bordered="true"][data-custom-class="true"][data-class-source="custom"][data-motion-source="default"]'
    )
    .first();
  await expect(compactBorderedRoot).toBeVisible();
}

async function runInteractiveWorkbenchFlow(docsRoot) {
  const interactivePlayground = docsRoot
    .locator("section.playground")
    .filter({ has: docsRoot.getByRole("heading", { name: "Interactive Playground" }) })
    .first();
  await expect(interactivePlayground).toBeVisible();

  const settingsButton = interactivePlayground
    .getByRole("button", { name: /Show settings|Hide settings/ })
    .first();
  await expect(settingsButton).toBeVisible();

  if (
    (await interactivePlayground.locator('[data-slot="empty-state-workbench-controls"]').count()) === 0
  ) {
    await settingsButton.click();
  }

  const controls = interactivePlayground
    .locator('[data-slot="empty-state-workbench-controls"]')
    .first();
  await expect(controls).toBeVisible();

  await controls
    .locator('[data-slot="empty-state-workbench-title"] input[type="text"]')
    .first()
    .fill("Interactive incident");
  await controls
    .locator('[data-slot="empty-state-workbench-description"] input[type="text"]')
    .first()
    .fill("Operator changed props from workbench controls.");

  await controls
    .locator(
      '[data-slot="empty-state-workbench-tone"] [data-slot="segmented-control-option"][data-index="2"]'
    )
    .first()
    .click();
  await controls
    .locator(
      '[data-slot="empty-state-workbench-align"] [data-slot="segmented-control-option"][data-index="1"]'
    )
    .first()
    .click();

  for (const slot of [
    "empty-state-workbench-toggle-compact",
    "empty-state-workbench-toggle-bordered",
    "empty-state-workbench-toggle-icon",
    "empty-state-workbench-toggle-actions",
    "empty-state-workbench-toggle-class",
  ]) {
    const checkbox = controls.locator(`[data-slot="${slot}"] input[type="checkbox"]`).first();
    await checkbox.setChecked(true);
  }

  const workbenchRoot = interactivePlayground
    .locator('[data-slot="empty-state-workbench"] [data-slot="empty-state"]')
    .first();
  await expect(workbenchRoot).toBeVisible();
  await expect(workbenchRoot).toHaveAttribute("data-tone", "accent");
  await expect(workbenchRoot).toHaveAttribute("data-align", "center");
  await expect(workbenchRoot).toHaveAttribute("data-compact", "true");
  await expect(workbenchRoot).toHaveAttribute("data-bordered", "true");
  await expect(workbenchRoot).toHaveAttribute("data-icon", "true");
  await expect(workbenchRoot).toHaveAttribute("data-actions", "true");
  await expect(workbenchRoot).toHaveAttribute("data-state", "rich");
  await expect(workbenchRoot).toHaveAttribute("data-ui-intent", "actionable");
  await expect(workbenchRoot).toHaveAttribute("data-ui-output-status", "validated");
  await expect(workbenchRoot).toHaveAttribute("data-ui-render-mode", "snapshot");
  await expect(workbenchRoot).toHaveAttribute("data-ui-fallback", "snapshot");
  await expect(workbenchRoot).toHaveAttribute("data-title-source", "custom");
  await expect(workbenchRoot).toHaveAttribute("data-description-source", "custom");
  await expect(workbenchRoot).toHaveAttribute("data-class-source", "custom");
  await expect(workbenchRoot).toHaveAttribute("data-ui-source", "custom");

  await expect(
    interactivePlayground.locator('[data-slot="empty-state-workbench"] [data-slot="empty-state-icon"]')
  ).toBeVisible();
  await expect(
    interactivePlayground.locator(
      '[data-slot="empty-state-workbench"] [data-slot="empty-state-actions"] [data-slot="button"]'
    )
  ).toHaveCount(1);

  for (const slot of ["empty-state-workbench-toggle-icon", "empty-state-workbench-toggle-actions"]) {
    const checkbox = controls.locator(`[data-slot="${slot}"] input[type="checkbox"]`).first();
    await checkbox.setChecked(false);
  }

  await expect(workbenchRoot).toHaveAttribute("data-state", "plain");
  await expect(workbenchRoot).not.toHaveAttribute("data-icon", "true");
  await expect(workbenchRoot).not.toHaveAttribute("data-actions", "true");
  await expect(workbenchRoot).toHaveAttribute("data-ui-intent", "informative");
}

test("docs-app empty-state uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const docsRoot = await gotoEmptyStateDocsAndWaitSettled(page);
  await assertSemanticReadySettledContracts(docsRoot);
});

test("docs-app empty-state flow is repeatable with semantic ready/settled breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoEmptyStateDocsAndWaitSettled(page);
  await assertSemanticReadySettledContracts(docsRoot);
  await runInteractiveWorkbenchFlow(docsRoot);

  await page.reload();
  const reloadedRoot = await gotoEmptyStateDocsAndWaitSettled(page);
  await assertSemanticReadySettledContracts(reloadedRoot);
  await runInteractiveWorkbenchFlow(reloadedRoot);
});

test("docs-app empty-state interactive playground keeps live preview in sync", async ({ page }) => {
  const docsRoot = await gotoEmptyStateDocsAndWaitSettled(page);
  await runInteractiveWorkbenchFlow(docsRoot);
});
