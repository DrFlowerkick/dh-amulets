import { test, expect } from "@playwright/test";

test.describe("Setup ID input behavior", () => {
  const validId = "4922889";
  const expectedNumPlayers = 3;
  const expectedLevel01 = "2x";
  const expectedLevel04 = "2x";
  const expectedLevel06 = "2x";
  const expectedLevel08 = "1x";
  const expectedLevel10 = "2x";
  const expectedLevel12 = "1x";
  const expectedLevel16 = "1x";
  const expectedLevel20 = "1x";

  test.beforeEach(async ({ page }) => {
    await page.goto("http://localhost:3000/");
    const menuButton = page.getByRole("button", { name: "Menü" });
    await menuButton.click();
  });

  test("show input field for Setup ID and buttons", async ({ page }) => {
    const editIdButton = page.getByTestId("switch-input-mode");
    await editIdButton.click();

    const input = page.getByTestId("input-setup-id");
    await expect(input).toBeVisible();
    await expect(input).toHaveAttribute("placeholder", "ID eingeben...");

    const submitSetupId = page.getByTestId("submit-setup-id");
    await expect(submitSetupId).toBeVisible();
    await expect(submitSetupId).toHaveClass(/opacity-50/);
    await expect(submitSetupId).toBeDisabled();

    const cancelInputMode = page.getByTestId("cancel-input-mode");
    await expect(cancelInputMode).toBeVisible();
  });

  test("show placeholder text of input if empty", async ({ page }) => {
    const editIdButton = page.getByTestId("switch-input-mode");
    await editIdButton.click();

    const input = page.getByTestId("input-setup-id");
    await expect(input).toBeVisible();
    await expect(input).toHaveAttribute("placeholder", "ID eingeben...");

    await input.fill("foo");
    await expect(input).toHaveValue("foo");

    await input.fill("");
    await expect(input).toHaveValue("");
    await expect(input).toHaveAttribute("placeholder", "ID eingeben...");
  });

  test("shows red text on invalid input", async ({ page }) => {
    const editIdButton = page.getByTestId("switch-input-mode");
    await editIdButton.click();

    const input = page.getByTestId("input-setup-id");
    await input.fill("XYZ123!");
    await expect(input).toHaveValue("XYZ123!");
    await expect(input).toHaveClass(/text-red-500/);

    await input.fill("");
    await expect(input).toHaveValue("");
    await expect(input).toHaveClass(/text-primary/);
  });

  test("Cancel does not change old Setup ID value", async ({ page }) => {
    const setupId = page.getByTestId("setup-id");
    const original = await setupId.textContent();

    const editIdButton = page.getByTestId("switch-input-mode");
    await editIdButton.click();

    const input = page.getByTestId("input-setup-id");
    await input.fill("INVALID");
    await expect(input).toHaveValue("INVALID");

    const cancelInputMode = page.getByTestId("cancel-input-mode");
    await cancelInputMode.click();

    await expect(setupId).toHaveText(original ?? "");
  });

  test("Valid input enables submit button", async ({ page }) => {
    const editIdButton = page.getByTestId("switch-input-mode");
    await editIdButton.click();

    const input = page.getByTestId("input-setup-id");
    await input.fill(validId);
    await expect(input).toHaveValue(validId);

    const submitSetupId = page.getByTestId("submit-setup-id");
    await expect(submitSetupId).not.toHaveClass(/opacity-50/);
    await expect(submitSetupId).not.toBeDisabled();
  });

  test("Valid input + submit updates URL and Setup", async ({ page }) => {
    const editIdButton = page.getByTestId("switch-input-mode");
    await editIdButton.click();

    const input = page.getByTestId("input-setup-id");
    await input.fill(validId);
    await expect(input).toHaveValue(validId);

    const submitSetupId = page.getByTestId("submit-setup-id");
    await submitSetupId.click();

    // Check URL
    await page.waitForURL(`**/setup/${expectedNumPlayers}`);

    const amuletLevel01 = page.getByTestId("current-level-01");
    await amuletLevel01.waitFor({ timeout: 100 }); // wait for rendering
    await expect(amuletLevel01).toHaveText(expectedLevel01);
    const amuletLevel04 = page.getByTestId("current-level-04");
    await expect(amuletLevel04).toHaveText(expectedLevel04);
    const amuletLevel06 = page.getByTestId("current-level-06");
    await expect(amuletLevel06).toHaveText(expectedLevel06);
    const amuletLevel08 = page.getByTestId("current-level-08");
    await expect(amuletLevel08).toHaveText(expectedLevel08);
    const amuletLevel10 = page.getByTestId("current-level-10");
    await expect(amuletLevel10).toHaveText(expectedLevel10);
    const amuletLevel12 = page.getByTestId("current-level-12");
    await expect(amuletLevel12).toHaveText(expectedLevel12);
    const amuletLevel16 = page.getByTestId("current-level-16");
    await expect(amuletLevel16).toHaveText(expectedLevel16);
    const amuletLevel20 = page.getByTestId("current-level-20");
    await expect(amuletLevel20).toHaveText(expectedLevel20);

    // menu should show Setup ID with new value
    const setupIdSpan = page.getByTestId("setup-id");
    await expect(setupIdSpan).toHaveText(validId);
  });
});
