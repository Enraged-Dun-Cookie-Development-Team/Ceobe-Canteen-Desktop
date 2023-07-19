<template>
  <div class="notification">
    <v-card>
      <v-img height="190" :src="info.imgUrl" cover class="text-white" @click="closeThis">
        <v-toolbar color="rgba(0, 0, 0, 0)" theme="dark">
          <template #append>
            <v-btn size="small" icon="fas fa-circle-xmark"></v-btn>
          </template>
        </v-toolbar>
      </v-img>
      <v-card-title class="pb-1 pt-1"> 小刻在{{ info.dataSource }}蹲到饼了！ </v-card-title>
      <v-card-subtitle>{{ info.cookieTime }}</v-card-subtitle>
      <v-card-text class="pt-1 pb-0 text">{{ info.cookieText }}</v-card-text>
      <v-card-actions class="pt-0">
        <v-spacer></v-spacer>
        <v-btn size="x-small" icon="fas fa-copy"></v-btn>
        <v-btn size="x-small" icon="fas fa-share-nodes"></v-btn>
        <v-btn size="x-small" icon="fas fa-link"></v-btn>
      </v-card-actions>
    </v-card>
  </div>
</template>

<script setup name="index">
import { getImage } from '@/utils/imageUtil';
import { reactive } from 'vue';

const info = reactive({
  imgUrl: getImage('/assets/image/logo/icon.png'),
  dataSource: '',
  cookieTime: '',
  cookieText: '',
});

const updatePageData = (newData) => {
  info.dataSource = newData.datasource;
  info.cookieTime = new Date(newData.timestamp.platform).toLocaleString();
  info.cookieText = newData.default_cookie.text;

  let images = newData.default_cookie.images;
  if (images) {
    if (newData.datasource.includes('微博')) {
      window.ceobeRequest.getHasRefererImageBase64(images[0].origin_url).then((res) => {
        info.imgUrl = 'data:image/jpeg;base64,' + res;
      });
    } else {
      info.imgUrl = images[0].origin_url;
    }
  } else {
    console.log('no image');
  }
};

window.notification.getInfo((_, data) => {
  updatePageData(data);
});

function closeThis() {
  window.notification.closeWindow();
}
</script>

<style rel="stylesheet/scss" lang="scss">
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
