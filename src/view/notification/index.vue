<template>
  <div class="notification">
    <v-card>
      <v-img height="190" :src="pageData.imgUrl" cover class="text-white" @click="closeThis">
        <v-toolbar color="rgba(0, 0, 0, 0)" theme="dark">
          <template v-slot:append>
            <v-btn size="small" icon="fas fa-circle-xmark"></v-btn>
          </template>
        </v-toolbar>
      </v-img>
      <v-card-title class="pb-1 pt-1">
        小刻在{{pageData.datasource}}蹲到饼了！
      </v-card-title>
      <v-card-subtitle>{{new Date(pageData.timestamp.platform).toLocaleString()}}</v-card-subtitle>
      <v-card-text class="pt-1 pb-0 text">{{pageData.default_cookie.text}}</v-card-text>
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
import {onMounted, reactive, computed} from "vue";

let pageData = reactive({})

window.notification.getInfo((_, data) => {
  console.log("will render", data);
  pageData = data;
  console.log(pageData);
  let images = data.default_cookie.images;
  console.log(images);
  if (images) {
    if (data.datasource.includes("微博")) {
      console.log("is weibo", images[0].origin_url);
        window.ceobeRequest.getHasRefererImageBase64(images[0].origin_url).then(res => {
          pageData.imgUrl = 'data:image/jpeg;base64,' + res;
      })
    } else {
      console.log("not weibo", images[0].origin_url);
      pageData.imgUrl = images[0].origin_url;
    }
  } else {
    console.log("no image");
    pageData.value.imgUrl = "/assets/image/logo/icon.png";
  }
});
console.log(pageData);

function closeThis(){
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
      &::-webkit-scrollbar {
        display: none; /* Chrome Safari */
      }
    }
  }
}
</style>
