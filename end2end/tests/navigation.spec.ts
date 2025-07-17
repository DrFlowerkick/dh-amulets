import { test, expect } from '@playwright/test';

test('Navigation is present', async ({ page }) => {
    await page.goto("http://localhost:3000/");

    await expect(page.getByRole('link', { name: 'Home' })).toBeVisible();
    await expect(page.getByTitle('Setup für zwei Spieler')).toBeVisible();
    await expect(page.getByTitle('Setup für drei Spieler')).toBeVisible();
    await expect(page.getByTitle('Setup für vier Spieler')).toBeVisible();
});
