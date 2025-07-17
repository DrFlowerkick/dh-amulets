import { test, expect } from '@playwright/test';

const themes = ['fantasy', 'caramellatte', 'coffee', 'business', 'synthwave', 'aqua'];

test.describe('Theme selector', () => {
    test('Default theme is aqua on first load', async ({ page }) => {
        await page.goto('http://localhost:3000/');

        // checking <html data-theme="aqua">
        const html = page.locator('html');
        await expect(html).toHaveAttribute('data-theme', 'aqua');

        // checking text of button
        const button = page.locator('button', { hasText: 'Thema wählen' });
        await expect(button).toContainText('Aktuell: aqua');
    });

    for (const theme of themes) {
        test(`Selecting theme "${theme}" sets data-theme and updates button`, async ({ page }) => {
            await page.goto('http://localhost:3000/');

            // open the popover
            const openPopover = page.getByRole('button', { name: /Thema wählen/i });
            await openPopover.click();

            // click on theme of the label
            const label = page.locator(`label[data-theme="${theme}"]`);
            await expect(label).toBeVisible();
            await label.click();

            // <html> has new theme
            const html = page.locator('html');
            await expect(html).toHaveAttribute('data-theme', theme);

            // button shows active theme
            await expect(openPopover).toContainText(`Aktuell: ${theme}`);

            // check if theme is stored
            const stored = await page.evaluate(() => localStorage.getItem('data-theme'));
            expect(stored).toBe(theme);
        });
    }
});

