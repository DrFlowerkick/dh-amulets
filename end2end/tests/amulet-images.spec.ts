import { test, expect } from '@playwright/test';

test('All 8 amulet images are present and loaded', async ({ page }) => {
    await page.goto('http://localhost:3000/setup/2');

    // list of expected path routes
    const expectedSources = [
        '/images/amulets/amulet_01.png',
        '/images/amulets/amulet_04.png',
        '/images/amulets/amulet_06.png',
        '/images/amulets/amulet_08.png',
        '/images/amulets/amulet_10.png',
        '/images/amulets/amulet_12.png',
        '/images/amulets/amulet_16.png',
        '/images/amulets/amulet_20.png',
    ];

    // all <img> elements with amulet pictures
    const images = page.locator('img[src^="/images/amulets/"]');
    await expect(images).toHaveCount(8);

    // get sources from DOM
    const actualSources = await images.evaluateAll((imgs) =>
        imgs.map((img) => (img as HTMLImageElement).getAttribute('src'))
    );

    // all path should exist
    for (const src of expectedSources) {
        expect(actualSources).toContain(src);
    }
});
