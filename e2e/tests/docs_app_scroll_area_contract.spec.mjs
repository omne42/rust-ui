import { expect, test } from "@playwright/test";

const SCROLL_AREA_PAGE = "/#/components/scroll-area";
const DOCS_ROOT = '[data-component="scroll-area"]';

async function gotoScrollArea(page) {
  await page.goto(SCROLL_AREA_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

async function resolveDocsRoot(page) {
  const docsRoot = page.locator(DOCS_ROOT).first();
  await expect(docsRoot).toBeVisible();
  return docsRoot;
}

test("docs-app scroll-area contract uses semantic selectors with wasm-safe ready waits", async ({
  page,
}) => {
  await gotoScrollArea(page);
  const docsRoot = await resolveDocsRoot(page);

  const helloRoot = docsRoot
    .locator('[data-slot="scroll-area"][data-orientation="vertical"][data-max-height="default"]')
    .first();
  const maxHeightRoot = docsRoot
    .locator('[data-slot="scroll-area"][data-orientation="vertical"][data-max-height="custom"]')
    .first();
  const horizontalRoot = docsRoot
    .locator('[data-slot="scroll-area"][data-orientation="horizontal"][data-class-source="custom"]')
    .first();
  const disabledRoot = docsRoot
    .locator('[data-slot="scroll-area"][data-orientation="both"][data-disabled="true"]')
    .first();

  for (const root of [helloRoot, maxHeightRoot, horizontalRoot, disabledRoot]) {
    await expect(root).toBeVisible();
    await expect(root).toHaveAttribute("data-ui-schema", "ui.scroll-area.agent-contract.v1");
    await expect(root).toHaveAttribute("data-ui-stream-support", "unsupported");
    await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
    await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");
    await expect(root).toHaveAttribute("data-ui-output-status", "verified");
  }

  await expect(helloRoot).toHaveAttribute("data-aria-source", "default");
  await expect(helloRoot).toHaveAttribute("data-motion-source", "default");
  await expect(maxHeightRoot).toHaveAttribute("data-max-height", "custom");
  await expect(maxHeightRoot).toHaveAttribute("data-motion-source", "default");
  await expect(horizontalRoot).toHaveAttribute("data-custom-class", "true");
  await expect(horizontalRoot).toHaveAttribute("data-ui-state", "enabled");

  const disabledViewport = disabledRoot.locator('[data-slot="scroll-area-viewport"]').first();
  await expect(disabledRoot).toHaveAttribute("data-ui-action", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-ui-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-ui-source", "is-prop");
  await expect(disabledViewport).toHaveAttribute("tabindex", "-1");
  await expect(disabledViewport).toHaveAttribute("aria-disabled", "true");
});

test("docs-app scroll-area interaction path uses semantic ready and settled breakpoints", async ({
  page,
}) => {
  await gotoScrollArea(page);
  const docsRoot = await resolveDocsRoot(page);

  const readyRoot = docsRoot
    .locator('[data-slot="scroll-area"][data-orientation="vertical"][data-max-height="custom"]')
    .first();
  const readyViewport = readyRoot.locator('[data-slot="scroll-area-viewport"]').first();

  await expect(readyRoot).toHaveAttribute("data-ui-action", "observe");
  await expect(readyRoot).toHaveAttribute("data-ui-state", "enabled");
  await expect(readyRoot).toHaveAttribute("data-ui-output-status", "verified");
  await expect(readyRoot).toHaveAttribute("data-motion-source", "default");
  await expect(readyViewport).toHaveAttribute("tabindex", "0");

  await readyViewport.focus();
  await expect(readyViewport).toBeFocused();
  await page.keyboard.press("PageDown");

  await expect(readyRoot).toHaveAttribute("data-ui-action", "observe");
  await expect(readyRoot).toHaveAttribute("data-ui-state", "enabled");
  await expect(readyRoot).toHaveAttribute("data-ui-output-status", "verified");
  await expect(readyRoot).toHaveAttribute("data-motion-source", "default");

  const disabledRoot = docsRoot
    .locator('[data-slot="scroll-area"][data-orientation="both"][data-disabled="true"]')
    .first();
  const disabledViewport = disabledRoot.locator('[data-slot="scroll-area-viewport"]').first();

  await expect(disabledViewport).toHaveAttribute("tabindex", "-1");
  await expect(disabledViewport).toHaveAttribute("aria-disabled", "true");

  await disabledViewport.evaluate((node) => {
    node.scrollTop = 120;
  });

  await expect(disabledRoot).toHaveAttribute("data-ui-action", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-ui-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app scroll-area key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await gotoScrollArea(page);
  let docsRoot = await resolveDocsRoot(page);

  let readyRoot = docsRoot
    .locator('[data-slot="scroll-area"][data-orientation="vertical"][data-max-height="custom"]')
    .first();
  let readyViewport = readyRoot.locator('[data-slot="scroll-area-viewport"]').first();

  await expect(readyRoot).toHaveAttribute("data-ui-action", "observe");
  await expect(readyRoot).toHaveAttribute("data-ui-state", "enabled");
  await expect(readyRoot).toHaveAttribute("data-ui-source", "legacy-prop");
  await expect(readyRoot).toHaveAttribute("data-ui-output-status", "verified");
  await expect(readyViewport).toHaveAttribute("tabindex", "0");

  await readyViewport.focus();
  await expect(readyViewport).toBeFocused();
  await page.keyboard.press("PageDown");

  await expect(readyRoot).toHaveAttribute("data-ui-action", "observe");
  await expect(readyRoot).toHaveAttribute("data-ui-state", "enabled");
  await expect(readyRoot).toHaveAttribute("data-ui-output-status", "verified");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  docsRoot = await resolveDocsRoot(page);
  readyRoot = docsRoot
    .locator('[data-slot="scroll-area"][data-orientation="vertical"][data-max-height="custom"]')
    .first();
  readyViewport = readyRoot.locator('[data-slot="scroll-area-viewport"]').first();

  await expect(readyRoot).toHaveAttribute("data-ui-action", "observe");
  await expect(readyRoot).toHaveAttribute("data-ui-state", "enabled");
  await expect(readyRoot).toHaveAttribute("data-ui-source", "legacy-prop");
  await expect(readyRoot).toHaveAttribute("data-ui-output-status", "verified");
  await expect(readyViewport).toHaveAttribute("tabindex", "0");

  await readyViewport.focus();
  await expect(readyViewport).toBeFocused();
  await page.keyboard.press("PageDown");

  await expect(readyRoot).toHaveAttribute("data-ui-action", "observe");
  await expect(readyRoot).toHaveAttribute("data-ui-state", "enabled");
  await expect(readyRoot).toHaveAttribute("data-ui-output-status", "verified");
});
