const { defineConfig } = require('@vue/cli-service');
module.exports = defineConfig({
  publicPath: './',
  transpileDependencies: true,
  pluginOptions: {
    electronBuilder: {
      preload: 'src/preload.js',
      customFileProtocol: './',
      builderOptions: {
        nsis: {
          oneClick: false, // 是否一键安装
          allowElevation: true, // 允许请求提升。若为false，则用户必须使用提升的权限重新启动安装程序。
          allowToChangeInstallationDirectory: true, //是否允许修改安装目录
          installerIcon: './build/image/icon.ico', // 安装时图标
          uninstallerIcon: './build/image/icon.ico', //卸载时图标
          installerHeaderIcon: './build/image/icon.ico', // 安装时头部图标
          createDesktopShortcut: true, // 是否创建桌面图标
          createStartMenuShortcut: true, // 是否创建开始菜单图标
          shortcutName: '小刻食堂', // 快捷方式名称
          runAfterFinish: true, //是否安装完成后运行
        },
        win: {
          icon: 'build/image/icon.ico', //图标路径
          target: [
            {
              target: 'nsis', //利用nsis制作安装程序
              arch: [
                'x64', //64位
                // "ia32" //32位
              ],
            },
          ],
        },
        extraResources: [
          {
            from: './public',
            to: './public',
          },
        ],
      },
    },
    vuetify: {
      // https://github.com/vuetifyjs/vuetify-loader/tree/next/packages/vuetify-loader
    },
  },
});
