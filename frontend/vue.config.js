const HtmlWebpackPlugin = require('html-webpack-plugin')

module.exports = {

  pages: {

    index: './src/main.ts',
    test: './src/test.js',

  },

  publicPath: '/static/',

  chainWebpack: config => {

    config.module
      .rule('scss')
      .oneOf('vue-modules')
      .use('sass-loader')
        .loader('/home/arnaud/Programming/learner/frontend/node_modules/sass-loader/dist/cjs.js')
        .tap(options => {
          options['webpackImporter'] = false;
          options['sassOptions'] = { includePaths: ['./node_modules'] };
          return options
        });

    config.module
      .rule('scss')
      .oneOf('vue')
      .use('sass-loader')
        .loader('/home/arnaud/Programming/learner/frontend/node_modules/sass-loader/dist/cjs.js')
        .tap(options => {
          options['webpackImporter'] = false;
          options['sassOptions'] = { includePaths: ['./node_modules'] };
          return options
        });

    config.module
      .rule('scss')
      .oneOf('normal-modules')
      .use('sass-loader')
        .loader('/home/arnaud/Programming/learner/frontend/node_modules/sass-loader/dist/cjs.js')
        .tap(options => {
          options['webpackImporter'] = false;
          options['sassOptions'] = { includePaths: ['./node_modules'] };
          return options
        });

    config.module
      .rule('scss')
      .oneOf('normal')
      .use('sass-loader')
        .loader('/home/arnaud/Programming/learner/frontend/node_modules/sass-loader/dist/cjs.js')
        .tap(options => {
          options['webpackImporter'] = false;
          options['sassOptions'] = { includePaths: ['./node_modules'] };
          return options
        });

  }
}
