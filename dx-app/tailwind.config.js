/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: ["./src/**/*.{rs,html,css}", "./dist/**/*.html"],
  theme: {
    extend: {
      colors: {
        'pale-yellow': '#FFF3C7',
        'peach': '#FEC7B4',
        'light-pink': '#FC819E',
        'vibrant-pink': '#F7418F'
      }
    }
  },
  plugins: [],
};
