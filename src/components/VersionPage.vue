<script setup name="version" lang="ts">
import { computed, defineEmits, defineProps, onMounted, reactive } from "vue";

import { app } from "@tauri-apps/api";

import operate from "@/api/operations/operate";
import { ReleaseVersion } from "@/api/resourceFetcher/version";

import DownloadSource from "./DownloadSource.vue";

const props = defineProps<{
  versionInfo: ReleaseVersion;
}>();
const emits = defineEmits<{
  (e: "close"): void;
}>();

// 如果更新版本即为前一强制更新版本，需要强制更新
const mandatory = computed(
  () =>
    props.versionInfo.previous_mandatory_version === props.versionInfo.version,
);

const version = reactive<{
  currentVersion: string;
  show: boolean;
  openUrl(url: string): void;
  close(): void;
  exist(): void;
}>({
  currentVersion: "",
  show: false,
  // 查看收支一览表
  openUrl(url: string) {
    console.log(url);
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
  });
});
</script>

<template>
  <div class="donate">
    <v-card>
      <v-card-text>
        <div v-if="mandatory" class="text-subtitle-1 pl-6">更新方式：强制</div>
        <div class="text-subtitle-1 pl-6">
          版本：{{ version.currentVersion }} >>> {{ versionInfo.version }}
          <br />
          {{ versionInfo.description }}
        </div>
        <div v-for="source in versionInfo.download_source" :key="source.name">
          <download-source :source="source"> </download-source>
        </div>
      </v-card-text>
      <v-card-actions class="justify-space-around flex-wrap">
        <v-btn v-if="mandatory" color="red" @click="version.exist"
          >退出小刻食堂
        </v-btn>
      </v-card-actions>
    </v-card>
  </div>
</template>

<style rel="stylesheet/scss" lang="scss"></style>
