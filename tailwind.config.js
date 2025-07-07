/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./**/*.{html,rs,css}",
  ],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
}

