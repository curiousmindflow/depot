import { test, expect } from "@playwright/test";

test("homepage loads", async ({ page }) => {
  await page.goto("/");
  await expect(page.locator("body")).toBeVisible();
});

test("api ping works", async ({ page }) => {
  const response = await page.request.get("/api/ping");
  expect(response.status()).toBe(200);

  const json = await response.json();
  expect(json.status).toBe("ok");
});
