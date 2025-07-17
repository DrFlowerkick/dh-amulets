import { test, expect } from '@playwright/test';

const setups = [
    { url: 'http://localhost:3000/setup/2', expected: 'Setup für zwei Spieler' },
    { url: 'http://localhost:3000/setup/3', expected: 'Setup für drei Spieler' },
    { url: 'http://localhost:3000/setup/4', expected: 'Setup für vier Spieler' }
];

for (const { url, expected } of setups) {
    test(`Page ${url} shows correct heading`, async ({ page }) => {
        await page.goto(url);
        await expect(page.locator('h2')).toHaveText(expected);
    });
}
