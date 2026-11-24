import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  clearScreen: false,
  build: { target: 'chrome105', outDir: 'dist', chunkSizeWarningLimit: 1200 },
});
