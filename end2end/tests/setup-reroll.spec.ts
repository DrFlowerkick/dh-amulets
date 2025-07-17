import { test, expect } from '@playwright/test';

test('Clicking "Neues Setup" generates a different amulet configuration', async ({ page }) => {
    await page.goto('http://localhost:3000/setup/2');


    // count <strong> elements of .amulet-cell
    const strong_entries = page.locator(".amulet-cell strong");
    await expect(strong_entries).toHaveCount(8);

    const numbersBefore = await page.locator('.amulet-cell strong').evaluateAll((elements) =>
        elements.map((el) => parseInt(el.textContent || '0', 10))
    );

    // click button for new setup
    const rerollButton = page.getByRole('button', { name: /Neues Setup/i });
    await expect(rerollButton).toBeVisible();
    await rerollButton.click();

    // short break waiting for new calculated values
    await page.waitForTimeout(100);

    const numbersAfter = await page.locator('.amulet-cell strong').evaluateAll((elements) =>
        elements.map((el) => parseInt(el.textContent || '0', 10))
    );

    // compare numbers
    expect(numbersAfter).not.toEqual(numbersBefore);
});
