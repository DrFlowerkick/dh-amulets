import { test, expect } from '@playwright/test';

test('Service Worker is registered (cross-browser safe)', async ({ page }) => {
    await page.goto('http://localhost:3000/');
    const isRegistered = await page.evaluate(async () => {
        const reg = await navigator.serviceWorker.getRegistration();
        return !!reg;
    });
    expect(isRegistered).toBeTruthy();
});
