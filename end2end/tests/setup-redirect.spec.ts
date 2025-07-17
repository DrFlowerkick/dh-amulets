import { test, expect } from '@playwright/test';

test.describe('Theme selection', () => {
    test('Setup without ID redirects to default setup for 2 players', async ({ page }) => {
        // call setup without ID
        await page.goto('http://localhost:3000/setup', { waitUntil: 'networkidle' });

        // expect redirection to /setup/2
        await expect(page).toHaveURL('http://localhost:3000/setup/2');

        // check titel
        await expect(page.locator('h2')).toHaveText('Setup für zwei Spieler');
        });

    test('Invalid setup ID redirects to default setup for 2 players', async ({ page }) => {
        // example invalid ID
        await page.goto('http://localhost:3000/setup/999', { waitUntil: 'networkidle' });

        // expect redirection to /setup/2
        await expect(page).toHaveURL('http://localhost:3000/setup/2');

        // check titel
        await expect(page.locator('h2')).toHaveText('Setup für zwei Spieler');
    });
});
