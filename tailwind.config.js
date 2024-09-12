// https://tailwindcss.com/blog/standalone-cli
// https://www.matsimitsu.com/blog/2022-01-04-taliwind-cli-with-yew-and-trunk

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './src/**/*.rs',
  ],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/typography'),
  ],
}

