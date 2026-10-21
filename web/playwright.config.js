import { defineConfig } from "@playwright/test";

export default defineConfig({
  testDir: "./tests",
  workers: 1,
  use: {
    baseURL: "http://127.0.0.1:4318",
    launchOptions: {
      args: ["--use-angle=swiftshader", "--enable-unsafe-swiftshader"],
    },
  },
  webServer: {
    command: "python3 -m http.server 4318 --bind 127.0.0.1 --directory ..",
    url: "http://127.0.0.1:4318/web/",
    reuseExistingServer: !process.env.CI,
  },
});
