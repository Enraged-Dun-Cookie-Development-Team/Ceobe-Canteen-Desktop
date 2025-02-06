<script setup lang="ts">
import { onMounted, onUnmounted, ref, watch } from "vue";

import { confirm } from "@tauri-apps/api/dialog";
import { emit, listen, UnlistenFn } from "@tauri-apps/api/event";
import { exit } from "@tauri-apps/api/process";
import { CloseRequestedEvent, getCurrent } from "@tauri-apps/api/window";

import { handleAsyncException } from "@/api/function";
import updateManager from "@/api/managers/UpdateManager";
import updater from "@/api/operations/updater";
import { ReleaseVersion } from "@/api/resourceFetcher/version";
import VersionPage from "@/components/VersionPage.vue";

const needDisplay = ref(false);
const forceUpdate = ref(false);
const latestVersionInfo = ref<ReleaseVersion | null>(null);
const unListen = ref<UnlistenFn | null>(null);
const getLatestVersion = async () => {
  const latestVersion = await updateManager.latestVersion();
  if (!latestVersion) {
    return;
  }
  latestVersionInfo.value = latestVersion;
  forceUpdate.value = await updater.judgmentVersion(
    latestVersion.previous_mandatory_version,
  );
  needDisplay.value = await updater.judgmentVersion(latestVersion.version);
};

const onClose = async (event: CloseRequestedEvent) => {
  event.preventDefault();
  if (forceUpdate.value) {
    const isConfirm = await confirm("强制更新版本，关闭更新窗口将退出软件", {
      title: "小刻食堂",
      type: "warning",
    });
    if (isConfirm) {
      await exit(0);
    }
  } else {
    await emit("updater-exit");
    await getCurrent().hide();
  }
};

onMounted(() => {
  getLatestVersion().catch(handleAsyncException);
  listen("check-update", () =>
    getLatestVersion().catch(handleAsyncException),
  ).then((exitListen) => (unListen.value = exitListen));
  getCurrent().onCloseRequested(onClose);
});

onUnmounted(() => {
  if (unListen.value) {
    unListen.value();
  }
});

watch(needDisplay, (value) => {
  if (value) {
    getCurrent().show();
  }
});
</script>

<template>
  <version-page
    v-if="needDisplay && latestVersionInfo != null"
    :version-info="latestVersionInfo"
  />
</template>

<style scoped lang="scss"></style>
