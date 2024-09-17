module.exports = {
  content: ["./html/**/*html", "./html/**/*.gohtml", "./js/**/*.js"],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms'),
    require('@tailwindcss/aspect-ratio'),
  ],
}
