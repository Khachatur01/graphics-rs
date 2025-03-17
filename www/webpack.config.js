const path = require('path');
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
  entry: "./bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  mode: "development",
  plugins: [
    // new CopyWebpackPlugin(['index.html'])
    new CopyPlugin({
      patterns: [
        {from: "index.html", to: "index.html"},
      ],
    }),
  ],
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true
  }
};