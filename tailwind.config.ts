import daisyui from "daisyui";
import type { Config } from 'tailwindcss'

/** @type {import('tailwindcss').Config} */
export default {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {},
  },
  daisyui: {
    themes: ["forest"],
  },
  plugins: [daisyui],
} satisfies Config
