# CeobeCanteenTauri

<div align="center">

[![LOGO](src-tauri/icons/128x128.png)](https://ceobecanteen.top/)

帮小刻找好多好吃的饼

[![Github](https://img.shields.io/badge/github-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/Enraged-Dun-Cookie-Development-Team/Ceobe-Canteen-Electron)
![License](https://img.shields.io/github/license/Enraged-Dun-Cookie-Development-Team/Ceobe-Canteen-Electron?style=for-the-badge)

[![latest](https://img.shields.io/badge/Ceobe_Canteen-release-blue?style=for-the-badge
)](https://github.com/Enraged-Dun-Cookie-Development-Team/Ceobe-Canteen-Electron/releases)
[![Static Badge](https://img.shields.io/badge/QQ%E7%BE%A4-362860473-fdba4b?style=for-the-badge&logo=tencentqq)
](http://qm.qq.com/cgi-bin/qm/qr?_wv=1027&k=bPMN910WLbN3FHQXzmAJEPyupOm_8bHX&authKey=Sr%2BCN9EXrGg7i2KnUIMdU9rSfaZBfQzm69wMyI004janRrg9MqV9PW7PRjb35SP2&noverify=0&group_code=362860473)

</div>


## Contributing

You can feel free to use English or Chinese to open an issue or pull request.

### BUG FIX
If you are going to fix an exist BUG, please create a branch from the **"master"**, then writing your changes in that branch. It is highly recommend to use `fix-` prfix of your branch.

In general, bug fix version will cause the patch version increase and it will not have any `rc`, `alpha`, `beta` pre-release version

If you finish your perfromance on bug fixing, remenber direct increase the *patch version* in your pull request, they are local in the following place
- `package.json`
- `src-tauri\tauri.config.json`
- `src-tauri\Cargo.toml`

### NEW FEATURES

If you are going to adding a new feature, please create a branch from the **"next"**, then writing your changes in that branch. It is hightly recommend to use 'feat-' prefix of your branch

In most situation, you should test your new feature first. Providing unit test or integrate test if possible.

Release contain new features will increase the *minor version*. In most cases, you do not need edit the version in your pull request

The release version with new features will have `beta` and `alpha` and `rc` pre-release. 
