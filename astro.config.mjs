// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';
import starlightCodeblockFullscreen from 'starlight-codeblock-fullscreen'

// https://astro.build/config
export default defineConfig({
	integrations: [
		starlight({
			plugins: [
				starlightCodeblockFullscreen({
					// addToUntitledBlocks: true,
				}),
			],
			title: {
				'zh-CN': 'Rustlings 简体中文版',
				en: 'Rustlings in Simplified Chinese',
			},
			logo: {
				src: './src/assets/logo.png',
				replacesTitle: true,
			},
			social: [
				{ icon: 'github', label: 'GitHub', href: 'https://github.com/SandmeyerX/rustlings-zh-cn' }
			],
			sidebar: [
				{
					label: 'Guides',
					items: [
						// Each item here is one entry in the navigation menu.
						{ label: '开始使用', slug: 'guides/get-started' },
					],
				},
				{
					label: 'Exercises',
					items: [
						{ autogenerate: { directory: 'exercises' } }
					]
				},
				{
					label: 'Reference',
					items: [{ autogenerate: { directory: 'reference' } }],
				},
			],
			defaultLocale: 'root',
			locales: {
				root: {
					label: '简体中文',
					lang: 'zh-CN',	
				},
				en: {
					label: 'English',
					lang: 'en',
				},
			},
		}),
	],
});
