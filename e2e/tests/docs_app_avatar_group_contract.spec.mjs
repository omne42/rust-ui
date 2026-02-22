import { expect, test } from "@playwright/test";

async function gotoAvatarGroupDocsAndWaitReady(page) {
  await page.goto("/#/components/avatar-group");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page
    .locator('[data-component="avatar-group"][data-slot="avatar-group"]')
    .first();
  await expect(docsRoot).toBeVisible();

  const readyGroup = docsRoot
    .locator(
      '[data-slot="avatar-group"][data-ui-schema="ui.avatar-group.agent.v1"][data-ui-stream-support="optional"][data-ui-stream-fallback="snapshot"][data-ui-output-status]'
    )
    .first();
  await expect(readyGroup).toBeVisible();
  await expect(readyGroup).toHaveAttribute("role", "group");
  await expect(readyGroup).toHaveAttribute(
    "data-ui-intent",
    "display-identity-collection"
  );

  return { docsRoot, readyGroup };
}

test("docs-app avatar-group uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const { docsRoot } = await gotoAvatarGroupDocsAndWaitReady(page);

  const emptyGroup = docsRoot
    .locator(
      '[data-slot="avatar-group"][data-state="empty"][data-empty="true"][data-count="0"]'
    )
    .first();
  await expect(emptyGroup).toBeVisible();
  await expect(emptyGroup).toHaveAttribute("data-ui-state", "empty");
  await expect(emptyGroup).toHaveAttribute("data-ui-action", "render-stable-roster");

  const overflowGroup = docsRoot
    .locator('[data-slot="avatar-group"][data-state="overflow"][data-has-overflow="true"]')
    .first();
  await expect(overflowGroup).toBeVisible();
  await expect(overflowGroup).toHaveAttribute("data-ui-state", "overflow");
  await expect(overflowGroup).toHaveAttribute(
    "data-ui-action",
    "render-overflow-summary"
  );

  const overflowBadge = overflowGroup.locator('[data-slot="avatar-group-overflow"]').first();
  await expect(overflowBadge).toBeVisible();
  await expect(overflowBadge).toHaveAttribute("data-count", /[1-9]/);
  await expect(overflowGroup.locator('[data-slot="avatar-group-item"]')).toHaveCount(3);
});

test("docs-app avatar-group keeps streaming/snapshot semantics readable and async-motion path explicitly N/A", async ({
  page,
}) => {
  const { docsRoot } = await gotoAvatarGroupDocsAndWaitReady(page);

  const streamingPreview = docsRoot.locator('[data-slot="avatar-group-streaming-preview"]').first();
  await expect(streamingPreview).toBeVisible();
  await expect(
    streamingPreview.locator(
      '[data-slot="avatar-group"][data-ui-stream-support="optional"][data-ui-stream-fallback="snapshot"][data-ui-output-status="verified"]'
    )
  ).toHaveCount(1);
  await expect(
    streamingPreview.locator('[data-slot="avatar-group-streaming-policy"]').first()
  ).toBeVisible();

  const sourceFirstPreview = docsRoot
    .locator('[data-slot="avatar-group-source-first-preview"]')
    .first();
  await expect(sourceFirstPreview).toBeVisible();
  await expect(
    sourceFirstPreview.locator('[data-slot="avatar-group-copy-ready-hint"]').first()
  ).toBeVisible();

  // AvatarGroup has no async loading or motion runtime path in current scope.
  await expect(docsRoot.locator('[data-slot="avatar-group"][aria-busy="true"]')).toHaveCount(0);
  await expect(docsRoot.locator('[data-slot="avatar-group"][data-loading="true"]')).toHaveCount(0);
  await expect(docsRoot.locator('[data-slot="avatar-group"][data-motion-state]')).toHaveCount(0);
});

test("docs-app avatar-group key flow is repeatable with semantic checkpoints", async ({
  page,
}) => {
  await gotoAvatarGroupDocsAndWaitReady(page);

  const overflowBeforeReload = page
    .locator(
      '[data-component="avatar-group"][data-slot="avatar-group"] [data-slot="avatar-group"][data-state="overflow"][data-has-overflow="true"]'
    )
    .first();
  await expect(overflowBeforeReload).toBeVisible();
  await expect(overflowBeforeReload).toHaveAttribute("data-ui-state", "overflow");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const overflowAfterReload = page
    .locator(
      '[data-component="avatar-group"][data-slot="avatar-group"] [data-slot="avatar-group"][data-state="overflow"][data-has-overflow="true"]'
    )
    .first();
  await expect(overflowAfterReload).toBeVisible();
  await expect(overflowAfterReload).toHaveAttribute("data-ui-state", "overflow");
});

test("docs-app avatar-group interactive playground updates semantic state markers with live controls", async ({
  page,
}) => {
  const { docsRoot } = await gotoAvatarGroupDocsAndWaitReady(page);

  const playground = docsRoot
    .locator('[data-slot="playground"]')
    .filter({ has: docsRoot.locator('[data-slot="avatar-group-workbench-controls"]') })
    .first();
  await expect(playground).toBeVisible();

  const controls = playground.locator('[data-slot="avatar-group-workbench-controls"]').first();
  const configuredGroup = playground
    .locator('[data-slot="avatar-group-workbench-configured"] [data-slot="avatar-group"]')
    .first();
  await expect(configuredGroup).toBeVisible();

  await expect(configuredGroup).toHaveAttribute("data-state", "overflow");
  await expect(configuredGroup).toHaveAttribute("data-size", "md");
  await expect(configuredGroup).toHaveAttribute("data-aria-label-source", "default");

  const rosterControl = controls.locator('[data-slot="segmented-control"]').first();
  const sizeControl = controls.locator('[data-slot="segmented-control"]').nth(1);
  const maxControl = controls.locator('[data-slot="segmented-control"]').nth(2);
  const customAriaSwitch = controls.locator('[data-slot="switch"]').first();
  const customClassSwitch = controls.locator('[data-slot="switch"]').nth(1);
  const rtlSwitch = controls.locator('[data-slot="switch"]').nth(2);

  await rosterControl.locator('[data-slot="segmented-control-option"][data-index="0"]').click();
  await expect(configuredGroup).toHaveAttribute("data-state", "empty");
  await expect(configuredGroup).toHaveAttribute("data-count", "0");

  await rosterControl.locator('[data-slot="segmented-control-option"][data-index="2"]').click();
  await maxControl.locator('[data-slot="segmented-control-option"][data-index="0"]').click();
  await expect(configuredGroup).toHaveAttribute("data-state", "overflow");
  await expect(configuredGroup).toHaveAttribute("data-max-visible", "2");

  await sizeControl.locator('[data-slot="segmented-control-option"][data-index="0"]').click();
  await expect(configuredGroup).toHaveAttribute("data-size", "sm");

  await customAriaSwitch.click();
  await expect(configuredGroup).toHaveAttribute("data-aria-label-source", "custom");

  await customClassSwitch.click();
  await expect(configuredGroup).toHaveAttribute("data-class-source", "custom");

  await rtlSwitch.click();
  await expect(configuredGroup).toHaveAttribute("dir", "rtl");

  await expect(playground.locator('[data-slot="avatar-group-workbench-state"]').first()).toBeVisible();
  await expect(playground.locator('[data-slot="avatar-group-spec-preview-na"]').first()).toBeVisible();

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedConfiguredGroup = page
    .locator(
      '[data-component="avatar-group"] [data-slot="avatar-group-workbench-configured"] [data-slot="avatar-group"]'
    )
    .first();
  await expect(reloadedConfiguredGroup).toBeVisible();
  await expect(reloadedConfiguredGroup).toHaveAttribute("data-state", "overflow");
  await expect(reloadedConfiguredGroup).toHaveAttribute("data-size", "md");
  await expect(reloadedConfiguredGroup).toHaveAttribute("data-aria-label-source", "default");
});

test("docs-app avatar-group source-first section exposes copy-ready starter and source anchors", async ({
  page,
}) => {
  const { docsRoot } = await gotoAvatarGroupDocsAndWaitReady(page);

  const sourceFirst = docsRoot.locator('[data-slot="avatar-group-source-first"]').first();
  await expect(sourceFirst).toBeVisible();

  const snippet = sourceFirst.locator('[data-slot="snippet"]').first();
  await expect(snippet).toBeVisible();
  await expect(snippet).toHaveAttribute("data-copyable", "true");

  const copyButton = sourceFirst.locator('[data-slot="snippet-copy-button"]').first();
  await expect(copyButton).toBeVisible();

  const snippetPre = sourceFirst.locator('[data-slot="snippet-pre"]').first();
  await expect(snippetPre).toContainText("use ui::{AvatarGroup, AvatarGroupItem, AvatarSize};");
  await expect(snippetPre).toContainText("<AvatarGroup");

  await expect(sourceFirst).toContainText("component-avatar-group");
  await expect(sourceFirst).toContainText("inject-css");
  await expect(sourceFirst).toContainText("UiRoot");

  const sourcePaths = sourceFirst.locator('[data-slot="avatar-group-source-paths"]').first();
  await expect(sourcePaths).toContainText("components/avatar-group/src/mod.rs");
  await expect(sourcePaths).toContainText("components/avatar-group/src/logic.rs");
  await expect(sourcePaths).toContainText("components/avatar-group/src/view.rs");
  await expect(sourcePaths).toContainText("components/avatar-group/src/styles.rs");

  await expect(sourceFirst.locator('[data-slot="avatar-group-source-sync-note"]').first()).toBeVisible();
});
