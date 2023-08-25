<template>
  <div class="index">
    <div v-if="!loadApp" class="read-me">
      <h1 class="online-yellow">小刻食堂免责声明</h1>
      <div class="read-me-content">
        <div>
          <p>
            欢迎使用由小刻食堂开发团队（以下简称「开发组」）开发的「小刻食堂」系列。在您使用本框架之前，请仔细阅读并完全理解本声明。一旦您开始使用小刻食堂，即表示您同意遵守本声明中的所有条款和条件。
          </p>

          <p>
            1. 使用目的：
            小刻食堂是一个免费开源的软件项目，旨在为用户提供抓取B站、微博、明日方舟通讯组等API数据的功能，并提供多种实用功能，如实时饼消息通知、快速跳转链接、饼内容分享、详细内容查看、饼内容图片生成、理智回复时间计算等。
          </p>

          <p>
            2. 合法合理使用：
            您承诺以合法、合理的方式使用小刻食堂，不得将其用于任何违法、侵权或其他恶意行为，也不得将其应用于违反当地法律法规的Web平台。
          </p>

          <p>
            3. 免责声明：
            开发组对于因使用小刻食堂而引起的任何意外、疏忽、合同损害、诽谤、版权或知识产权侵权以及由此产生的损失（包括但不限于直接、间接、附带或衍生的损失等）不承担任何法律责任。
          </p>

          <p>
            4. 风险承担：
            用户明确同意，使用小刻食堂所存在的风险和后果将完全由用户自行承担，开发组不承担任何法律责任。
          </p>

          <p>
            5. 合法发布和使用：
            用户在遵守本声明的前提下，可在《MIT开源许可证》所允许的范围内合法地发布、传播和使用小刻食堂。对于因违反本声明或法律法规造成的法律责任（包括但不限于民事赔偿和刑事责任），将由违约者自行承担。
          </p>

          <p>
            6. 知识产权：
            小刻食堂及其相关产品享有开发组的知识产权保护，包括但不限于商标权、专利权、著作权、商业秘密等，受相关法律法规保护。
          </p>

          <p>
            7. 条款变更：
            开发组有权随时单方面修改本声明条款及附件内容，并通过消息推送、网页公告等方式进行公布。变更后的条款在公布后立即生效，用户继续使用即表示您已充分阅读、理解并接受修改后的声明内容。
          </p>

          <p>
            请在使用小刻食堂之前仔细阅读并理解本声明。如果您有任何疑问，请咨询相关法律专业人士。感谢您的合作与支持！
          </p>

          <p>小刻食堂开发团队</p>
        </div>
      </div>

      <div class="text-right">
        <v-btn elevation="0" density="default" @click="dialog = true"
          >不同意，退出程序</v-btn
        >

        <v-btn
          color="#e6a23c"
          elevation="0"
          style="color: #fff"
          class="ml-3"
          density="default"
          @click="agreeApp"
          >我已阅读以上全部条款并同意以上条款。开始蹲饼！
        </v-btn>
      </div>
    </div>
    <div v-else>
      <app-top></app-top>
      <app-main></app-main>
    </div>
  </div>
  <v-dialog v-model="dialog" persistent width="auto">
    <v-card>
      <v-card-title class="text-h5">裆真?</v-card-title>
      <v-card-text
        >你选得对，但小刻食堂是一款优秀的蹲饼器，用于实时获取兔兔发的动态。</v-card-text
      >
      <v-card-actions class="justify-end">
        <v-btn color="#f56c6c" variant="text" @click="exitApp">
          我不用小刻食堂</v-btn
        >
        <v-btn color="#e6a23c" variant="text" @click="dialog = false">
          刚刚是手滑，我要同意条款</v-btn
        >
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup name="index" lang="ts">
import { onMounted, ref } from "vue";
import AppTop from "./AppTop.vue";
import AppMain from "../view/AppMain.vue";
import operate from "../api/operations/operate";
import storage from "../api/operations/localStorage";

const loadApp = ref(false);
const dialog = ref(false);
const exitApp = () => {
  operate.exit();
};
const agreeApp = () => {
  storage.setItem("agreeApp", "true");
  operate.bootSetting(true).then(() => {});
  loadApp.value = true;
};

onMounted(() => {
  let result = storage.getItem("agreeApp");
  if (result == "true") {
    loadApp.value = true;
    return;
  }
  loadApp.value = false;
});
</script>

<style lang="scss">
.read-me {
  overflow: auto;
  margin: 0 auto;
  padding: 20px;
  width: 100vw;
  height: 100vh;
  font-family: Arial, sans-serif;
  line-height: 1.6;

  h1,
  h2 {
    text-align: center;
  }

  p {
    margin-bottom: 10px;
  }

  strong {
    font-weight: bold;
  }
}
</style>
