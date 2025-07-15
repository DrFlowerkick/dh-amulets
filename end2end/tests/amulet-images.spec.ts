import { test, expect } from '@playwright/test';

test('All 8 amulet images are present and loaded', async ({ page }) => {
    await page.goto('http://localhost:3000/setup/2');

    // Liste der erwarteten Pfade
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

    // Alle <img>-Elemente mit Amulett-Bildern
    const images = page.locator('img[src^="/images/amulets/"]');
    await expect(images).toHaveCount(8);

    // Hole die tatsächlichen srcs aus dem DOM
    const actualSources = await images.evaluateAll((imgs) =>
        imgs.map((img) => (img as HTMLImageElement).getAttribute('src'))
    );

    // Erwartung: Alle erwarteten Pfade sind vorhanden
    for (const src of expectedSources) {
        expect(actualSources).toContain(src);
    }
});
