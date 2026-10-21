import { test, expect } from "@playwright/test";

const molecules = [
  "methane",
  "atp",
  "caffeine",
  "complex",
  "ethylene",
  "glucose",
  "propane",
];

for (const molecule of molecules) {
  test(`${molecule} renders through Rust and responds to rotation and zoom`, async ({
    page,
  }) => {
    const errors = [];
    page.on("pageerror", (error) => errors.push(error.message));
    await page.goto(`/web/?molecule=${molecule}`);
    await expect(page.locator("#status")).toBeEmpty();
    const canvas = page.locator("canvas");
    const before = await canvas.screenshot();
    await canvas.hover({ position: { x: 100, y: 100 } });
    await page.mouse.down();
    await page.mouse.move(400, 240, { steps: 8 });
    await page.mouse.up();
    await expect
      .poll(async () => Buffer.compare(before, await canvas.screenshot()))
      .not.toBe(0);
    const rotated = await canvas.screenshot();
    await page.mouse.wheel(0, -200);
    await expect
      .poll(async () => Buffer.compare(rotated, await canvas.screenshot()))
      .not.toBe(0);
    expect(errors).toEqual([]);
  });
}

test("Wasm preserves methane geometry, paints pixels, and rejects invalid input", async ({
  page,
}) => {
  await page.goto("/web/");
  await expect(page.locator("#status")).toBeEmpty();
  const result = await page.evaluate(async () => {
    const { default: init, MoleculeViewer } =
      await import("/web/pkg/molecular_visualization.js");
    await init();
    const canvas = document.createElement("canvas");
    canvas.style.cssText = "width:700px;height:450px";
    document.body.append(canvas);
    const pdb = await (await fetch("/resources/pdb/methane.pdb")).text();
    const viewer = new MoleculeViewer(canvas, pdb);
    viewer.draw(700, 450);
    const gl = canvas.getContext("webgl2");
    const pixels = new Uint8Array(700 * 450 * 4);
    gl.readPixels(0, 0, 700, 450, gl.RGBA, gl.UNSIGNED_BYTE, pixels);
    const background = pixels.slice(0, 3);
    let painted = 0;
    for (let i = 0; i < pixels.length; i += 4) {
      if (
        pixels[i] !== background[0] ||
        pixels[i + 1] !== background[1] ||
        pixels[i + 2] !== background[2]
      )
        painted++;
    }
    const counts = [viewer.atom_count, viewer.bond_count];
    let invalidSize = false,
      invalidPdb = false;
    try {
      viewer.draw(0, 450);
    } catch {
      invalidSize = true;
    }
    viewer.free();
    try {
      new MoleculeViewer(canvas, "not a PDB");
    } catch {
      invalidPdb = true;
    }
    return { counts, painted, invalidSize, invalidPdb, error: gl.getError() };
  });
  expect(result.counts).toEqual([5, 4]);
  expect(result.painted).toBeGreaterThan(1000);
  expect(result.error).toBe(0);
  expect(result.invalidSize).toBe(true);
  expect(result.invalidPdb).toBe(true);
});

test("restores a lost WebGL context", async ({ page }) => {
  await page.goto("/web/");
  await expect(page.locator("#status")).toBeEmpty();
  await page.evaluate(() => {
    const extension = document
      .querySelector("canvas")
      .getContext("webgl2")
      .getExtension("WEBGL_lose_context");
    extension.loseContext();
    setTimeout(() => extension.restoreContext(), 300);
  });
  await expect(page.locator("#status")).toContainText("Graphics context lost");
  await expect(page.locator("#status")).toBeEmpty();
  await page.locator("canvas").press("ArrowRight");
  await expect(page.locator("#status")).toBeEmpty();
});

test("reports an unavailable molecule", async ({ page }) => {
  await page.goto("/web/?molecule=missing");
  await expect(page.locator("#status")).toContainText(
    "Could not load molecule",
  );
});
