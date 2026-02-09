import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import { resolve } from 'path'

function normalizeBase(basePath: string) {
  const withLeadingSlash = basePath.startsWith('/') ? basePath : `/${basePath}`
  return withLeadingSlash.endsWith('/') ? withLeadingSlash : `${withLeadingSlash}/`
}

function resolveBase() {
  if (process.env.VITE_BASE_PATH) {
    return normalizeBase(process.env.VITE_BASE_PATH)
  }

  if (!process.env.GITHUB_ACTIONS) {
    return '/'
  }

  const repositoryName = process.env.GITHUB_REPOSITORY?.split('/')[1]
  if (!repositoryName || repositoryName.endsWith('.github.io')) {
    return '/'
  }

  return `/${repositoryName}/`
}

export default defineConfig({
  plugins: [react()],
  base: resolveBase(),
  resolve: {
    alias: {
      '@': resolve(__dirname, './src'),
    },
  },
})
