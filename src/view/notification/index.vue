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
      <v-card-title class="pb-1 pt-1">小刻在{{ pageData.dataSource }}蹲到饼了！</v-card-title>
      <v-card-subtitle>{{ pageData.timeForDisplay }}</v-card-subtitle>
      <v-card-text class="pt-1 pb-0 text">{{ pageData.content }}</v-card-text>
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
import { onMounted, ref } from 'vue';

let pageData = ref({});
window.notification.getInfo((event, data) => {
  pageData.value = data;
  if (pageData.value.dataSource.includes('微博')) {
    window.ceobeRequest.getHasRefererImageBase64(pageData.value.coverImage).then(res => {
      pageData.value.imgUrl = 'data:image/jpeg;base64,' + res;
    });
  } else {
    pageData.value.imgUrl = pageData.value.coverImage;
  }
});
onMounted(() => {
  setTimeout(() => {
    closeThis();
  }, 10000);
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
      &::-webkit-scrollbar {
        display: none; /* Chrome Safari */
      }
    }
  }
}
</style>
