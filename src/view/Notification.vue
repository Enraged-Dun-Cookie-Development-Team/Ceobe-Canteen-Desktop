<template>
  <div class="notification ">
    <v-card>
      <v-img
          :cover="info.setImg"
          :src="info.imgUrl"
          class="text-white"
          height="190"
          @click="closeThis"
      >
        <v-toolbar color="rgba(0, 0, 0, 0)" theme="dark">
          <template #append>
            <v-btn icon="fas fa-circle-xmark" size="xs" @click.stop="appWindow.hide()"   class="elevation-4" color="amber-accent-4"></v-btn>
          </template>
        </v-toolbar>
      </v-img>
      <v-card-title class="pb-1 pt-1">
        小刻在{{ info.dataSource }}蹲到饼了！
      </v-card-title>
      <v-card-subtitle>{{ info.cookieTime }}</v-card-subtitle>
      <v-card-text class="pt-1 pb-0 text">{{ info.cookieText }}</v-card-text>
    </v-card>
  </div>
</template>

<script lang="ts" setup>
import {onMounted, onUnmounted, ref} from "vue";
import {getImage} from "@/utils/imageUtil";
import ceobeRequest from "../api/operations/ceobeRequest";
import notification from "../api/operations/notification";
import {Cookie} from "@/api/resourceFetcher/cookieList";
import {UnlistenFn} from "@tauri-apps/api/event";
import operate from "@/api/operations/operate";
import {appWindow} from "@tauri-apps/api/window";

const info = ref({
  setImg: false,
  imgUrl: getImage("/assets/image/logo/icon.png"),
  dataSource: "",
  cookieTime: "",
  cookieText: "",
});

const updatePageData = (newData: Cookie) => {

  info.value.dataSource = newData.datasource;
  if (newData.timestamp.platform_precision !== "none") {
    info.value.cookieTime = new Date(newData.timestamp.platform!).toLocaleString();
  }
  info.value.cookieText = newData.default_cookie.text;

  let images = newData.default_cookie.images;
  if (images) {
    info.value.setImg = true;
    if (newData.datasource.includes("微博")) {
      ceobeRequest
          .getHasRefererImageBase64(images[0].origin_url)
          .then((res) => {
            info.value.imgUrl = "data:image/jpeg;base64," + res;
          });
    } else {
      info.value.imgUrl = images[0].origin_url;

    }
  } else {
    console.log("no image");
    info.value.imgUrl = getImage("/assets/image/logo/icon.png")
    info.value.setImg = false;
  }


};
let unliten: UnlistenFn;
onMounted(() => {
  notification.getInfo(async (_, data) => {
    await operate.hideNotifyIcon()
    console.log("get Info: ", data)
    updatePageData(data);
    await appWindow.show()
    setTimeout(()=>{
      appWindow.hide()
    },10000);
    if (await notification.needBeep()) {

      await operate.messageBeep()
    }
  }).then((closer) => {
    unliten = closer
  });
})

onUnmounted(() => {
  if (unliten) {
    unliten()
  }
})

function closeThis() {
  notification.closeWindow();
}
</script>

<style lang="scss" rel="stylesheet/scss">
html {
  overflow: hidden;
}

.notification {
  .v-card {
    width: 100vw;
    height: 100vh;

    img {
      object-position: top;
    }

    .text {
      overflow: auto;
      height: 75px;
      white-space: break-spaces;

      &::-webkit-scrollbar {
        display: none; /* Chrome Safari */
      }
    }
  }
}
</style>
