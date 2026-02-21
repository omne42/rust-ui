import { expect, test } from "@playwright/test";

const FLIP_CARD_PAGE = "/#/components/flip-card";
const MARKER_ROOT =
  '[data-component="flip-card"] #docs-flip-card[data-slot="flip-card"][data-ui-schema="ui.flip-card.agent-contract"]';
const TOGGLE_ROOT =
  '[data-component="flip-card"] #docs-flip-card-toggle[data-slot="flip-card"][data-flip-mode="toggle"]';
const DISABLED_ROOT =
  '[data-component="flip-card"] #docs-flip-card-disabled[data-slot="flip-card"][data-disabled="true"]';

async function gotoFlipCard(page) {
  await page.goto(FLIP_CARD_PAGE);
  await page.locator("body:not(:has(#boot))").waitFor();
}

async function resolveToggleRoot(page) {
  const root = page.locator(TOGGLE_ROOT).first();
  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("data-ui-schema", "ui.flip-card.agent-contract");
  await expect(root).toHaveAttribute("data-flip-mode", "toggle");
  await expect(root).toHaveAttribute("data-state", "default");
  await expect(root).toHaveAttribute("data-ui-state", "default");
  await expect(root).toHaveAttribute("data-ui-action", "snapshot-render");
  await expect(root).toHaveAttribute("aria-pressed", "false");
  return root;
}

async function runToggleKeyFlow(page, root) {
  await root.focus();
  await expect(root).toBeFocused();
  await expect(root).toHaveAttribute("data-ui-action", "focus");

  await page.keyboard.press("Enter");
  await expect(root).toHaveAttribute("data-ui-action", "toggle");
  await expect(root).toHaveAttribute("data-state", "flipped");
  await expect(root).toHaveAttribute("data-ui-state", "flipped");
  await expect(root).toHaveAttribute("aria-pressed", "true");

  await page.keyboard.press("Space");
  await expect(root).toHaveAttribute("data-state", "default");
  await expect(root).toHaveAttribute("data-ui-state", "default");
  await expect(root).toHaveAttribute("aria-pressed", "false");
}

async function resolveMarkerRoot(page) {
  const root = page.locator(MARKER_ROOT).first();
  const front = root.locator('[data-slot="flip-card-front"]').first();
  const back = root.locator('[data-slot="flip-card-back"]').first();

  await expect(root).toBeVisible();
  await expect(root).toHaveAttribute("role", "button");
  await expect(root).toHaveAttribute("data-ui-schema-version", "v1");
  await expect(root).toHaveAttribute("data-ui-intent", "flip.interaction");
  await expect(root).toHaveAttribute("data-ui-source", "state-primitives");
  await expect(root).toHaveAttribute("data-ui-config-policy", "whitelist");
  await expect(root).toHaveAttribute("data-ui-action", "snapshot-render");

  await expect(root).toHaveAttribute("data-state", "default");
  await expect(root).toHaveAttribute("data-visible", "default");
  await expect(root).toHaveAttribute("data-flip-mode", "hover");
  await expect(root).toHaveAttribute("data-flip-mode-source", "is_flip_on_hover");
  await expect(root).toHaveAttribute("data-motion-source", "custom");
  await expect(root).toHaveAttribute("data-class-source", "custom");
  await expect(root).toHaveAttribute("data-id-source", "custom");
  await expect(root).toHaveAttribute("data-ui-state", "default");
  await expect(root).toHaveAttribute("aria-pressed", "false");

  await expect(front).toHaveAttribute("data-visible", "true");
  await expect(front).toHaveAttribute("data-hidden", "false");
  await expect(back).toHaveAttribute("data-visible", "false");
  await expect(back).toHaveAttribute("data-hidden", "true");

  return { root, front, back };
}

test("docs-app flip-card uses semantic selectors with wasm-safe stable readiness waits", async ({
  page,
}) => {
  await gotoFlipCard(page);
  await resolveMarkerRoot(page);
});

test("docs-app flip-card animation path covers ready and settled semantic breakpoints", async ({
  page,
}) => {
  await gotoFlipCard(page);

  const { root, front, back } = await resolveMarkerRoot(page);
  await root.hover();
  await expect(root).toHaveAttribute("data-ui-action", "hover-enter");
  await expect(root).toHaveAttribute("data-hovered", "true");
  await expect(root).toHaveAttribute("data-flipped", "true");
  await expect(root).toHaveAttribute("data-state", "flipped");
  await expect(root).toHaveAttribute("data-ui-state", "flipped");
  await expect(root).toHaveAttribute("aria-pressed", "true");
  await expect(front).toHaveAttribute("data-visible", "false");
  await expect(front).toHaveAttribute("data-hidden", "true");
  await expect(back).toHaveAttribute("data-visible", "true");
  await expect(back).toHaveAttribute("data-hidden", "false");

  await page.locator("body").hover();
  await expect(root).toHaveAttribute("data-ui-action", "hover-leave");
  await expect(root).toHaveAttribute("data-state", "default");
  await expect(root).toHaveAttribute("data-ui-state", "default");
  await expect(root).toHaveAttribute("aria-pressed", "false");
  await expect(root).not.toHaveAttribute("data-hovered", "true");
  await expect(front).toHaveAttribute("data-visible", "true");
  await expect(front).toHaveAttribute("data-hidden", "false");
  await expect(back).toHaveAttribute("data-visible", "false");
  await expect(back).toHaveAttribute("data-hidden", "true");

  const toggleRoot = await resolveToggleRoot(page);
  await runToggleKeyFlow(page, toggleRoot);

  const disabledRoot = page.locator(DISABLED_ROOT).first();
  await expect(disabledRoot).toBeVisible();
  await expect(disabledRoot).toHaveAttribute("aria-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("tabindex", "-1");
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("data-ui-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("data-ui-action", "snapshot-render");
  await disabledRoot.click();
  await expect(disabledRoot).toHaveAttribute("data-state", "default");
  await expect(disabledRoot).toHaveAttribute("data-ui-action", "snapshot-render");
});

test("docs-app flip-card key flow is repeatable and failures map to semantic breakpoints", async ({
  page,
}) => {
  await gotoFlipCard(page);

  let toggleRoot = await resolveToggleRoot(page);
  await runToggleKeyFlow(page, toggleRoot);

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();

  toggleRoot = await resolveToggleRoot(page);
  await runToggleKeyFlow(page, toggleRoot);
});

test("docs-app flip-card high-risk paths keep focus keyboard and disabled branches semantically explicit", async ({
  page,
}) => {
  await gotoFlipCard(page);

  const toggleRoot = await resolveToggleRoot(page);
  await toggleRoot.focus();
  await expect(toggleRoot).toBeFocused();
  await expect(toggleRoot).toHaveAttribute("data-ui-action", "focus");
  await page.keyboard.press("Enter");
  await expect(toggleRoot).toHaveAttribute("data-ui-action", "toggle");
  await expect(toggleRoot).toHaveAttribute("data-state", "flipped");
  await expect(toggleRoot).toHaveAttribute("data-ui-state", "flipped");
  await expect(toggleRoot).toHaveAttribute("aria-pressed", "true");
  await page.locator("body").click();
  await expect(toggleRoot).toHaveAttribute("data-ui-action", "blur");
  await expect(toggleRoot).toHaveAttribute("data-state", "flipped");
  await expect(toggleRoot).toHaveAttribute("data-ui-state", "flipped");

  const disabledRoot = page.locator(DISABLED_ROOT).first();
  await expect(disabledRoot).toBeVisible();
  await expect(disabledRoot).toHaveAttribute("data-disabled", "true");
  await expect(disabledRoot).toHaveAttribute("data-ui-state", "disabled");
  await expect(disabledRoot).toHaveAttribute("aria-disabled", "true");
  await disabledRoot.click();
  await expect(disabledRoot).toHaveAttribute("data-state", "default");
  await expect(disabledRoot).toHaveAttribute("data-ui-action", "snapshot-render");
});
