import { test, expect } from "@playwright/test";

test("Setup ID menu flow and clipboard copy", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  // Open menu
  const menuButton = page.getByRole("button", { name: "Menü" });
  await expect(menuButton).toBeVisible();
  await menuButton.click();

  // Check initial Setup ID is "No Setup ID"
  const setupIdSpan = page.getByTestId("setup-id");
  await expect(setupIdSpan).toHaveText("No Setup ID");

  // Generate a setup
  await page.goto("http://localhost:3000/setup/2");

  // Setup ID should now be a valid 7-character hex string
  await expect(setupIdSpan).not.toHaveText("No Setup ID");
  const firstId = await setupIdSpan.textContent();
  expect(firstId).toMatch(/^[0-9A-F]{7}$/);

  // Generate a new setup
  const reloadButton = page.getByRole("button", { name: /Neues Setup/i });
  await expect(reloadButton).toBeVisible();
  await reloadButton.click();

  // Setup ID should have changed
  const secondId = await setupIdSpan.textContent();
  expect(secondId).not.toBe(firstId);

  // Open menu again
  await menuButton.click();

  // Test clipboard copy (Chromium only)
  const copyButton = page.getByRole("button", { name: "Copy to clipboard" });
  await copyButton.click();

  // Check that the "✔Kopiert" message appears
  await page.waitForTimeout(1100);
  await expect(page.getByTestId("copy-toast")).toHaveClass(/opacity-100/);
  await expect(page.getByTestId("clipboard")).toHaveClass(/animate-bounce/);

  // Wait a bit and check that it disappears
  await page.waitForTimeout(2100); // depends on your timeout logic
  await expect(page.getByTestId("copy-toast")).toHaveClass(/opacity-0/);
  await expect(page.getByTestId("clipboard")).toHaveClass(/animate-none/);
});
