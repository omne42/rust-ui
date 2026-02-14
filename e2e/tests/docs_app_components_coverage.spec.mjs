import { expect, test } from "@playwright/test";

const coverageMode = process.env.E2E_COVERAGE ?? "sample";
const sampleLimit = Number.parseInt(process.env.E2E_SAMPLE_LIMIT ?? "20", 10);

function uniq(values) {
  return [...new Set(values)];
}

test("docs-app components pages render playgrounds (sample)", async ({ page }) => {
  test.skip(coverageMode !== "sample", "set E2E_COVERAGE=sample to run");

  await page.goto("/#/components");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-component-grid")).toBeVisible();

  const hrefs = await page.locator("a.docs-component-tile").evaluateAll((els) =>
    els
      .map((el) => el.getAttribute("href"))
      .filter((value) => typeof value === "string" && value.includes("/components/"))
  );

  const slugs = uniq(
    hrefs
      .map((href) => href.split("/components/")[1])
      .map((slug) => slug?.trim())
      .filter(Boolean)
  );

  expect(slugs.length).toBeGreaterThan(0);
  const limit = Number.isFinite(sampleLimit) ? Math.max(1, sampleLimit) : 20;

  for (const slug of slugs.slice(0, limit)) {
    await page.goto(`/#/components/${slug}`);
    await page.locator("body:not(:has(#boot))").waitFor();
    await expect(page.locator(".docs-page-title")).toBeVisible();
    await expect(page.locator("section.playground").first()).toBeVisible();
  }
});

test("docs-app components pages render playgrounds (all)", async ({ page }) => {
  test.skip(coverageMode !== "all", "set E2E_COVERAGE=all to run");
  test.slow();

  await page.goto("/#/components");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-component-grid")).toBeVisible();

  const hrefs = await page.locator("a.docs-component-tile").evaluateAll((els) =>
    els
      .map((el) => el.getAttribute("href"))
      .filter((value) => typeof value === "string" && value.includes("/components/"))
  );

  const slugs = uniq(
    hrefs
      .map((href) => href.split("/components/")[1])
      .map((slug) => slug?.trim())
      .filter(Boolean)
  );

  expect(slugs.length).toBeGreaterThan(0);

  for (const slug of slugs) {
    await page.goto(`/#/components/${slug}`);
    await page.locator("body:not(:has(#boot))").waitFor();
    await expect(page.locator(".docs-page-title")).toBeVisible();
    await expect(page.locator("section.playground").first()).toBeVisible();
  }
});
