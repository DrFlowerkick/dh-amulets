import { test, expect } from '@playwright/test';

const invalidRoutes = [
    'http://localhost:3000/foobar',
    'http://localhost:3000/setup/abc',
    'http://localhost:3000/edit',
    'http://localhost:3000/xyz/123',
    'http://localhost:3000/not/a/real/path'
];

for (const route of invalidRoutes) {
    test(`Invalid route '${route}' shows 404 message`, async ({ page }) => {
        await page.goto(route);
        await expect(page.locator('body')).toContainText('Page not found.');
    });
}
