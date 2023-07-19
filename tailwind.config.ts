import type { Config } from 'tailwindcss'

export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {},
  },
  daisyui: {
    themes: ["coffee"],
  },
  plugins: [require("daisyui")],
} satisfies Config

