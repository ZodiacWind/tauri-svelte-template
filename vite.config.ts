import { sveltekit } from '@sveltejs/kit/vite'
import { defineConfig } from 'vite'
import path from 'path'

const host = process.env.TAURI_DEV_HOST;

export default defineConfig({
	plugins: [sveltekit()],
	resolve: {
		alias: {
			'@': path.resolve(__dirname, 'src')  // ✅ 设置 @ 指向 src
		}
	},
	clearScreen: false,
	server: {
		port: 3000,
		strictPort: true,
		fs: {
			allow: ['..'],
		},
		hmr: host
			? {
				protocol: 'ws',
				host,
				port: 3001,
			}
			: undefined,
		watch: {
			ignored: ['**/src-tauri/**'],
		},
	},
	build: {
		target: ['chrome64', 'edge79', 'firefox62', 'safari11.1'],
	},
	css: {
		preprocessorOptions: {
			scss: {
				sourceMap: true,
				api: 'modern-compiler'
			}
		}
	}
});
