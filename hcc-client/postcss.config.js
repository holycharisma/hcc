const tailwindcss = require("tailwindcss");
module.exports = { plugins: [
  ["postcss-preset-env", {
    importFrom: "css/app.css"
  }], 
  tailwindcss
] };
