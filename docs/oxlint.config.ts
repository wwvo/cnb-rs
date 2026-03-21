import { defineConfig } from 'oxlint'

export default defineConfig({
  ignorePatterns: ['node_modules/**', '.vitepress/cache/**', '.vitepress/dist/**'],
  overrides: [
    {
      files: ['.vitepress/config.ts', '.vitepress/sidebar.ts'],
      env: {
        node: true,
      },
    },
    {
      files: ['.vitepress/theme/**/*.{ts,vue}'],
      env: {
        browser: true,
      },
    },
  ],
})
