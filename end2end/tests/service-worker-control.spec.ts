import { test, expect } from '@playwright/test';

test('Service Worker controls the page (only Chromium)', async ({ page, browserName }) => {
  test.skip(browserName !== 'chromium', 'Service worker control check only works reliably in Chromium');
  
  await page.goto('http://localhost:3000/');
  await page.waitForTimeout(500);
  await page.reload();

  const controlled = await page.evaluate(() => navigator.serviceWorker?.controller !== null);
  expect(controlled).toBeTruthy();
});
