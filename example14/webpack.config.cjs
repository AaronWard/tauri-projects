const { VueLoaderPlugin } = require('vue-loader');
const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
  mode: 'development',
  entry: './src/main.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
    publicPath: '/',
  },
  module: {
    rules: [
      {
        test: /\.vue$/,
        loader: 'vue-loader'
      },
      {
        test: /\.css$/,
        use: [
          'vue-style-loader', // This loader applies CSS to the DOM by injecting a <style> tag
          'css-loader'       // The css-loader interprets @import and url() like import/require() and will resolve them.
        ]
      },
      {
        test: /\.(png|jpg|gif|svg)$/,
        loader: 'file-loader',
        options: {
          name: '[name].[ext]?[hash]'
        }
      }
    ]
  },
  plugins: [
    new VueLoaderPlugin(),
    new HtmlWebpackPlugin({
      template: './index.html' // Make sure the path to index.html is correct.
    })
  ],
  resolve: {
    alias: {
      'vue$': 'vue/dist/vue.esm-bundler.js' // Ensure the alias 'vue$' points to the correct Vue distribution file
    },
    extensions: ['.js', '.vue', '.json']
  },
  devServer: {
    historyApiFallback: true,
    hot: true,
    host: 'localhost',
    port: 8080,
    static: {
      directory: path.join(__dirname, 'dist'), // Ensure this points to the directory where index.html is served
    },
    client: {
      overlay: {
        errors: true,
        warnings: true,
      },
    }
  },
  performance: {
    hints: false
  },
  devtool: 'eval-source-map'
};

if (process.env.NODE_ENV === 'production') {
  module.exports.devtool = 'source-map';
  module.exports.mode = 'production';
}
