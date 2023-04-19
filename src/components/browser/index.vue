<template>
  <div class="browser w-100">
    <v-toolbar :title="query.source">
      <template #prepend>
        <v-img :src="getImage(query.icon)" width="26" class="border-radius-50"></v-img>
      </template>
      <v-btn size="small" @click="back" icon="fas fa-window-restore" title="使用浏览器打开"></v-btn>
      <v-btn size="small" @click="back" icon="fas fa-circle-xmark" title="关闭"></v-btn>
    </v-toolbar>
    <webview :src="query.url"></webview>
  </div>
</template>

<link rel="stylesheet" type="text/css" href="">

<script setup name="index">
import {useRoute} from "vue-router";
import {onMounted, reactive, ref} from "vue";
import router from "@/router";
import {getImage} from "@/utils/imageUtil"


const route = useRoute();
const query = ref({})

const webviewWindow = reactive({
  webview: null,
  init() {
    webviewWindow.webview = document.querySelector('webview')
    webviewWindow.webview.addEventListener('dom-ready', (data) => {
      webviewWindow.insertCss();
      // webviewWindow.insertJs();
    })
  },
  insertCss() {
    // b站
    webviewWindow.webview.insertCSS(`


        `);
  },
  insertJs() {
    webviewWindow.webview.executeJavaScript(``).then(res => {
     console.log(res)
    })
  }
})

function back() {
  router.back();
}

onMounted(() => {
  query.value = route.query
  webviewWindow.init();
})

</script>

<style rel="stylesheet/scss" lang="scss">
.browser {
  .title {
    height: 30px;
  }

  webview {
    width: 100%;
    height: 100%;
  }
}
</style>