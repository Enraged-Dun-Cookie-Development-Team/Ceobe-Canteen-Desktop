<template>
  <div class="announcement">
    <v-carousel :show-arrows="false">
      <v-carousel-item
          v-for="(item,i) in announcementInfo.data"
          :key="i"
      >
        <div v-html="item.html"></div>
      </v-carousel-item>
    </v-carousel>
  </div>
</template>

<script setup name="announcement">

import {onMounted, reactive} from "vue";
import {changeToCCT} from "@/utils/timeUtil";

const announcementInfo = reactive({
  data: [],
  getData() {
    window.ceobeRequest.getAnnouncementInfo().then(res => {
      announcementInfo.data = res.data.filter(
          (x) =>
              new Date(x.start_time) <= changeToCCT(new Date()) &&
              new Date(x.over_time) >= changeToCCT(new Date())
      );
    })
  }
})

onMounted(() => {
  announcementInfo.getData();
})
</script>

<style rel="stylesheet/scss" lang="scss">
.announcement {
}
</style>