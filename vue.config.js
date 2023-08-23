const { defineConfig } = require('@vue/cli-service');

// https://stackoverflow.com/questions/69692842/error-message-error0308010cdigital-envelope-routinesunsupported/72219174#72219174
const crypto = require('crypto');

try {
  crypto.createHash('md4');
} catch (e) {
  console.warn('Crypto "MD4" is not supported anymore by this Node.js version');
  const origCreateHash = crypto.createHash;
  crypto.createHash = (alg, opts) => {
    return origCreateHash(alg === 'md4' ? 'md5' : alg, opts);
  };
}
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
        mac: {
          icon: 'build/image/icon.png',
          category: 'public.app-category.productivity',
          artifactName: '${productName}-${version}.${ext}',
          target: ['dmg'],
        },
        linux: {
          icon: 'build/image/icon.png',
          target: ['deb'],
        },
        extraResources: [
          {
            from: './public',
            to: './public',
          },
          {
            from: './src/assets',
            to: './assets',
          },
        ],
      },
    },
    vuetify: {
      // https://github.com/vuetifyjs/vuetify-loader/tree/next/packages/vuetify-loader
    },
  },
});
