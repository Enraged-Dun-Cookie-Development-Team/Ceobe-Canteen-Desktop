<template>
  <div class="donate">
    <v-card>
      <v-toolbar color="#e6a23c">
        <v-toolbar-title>版本更新</v-toolbar-title>
        <v-spacer></v-spacer>
        <v-btn 
          v-if="!force"
          variant="text"
          icon="fa-solid fa-xmark"
          @click="version.close"
        ></v-btn>
      </v-toolbar>
      <v-card-text>
        <div v-if="force" class="text-subtitle-1 pl-6">
          更新方式：强制
        </div>
        <div class="text-subtitle-1 pl-6">
          版本：{{ version.currentVersion }} >>> {{ versionInfo.version }} <br>
          {{ versionInfo.description }}
        </div>
      </v-card-text>
      <v-card-actions class="justify-space-around flex-wrap">
        <v-btn
          color="#e6a23c"
          @click="version.openUrl(versionInfo.exe)"
          >Windows安装包
        </v-btn>
        <v-btn
          color="#e6a23c"
          @click="version.openUrl(versionInfo.spare_exe)"
          >Windows备用安装包
        </v-btn>
        <v-btn
          color="#e6a23c"
          @click="version.openUrl(versionInfo.dmg)"
          >Mac安装包
        </v-btn>
        <v-btn
          color="#e6a23c"
          @click="version.openUrl(versionInfo.spare_dmg)"
          >Mac备用安装包
        </v-btn>
        <v-btn
          color="#e6a23c"
          @click="version.openUrl('')"
          >Ubuntu安装包
        </v-btn>
        <v-btn
          color="#e6a23c"
          @click="version.openUrl('')"
          >Ubuntu备用安装包
        </v-btn>
        <v-btn
          color="#e6a23c"
          @click="version.openUrl(versionInfo.baidu)"
          >百度云链接{{ versionInfo.baidu_text }}
        </v-btn>
        <v-btn
          v-if="force"
          color="red"
          @click="version.exist"
          >退出小刻食堂
        </v-btn>
      </v-card-actions>
    </v-card>
  </div>
</template>

<script setup name="version" lang="ts">
import { defineEmits, defineProps, onMounted, reactive, ref } from "vue";
import operate from "@/api/operations/operate";
import { DesktopVersion } from "@/api/resourceFetcher/version";
import { app } from "@tauri-apps/api";

const props = defineProps<{
  versionInfo: DesktopVersion;
  force: boolean
}>();
const emits = defineEmits({
  close: null,
});
const version = reactive<{
  currentVersion: string;
  show: boolean;
  openUrl(url: string): void;
  close(): void;
  exist(): void;
}>({
  currentVersion: '',
  show: false,
  // 查看收支一览表
  openUrl(url: string) {
    console.log(url)
    if (!url) {
      return;
    }
    operate.openUrlInBrowser(url);
  },
  close() {
    emits("close");
  },
  exist() {
    operate.exit();
  },
});
onMounted(() => {
  app.getVersion().then((nowVersion: string) => {
    version.currentVersion = nowVersion;
  })
})
</script>

<style rel="stylesheet/scss" lang="scss"></style>
