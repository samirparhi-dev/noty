/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs", // Ensure Tailwind scans your Rust files for class names
    "./index.html",
  ],
  theme: {
    extend: {},
  },
  plugins: [],
};
