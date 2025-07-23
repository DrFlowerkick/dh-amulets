import { test, expect } from "@playwright/test";

test("Navigation is present", async ({ page }) => {
  await page.goto("http://localhost:3000/");

  // test links to exist
  await expect(
    page.getByRole("link", { name: "Drachenhüter Amulett Setup" })
  ).toBeVisible();
  await expect(page.getByTitle("Setup für zwei Spieler")).toBeVisible();
  await expect(page.getByTitle("Setup für drei Spieler")).toBeVisible();
  await expect(page.getByTitle("Setup für vier Spieler")).toBeVisible();

  // test click on link to reach correct target
  await page.getByRole("link", { name: "Drachenhüter Amulett Setup" }).click();
  await expect(page).toHaveURL("http://localhost:3000/");

  await page.getByTitle("Setup für zwei Spieler").click();
  await expect(page).toHaveURL("http://localhost:3000/setup/2");

  await page.getByTitle("Setup für drei Spieler").click();
  await expect(page).toHaveURL("http://localhost:3000/setup/3");

  await page.getByTitle("Setup für vier Spieler").click();
  await expect(page).toHaveURL("http://localhost:3000/setup/4");
});
