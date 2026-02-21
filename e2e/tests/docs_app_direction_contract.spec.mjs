import { expect, test } from "@playwright/test";

test("docs-app direction provider exposes stable semantic selector contracts", async ({
  page,
}) => {
  await page.goto("/#/components/direction-provider");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-component="direction-provider"]');
  await expect(root).toHaveCount(1);
  await expect(root).toBeVisible();

  const stateMatrix = root.locator('[data-slot="direction-state-matrix"]');
  await expect(stateMatrix).toBeVisible();

  const ltr = stateMatrix.locator(
    '[data-slot="direction-provider"][data-direction="ltr"][data-direction-source="direction"]'
  );
  await expect(ltr).toHaveCount(1);
  await expect(ltr).toHaveAttribute("dir", "ltr");
  await expect(ltr).toHaveAttribute("data-direction", "ltr");
  await expect(ltr).toHaveAttribute("data-direction-source", "direction");

  const rtl = stateMatrix.locator(
    '[data-slot="direction-provider"][data-direction="rtl"][data-direction-source="direction"]'
  );
  await expect(rtl).toHaveCount(1);
  await expect(rtl).toHaveAttribute("dir", "rtl");
  await expect(rtl).toHaveAttribute("data-direction", "rtl");
  await expect(rtl).toHaveAttribute("data-direction-source", "direction");

  const fallback = stateMatrix.locator(
    '[data-slot="direction-provider"][data-direction="ltr"][data-direction-source="default"]'
  );
  await expect(fallback).toHaveCount(1);
  await expect(fallback).toHaveAttribute("dir", "ltr");
  await expect(fallback).toHaveAttribute("data-direction-source", "default");
});

test("docs-app direction provider route flow is repeatable", async ({ page }) => {
  await page.goto("/#/components/direction-provider");
  await page.locator("body:not(:has(#boot))").waitFor();

  const snapshotReady = page.locator(
    '[data-component="direction-provider"] [data-slot="direction-snapshot-demo"]'
  );
  await expect(snapshotReady).toBeVisible();

  const snapshotProvider = page.locator(
    '[data-component="direction-provider"] [data-slot="direction-provider"][lang="en"][data-direction="ltr"][data-direction-source="direction"]'
  );
  await expect(snapshotProvider).toHaveCount(1);
  await expect(snapshotProvider).toHaveAttribute("dir", "ltr");
  await expect(snapshotProvider).toHaveAttribute("lang", "en");

  await page.goto("/#/components/spacer");
  await page.locator("body:not(:has(#boot))").waitFor();
  await page.goto("/#/components/direction-provider");
  await page.locator("body:not(:has(#boot))").waitFor();

  await expect(snapshotReady).toBeVisible();
  await expect(snapshotProvider).toHaveAttribute("dir", "ltr");
  await expect(snapshotProvider).toHaveAttribute("lang", "en");
});

test("docs-app direction provider interactive playground updates semantic markers deterministically", async ({
  page,
}) => {
  await page.goto("/#/components/direction-provider");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-component="direction-provider"]');
  const interactive = root.locator('[data-slot="direction-interactive-playground"]');
  await expect(interactive).toBeVisible();

  const preview = interactive.locator('[data-slot="direction-interactive-preview"]');
  const provider = preview.locator('[data-slot="direction-provider"]');
  const feedback = interactive.locator('[data-slot="direction-interactive-feedback"]');

  await expect(provider).toHaveCount(1);
  await expect(provider).toHaveAttribute("data-direction", "ltr");
  await expect(provider).toHaveAttribute("data-direction-source", "direction");
  await expect(feedback).toHaveAttribute("data-current-source", "direction");
  await expect(feedback).toHaveAttribute("data-current-direction", "ltr");

  await interactive.locator('[data-slot="direction-interactive-value-rtl"]').click();
  await expect(provider).toHaveAttribute("data-direction", "rtl");

  await interactive
    .locator('[data-slot="direction-interactive-source-dir-alias"]')
    .click();
  await expect(provider).toHaveAttribute("data-direction-source", "dir-alias");
  await expect(provider).toHaveAttribute("dir", "rtl");
  await expect(feedback).toHaveAttribute("data-current-source", "dir-alias");

  await interactive.locator('[data-slot="direction-interactive-lang-ar"]').click();
  await expect(provider).toHaveAttribute("lang", "ar");
  await expect(feedback).toHaveAttribute("data-current-lang", "ar");

  await interactive
    .locator('[data-slot="direction-interactive-class-toggle"]')
    .click();
  await expect(provider).toHaveClass(/docs-direction-rtl/);
  await expect(feedback).toHaveAttribute("data-current-class", "docs-direction-rtl");

  await interactive
    .locator('[data-slot="direction-interactive-source-default"]')
    .click();
  await expect(provider).toHaveAttribute("data-direction-source", "default");
  await expect(provider).toHaveAttribute("data-direction", "ltr");
  await expect(feedback).toHaveAttribute("data-current-source", "default");
  await expect(feedback).toHaveAttribute("data-current-direction", "ltr");

  await interactive.locator('[data-slot="direction-interactive-reset"]').click();
  await expect(provider).toHaveAttribute("data-direction-source", "direction");
  await expect(provider).toHaveAttribute("data-direction", "ltr");
  await expect(feedback).toHaveAttribute("data-current-source", "direction");
  await expect(feedback).toHaveAttribute("data-current-direction", "ltr");
});
