import { defineConfig } from 'vite';

export default defineConfig({
  root: 'ui',
  server: {
    port: 1430,
    strictPort: true,
  },
  build: {
    outDir: 'ui-dist',
  },
});
