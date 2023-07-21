const { defineConfig } = require('@vue/cli-service');
module.exports = defineConfig({
  publicPath: './',
  transpileDependencies: true,
  pluginOptions: {
    electronBuilder: {
      preload: 'src/preload.js',
      customFileProtocol: './',
    },
    vuetify: {
      // https://github.com/vuetifyjs/vuetify-loader/tree/next/packages/vuetify-loader
    },
  },
});
