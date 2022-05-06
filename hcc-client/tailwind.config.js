module.exports = {
  content: [],
  theme: {
    letterSpacing: {
      tighter: "-0.5rem",
      widest: "3rem"
    },
    extend: {
      
    },
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
    '../hcc-server/templates/**/*.j2',
    '../hcc-server/src/**/*.rs'
  ],
}
