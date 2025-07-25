import { test, expect } from '@playwright/test';

test('Clicking "Neues Setup" icon generates a different amulet configuration', async ({ page }) => {
    await page.goto('http://localhost:3000/setup/2');

    const strong_entries = page.locator(".amulet-cell strong");
    await expect(strong_entries).toHaveCount(8);

    const numbersBefore = await page.locator('.amulet-cell strong').evaluateAll((elements) =>
        elements.map((el) => parseInt(el.textContent || '0', 10))
    );

    // locate the reload icon button by aria-label
    const reloadButton = page.getByRole('button', { name: /Neues Setup/i });
    await expect(reloadButton).toBeVisible();
    await reloadButton.click();

    // wait for new values
    await page.waitForTimeout(100);

    const numbersAfter = await page.locator('.amulet-cell strong').evaluateAll((elements) =>
        elements.map((el) => parseInt(el.textContent || '0', 10))
    );

    expect(numbersAfter).not.toEqual(numbersBefore);
});
