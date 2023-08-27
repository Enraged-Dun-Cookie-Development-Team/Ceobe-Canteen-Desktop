<template>
  <div class="setting">
    <v-card>
      <v-toolbar color="#e6a23c">
        <v-toolbar-title>设置</v-toolbar-title>
        <v-spacer></v-spacer>
        <v-btn
            icon="fa-solid fa-xmark"
            variant="text"
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
                v-model="setting.autoBoot"
                color="#ffba4b"
                @change="setting.setAutoBoot"
            ></v-switch>
          </div>
        </div>
      </v-card-item>
      <v-card-item>
        <div class="d-flex justify-space-between align-center w-100">
          <div>
            <v-card-title>版本</v-card-title>
            <v-card-subtitle>当前版本 {{ setting.currentVersion }}</v-card-subtitle>
          </div>
          <div>
            <v-btn color="#ffba4b" @click="setting.checkUpdate"
            >检查更新
            </v-btn
            >
<!--test send notification        -->
            <v-btn
                @click="sen_d">
              弹窗
            </v-btn>
          </div>
        </div>
      </v-card-item>
    </v-card>
    <v-snackbar v-model="showDownload">
      检测到了新版本，即将跳转到下载网页
    </v-snackbar>
    <v-snackbar v-model="showAlreadyNewest">
      已经是最新版本，无需下载
    </v-snackbar>
  </div>
</template>

<script lang="ts" name="setting" setup>
import {computed, onMounted, reactive, ref} from "vue";
import updater from "../api/operations/updater";
import operate from "../api/operations/operate";
import {getVersion} from "../api/resourceFetcher/version";
import {app, notification} from "@tauri-apps/api";
import {P} from "@tauri-apps/api/event-41a9edf5";

const emits = defineEmits({
  close: null,
});

// test send notification
const sen_d = () => {
  operate.openNotificationWindow({
    datasource: 'kkwd',
    default_cookie: {
      images: [{compress_url:null,origin_url: 'https://i0.hdslb.com/bfs/new_dyn/2956e376fb056cf79cc95bcf585dbbc0161775300.jpg'}],
      text: '欸嘿嘿，桃金娘的脚小小的~香香的~.'
    },
    icon: '/assets/icon/anime.png',
    source: {data: '123', type: 'nn'},
    timestamp: {fetcher: 114514, platform: 114514, platform_precision: 'minute'}
  })
}

const showDownload = computed(() => setting.versionState == "UpdateAvailable")
const showAlreadyNewest = computed(() => setting.versionState == "Newest")

const setting = reactive<{
  // auto boot setting
  autoBoot: boolean,
  initAutoBoot: () => void,
  setAutoBoot: () => void,
  // quit 
  close: () => void,
  // updater setting
  versionState: "Newest" | "UpdateAvailable" | "Unknown"
  currentVersion: string,
  checkUpdate: () => void
  getAppVersion: () => void
}>({
  autoBoot: false,
  setAutoBoot() {
    operate.bootSetting(setting.autoBoot).then(() => {
    });
  },
  initAutoBoot() {
    operate.getBootSetting().then((res) => {
      setting.autoBoot = res;
    });
  },
  close() {
    emits("close");
  },

  versionState: "Unknown",
  currentVersion: "<UNKNOWN>",
  checkUpdate(): void {
    getVersion().then((res) => {
      console.log("Desktop Version", res);
      if (res.data.data == null) {
        return;
      }
      console.log(res);

      updater
          .judgmentVersion(res.data.data.version)
          .then((result) => {
            if (result) {
              setting.versionState = "UpdateAvailable";
              operate.openUrlInBrowser("https://www.ceobecanteen.top/#/");
              setTimeout(() => {
                setting.versionState = "Unknown";
              }, 3000);
            } else {
              setting.versionState = "Newest";
              setTimeout(() => {
                setting.versionState = "Unknown";
              }, 3000);
            }
          })
          .catch(() => {
            setting.versionState = "Newest";
            setTimeout(() => {
              setting.versionState = "Unknown";
            }, 3000);
          });
    }).catch(async (error) => {
      console.log("Failure loading New Version")
      if (!await notification.isPermissionGranted()) {
        await notification.requestPermission()
      }
      notification.sendNotification({
        title: "小刻出错了！",
        icon: "/asserts/icon.png",
        body: error.toString()
      })
    });
  },
  getAppVersion: () => {
    app.getVersion()
        .then((v: string) => setting.currentVersion = v);
  },

});

onMounted(async () => {
  setting.initAutoBoot();
  setting.getAppVersion();
});
</script>

<style lang="scss" rel="stylesheet/scss"></style>
