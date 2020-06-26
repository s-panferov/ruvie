const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js",
  },
  output: {
    path: dist,
    filename: "[name].js",
  },
  devServer: {
    contentBase: dist,
    proxy: {
      "/api/*": "http://localhost:3000",
    },
  },
  plugins: [
    new CopyPlugin([path.resolve(__dirname, "static")]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
      // extraArgs: "--out-name index",
      watchDirectories: [
        path.resolve(__dirname, "..", "..", "packages/ruvie/src"),
        path.resolve(__dirname, "..", "..", "packages/ruvie-css/src"),
      ],
    }),
  ],
};
