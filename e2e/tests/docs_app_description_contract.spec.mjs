import { expect, test } from "@playwright/test";

const DESCRIPTION_PAGE = "/#/components/description";

async function openDescriptionDocs(page) {
  await page.goto(DESCRIPTION_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();

  const docsRoot = page.locator('[data-component="description"][data-slot="description"]').first();
  await expect(docsRoot).toBeVisible();
  return docsRoot;
}

function descriptionLocators(docsRoot) {
  const defaultDescription = docsRoot
    .locator('[data-slot="description"][data-tone="default"][data-state="default"]')
    .first();
  const customAria = docsRoot
    .locator('[data-slot="description"][data-aria-source="custom"]')
    .first();
  const customClass = docsRoot
    .locator('[data-slot="description"][data-class-source="custom"][data-custom-class="true"]')
    .first();

  return { defaultDescription, customAria, customClass };
}

test("docs-app description uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const docsRoot = await openDescriptionDocs(page);
  const { defaultDescription, customAria, customClass } = descriptionLocators(docsRoot);

  await expect(defaultDescription).toHaveAttribute("data-ui-schema", "ui.description.agent-contract.v1");
  await expect(defaultDescription).toHaveAttribute("data-ui-intent", "text-assistance");
  await expect(defaultDescription).toHaveAttribute("data-ui-action", "render-snapshot");
  await expect(defaultDescription).toHaveAttribute("data-ui-stream-support", "optional");
  await expect(defaultDescription).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(defaultDescription).toHaveAttribute("data-ui-output-status", "verified");
  await expect(defaultDescription).toHaveAttribute("data-aria-source", "default");
  await expect(defaultDescription).toHaveAttribute("data-class-source", "default");
  await expect(defaultDescription).toHaveAttribute("aria-label", "Description");
  await expect(defaultDescription).not.toHaveAttribute("role", /.+/);

  await expect(customAria).toHaveAttribute("aria-label", "Name helper");
  await expect(customAria).toHaveAttribute("data-aria-source", "custom");
  await expect(customClass).toHaveAttribute("data-class-source", "custom");
  await expect(customClass).toHaveAttribute("data-custom-class", "true");
});

test("docs-app description key flow remains repeatable with semantic ready checkpoints", async ({
  page,
}) => {
  const docsRoot = await openDescriptionDocs(page);
  const { defaultDescription } = descriptionLocators(docsRoot);

  await expect(defaultDescription).toHaveAttribute("data-state", "default");
  await expect(defaultDescription).toHaveAttribute("data-ui-state", "default");
  await expect(defaultDescription).toHaveAttribute("data-ui-output-status", "verified");

  await page.goto("/#/components/error-message");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(
    page.locator('[data-component="error-message"][data-slot="error-message"]').first(),
  ).toBeVisible();

  const reloadedRoot = await openDescriptionDocs(page);
  const { defaultDescription: reloadedDefaultDescription } =
    descriptionLocators(reloadedRoot);
  await expect(reloadedDefaultDescription).toHaveAttribute("data-state", "default");
  await expect(reloadedDefaultDescription).toHaveAttribute("data-ui-state", "default");
  await expect(reloadedDefaultDescription).toHaveAttribute("data-ui-output-status", "verified");
});
