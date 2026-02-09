import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { resolve } from 'path';
export default defineConfig({
    plugins: [react()],
    base: process.env.GITHUB_ACTIONS ? '/cxx2flow/' : '/',
    resolve: {
        alias: {
            '@': resolve(__dirname, './src'),
        },
    },
});
