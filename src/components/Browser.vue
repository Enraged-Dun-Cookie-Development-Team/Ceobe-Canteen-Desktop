<template>
  <div class="browser w-100">
    <v-toolbar :title="query.source">
      <template #prepend>
        <v-img
          :src="getImage(query.icon)"
          class="border-radius-50"
          width="26"
        ></v-img>
      </template>
      <v-btn
        icon="fas fa-circle-xmark"
        size="small"
        title="关闭"
        @click="back"
      ></v-btn>
    </v-toolbar>
<!--    <iframe-->
<!--      :src="query.url"-->
<!--      :style="{ width: query.width ? query.width : '100%' }"-->
<!--      :useragent="query.useragent ? query.useragent : null"-->
<!--      class="webview"-->
<!--      style="margin: auto"-->
<!--    ></iframe>-->
<!--    <iframe-->
<!--      :src="query.url"-->
<!--      :style="{ width: query.width ? query.width : '100%' }"-->
<!--      :useragent="query.useragent ? query.useragent : null"-->
<!--      style="margin: auto"-->
<!--    ></iframe>-->
  </div>
</template>

<script lang="ts" name="index" setup>
import { useRoute, useRouter } from "vue-router";
import { onMounted, onUnmounted, reactive, ref, watch } from "vue";
import { getImage } from "../utils/imageUtil";
import {invoke} from "@tauri-apps/api";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

const router = useRouter();
const route = useRoute();
const query = ref({});

interface WebView extends HTMLElement {
  insertCSS: (string) => void;
  executeJavaScript: (string) => any;
}

const webviewWindow = reactive<{
  webview: null | WebView;
  init: () => void;
  insertCss: () => void;
  insertJs: () => void;
}>({
  webview: null,
  init() {
    webviewWindow.webview = document.querySelector("iframe");
    if (webviewWindow.webview)
      webviewWindow.webview.addEventListener("dom-ready", () => {
        webviewWindow.insertCss();
        // webviewWindow.insertJs();
      });
  },
  insertCss() {
    if (webviewWindow.webview) {
      // b站
      //@ts:ignore
      webviewWindow.webview.insertCSS(`
        #bili-header-container,
        #internationalHeader,
        .van-popover,
        .international-header,
        .login-tip,
        .bili-dyn-card-link-common,
        .bili-dyn-item__footer,
        .bili-dyn-item__panel{
          display:none!important
        }
        `);

      // 微博
      webviewWindow.webview.insertCSS(`
    [class^="Frame_top_"],
    [class^="Frame_side_"],
    [class^="Bar_main_"],
    [class^="title_wrap_"],
    .woo-panel-main>footer,
    [class^="Detail_box_"],
    [class^="Main_side_"]{
          display:none!important
        }
        `);
    }
  },
  insertJs() {
    if (webviewWindow.webview) {
      webviewWindow.webview.executeJavaScript("").then((res) => {
        console.log(res);
      });
    }
  },
});

function back() {
  invoke("back_preview")
  router.push({
    path: "/",
  });
}

watch(
  () => route.query,
  () => {
    query.value = route.query;
    webviewWindow.init();
  },
);

let unlisten: UnlistenFn;
onMounted(() => {
  query.value = route.query;
  webviewWindow.init();
  listen("close-main", () => {
    back();
  }).then((unListen: UnlistenFn) => {
    unlisten = unListen
  });
});
onUnmounted(() => {
  unlisten();
})
</script>

<style lang="scss" rel="stylesheet/scss">
.browser {
  .title {
    height: 30px;
  }

  .webview {
    width: 100%;
    height: calc(100% - 64px);
  }
}
</style>
