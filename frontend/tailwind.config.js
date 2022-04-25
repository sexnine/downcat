const colors = require("tailwindcss/colors");

module.exports = {
  content: ["./index.html", "./src/**/*.{vue,js,ts,jsx,tsx}"],
  theme: {
    extend: {
      fontFamily: {
        inter: ["Inter", "Helvetica", "Arial", "sans-serif"],
      },
    },
    colors: {
      red: colors.red,
      gray: colors.gray,
      white: colors.white,
      black: colors.black,
      blue: colors.blue,
      green: colors.green,
    },
  },
  plugins: [],
};
