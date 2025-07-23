import { test, expect } from '@playwright/test';

const themes = ['fantasy', 'caramellatte', 'coffee', 'business', 'synthwave', 'aqua'];

test.describe('Theme selector', () => {
    test('Default theme is aqua on first load', async ({ page }) => {
        await page.goto('http://localhost:3000/');

        // <html data-theme="aqua">
        const html = page.locator('html');
        await expect(html).toHaveAttribute('data-theme', 'aqua');

        // open menu → find name of current theme and button "Thema wählen"
        const menuButton = page.getByRole('button', { name: 'Menü' });
        await menuButton.click();

        const themeButton = page.getByRole('button', { name: /Thema wählen/i });
        await expect(themeButton).toBeVisible();
        const currentTheme = page.getByTestId('current-theme');
        await expect(currentTheme).toHaveText('aqua');
        await expect(currentTheme).toBeVisible();
    });

    for (const theme of themes) {
        test(`Selecting theme "${theme}" sets data-theme and updates button`, async ({ page }) => {
            await page.goto('http://localhost:3000/');

            // open menu → find name of current theme and button "Thema wählen"
            const menuButton = page.getByRole('button', { name: 'Menü' });
            await menuButton.click();

            const themeTrigger = page.getByRole('button', { name: /Thema wählen/i });
            // make sure the button is there
            await expect(themeTrigger).toHaveCount(1); 

            await expect(themeTrigger).toBeVisible();
            await themeTrigger.click();

            // click theme label
            const label = page.locator(`label[data-theme="${theme}"]`);
            await expect(label).toBeVisible();
            await label.click();

            // check HTML attribut
            const html = page.locator('html');
            await expect(html).toHaveAttribute('data-theme', theme);

            // check span with theme name
            const currentTheme = page.getByTestId('current-theme');
            await expect(currentTheme).toHaveText(theme);
            await expect(currentTheme).toBeVisible();

            // check local storage
            const stored = await page.evaluate(() => localStorage.getItem('data-theme'));
            expect(stored).toBe(theme);
        });
    }
});
