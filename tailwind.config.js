/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.{html,rs}",
    "./index.html"
],
  theme: {
    extend: {
      colors: {
        "lg-Cultured": "#F5F5F5",
        "lg-Lotion": "#FAFAFA",
        "lg-Bright_Gray": "#ECECEC",
        "lg-Gainsboro": "#DEDEDE",
        "lg-Light_Gray": "#D4D4D4",
      }
    },
  },
  plugins: [],
}

