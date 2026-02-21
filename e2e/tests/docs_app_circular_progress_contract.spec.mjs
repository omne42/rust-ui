import { expect, test } from "@playwright/test";

const WASM_READY_SELECTOR = "body:not(:has(#boot))";

async function gotoCircularProgressDocsAndWaitSettled(page) {
  await page.goto("/#/components/circular-progress");
  await page.locator(WASM_READY_SELECTOR).waitFor();

  const docsRoot = page.locator('[data-component="circular-progress"]').first();
  await expect(docsRoot).toBeVisible();

  const settled = docsRoot
    .locator(
      '[data-slot="circular-progress"][data-ui-schema="ui.circular-progress.agent-contract"][data-ui-schema-version="v1"][data-ui-state="indeterminate"][role="progressbar"]'
    )
    .first();
  await expect(settled).toBeVisible();

  return docsRoot;
}

test("docs-app circular-progress uses semantic selectors with wasm-stable wait strategy", async ({
  page,
}) => {
  const docsRoot = await gotoCircularProgressDocsAndWaitSettled(page);

  const indicators = docsRoot.locator(
    '[data-slot="circular-progress"][role="progressbar"][data-state="indeterminate"]'
  );
  const indicatorCount = await indicators.count();
  expect(indicatorCount).toBeGreaterThan(0);

  await expect(
    docsRoot
      .locator(
        '[data-slot="circular-progress"][data-size-source="default"][data-thickness-source="default"][data-label-source="default"]'
      )
      .first()
  ).toBeVisible();

  await expect(
    docsRoot
      .locator(
        '[data-slot="circular-progress"][data-size-source="custom"][data-thickness-source="custom"][data-label-source="custom"][data-custom-class="true"][data-class-source="custom"]'
      )
      .first()
  ).toBeVisible();
});

test("docs-app circular-progress animation path keeps semantic ready/settled breakpoints", async ({
  page,
}) => {
  let docsRoot = await gotoCircularProgressDocsAndWaitSettled(page);

  const streamPolicy = docsRoot.locator('[data-slot="circular-progress-streaming-policy"]').first();
  await expect(streamPolicy).toHaveText(/fallback=snapshot/);

  const animated = docsRoot
    .locator('[data-slot="circular-progress"][data-motion="spin"][data-state="indeterminate"]')
    .first();
  await expect(animated).toBeVisible();
  await expect(animated).toHaveAttribute("data-ui-intent", "progress.indeterminate");
  await expect(animated).toHaveAttribute("data-ui-action", "render");
  await expect(animated).toHaveAttribute("data-ui-source", "state-primitives");

  await page.reload();
  docsRoot = await gotoCircularProgressDocsAndWaitSettled(page);

  const afterReload = docsRoot
    .locator('[data-slot="circular-progress"][data-motion="spin"][data-state="indeterminate"]')
    .first();
  await expect(afterReload).toBeVisible();
  await expect(afterReload).toHaveAttribute("data-motion", "spin");
  await expect(afterReload).toHaveAttribute("data-state", "indeterminate");
});

test("docs-app circular-progress key flow regression uses semantic breakpoints for diagnosis", async ({
  page,
}) => {
  let docsRoot = await gotoCircularProgressDocsAndWaitSettled(page);

  await test.step("open route reaches semantic ready breakpoint", async () => {
    const ready = docsRoot
      .locator(
        '[data-slot="circular-progress"][data-ui-schema="ui.circular-progress.agent-contract"][data-ui-state="indeterminate"][data-ui-action="render"][data-ui-source="state-primitives"][role="progressbar"]'
      )
      .first();

    await expect(ready).toBeVisible();
    await expect(ready).toHaveAttribute("data-state", "indeterminate");
  });

  await test.step("interaction keeps source markers diagnosable", async () => {
    const defaults = docsRoot
      .locator(
        '[data-slot="circular-progress"][data-size-source="default"][data-thickness-source="default"][data-label-source="default"][data-class-source="default"]'
      )
      .first();
    const custom = docsRoot
      .locator(
        '[data-slot="circular-progress"][data-size-source="custom"][data-thickness-source="custom"][data-label-source="custom"][data-class-source="custom"][data-custom-class="true"]'
      )
      .first();

    await expect(defaults).toBeVisible();
    await expect(custom).toBeVisible();
  });

  await test.step("reopen/remount keeps settled breakpoint stable", async () => {
    await page.reload();
    docsRoot = await gotoCircularProgressDocsAndWaitSettled(page);

    const settled = docsRoot
      .locator(
        '[data-slot="circular-progress"][data-motion="spin"][data-state="indeterminate"][data-ui-intent="progress.indeterminate"][role="progressbar"]'
      )
      .first();

    await expect(settled).toBeVisible();
    await expect(settled).toHaveAttribute("data-motion", "spin");
    await expect(settled).toHaveAttribute("data-state", "indeterminate");
  });
});

test("docs-app circular-progress interactive playground updates props and semantic markers", async ({
  page,
}) => {
  let docsRoot = await gotoCircularProgressDocsAndWaitSettled(page);
  const controls = docsRoot.locator('[data-slot="circular-progress-workbench-controls"]').first();
  const preview = docsRoot.locator('[data-slot="circular-progress-workbench-preview"]').first();

  await expect(controls).toBeVisible();
  await expect(preview).toBeVisible();

  await test.step("adjust size/thickness props and observe custom source markers", async () => {
    await controls.locator('[data-slot="circular-progress-workbench-size-24"]').click();
    await controls.locator('[data-slot="circular-progress-workbench-thickness-3"]').click();

    const indicator = preview
      .locator(
        '[data-slot="circular-progress"][data-size-source="custom"][data-thickness-source="custom"][data-state="indeterminate"]'
      )
      .first();
    await expect(indicator).toBeVisible();
  });

  await test.step("toggle label/class source and observe semantic marker updates", async () => {
    await controls.locator('[data-slot="circular-progress-workbench-label-custom"]').click();
    await controls.locator('[data-slot="circular-progress-workbench-class-custom"]').click();

    const indicator = preview
      .locator(
        '[data-slot="circular-progress"][data-label-source="custom"][data-class-source="custom"][data-custom-class="true"]'
      )
      .first();
    await expect(indicator).toBeVisible();
  });

  await test.step("toggle direction and verify semantic locale attrs", async () => {
    await controls.locator('[data-slot="circular-progress-workbench-dir-rtl"]').click();

    const indicator = preview
      .locator('[data-slot="circular-progress"][dir="rtl"][lang="ar"][role="progressbar"]')
      .first();
    await expect(indicator).toBeVisible();
  });

  await test.step("replay flow after remount remains deterministic", async () => {
    await page.reload();
    docsRoot = await gotoCircularProgressDocsAndWaitSettled(page);

    const replayControls = docsRoot
      .locator('[data-slot="circular-progress-workbench-controls"]')
      .first();
    const replayPreview = docsRoot
      .locator('[data-slot="circular-progress-workbench-preview"]')
      .first();

    await replayControls.locator('[data-slot="circular-progress-workbench-size-default"]').click();
    await replayControls
      .locator('[data-slot="circular-progress-workbench-thickness-default"]')
      .click();
    await replayControls.locator('[data-slot="circular-progress-workbench-label-default"]').click();
    await replayControls.locator('[data-slot="circular-progress-workbench-class-default"]').click();
    await replayControls.locator('[data-slot="circular-progress-workbench-dir-ltr"]').click();

    const indicator = replayPreview
      .locator(
        '[data-slot="circular-progress"][data-size-source="default"][data-thickness-source="default"][data-label-source="default"][data-class-source="default"][dir="ltr"]'
      )
      .first();
    await expect(indicator).toBeVisible();
  });
});
