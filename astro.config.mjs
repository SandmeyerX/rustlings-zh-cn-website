// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	integrations: [
		starlight({
			title: 'Rustlings 简体中文版',
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
						{ label: 'Example Guide', slug: 'guides/example' },
					],
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
