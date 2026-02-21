import { expect, test } from "@playwright/test";

async function gotoIllustratedMessageDocsAndWaitSettled(page) {
  await page.goto("/#/components/illustrated-message");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="illustrated-message"]').first();
  await expect(docsRoot).toBeVisible();

  const settledRoot = docsRoot
    .locator(
      '[data-slot="illustrated-message"][data-ui-schema="ui.illustrated-message.agent-contract"][data-ui-schema-version="v1"][data-ui-action="render-snapshot"][data-ui-streaming-policy="optional"][data-ui-streaming-fallback="snapshot"][data-ui-output-status="validated"]'
    )
    .first();
  await expect(settledRoot).toBeVisible();

  return { docsRoot, settledRoot };
}

async function runRepeatableKeyFlowWithSemanticBreakpoints(docsRoot) {
  const richRoot = docsRoot
    .locator(
      '[data-slot="illustrated-message"][data-view-state="populated"][data-ui-state="populated"][data-ui-source="custom"][data-illustration-state="shown"][data-actions-state="shown"][data-illustration-source="provided"][data-actions-source="provided"]'
    )
    .first();
  await expect(richRoot).toBeVisible();
  await expect(richRoot.locator('[data-slot="illustrated-message-illustration"]')).toHaveCount(1);
  await expect(
    richRoot.locator('[data-slot="illustrated-message-actions"] [data-slot="button"]')
  ).toHaveCount(1);

  const descriptionOnlyRoot = docsRoot
    .locator(
      '[data-slot="illustrated-message"][data-title-state="hidden"][data-description-state="shown"][data-title-source="missing"][data-description-source="provided"]'
    )
    .first();
  await expect(descriptionOnlyRoot).toBeVisible();
  await expect(descriptionOnlyRoot.locator('[data-slot="illustrated-message-title"]')).toHaveCount(0);
  await expect(descriptionOnlyRoot.locator('[data-slot="illustrated-message-description"]')).toHaveCount(1);

  await expect(
    docsRoot
      .locator(
        '[data-slot="illustrated-message"][data-ui-state="populated"][data-ui-output-status="validated"][data-ui-streaming-fallback="snapshot"]'
      )
      .first()
  ).toBeVisible();
}

test("docs-app illustrated-message uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const { docsRoot, settledRoot } = await gotoIllustratedMessageDocsAndWaitSettled(page);

  const rootCount = await docsRoot.locator('[data-slot="illustrated-message"]').count();
  expect(rootCount).toBeGreaterThanOrEqual(9);
  await expect(settledRoot).toHaveAttribute("aria-live", "off");
  await expect(settledRoot).toHaveAttribute("data-view-state", "populated");
  await expect(settledRoot).toHaveAttribute("data-content-state", "shown");

  const defaultRoot = docsRoot
    .locator(
      '[data-slot="illustrated-message"][data-title-state="shown"][data-description-state="shown"][data-illustration-state="hidden"][data-actions-state="hidden"][data-title-source="provided"][data-description-source="provided"][data-illustration-source="missing"][data-actions-source="missing"]'
    )
    .first();
  await expect(defaultRoot).toBeVisible();

  await expect(
    docsRoot.locator('[data-slot="illustrated-message-streaming-preview"]').first()
  ).toBeVisible();
  await expect(
    docsRoot.locator('[data-slot="illustrated-message-streaming-policy"]').first()
  ).toBeVisible();
  await expect(
    docsRoot.locator('[data-slot="illustrated-message-copy-ready-hint"]').first()
  ).toBeVisible();
});

test("docs-app illustrated-message key flow is repeatable with semantic contract breakpoints", async ({
  page,
}) => {
  const { docsRoot } = await gotoIllustratedMessageDocsAndWaitSettled(page);
  await runRepeatableKeyFlowWithSemanticBreakpoints(docsRoot);

  await page.reload();
  const { docsRoot: reloadedRoot } = await gotoIllustratedMessageDocsAndWaitSettled(page);
  await runRepeatableKeyFlowWithSemanticBreakpoints(reloadedRoot);
});

test("docs-app illustrated-message interactive playground updates preview state markers", async ({
  page,
}) => {
  const { docsRoot } = await gotoIllustratedMessageDocsAndWaitSettled(page);
  const playground = docsRoot
    .locator("section.playground")
    .filter({
      has: docsRoot.getByRole("heading", { name: "Interactive Playground (Props + State + Preview)" }),
    })
    .first();
  await expect(playground).toBeVisible();

  const controlsToggle = playground.getByRole("button", { name: /Show settings|Hide settings/ }).first();
  await expect(controlsToggle).toBeVisible();
  if ((await playground.locator('[data-slot="illustrated-message-workbench-controls"]').count()) === 0) {
    await controlsToggle.click();
  }

  const controls = playground.locator('[data-slot="illustrated-message-workbench-controls"]').first();
  await expect(controls).toBeVisible();
  const preview = playground.locator('[data-slot="illustrated-message-workbench-preview"]').first();
  const root = preview.locator('[data-slot="illustrated-message"]').first();

  await expect(root).toHaveAttribute("data-orientation", "vertical");
  await expect(root).toHaveAttribute("data-title-state", "shown");
  await expect(root).toHaveAttribute("data-description-state", "shown");
  await expect(root).toHaveAttribute("data-illustration-state", "hidden");
  await expect(root).toHaveAttribute("data-actions-state", "hidden");

  await controls
    .locator('[data-slot="illustrated-message-workbench-toggle-illustration"]')
    .locator('input[type="checkbox"]')
    .first()
    .setChecked(true);
  await controls
    .locator('[data-slot="illustrated-message-workbench-toggle-actions"]')
    .locator('input[type="checkbox"]')
    .first()
    .setChecked(true);
  await controls
    .locator('[data-slot="illustrated-message-workbench-toggle-title"]')
    .locator('input[type="checkbox"]')
    .first()
    .setChecked(false);
  await controls
    .locator('[data-slot="illustrated-message-workbench-toggle-rtl"]')
    .locator('input[type="checkbox"]')
    .first()
    .setChecked(true);

  await controls
    .locator(
      '[data-slot="illustrated-message-workbench-orientation"] [data-slot="segmented-control-option"][data-index="1"]'
    )
    .first()
    .click();

  await expect(root).toHaveAttribute("data-orientation", "horizontal");
  await expect(root).toHaveAttribute("data-title-state", "hidden");
  await expect(root).toHaveAttribute("data-description-state", "shown");
  await expect(root).toHaveAttribute("data-illustration-state", "shown");
  await expect(root).toHaveAttribute("data-actions-state", "shown");
  await expect(root).toHaveAttribute("dir", "rtl");
  await expect(root.locator('[data-slot="illustrated-message-illustration"]')).toHaveCount(1);
  await expect(root.locator('[data-slot="illustrated-message-actions"] [data-slot="button"]')).toHaveCount(1);
  await expect(preview.locator('[data-slot="illustrated-message-workbench-state"]')).toContainText(
    "orientation=horizontal, title=false, description=true, illustration=true, actions=true, rtl=true"
  );
});
