import { expect, test } from "@playwright/test";

test("docs-app ripple contract uses stable semantic selectors", async ({ page }) => {
  await page.goto("/#/components/motion-ripple");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-component="motion-ripple"]').first();
  await expect(root).toBeVisible();

  const ripples = root.locator('[data-slot="ripple"][data-ui-schema="ripple.v1"]');
  await expect(ripples).toHaveCount(6);

  const defaultRipple = root
    .locator('[data-slot="ripple"][data-boundary="bounded"][data-state="animated"][data-duration-ms="180"]')
    .first();
  const slowRipple = root
    .locator('[data-slot="ripple"][data-boundary="bounded"][data-state="animated"][data-duration-ms="880"]')
    .first();
  const staticRipple = root
    .locator('[data-slot="ripple"][data-boundary="bounded"][data-state="static"][data-duration-ms="180"]')
    .first();
  const customRipple = root
    .locator('[data-slot="ripple"][data-boundary="bounded"][data-state="animated"][data-duration-ms="620"]')
    .first();
  const unboundedRipple = root
    .locator('[data-slot="ripple"][data-boundary="unbounded"][data-state="animated"][data-duration-ms="520"]')
    .first();

  await expect(defaultRipple).toHaveAttribute("aria-hidden", "true");
  await expect(defaultRipple).toHaveAttribute("data-motion-source", "default");
  await expect(slowRipple).toHaveAttribute("data-motion-source", "custom");
  await expect(staticRipple).toHaveAttribute("data-motion-source", "custom");
  await expect(staticRipple).toHaveAttribute("data-custom-motion", "true");
  await expect(customRipple).toHaveAttribute("data-class-source", "custom");
  await expect(unboundedRipple).toHaveAttribute("data-class-source", "custom");
});

test("docs-app ripple pointer flow uses wasm-ready and settled waits", async ({ page }) => {
  await page.goto("/#/components/motion-ripple");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-component="motion-ripple"]').first();
  await expect(root).toBeVisible();

  const unboundedRipple = root
    .locator('[data-slot="ripple"][data-boundary="unbounded"][data-duration-ms="520"]')
    .first();
  await expect(unboundedRipple).toHaveAttribute("data-state", "animated");

  await expect
    .poll(
      () =>
        unboundedRipple.evaluate((el) => {
          const trigger = el.closest("button");
          return trigger ? "present" : "missing";
        }),
      { timeout: 5_000 },
    )
    .toBe("present");

  await unboundedRipple.evaluate((el) => {
    const trigger = el.closest("button");
    if (!trigger) {
      throw new Error("unbounded ripple trigger button missing");
    }
    trigger.click();
  });

  await expect
    .poll(
      () =>
        unboundedRipple.evaluate(
          (el) => el.style.getPropertyValue("--ui-ripple-origin-x").trim(),
        ),
      { timeout: 5_000 },
    )
    .toBe("18%");

  await expect
    .poll(
      () =>
        unboundedRipple.evaluate(
          (el) => el.style.getPropertyValue("--ui-ripple-origin-y").trim(),
        ),
      { timeout: 5_000 },
    )
    .toBe("48%");

  await expect
    .poll(
      () =>
        unboundedRipple.evaluate(
          (el) => el.style.getPropertyValue("--ui-ripple-duration-ms").trim(),
        ),
      { timeout: 5_000 },
    )
    .toBe("520ms");

  const prefersReducedMotion = await page.evaluate(() =>
    window.matchMedia("(prefers-reduced-motion: reduce)").matches,
  );

  if (!prefersReducedMotion) {
    await expect
      .poll(() => unboundedRipple.evaluate((el) => el.getAnimations().length), { timeout: 5_000 })
      .toBeGreaterThan(0);
  }

  await expect
    .poll(() => unboundedRipple.evaluate((el) => el.getAnimations().length), { timeout: 5_000 })
    .toBe(0);
});

test("docs-app ripple key flow is repeatable with semantic breakpoints", async ({ page }) => {
  await page.goto("/#/components/motion-ripple");
  await page.locator("body:not(:has(#boot))").waitFor();

  const root = page.locator('[data-component="motion-ripple"]').first();
  await expect(root).toBeVisible();

  const assertUnboundedRippleKeyboardFlow = async () => {
    const unboundedRipple = root
      .locator('[data-slot="ripple"][data-boundary="unbounded"][data-duration-ms="520"]')
      .first();

    await expect(unboundedRipple).toHaveAttribute("data-ui-schema", "ripple.v1");
    await expect(unboundedRipple).toHaveAttribute("data-boundary", "unbounded");
    await expect(unboundedRipple).toHaveAttribute("data-duration-ms", "520");
    await expect(unboundedRipple).toHaveAttribute("data-motion-source", "custom");
    await expect(unboundedRipple).toHaveAttribute("data-class-source", "custom");

    await expect
      .poll(
        () =>
          unboundedRipple.evaluate((el) => {
            const trigger = el.closest("button");
            return trigger ? "present" : "missing";
          }),
        { timeout: 5_000 },
      )
      .toBe("present");

    await unboundedRipple.evaluate((el) => {
      const trigger = el.closest("button");
      if (!trigger) {
        throw new Error("unbounded ripple trigger button missing");
      }
      trigger.focus();
    });

    await expect
      .poll(
        () =>
          unboundedRipple.evaluate((el) => {
            const trigger = el.closest("button");
            return trigger && document.activeElement === trigger ? "focused" : "not-focused";
          }),
        { timeout: 5_000 },
      )
      .toBe("focused");

    await page.keyboard.press("Enter");

    await expect
      .poll(
        () =>
          unboundedRipple.evaluate(
            (el) => el.style.getPropertyValue("--ui-ripple-origin-x").trim(),
          ),
        { timeout: 5_000 },
      )
      .toBe("18%");

    await expect
      .poll(
        () =>
          unboundedRipple.evaluate(
            (el) => el.style.getPropertyValue("--ui-ripple-origin-y").trim(),
          ),
        { timeout: 5_000 },
      )
      .toBe("48%");

    await expect
      .poll(
        () =>
          unboundedRipple.evaluate(
            (el) => el.style.getPropertyValue("--ui-ripple-duration-ms").trim(),
          ),
        { timeout: 5_000 },
      )
      .toBe("520ms");

    await expect
      .poll(() => unboundedRipple.evaluate((el) => el.getAnimations().length), { timeout: 5_000 })
      .toBe(0);
  };

  await assertUnboundedRippleKeyboardFlow();

  await page.reload();
  await page.locator("body:not(:has(#boot))").waitFor();
  await expect(root).toBeVisible();

  await assertUnboundedRippleKeyboardFlow();
});
