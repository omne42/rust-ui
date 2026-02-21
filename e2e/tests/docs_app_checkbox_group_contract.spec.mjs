import { expect, test } from "@playwright/test";

const CHECKBOX_GROUP_PAGE = "/#/components/checkbox-group";
const WASM_READY = "body:not(:has(#boot))";

async function gotoCheckboxGroupDocsAndWaitReady(page) {
  await page.goto(CHECKBOX_GROUP_PAGE);
  await page.locator(WASM_READY).waitFor();

  const validationGroup = page
    .locator('#docs-checkbox-group[data-slot="checkbox-group"]')
    .first();
  const groupCheckboxes = validationGroup
    .locator('[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"]');

  await expect(validationGroup).toBeVisible();
  await expect(validationGroup).toHaveAttribute("data-ui-schema", "ui.checkbox-group.agent-contract");
  await expect(validationGroup).toHaveAttribute("data-ui-stream-fallback", "snapshot");
  await expect(validationGroup).toHaveAttribute("data-ui-output-status", "verified");
  await expect(validationGroup).toHaveAttribute("data-state-source", "semantic-props");
  await expect(validationGroup).toHaveAttribute("data-motion-phase", "inactive");
  await expect(groupCheckboxes).toHaveCount(3);

  return { validationGroup, groupCheckboxes };
}

test("docs-app checkbox-group contract uses semantic selectors with wasm-stable ready waits", async ({
  page,
}) => {
  const { validationGroup } = await gotoCheckboxGroupDocsAndWaitReady(page);
  const checkedBoxes = validationGroup
    .locator('[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"][aria-checked="true"]');
  const uncheckedBoxes = validationGroup
    .locator('[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"][aria-checked="false"]');

  await expect(validationGroup).toHaveAttribute("data-valid", "true");
  await expect(checkedBoxes).toHaveCount(1);
  await expect(uncheckedBoxes).toHaveCount(2);
});

test("docs-app checkbox-group covers ready/settled semantic breakpoints for validation and motion paths", async ({
  page,
}) => {
  let { validationGroup } = await gotoCheckboxGroupDocsAndWaitReady(page);
  const bananaCheckbox = validationGroup
    .locator('[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"][aria-checked="true"]')
    .first();

  await expect(validationGroup).toHaveAttribute("data-valid", "true");
  await bananaCheckbox.click();

  await expect(validationGroup).toHaveAttribute("data-invalid", "true");
  await expect(validationGroup).toHaveAttribute("data-shows-error", "true");
  await expect(validationGroup).toHaveAttribute("data-ui-action", "render-semantic-with-error");
  await expect(validationGroup).toHaveAttribute("data-ui-state", "enabled-invalid");
  await expect(validationGroup).toHaveAttribute("data-motion-phase", "active");

  const recoveryCheckbox = validationGroup
    .locator('[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"][aria-checked="false"]')
    .first();
  await recoveryCheckbox.click();
  await expect(validationGroup).toHaveAttribute("data-valid", "true");
  await expect(validationGroup).toHaveAttribute("data-ui-action", "render-semantic");
  await expect(validationGroup).toHaveAttribute("data-ui-state", "enabled-valid");
  await expect(validationGroup).toHaveAttribute("data-motion-phase", "inactive");

  await page.reload();
  ({ validationGroup } = await gotoCheckboxGroupDocsAndWaitReady(page));
  await expect(validationGroup).toHaveAttribute("data-valid", "true");
  await expect(
    validationGroup.locator(
      '[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"][aria-checked="true"]'
    )
  ).toHaveCount(1);
});

test("docs-app checkbox-group key flow is repeatable and failures map to semantic breakpoints", async ({
  page,
}) => {
  await page.goto(CHECKBOX_GROUP_PAGE);
  await page.locator(WASM_READY).waitFor();

  for (const cycle of [1, 2]) {
    const validationGroup = page
      .locator('#docs-checkbox-group[data-slot="checkbox-group"]')
      .first();
    const checkedBoxes = validationGroup
      .locator('[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"][aria-checked="true"]');
    const uncheckedBoxes = validationGroup
      .locator('[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"][aria-checked="false"]');

    await expect(validationGroup).toHaveAttribute("data-valid", "true");
    await expect(checkedBoxes).toHaveCount(1);
    await expect(uncheckedBoxes).toHaveCount(2);

    const firstUnchecked = uncheckedBoxes.first();
    await firstUnchecked.focus();
    await expect(firstUnchecked).toBeFocused();
    await page.keyboard.press("Space");

    await expect(validationGroup).toHaveAttribute("data-valid", "true");
    await expect(checkedBoxes).toHaveCount(2);
    await expect(uncheckedBoxes).toHaveCount(1);

    const firstChecked = checkedBoxes.first();
    await firstChecked.focus();
    await expect(firstChecked).toBeFocused();
    await page.keyboard.press("Space");

    await expect(validationGroup).toHaveAttribute("data-invalid", "true");
    await expect(validationGroup).toHaveAttribute("data-shows-error", "true");
    await expect(validationGroup).toHaveAttribute("data-ui-action", "render-semantic-with-error");
    await expect(validationGroup).toHaveAttribute("data-ui-state", "enabled-invalid");
    await expect(validationGroup).toHaveAttribute("data-motion-phase", "active");

    const recoveryCheckbox = uncheckedBoxes.first();
    await recoveryCheckbox.focus();
    await expect(recoveryCheckbox).toBeFocused();
    await page.keyboard.press("Space");

    await expect(validationGroup).toHaveAttribute("data-valid", "true");
    await expect(validationGroup).toHaveAttribute("data-ui-action", "render-semantic");
    await expect(validationGroup).toHaveAttribute("data-ui-state", "enabled-valid");
    await expect(validationGroup).toHaveAttribute("data-motion-phase", "inactive");

    if (cycle === 1) {
      await page.reload();
      await page.locator(WASM_READY).waitFor();
    }
  }
});

test("docs-app checkbox-group high-risk paths cover focus keyboard and disabled semantic breakpoints", async ({
  page,
}) => {
  const { validationGroup } = await gotoCheckboxGroupDocsAndWaitReady(page);

  const trigger = validationGroup
    .locator('[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"][aria-checked="false"]')
    .first();
  await trigger.focus();
  await expect(trigger).toBeFocused();
  await page.keyboard.press("Space");

  await expect(validationGroup).toHaveAttribute("data-ui-action", "render-semantic");
  await expect(validationGroup).toHaveAttribute("data-ui-state", "enabled-valid");

  const disabledGroup = page
    .locator('#docs-checkbox-group-disabled[data-slot="checkbox-group"]')
    .first();
  const disabledCheckboxes = disabledGroup
    .locator('[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"]');

  await expect(disabledGroup).toBeVisible();
  await expect(disabledGroup).toHaveAttribute("data-disabled", "true");
  await expect(disabledGroup).toHaveAttribute("data-ui-state", "disabled-valid");
  await expect(disabledCheckboxes).toHaveCount(2);
  await expect(disabledCheckboxes.first()).toHaveAttribute("aria-disabled", "true");
  await expect(disabledCheckboxes.nth(1)).toHaveAttribute("aria-disabled", "true");
  await expect(
    disabledGroup.locator(
      '[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"][aria-checked="true"]'
    )
  ).toHaveCount(1);
  await expect(
    disabledGroup.locator(
      '[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"][aria-checked="false"]'
    )
  ).toHaveCount(1);

  await disabledCheckboxes.first().click({ force: true });
  await disabledCheckboxes.nth(1).click({ force: true });
  await expect(disabledGroup).toHaveAttribute("data-ui-state", "disabled-valid");
  await expect(
    disabledGroup.locator(
      '[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"][aria-checked="true"]'
    )
  ).toHaveCount(1);
  await expect(
    disabledGroup.locator(
      '[data-slot="checkbox-group-list"] [data-slot="checkbox"][role="checkbox"][aria-checked="false"]'
    )
  ).toHaveCount(1);
});
