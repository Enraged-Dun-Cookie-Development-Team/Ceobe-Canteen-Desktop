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
      <SettingItem sub-title="每次开机就能自动蹲饼呢~" title="开机自启">
        <template v-slot:action="{color}">
          <v-switch
              v-model="setting.autoBoot"
              :color=color
              @change="setting.setAutoBoot"
          >
          </v-switch>
        </template>

      </SettingItem>
      <SettingItem title="版本">
        <template v-slot:sub-title>
          当前版本 {{ setting.currentVersion }}
        </template>
        <template v-slot:action="{color}">
          <v-btn :color="color" @click="setting.checkUpdate"
          >检查更新
          </v-btn
          >
        </template>
      </SettingItem>
      <SettingItem
          title="测试弹窗"
      >
        <template v-slot:action>
          <v-btn @click="sen_d">
            弹窗
          </v-btn>
        </template>
      </SettingItem>
      <SettingItem>
        <template v-slot:fill-action>
          <v-select v-model="setting.notify_mode" :hint='setting.notify_mode.tip'
                    :items="allNotifyMode"
                    item-title="value"
                    persistent-hint
                    return-object
                    label="通知设置"
                    @update:model-value="setting.setNotifyMode()"
          >
          </v-select>
        </template>

      </SettingItem>
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
import {computed, onMounted, PropType, reactive, ref} from "vue";
import operate from "../api/operations/operate";
import SettingItem from "@/components/SettingItem/SettingItem.vue";
import notification, {allNotifyMode, NotifyMode} from "@/api/operations/notification";
import {app} from "@tauri-apps/api";

const emits = defineEmits({
  close: null,
  checkUpdate: null,
});

enum versionStateType {
  Newest = "Newest",
  UpdateAvailable = "UpdateAvailable",
  Unknown = "Unknown",
}

const props = defineProps({
  versionState: {
    type: String as PropType<versionStateType>,
    default: () => "Unknown"
  },
});
// test send notification
const sen_d = () => {
  operate.openNotificationWindow({
    datasource: 'kkwd',
    default_cookie: {
      images: [{compress_url: null, origin_url: 'https://i0.hdslb.com/bfs/new_dyn/2956e376fb056cf79cc95bcf585dbbc0161775300.jpg'}],
      text: '欸嘿嘿，桃金娘的脚小小的~香香的~.'
    },
    icon: '/assets/icon/anime.png',
    source: {data: '123', type: 'nn'},
    timestamp: {fetcher: 114514, platform: 114514, platform_precision: 'minute'}
  })
}

const showDownload = computed(() => props.versionState == "UpdateAvailable")
const showAlreadyNewest = computed(() => props.versionState == "Newest")

const setting = reactive<{
  // auto boot setting
  autoBoot: boolean,
  initAutoBoot: () => void,
  setAutoBoot: () => void,
  // quit 
  close: () => void,
  checkUpdate: () => void
  // notification
  notify_mode: { idx: string, tip: string, value: string },
  initNotifyMode: () => void
  setNotifyMode: () => void
  //version
  currentVersion: string,
  getAppVersion: () => void
}>({
  currentVersion: "",
  getAppVersion: () => {
    app.getVersion().then((version) => {
      setting.currentVersion = version
    })
  },
  notify_mode: NotifyMode.PopUpAndBeep,
  initNotifyMode: () => {
    notification.getNotificationMode()
        .then((v) => setting.notify_mode = v)
  },
  setNotifyMode: () => {

    notification.setNotificationMode(setting.notify_mode.idx)
  },
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

  checkUpdate(): void {
    emits("checkUpdate");
  },
});

onMounted(async () => {
  setting.initAutoBoot();
  setting.initNotifyMode();
  setting.getAppVersion();
});
</script>

<style lang="scss" rel="stylesheet/scss"></style>
