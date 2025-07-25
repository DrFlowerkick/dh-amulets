import { test, expect } from "@playwright/test";

const setups = [
  {
    url: "http://localhost:3000/setup/2",
    expected: /Setup für\s*zwei\s*Spieler/,
  },
  {
    url: "http://localhost:3000/setup/3",
    expected: /Setup für\s*drei\s*Spieler/,
  },
  {
    url: "http://localhost:3000/setup/4",
    expected: /Setup für\s*vier\s*Spieler/,
  },
];

for (const { url, expected } of setups) {
  test(`Page ${url} shows correct heading`, async ({ page }) => {
    await page.goto(url);
    await expect(page.getByTestId("setup-heading")).toHaveText(expected);
  });
}
