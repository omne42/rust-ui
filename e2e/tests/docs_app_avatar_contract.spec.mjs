import { expect, test } from "@playwright/test";

async function gotoAvatarDocsAndWaitSettled(page) {
  await page.goto("/#/components/avatar");
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="avatar"]').first();
  await expect(docsRoot).toBeVisible();

  const settledAvatar = docsRoot
    .locator(
      '[data-slot="avatar"][data-ui-schema="ui.avatar.agent.v1"][data-intent="display-identity"][data-state]'
    )
    .first();
  await expect(settledAvatar).toBeVisible();
  await expect(settledAvatar).toHaveAttribute("data-state", /(image|fallback)/);

  return docsRoot;
}

test("docs-app avatar uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const docsRoot = await gotoAvatarDocsAndWaitSettled(page);

  const imageAvatar = docsRoot
    .locator('[data-slot="avatar"][data-has-src="true"][data-state="image"][data-image="true"]')
    .first();
  await expect(imageAvatar).toBeVisible();
  await expect(imageAvatar).toHaveAttribute("data-action", "image-fallback-on-error");
  await expect(imageAvatar).toHaveAttribute("data-intent", "display-identity");

  const fallbackAvatar = docsRoot
    .locator(
      '[data-slot="avatar"][data-state="fallback"][data-fallback="true"][data-label-source="fallback"]'
    )
    .first();
  await expect(fallbackAvatar).toBeVisible();
  await expect(fallbackAvatar).toHaveAttribute("data-action", "passive-fallback");
  await expect(fallbackAvatar).toHaveAttribute("data-source", "fallback");
  await expect(fallbackAvatar).toHaveAttribute("role", "img");
  await expect(fallbackAvatar).toHaveAttribute("aria-label", /.+/);
});

test("docs-app avatar flow is repeatable via semantic breakpoints", async ({
  page,
}) => {
  const docsRoot = await gotoAvatarDocsAndWaitSettled(page);

  const customAvatar = docsRoot
    .locator('[data-slot="avatar"][data-custom-class="true"][data-label-source="alt"]')
    .first();
  await expect(customAvatar).toBeVisible();
  await expect(customAvatar).toHaveAttribute("data-state", "fallback");
  await expect(customAvatar).toHaveAttribute("data-fallback", "true");
  await expect(customAvatar).toHaveAttribute("data-source", "alt");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedCustomAvatar = page
    .locator(
      '[data-component="avatar"] [data-slot="avatar"][data-custom-class="true"][data-label-source="alt"]'
    )
    .first();
  await expect(reloadedCustomAvatar).toBeVisible();
  await expect(reloadedCustomAvatar).toHaveAttribute("data-state", "fallback");
  await expect(reloadedCustomAvatar).toHaveAttribute("data-fallback", "true");
});

test("docs-app avatar interactive playground updates state markers with semantic controls", async ({
  page,
}) => {
  const docsRoot = await gotoAvatarDocsAndWaitSettled(page);

  const playground = docsRoot
    .locator('[data-slot="playground"]')
    .filter({ has: docsRoot.locator('[data-slot="avatar-workbench-controls"]') })
    .first();
  await expect(playground).toBeVisible();

  const controls = playground.locator('[data-slot="avatar-workbench-controls"]').first();
  const configuredAvatar = playground
    .locator('[data-slot="avatar-workbench-configured"] [data-slot="avatar"]')
    .first();
  await expect(configuredAvatar).toBeVisible();

  await expect(configuredAvatar).toHaveAttribute("data-state", "image");
  await expect(configuredAvatar).toHaveAttribute("data-label-source", "name");
  await expect(configuredAvatar).toHaveAttribute("data-size", "md");

  const modeControl = controls.locator('[data-slot="segmented-control"]').first();
  const sizeControl = controls.locator('[data-slot="segmented-control"]').nth(1);
  const useAltSwitch = controls.locator('[data-slot="switch"]').first();
  const customClassSwitch = controls.locator('[data-slot="switch"]').nth(1);

  await modeControl.locator('[data-slot="segmented-control-option"][data-index="2"]').click();
  await expect(configuredAvatar).toHaveAttribute("data-state", "fallback");
  await expect(configuredAvatar).toHaveAttribute("data-label-source", "fallback");

  await useAltSwitch.click();
  await expect(configuredAvatar).toHaveAttribute("data-label-source", "alt");

  await sizeControl.locator('[data-slot="segmented-control-option"][data-index="0"]').click();
  await expect(configuredAvatar).toHaveAttribute("data-size", "sm");

  await customClassSwitch.click();
  await expect(configuredAvatar).toHaveAttribute("data-custom-class", "true");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedConfiguredAvatar = page
    .locator('[data-component="avatar"] [data-slot="avatar-workbench-configured"] [data-slot="avatar"]')
    .first();
  await expect(reloadedConfiguredAvatar).toBeVisible();
  await expect(reloadedConfiguredAvatar).toHaveAttribute("data-state", "image");
  await expect(reloadedConfiguredAvatar).toHaveAttribute("data-label-source", "name");
  await expect(reloadedConfiguredAvatar).toHaveAttribute("data-size", "md");
});

test("docs-app avatar source-first section exposes copy-ready starter and source anchors", async ({
  page,
}) => {
  const docsRoot = await gotoAvatarDocsAndWaitSettled(page);

  const sourceFirst = docsRoot.locator('[data-slot="avatar-source-first"]').first();
  await expect(sourceFirst).toBeVisible();

  const snippet = sourceFirst.locator('[data-slot="snippet"]').first();
  await expect(snippet).toBeVisible();
  await expect(snippet).toHaveAttribute("data-copyable", "true");

  const copyButton = sourceFirst.locator('[data-slot="snippet-copy-button"]').first();
  await expect(copyButton).toBeVisible();

  await expect(sourceFirst.locator('[data-slot="snippet-pre"]').first()).toContainText(
    "use ui::{Avatar, AvatarSize};"
  );
  await expect(sourceFirst.locator('[data-slot="snippet-pre"]').first()).toContainText(
    '<Avatar name="Ada Lovelace".to_string() size=AvatarSize::Md />'
  );

  await expect(sourceFirst).toContainText("component-avatar");
  await expect(sourceFirst).toContainText("inject-css");
  await expect(sourceFirst).toContainText("UiRoot");

  const sourcePaths = sourceFirst.locator('[data-slot="avatar-source-paths"]').first();
  await expect(sourcePaths).toContainText("components/avatar/src/view.rs");
  await expect(sourcePaths).toContainText("components/avatar/src/logic.rs");
});
