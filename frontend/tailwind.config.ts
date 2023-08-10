import forms from '@tailwindcss/forms';
import typography from '@tailwindcss/typography';
import daisyui from 'daisyui';
import type { Config } from 'tailwindcss';

export default {
	content: ['./src/**/*.{ts,svelte,html}'],
	theme: {
		extend: {}
	},
	daisyui: {
		themes: ['dark', 'light', 'cupcake']
	},
	plugins: [daisyui, forms, typography],
	safelist: ['alert-success', 'alert-error', 'alert-info', 'input-error']
} satisfies Config;
