<template>
  <div class="setting" data-tauri-drag-region>
    <v-card>
      <v-toolbar id="drag" color="#e6a23c">
        <v-toolbar-title>设置</v-toolbar-title>
        <v-spacer></v-spacer>
        <v-btn
          icon="fa-solid fa-xmark"
          variant="text"
          @click="setting.close"
        ></v-btn>
      </v-toolbar>
      <SettingItem sub-title="每次开机就能自动蹲饼呢~" title="开机自启">
        <template v-slot:action="{ color }">
          <v-switch
            v-model="setting.autoBoot"
            :color="color"
            @change="setting.setAutoBoot"
          >
          </v-switch>
        </template>
      </SettingItem>
      <SettingItem title="版本">
        <template v-slot:sub-title>
          当前版本 {{ setting.currentVersion }}
        </template>
        <template v-slot:action="{ color }">
          <v-btn :color="color" @click="setting.checkUpdate">检查更新 </v-btn>
        </template>
      </SettingItem>
      <SettingItem v-if="isDebug" title="测试弹窗">
        <template v-slot:action>
          <v-btn @click="sen_d"> 弹窗 </v-btn>
        </template>
      </SettingItem>
      <SettingItem>
        <template v-slot:fill-action>
          <v-select
            v-model="setting.notify_mode"
            :hint="setting.notify_mode.tip"
            :items="allNotifyMode"
            item-title="value"
            label="通知设置"
            persistent-hint
            return-object
            @update:model-value="setting.setNotifyMode()"
          >
          </v-select>
        </template>
      </SettingItem>
    </v-card>
    <v-snackbar v-model="showDownload">
      检测到了新版本，即将跳转到下载页面
    </v-snackbar>
    <v-snackbar v-model="showAlreadyNewest">
      已经是最新版本，无需下载
    </v-snackbar>
  </div>
  <v-dialog
    v-model="version.show"
    persistent
    transition="dialog-top-transition"
    width="600"
  >
    <version-page
      :force="version.force"
      :versionInfo="version.version_info"
      @close="version.show = false"
    ></version-page>
  </v-dialog>
</template>

<script lang="ts" name="setting" setup>
import { computed, onMounted, reactive, ref } from "vue";
import VersionPage from "./VersionPage.vue";
import operate from "@/api/operations/operate";
import SettingItem from "@/components/SettingItem/SettingItem.vue";
import notification, {
  allNotifyMode,
  NotifyMode,
} from "@/api/operations/notification";
import { app, invoke } from "@tauri-apps/api";
import updater, { VersionStateType } from "@/api/operations/updater";
import { closeSettingPage } from "@/api/function";
import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/api/notification";
import { getVersion, DesktopVersion } from "@/api/resourceFetcher/version";

const versionState = ref<VersionStateType>(VersionStateType.Unknown);

const isDebug = ref<boolean>(true);

const snackbarShowable = ref(true);

// test send notification
const sen_d = () => {
  setTimeout(() => {
    operate.openNotificationWindow({
      datasource: "kkwd",
      default_cookie: {
        images: [
          {
            compress_url: null,
            origin_url:
              "https://i0.hdslb.com/bfs/new_dyn/2956e376fb056cf79cc95bcf585dbbc0161775300.jpg",
          },
        ],
        text: "欸嘿嘿，桃金娘的脚小小的~香香的~.aaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
      },
      icon: "/assets/icon/anime.png",
      source: { data: "123", type: "nn" },
      timestamp: {
        fetcher: 114514,
        platform: 114514,
        platform_precision: "minute",
      },
      item: {
        id: "123",
        title: "桃金娘",
        url: "https://www.bilibili.com",
      },
    });
  }, 5000);
};

const showDownload = computed(
  () =>
    versionState.value === VersionStateType.UpdateAvailable &&
    snackbarShowable.value,
);
const showAlreadyNewest = computed(
  () =>
    versionState.value === VersionStateType.Newest && snackbarShowable.value,
);

const setting = reactive<{
  // auto boot setting
  autoBoot: boolean;
  initAutoBoot: () => void;
  setAutoBoot: () => void;
  // quit
  close: () => void;
  checkUpdate: () => void;
  // notification
  notify_mode: { idx: number; tip: string; value: string };
  initNotifyMode: () => void;
  setNotifyMode: () => void;
  //version
  currentVersion: string;
  getAppVersion: () => void;
}>({
  currentVersion: "",
  getAppVersion: () => {
    app.getVersion().then((version) => {
      setting.currentVersion = version;
    });
  },
  notify_mode: NotifyMode.PopUpAndBeep,
  initNotifyMode: () => {
    notification.getNotificationMode().then((v) => (setting.notify_mode = v));
  },
  setNotifyMode: () => {
    notification.setNotificationMode(setting.notify_mode.idx);
  },
  autoBoot: false,
  setAutoBoot() {
    operate.bootSetting(setting.autoBoot).then(() => {});
  },
  initAutoBoot() {
    operate.getBootSetting().then((res) => {
      setting.autoBoot = res;
    });
  },
  close() {
    closeSettingPage();
  },

  checkUpdate(): void {
    version.getNewestVersion();
  },
});

const ErrVersionInfo: DesktopVersion = {
  baidu: "<Missing>",
  baidu_text: "<Missing>",
  description: "<Missing>",
  dmg: "<Missing>",
  exe: "<Missing>",
  force: false,
  last_force_version: "<Missing>",
  spare_dmg: "<Missing>",
  spare_exe: "<Missing>",
  version: "<Missing>",
};

const version = reactive<{
  show: boolean;
  version_info: DesktopVersion;
  force: boolean;
  getNewestVersion(): void;
}>({
  show: false,
  force: false,
  version_info: ErrVersionInfo,
  async getNewestVersion() {
    try {
      versionState.value = VersionStateType.Newest;
      const currentVersion = await getVersion();
      version.version_info = currentVersion.data.data;
      // version.force = await updater.judgmentVersion(version.version_info.last_force_version)
      version.show = await updater.judgmentVersion(
        version.version_info.version,
      );
      if (version.show) {
        versionState.value = VersionStateType.UpdateAvailable;
      }
      snackbarShowable.value = true;
      setTimeout(() => {
        snackbarShowable.value = false;
      }, 5000);
    } catch (error: any) {
      console.log("Failure loading New Version");
      if (!(await isPermissionGranted())) {
        await requestPermission();
      }
      sendNotification({
        title: "小刻出错了！",
        icon: "/asserts/icon.png",
        body: error.toString(),
      });
    }
  },
});

onMounted(async () => {
  setting.initAutoBoot();
  setting.initNotifyMode();
  setting.getAppVersion();
  isDebug.value = await invoke("is_debug");
});
</script>

<style lang="scss" rel="stylesheet/scss"></style>
