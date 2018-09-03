/* @flow */
import MiniCssExtractPlugin from 'mini-css-extract-plugin';
import path from 'path';
import sass from 'sass';

export default {
  entry: './web-src/index.js',
  output: {
    path: path.resolve(__dirname, 'web-src', 'static'),
    filename: '[name].js',
  },
  devtool: 'cheap-source-map',
  serve: {
    content: path.resolve(__dirname, 'web-src'),
  },
  plugins: [
    new MiniCssExtractPlugin({
      path: path.resolve(__dirname, 'web-src', 'static'),
      filename: "[name].css",
      chunkFilename: "[id].css"
    }),
  ],
  module: {
    rules: [
      {
        enforce: 'pre',
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: 'eslint-loader',
      },
      {
        test: /\.scss$/,
        use: [
          MiniCssExtractPlugin.loader,
          'css-loader',
          {
            loader: 'sass-loader',
            options: {
              implementation: sass,
            },
          },
        ],
      },
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: 'babel-loader',
      },
    ],
  },
};
