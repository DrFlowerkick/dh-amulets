import { test, expect } from "@playwright/test";

test("Open menu at click on Hamburger button", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  // find Hamburger-Button and click
  const menuButton = page.getByRole("button", { name: "Menü" });
  await expect(menuButton).toBeVisible();
  await menuButton.click();

  // check menu text
  await expect(
    page.getByRole("heading", { name: "Menü", level: 3 })
  ).toBeVisible();

  // check visual separators
  const separators = page.getByRole("separator");
  await expect(separators).toHaveCount(2);
  await expect(separators.nth(0)).toBeVisible();
  await expect(separators.nth(1)).toBeVisible();
});
