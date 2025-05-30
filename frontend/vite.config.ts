/// <reference types="vitest/config" />
import tailwindcss from '@tailwindcss/vite';
import react from '@vitejs/plugin-react-swc';
import path from 'path';
import { defineConfig } from 'vite';

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
  assetsInclude: ['**/*.md'],
  test: {
    environment: 'jsdom',
    globals: true,
    setupFiles: 'src/vitest.setup.ts',
    include: ['src/**/*.{test,spec}.{js,ts,jsx,tsx}'],
    coverage: {
      reporter: ['text', 'html'],
      exclude: [
        'node_modules/',
        'dist/',
        '**/*.d.ts',
        '**/vite.config.ts',
        '**/vitest.setup.ts',
      ],
      thresholds: {
        lines: 80, // to adjust the threshold for lines of code coverage
        functions: 80,
        branches: 80,
        statements: 80,
        // You can also set thresholds for functions, branches, and statements
      },
    },
  },
});
