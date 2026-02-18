import { expect, test } from "@playwright/test";

test("docs-app: underlay agent + streaming contract markers are stable", async ({
  page,
}) => {
  await page.goto("/#/components/underlay");
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(page.locator(".docs-page-title")).toBeVisible();

  const underlay = page.locator('#docs-underlay-ai[data-slot="underlay"]');
  const controls = page.locator('[data-slot="underlay-ai-controls"]');

  await expect(underlay).toHaveAttribute("data-ui-schema", "ui.underlay.agent-contract");
  await expect(underlay).toHaveAttribute("data-ui-schema-version", "1");
  await expect(underlay).toHaveAttribute("data-ui-intent", "overlay-dismiss");
  await expect(underlay).toHaveAttribute("data-ui-stream-support", "optional");
  await expect(underlay).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(underlay).toHaveAttribute("data-ui-stream-mode", "snapshot");
  await expect(underlay).toHaveAttribute("data-ui-output-status", "verified");

  await controls.locator('[data-action="toggle-mode"]').click();
  await expect(underlay).toHaveAttribute("data-ui-stream-mode", "streaming");

  await controls.locator('[data-action="cycle-status"]').click();
  await expect(underlay).toHaveAttribute("data-ui-output-status", "submittable");

  await controls.locator('[data-action="open"]').click();
  await expect(underlay).toHaveAttribute("data-state", "open");
  await expect(underlay).toHaveAttribute("data-open", "true");

  await underlay.click();
  await expect(underlay).toHaveAttribute("data-state", "closed");
  await expect(underlay).not.toHaveAttribute("data-open", "true");
});
