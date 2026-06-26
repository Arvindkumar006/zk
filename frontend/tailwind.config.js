/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        darkBg: '#0B0F19',
        cardBg: '#111827',
        inputBg: '#1E293B',
        stellarGreen: '#00E676',
      },
    },
  },
  plugins: [],
}
