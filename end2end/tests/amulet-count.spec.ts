import { test, expect } from "@playwright/test";

const setups = [
    { url: "http://localhost:3000/setup/2", expectedTotal: 16 },
    { url: "http://localhost:3000/setup/3", expectedTotal: 12 },
    { url: "http://localhost:3000/setup/4", expectedTotal: 8 },
];

for (const { url, expectedTotal } of setups) {
    test(`Total amulet count for ${url} is ${expectedTotal}`, async ({
        page,
    }) => {
        await page.goto(url);

        // count <strong> elements of .amulet-cell
        const strong_entries = page.locator(".amulet-cell strong");
        await expect(strong_entries).toHaveCount(8);

        // get values of strong elements
        const numbers = await strong_entries
        .evaluateAll((elements) =>
            elements.map((el) => parseInt(el.textContent || "0", 10))
        );

        const sum = numbers.reduce((a, b) => a + b, 0);

        expect(sum).toBe(expectedTotal);
    });
}
