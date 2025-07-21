import { sveltekit } from '@sveltejs/kit/vite'
import { defineConfig } from 'vite'

export default defineConfig({
	server: {
		port: 3000,
		strictPort: true,
		fs: {
			// Allow serving files from one level up to the project root
			allow: ['..'],
		},
	},
	build: {
		target: ['chrome64', 'edge79', 'firefox62', 'safari11.1'],
	},
	plugins: [sveltekit()],
})
