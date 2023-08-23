<template>
  <div class="announcement mt-2">
    <v-card>
      <v-carousel
        v-model="announcementInfo.index"
        cycle
        interval="3000"
        :show-arrows="false"
        height="160"
        :continuous="false"
        hide-delimiter-background
        delimiter-icon="mdi: mdi-square"
      >
        <v-carousel-item v-for="(item, i) in announcementInfo.data" :key="i">
          <div v-html="item.html"></div>
        </v-carousel-item>
      </v-carousel>
    </v-card>
  </div>
</template>

<script setup name="announcement">
import { onMounted, reactive } from 'vue';
import { changeToCCT } from '@/utils/timeUtil.ts';

const announcementInfo = reactive({
  data: [],
  index: 0,
  getData() {
    window.ceobeRequest.getAnnouncementInfo().then((res) => {
      if (res.status == 200) {
        announcementInfo.data = res.data.data.filter(
          (x) => new Date(x.start_time) <= changeToCCT(new Date()) && new Date(x.over_time) >= changeToCCT(new Date())
        );
      }
    });
  },
});

onMounted(() => {
  announcementInfo.getData();
});
</script>

<style rel="stylesheet/scss" lang="scss">
.announcement {
  .online-area {
    display: flex;
    justify-content: center;
    align-items: center;
    margin-top: 7px;
    font-size: 14px;

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
