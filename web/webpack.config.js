const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  mode: "development",
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "chip8.js",
  },
  plugins: [new CopyWebpackPlugin(["index.html", "style.css", { from: "roms", to: "roms" }])],
};
