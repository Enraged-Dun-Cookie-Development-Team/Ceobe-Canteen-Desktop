<template>
  <div class="video mt-2 d-flex flex-wrap justify-space-between">
    <v-card v-for="(item, index) in videoInfo.list" :key="index" width="320" class="mb-2">
      <!--      {{ item.video_link }}-->
      <v-img class="align-end text-white" width="100%" :src="item.cover_img.split('@')[0]" cover>
        <v-card-title class="text-right">{{ item.author }}</v-card-title>
      </v-img>

      <v-card-subtitle class="py-2">
        {{ item.title }}
      </v-card-subtitle>
    </v-card>
    <i />
    <i />
    <i />
    <i />
    <i />
  </div>
</template>

<script setup name="video">
import { onMounted, reactive } from 'vue';
import { getVideoList } from '@/api/list';
import { changeToCCT } from '@/utils/timeUtil';

const videoInfo = reactive({
  list: [],
  getList() {
    getVideoList().then((res) => {
      // 快捷连接
      videoInfo.list = res.data.data.filter(
        (x) => new Date(x.start_time) <= changeToCCT(new Date()) && new Date(x.over_time) >= changeToCCT(new Date())
      );
    });
  },
});

onMounted(() => {
  videoInfo.getList();
});
</script>

<style rel="stylesheet/scss" lang="scss">
.video {
  .v-card {
    .v-card-title {
      text-shadow: 0 0 5px black;
    }

    .v-img__img {
      object-position: top;
    }

    .v-card-subtitle {
      white-space: normal;
    }
  }

  i {
    width: 320px;
  }
}
</style>
