<template>
  <div class="announcement">
    <v-card>
      <v-carousel :show-arrows="false"
                  height="160"
                  :continuous="false"
                  v-model="announcementInfo.index"
                  hide-delimiter-background
                  delimiter-icon="mdi: mdi-square"
      >
        <v-carousel-item
            v-for="(item,i) in announcementInfo.data"
            :key="i"
        >
          <div v-html="item.html"></div>
        </v-carousel-item>
      </v-carousel>
    </v-card>

  </div>
</template>

<script setup name="announcement">

import {onMounted, reactive} from "vue";
import {changeToCCT} from "@/utils/timeUtil";

const announcementInfo = reactive({
  data: [],
  index: 0,
  getData() {
    window.ceobeRequest.getAnnouncementInfo().then(res => {
      announcementInfo.data = res.data.data.filter(
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
  .online-area {
    display: flex;
    align-items: center;
    font-size: 14px;
    margin-top: 7px;

    p {
      margin: 0;
    }

    // 菜单按钮

    drawer {
      color: #dd558a;
    }

    // 设置按钮

    setting {
      color: #dd55c4;
    }

    .online-title-img {
      margin: auto 10px;
      height: 105px;

      &.radius {
        border-radius: 4px;
      }
    }
  }
}
</style>