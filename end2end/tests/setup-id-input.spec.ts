import { test, expect } from "@playwright/test";

test.describe("Setup ID input behavior", () => {
  const validId = "4922889";
  const expectedPlayers = 3;
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

  test("shows red border on invalid input", async ({ page }) => {
    const input = page.getByLabel("Setup ID Input");
    await input.fill("XYZ123!");
    await input.blur();
    await expect(input).toHaveClass(/border-red-500/);
  });

  test("Escape restores old value", async ({ page }) => {
    const input = page.getByLabel("Setup ID Input");
    const original = await input.inputValue();

    await input.fill("INVALID");
    await input.press("Escape");

    await expect(input).toHaveValue(original);
  });

  test("Enter on invalid input does not submit and keeps value", async ({ page }) => {
    const input = page.getByLabel("Setup ID Input");
    const original = await input.inputValue();

    await input.fill("!!!INVALID!!!");
    await input.press("Enter");

    // Value stays unchanged (no restore, but also no navigation)
    await expect(input).toHaveValue("!!!INVALID!!!");
    await expect(page).toHaveURL(/\/$/); // URL should not change
  });

  test("Tab on invalid input restores original value", async ({ page }) => {
    const input = page.getByLabel("Setup ID Input");
    const original = await input.inputValue();

    await input.fill("XXX_BAD_ID");
    await input.press("Tab");

    await expect(input).toHaveValue(original);
  });

  test("Valid input + Enter updates URL and Setup", async ({ page }) => {
    const input = page.getByLabel("Setup ID Input");

    await input.fill(validId);
    await input.press("Enter");

    await expect(page).toHaveURL(new RegExp(`http://localhost:3000/setup/${expectedPlayers}`));

    const amuletLevel01 = page.getByTestId('current-level-01');
    await expect(amuletLevel01).toHaveValue(expectedLevel01);
    const amuletLevel04 = page.getByTestId('current-level-04');
    await expect(amuletLevel04).toHaveValue(expectedLevel04);
    const amuletLevel06 = page.getByTestId('current-level-06');
    await expect(amuletLevel06).toHaveValue(expectedLevel06);
    const amuletLevel08 = page.getByTestId('current-level-08');
    await expect(amuletLevel08).toHaveValue(expectedLevel08);
    const amuletLevel10 = page.getByTestId('current-level-10');
    await expect(amuletLevel10).toHaveValue(expectedLevel10);
    const amuletLevel12 = page.getByTestId('current-level-12');
    await expect(amuletLevel12).toHaveValue(expectedLevel12);
    const amuletLevel16 = page.getByTestId('current-level-16');
    await expect(amuletLevel16).toHaveValue(expectedLevel16);
    const amuletLevel20 = page.getByTestId('current-level-20');
    await expect(amuletLevel20).toHaveValue(expectedLevel20);
  });
});
