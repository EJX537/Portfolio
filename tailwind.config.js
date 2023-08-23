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
        
        "dg-Cultured": "#232526",
        "dg-Lotion": "#202223",
        "dg-Bright_Gray": "#282a2b",
        "dg-Gainsboro": "#2f3234",
        "dg-Light_Gray": "#34383a",
      },
      screens: {
        'ss': '500px',
      }
    },
  },
  plugins: [],
}

