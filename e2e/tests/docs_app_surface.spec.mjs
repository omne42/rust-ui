import { expect, test } from "@playwright/test";

test("docs-app: surface contract markers are observable", async ({ page }) => {
  await test.step("route-open-and-wasm-ready", async () => {
    await page.goto("/#/components/surface");
    await page.locator("body:not(:has(#boot))").waitFor();
  });

  const surfaces = page.locator('section[data-slot="surface"][data-state]');
  await test.step("surface-list-visible", async () => {
    await expect(surfaces.first()).toHaveAttribute("data-slot", "surface");
    await expect(surfaces).toHaveCount(4);
  });

  await test.step("checkpoint-default-raised", async () => {
    const first = surfaces.nth(0);
    await expect(first).toHaveAttribute("role", "region");
    await expect(first).toHaveAttribute("data-tone", "default");
    await expect(first).toHaveAttribute("data-elevation", "raised");
    await expect(first).toHaveAttribute("data-state", "padded");
    await expect(first).toHaveAttribute("data-aria-source", "default");
  });

  await test.step("checkpoint-subtle-bordered", async () => {
    const second = surfaces.nth(1);
    await expect(second).toHaveAttribute("data-bordered", "true");
    await expect(second).toHaveAttribute("data-bordered-source", "is-prop");
  });

  await test.step("checkpoint-strong-plain", async () => {
    const third = surfaces.nth(2);
    await expect(await third.getAttribute("data-padded")).toBeNull();
    await expect(third).toHaveAttribute("data-state", "plain");
    await expect(third).toHaveAttribute("data-padded-source", "is-prop");
  });

  await test.step("checkpoint-custom-aria-and-class", async () => {
    const custom = surfaces.nth(3);
    await expect(custom).toHaveAttribute("data-aria-source", "custom");
    await expect(custom).toHaveAttribute("data-class-source", "custom");
    await expect(custom).toHaveAttribute("data-custom-class", "true");
  });
});

test("docs-app: surface key flow is repeatable after reload", async ({ page }) => {
  await page.goto("/#/components/surface");
  await page.locator("body:not(:has(#boot))").waitFor();

  const firstSurface = page.locator('section[data-slot="surface"][data-state]').first();
  await expect(firstSurface).toHaveAttribute("data-state", "padded");
  await expect(firstSurface).toHaveAttribute("data-tone", "default");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedFirstSurface = page.locator('section[data-slot="surface"][data-state]').first();
  await expect(reloadedFirstSurface).toHaveAttribute("data-state", "padded");
  await expect(reloadedFirstSurface).toHaveAttribute("data-tone", "default");
});
