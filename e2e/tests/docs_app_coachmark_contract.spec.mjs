import { expect, test } from "@playwright/test";

const COACHMARK_PAGE = "/#/components/coachmark";

async function gotoCoachmark(page) {
  await page.goto(COACHMARK_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

async function resolveOpenCoachmarkContent(page, mode = "uncontrolled") {
  const root = page
    .locator(
      `[data-component="coachmark"] [data-slot="coachmark-content"][data-open-mode="${mode}"]`,
    )
    .first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-ui-schema", "ui.coachmark.agent-contract.v1");
  await expect(root).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(root).toHaveAttribute("data-ui-stream-mode", "snapshot");
  return root;
}

test("docs-app coachmark uses semantic selectors with wasm-stable ready waits", async ({ page }) => {
  await gotoCoachmark(page);
  const content = await resolveOpenCoachmarkContent(page, "uncontrolled");

  await expect(content).toHaveAttribute("data-state", "enabled");
  await expect(content).toHaveAttribute("data-asset", "present");
  await expect(content).toHaveAttribute("data-asset-source", "variant");
  await expect(content).toHaveAttribute("data-cta", "dual");
  await expect(content).toHaveAttribute("data-steps", "present");
  await expect(content).toHaveAttribute("data-ui-intent", "guided-tour");
  await expect(content).toHaveAttribute("data-ui-action", "navigate-step");
  await expect(content).toHaveAttribute("data-ui-output-status", "verified");
});

test("docs-app coachmark key flow keeps controlled markers stable", async ({ page }) => {
  await gotoCoachmark(page);

  const toggle = page.getByRole("button", { name: "Toggle controlled coachmark" }).first();
  await expect(toggle).toBeVisible();
  await toggle.click();

  const content = await resolveOpenCoachmarkContent(page, "controlled");
  await expect(content).toHaveAttribute("data-asset-source", "image");
  await expect(content).toHaveAttribute("data-actions", "present");
  await expect(content).toHaveAttribute("data-ui-source", "external");
  await expect(content).toHaveAttribute("data-stream-mode", "snapshot");
  await expect(content).toHaveAttribute("data-output-status", "verified");
});

test("docs-app coachmark key flow is repeatable after reload", async ({ page }) => {
  await gotoCoachmark(page);

  const toggle = page.getByRole("button", { name: "Toggle controlled coachmark" }).first();
  await toggle.click();
  let content = await resolveOpenCoachmarkContent(page, "controlled");
  await expect(content).toHaveAttribute("data-ui-source", "external");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  content = await resolveOpenCoachmarkContent(page, "uncontrolled");
  await expect(content).toHaveAttribute("data-open-mode", "uncontrolled");
  await expect(content).toHaveAttribute("data-ui-source", "internal");
});
