module.exports = {
  content: [],
  theme: {
    extend: {},
  },
  plugins: [
    require('@tailwindcss/forms')({
      strategy: 'class',
    })
  ],
  content: [
    './assets/**/*.html',
    './src/**/*.rs',
    './css/**/*.css',
    '../hcc-server/templates/**/*.j2'
  ],
}
