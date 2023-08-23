<template>
  <div class="browser w-100">
    <v-toolbar :title="query.source">
      <template #prepend>
        <v-img :src="getImage(query.icon)" width="26" class="border-radius-50"></v-img>
      </template>
      <v-btn size="small" icon="fas fa-circle-xmark" title="关闭" @click="back"></v-btn>
    </v-toolbar>
    <webview
      class="webview"
      :src="query.url"
      style="margin: auto"
      :style="{ width: query.width ? query.width : '100%' }"
      :useragent="query.useragent ? query.useragent : null"
    ></webview>
    <webview
      :src="query.url"
      style="margin: auto"
      :style="{ width: query.width ? query.width : '100%' }"
      :useragent="query.useragent ? query.useragent : null"
    ></webview>
  </div>
</template>

<script setup name="index">
import {useRoute, useRouter} from "vue-router";
import { onMounted, reactive, ref, watch } from "vue";
import { getImage } from "@/utils/imageUtil.ts";

const router = useRouter();
const route = useRoute();
const query = ref({});

const webviewWindow = reactive({
    webview: null,
    init() {
        webviewWindow.webview = document.querySelector("webview");
        webviewWindow.webview.addEventListener("dom-ready", () => {
            webviewWindow.insertCss();
            // webviewWindow.insertJs();
        });
    },
    insertCss() {
    // b站
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
    },
    insertJs() {
        webviewWindow.webview.executeJavaScript("").then((res) => {
            console.log(res);
        });
    },
});

function back() {
    router.push({
        path: "/",
    });
}

watch(
    () => route.query,
    () => {
        query.value = route.query;
        webviewWindow.init();
    }
);

onMounted(() => {
    query.value = route.query;
    webviewWindow.init();
});
</script>

<style rel="stylesheet/scss" lang="scss">
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
