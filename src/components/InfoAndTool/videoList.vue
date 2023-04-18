<template>
  <div class="video d-flex flex-wrap justify-space-between">
    <v-row>
      <v-col :cols="6" v-for="item in videoInfo.list">
        <v-card
            class="mx-auto"
            max-width="300"

        >
          <!--      {{item.video_link}}-->
          <v-img
              class="align-end text-white"
              height="100"
              :src="item.base64Url"
              cover
          >
            <v-card-title class="text-right">{{ item.author }}</v-card-title>
          </v-img>

          <v-card-subtitle class="pt-4">
            {{ item.title }}
          </v-card-subtitle>
        </v-card>
      </v-col>
    </v-row>

  </div>
</template>

<script setup name="video">
import {onMounted, reactive} from "vue";
import {getVideoList} from "@/api/list";
import {changeToCCT} from "@/utils/timeUtil";

const videoInfo = reactive({
  list: [],
  getList() {
    getVideoList().then(res => {
      // 快捷连接
      videoInfo.list = res.data.data.filter(
          (x) =>
              new Date(x.start_time) <= changeToCCT(new Date()) &&
              new Date(x.over_time) >= changeToCCT(new Date())
      );
      let promiseList = [];
      videoInfo.list.forEach(item => {
        promiseList.push(window.ceobeRequest.getHasRefererImageBase64(item.cover_img.split('@')[0], null))
      })
      Promise.all(promiseList).then(res => {
        res.forEach((item, index) => {
          videoInfo.list[index].base64Url = "data:image/jpeg;base64," + item;
        })
      })
    })
  }
})

onMounted(() => {
  videoInfo.getList();
})
</script>

<style rel="stylesheet/scss" lang="scss">
.video {

  .v-card-title {
    text-shadow: 0 0 5px black;
  }

  .v-img__img {
    object-position: top;
  }

  .v-card-subtitle{
    white-space: normal;
  }
}
</style>