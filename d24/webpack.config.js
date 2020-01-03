const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

const outDir = path.join(__dirname, 'dist');

module.exports = {
    entry: './index.js',
    output: {
        filename: 'main.js',
        path: outDir,
    },
    devServer: {
        contentBase: outDir,
        compress: true,
        port: 9000
    },
    mode: "development",
    devtool: "inline-source-map",
    plugins: [
        new CopyWebpackPlugin(['index.html'])
    ],
    module: {
        rules: [
            {
                test: /\.txt$/i,
                use: 'raw-loader',
            },
        ],
    }
};