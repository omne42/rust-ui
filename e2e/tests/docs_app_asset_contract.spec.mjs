import { expect, test } from "@playwright/test";

async function gotoAssetDocsAndWaitSettled(page) {
  await page.goto("/#/components/asset");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="asset"]').first();
  await expect(docsRoot).toBeVisible();

  const settledAsset = docsRoot
    .locator(
      '[data-slot="asset"][data-ui-schema="ui.asset.v1"][data-ui-stream-support="optional"][data-ui-stream-fallback="snapshot"][data-ui-output-status="verified"]'
    )
    .first();
  await expect(settledAsset).toBeVisible();
  await expect(settledAsset).toHaveAttribute("role", "img");
  await expect(settledAsset).toHaveAttribute("aria-label", /.+/);

  return docsRoot;
}

test("docs-app asset uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const docsRoot = await gotoAssetDocsAndWaitSettled(page);

  const stateMarkersAsset = docsRoot
    .locator(
      '[data-slot="asset"][data-variant="custom"][data-size="800"][data-state="selected"][data-custom-class="true"][data-class-source="custom"][data-content-source="custom-slot"]'
    )
    .first();
  await expect(stateMarkersAsset).toBeVisible();
  await expect(stateMarkersAsset).toHaveAttribute("data-selection-source", "external-prop");
  await expect(stateMarkersAsset).toHaveAttribute("data-focus-source", "external-prop");
  await expect(stateMarkersAsset).toHaveAttribute("data-ui-label-source", "custom");
  await expect(stateMarkersAsset).toHaveAttribute("data-ui-content-source", "custom-slot");
  await expect(stateMarkersAsset).toHaveAttribute("data-ui-motion-source", "default");
});

test("docs-app asset flow is repeatable via semantic ready/settled checkpoints", async ({
  page,
}) => {
  const docsRoot = await gotoAssetDocsAndWaitSettled(page);

  const controlledAsset = docsRoot
    .locator(
      '[data-slot="asset"][data-selected="true"][data-focused="true"][data-selection-source="external-prop"][data-focus-source="external-prop"]'
    )
    .first();
  await expect(controlledAsset).toBeVisible();
  await expect(controlledAsset).toHaveAttribute("data-state", /(selected|focused)/);
  await expect(controlledAsset).toHaveAttribute("data-ui-state", /(selected|focused)/);
  await expect(controlledAsset).toHaveAttribute("data-ui-stream-fallback", "snapshot");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedControlledAsset = page
    .locator(
      '[data-component="asset"] [data-slot="asset"][data-selected="true"][data-focused="true"][data-selection-source="external-prop"][data-focus-source="external-prop"]'
    )
    .first();
  await expect(reloadedControlledAsset).toBeVisible();
  await expect(reloadedControlledAsset).toHaveAttribute("data-state", /(selected|focused)/);
  await expect(reloadedControlledAsset).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app asset interactive playground updates props/state with semantic contracts", async ({
  page,
}) => {
  const docsRoot = await gotoAssetDocsAndWaitSettled(page);

  const controls = docsRoot.locator('[data-slot="asset-interactive-controls"]').first();
  await expect(controls).toBeVisible();

  const interactiveAsset = docsRoot
    .locator('[data-slot="asset-interactive-preview"] [data-slot="asset"]')
    .first();
  await expect(interactiveAsset).toBeVisible();
  await expect(interactiveAsset).toHaveAttribute("data-selection-source", "external-prop");
  await expect(interactiveAsset).toHaveAttribute("data-focus-source", "external-prop");

  await controls.getByLabel("Selected").check();
  await expect(interactiveAsset).toHaveAttribute("data-selected", "true");
  await expect(interactiveAsset).toHaveAttribute("data-state", /(selected|focused)/);

  await controls.getByLabel("Focused").check();
  await expect(interactiveAsset).toHaveAttribute("data-focused", "true");

  await controls.getByLabel("Variant").selectOption("folder");
  await expect(interactiveAsset).toHaveAttribute("data-variant", "folder");
  await expect(interactiveAsset).toHaveAttribute("data-content-source", /(builtin-icon|fallback-icon)/);

  await controls.getByLabel("Variant").selectOption("custom");
  await controls.getByLabel("Use custom slot").check();
  await expect(interactiveAsset).toHaveAttribute("data-variant", "custom");
  await expect(interactiveAsset).toHaveAttribute("data-content-source", "custom-slot");
});

test("docs-app asset source-first docs expose copy-ready code and real source paths", async ({
  page,
}) => {
  const docsRoot = await gotoAssetDocsAndWaitSettled(page);

  const firstPlayground = docsRoot.locator('[data-slot="playground"]').first();
  await firstPlayground.locator('[data-slot="playground-toggle-code"]').click();
  await expect(firstPlayground.locator('[data-slot="playground-code"]')).toBeVisible();

  const sourceFirst = docsRoot.locator('[data-slot="asset-source-first"]').first();
  await expect(sourceFirst).toBeVisible();
  await expect(
    sourceFirst.locator('[data-slot="asset-source-first-paths"]')
  ).toBeVisible();
  await expect(sourceFirst).toContainText("components/asset/src/view.rs");
  await expect(sourceFirst).toContainText("crates/ui/src/lib.rs");
  await expect(sourceFirst.locator('[data-slot="asset-source-first-prerequisites"]')).toBeVisible();
});
