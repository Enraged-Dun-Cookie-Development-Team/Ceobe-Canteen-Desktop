<template>
  <div class="setting">
    <v-card>
      <v-toolbar color="#e6a23c">
        <v-toolbar-title>设置</v-toolbar-title>
        <v-spacer></v-spacer>
        <v-btn
          variant="text"
          icon="fa-solid fa-xmark"
          @click="setting.close"
        ></v-btn>
      </v-toolbar>
      <v-card-item>
        <div class="d-flex justify-space-between align-center w-100">
          <div>
            <v-card-title>开机自启</v-card-title>
            <v-card-subtitle>每次开机就能自动蹲饼呢~</v-card-subtitle>
          </div>
          <div>
            <v-switch
              v-model="setting.isBoot"
              color="#ffba4b"
              @change="setting.changeBoot"
            ></v-switch>
          </div>
        </div>
      </v-card-item>
      <v-card-item>
        <div class="d-flex justify-space-between align-center w-100">
          <div>
            <v-card-title>版本</v-card-title>
            <v-card-subtitle>当前版本 {{ version }}</v-card-subtitle>
          </div>
          <div>
            <v-btn color="#ffba4b" @click="setting.checkVersion"
              >检查更新</v-btn
            >
          </div>
        </div>
      </v-card-item>
    </v-card>
    <v-snackbar v-model="setting.showDownload">
      检测到了新版本，即将跳转到下载网页
    </v-snackbar>
    <v-snackbar v-model="setting.showAlreadyNewest">
      已经是最新版本，无需下载
    </v-snackbar>
  </div>
</template>

<script setup name="setting" lang="ts">
import { onMounted, reactive, ref } from "vue";
import updater from "../api/operations/updater";
import operate from "../api/operations/operate";
import { getVersion } from "../api/resourceFetcher/version";
import { app } from "@tauri-apps/api";

const emits = defineEmits({
  close: null,
});
const version = ref("0.0.0");
const setting = reactive({
  isBoot: false,
  changeBoot() {
    operate.bootSetting(setting.isBoot).then(() => {});
  },
  initBoot() {
    operate.getBootSetting().then((res) => {
      setting.isBoot = res;
    });
  },
  close() {
    emits("close");
  },

  showDownload: false,
  showAlreadyNewest: false,
  checkVersion() {
    getVersion().then(async (res) => {
      console.log(res);
      if (res.data.data == null) {
        return;
      }
      console.log(res);

      updater
        .judgmentVersion(res.data.data.version)
        .then((result) => {
          if (result) {
            setting.showDownload = true;
            operate.openUrlInBrowser("https://www.ceobecanteen.top/#/");
            setTimeout(() => {
              setting.showDownload = false;
            }, 3000);
          } else {
            setting.showAlreadyNewest = true;
            setTimeout(() => {
              setting.showAlreadyNewest = false;
            }, 3000);
          }
        })
        .catch(() => {
          setting.showAlreadyNewest = true;
          setTimeout(() => {
            setting.showAlreadyNewest = false;
          }, 3000);
        });
    });
  },
});

onMounted(async () => {
  setting.initBoot();
  version.value = await app.getVersion();
});
</script>

<style rel="stylesheet/scss" lang="scss"></style>
