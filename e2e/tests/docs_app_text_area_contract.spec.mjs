import { expect, test } from "@playwright/test";

test("docs-app text-area exposes stable markers for controlled value and invalid toggle", async ({
  page,
}) => {
  await page.goto("/#/components/text-area");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-slot="text-area"]').filter({
    has: page.locator("#docs-text-area-markers"),
  });
  const input = root.locator("#docs-text-area-markers");
  const toggleInvalid = page.getByRole("button", { name: "Mark marker invalid" });

  await expect(root).toBeVisible();
  await expect(input).toBeVisible();
  await expect(root).toHaveAttribute("data-motion-source", "custom");
  await expect(root).toHaveAttribute("data-value-control-mode", "controlled");
  await expect(root).toHaveAttribute("data-default-value-source", "custom");
  await expect(root).toHaveAttribute("data-value-change-source", "on_value_change");
  await expect(root).toHaveAttribute("data-requirement", "required");
  await expect(root).toHaveAttribute("data-label-source", "custom");
  await expect(root).toHaveAttribute("data-description-source", "custom");
  await expect(root).toHaveAttribute("data-error-source", "custom");
  await expect(root).toHaveAttribute("data-placeholder-source", "custom");
  await expect(root).toHaveAttribute("data-rows-source", "custom");
  await expect(root).toHaveAttribute("data-state", "ready");
  await expect(root).toHaveAttribute("data-value", "filled");

  await input.fill("updated release notes");
  await expect(input).toHaveValue("updated release notes");
  await expect(root).toHaveAttribute("data-value", "filled");

  await toggleInvalid.click();
  await expect(root).toHaveAttribute("data-state", "invalid");
  await expect(root).toHaveAttribute("data-invalid", "true");
  await expect(page.getByText("Release notes are required")).toBeVisible();

  await toggleInvalid.focus();
  await page.keyboard.press("Enter");
  await expect(root).toHaveAttribute("data-state", "ready");
});
