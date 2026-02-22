import { expect, test } from "@playwright/test";

test("docs-app form matrix keeps semantic state markers queryable with wasm-stable waits", async ({
  page,
}) => {
  await page.goto("/#/components/form");
  await page.locator("body:not(:has(#boot))").waitFor();

  const defaultForm = page
    .locator('[data-slot="form"]')
    .filter({ has: page.locator("#docs-form-matrix-default") })
    .first();
  const requiredForm = page
    .locator('[data-slot="form"]')
    .filter({ has: page.locator("#docs-form-matrix-required") })
    .first();
  const disabledForm = page
    .locator('[data-slot="form"]')
    .filter({ has: page.locator("#docs-form-matrix-disabled") })
    .first();
  const readOnlyForm = page
    .locator('[data-slot="form"]')
    .filter({ has: page.locator("#docs-form-matrix-readonly") })
    .first();

  await expect(defaultForm).toBeVisible();
  await expect(defaultForm).toHaveAttribute("data-label-position", "top");
  await expect(defaultForm).toHaveAttribute("data-label-align", "start");
  await expect(defaultForm).toHaveAttribute("data-state-source", "logic.rs::resolve_view_state");
  await expect(defaultForm).not.toHaveAttribute("data-required", "true");
  await expect(defaultForm).not.toHaveAttribute("data-disabled", "true");
  await expect(defaultForm).not.toHaveAttribute("data-readonly", "true");

  await expect(requiredForm).toBeVisible();
  await expect(requiredForm).toHaveAttribute("data-required", "true");
  await expect(requiredForm).toHaveAttribute("data-label-position", "left");
  await expect(requiredForm).toHaveAttribute("data-label-align", "end");

  await expect(disabledForm).toBeVisible();
  await expect(disabledForm).toHaveAttribute("data-disabled", "true");
  await expect(disabledForm).toHaveAttribute("aria-disabled", "true");

  await expect(readOnlyForm).toBeVisible();
  await expect(readOnlyForm).toHaveAttribute("data-readonly", "true");
});

test("docs-app form playground code panel remains copy-ready via semantic selectors", async ({
  page,
}) => {
  await page.goto("/#/components/form");
  await page.locator("body:not(:has(#boot))").waitFor();

  const playground = page
    .locator("section.playground")
    .filter({ has: page.locator("#docs-form-hello") })
    .first();
  await expect(playground).toBeVisible();

  const codeToggle = playground
    .getByRole("button", { name: /Show code|Hide code/ })
    .first();
  await expect(codeToggle).toBeVisible();

  const codeBlock = playground.locator('[data-slot="code-block"]').first();
  if ((await codeBlock.count()) === 0) {
    await codeToggle.click();
  }

  await expect(codeBlock).toBeVisible();
  await expect(codeBlock).toHaveAttribute("data-copyable", "true");

  const code = playground.locator('[data-slot="code-block-code"]').first();
  await expect(code).toContainText("use leptos::prelude::*;");
  await expect(code).toContainText("use ui::*;");
  await expect(code).toContainText("<Form>");
  await expect(code).toContainText("id=\"docs-form-hello\"");
});

test("docs-app form key flow is repeatable with semantic breakpoints for focus and keyboard paths", async ({
  page,
}) => {
  await page.goto("/#/components/form");
  await page.locator("body:not(:has(#boot))").waitFor();

  const workbenchForm = page
    .locator('[data-slot="form"]')
    .filter({ has: page.locator("#docs-form-name") })
    .first();
  const nameInput = page.locator("#docs-form-name");
  const emailInput = page.locator("#docs-form-email");

  await expect(workbenchForm).toBeVisible();
  await expect(workbenchForm).toHaveAttribute("data-state-source", "logic.rs::resolve_view_state");
  await expect(workbenchForm).not.toHaveAttribute("data-required", "true");
  await expect(workbenchForm).not.toHaveAttribute("data-disabled", "true");

  await nameInput.fill("Linus");
  await expect(nameInput).toHaveValue("Linus");
  await nameInput.focus();
  await expect(nameInput).toBeFocused();

  await page.keyboard.press("Tab");
  await expect(emailInput).toBeFocused();
  await emailInput.fill("linus@example.com");
  await expect(emailInput).toHaveValue("linus@example.com");

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  const reloadedWorkbenchForm = page
    .locator('[data-slot="form"]')
    .filter({ has: page.locator("#docs-form-name") })
    .first();
  const reloadedNameInput = page.locator("#docs-form-name");
  const reloadedEmailInput = page.locator("#docs-form-email");

  await expect(reloadedWorkbenchForm).toBeVisible();
  await expect(reloadedWorkbenchForm).toHaveAttribute(
    "data-state-source",
    "logic.rs::resolve_view_state",
  );
  await expect(reloadedWorkbenchForm).not.toHaveAttribute("data-required", "true");
  await expect(reloadedNameInput).toHaveValue("");
  await expect(reloadedEmailInput).toHaveValue("");
});
