/* @flow */

import path from 'path';

export default {
  entry: {
    main: './web-src/index.js',
  },
  output: {
    path: path.resolve(__dirname, 'web-src', 'static'),
    filename: '[name].js',
  },
  devtool: 'cheap-source-map',
  module: {
    rules: [
      {
        enforce: 'pre',
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: 'eslint-loader',
      },
      {
        test: /\.js$/,
        exclude: /node_modules/,
        use: {
          loader: 'babel-loader',
        },
      },
    ],
  },
};
