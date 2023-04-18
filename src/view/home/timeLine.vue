<template>
  <div class="time-line">
    <v-timeline
        align="start"
        side="end"
        truncate-line="start"
    >
      <v-timeline-item
          v-for="item in home.timeLineData"
          :key="item.id"
          :left="true"
          fill-dot="fill-dot"
          dot-color="#fff"
          size="50"
      >
        <template v-slot:icon>
          <v-avatar :image="getImage(item.parent.img)"></v-avatar>
        </template>
        <component :is="component.getComponentName(item)" :info="item"></component>
      </v-timeline-item>
    </v-timeline>
  </div>
</template>

<script setup name="timeLine">
import {getCurrentInstance, defineAsyncComponent, onMounted, reactive} from "vue";
import {sourceInfo} from "@/constant"
import {getImage} from "@/utils/imageUtil"

import Music from "@/components/Card/music"
import Info from "@/components/Card/common"
import Terra from "@/components/Card/terra"

const {proxy} = getCurrentInstance();
const home = reactive({
  data: [],
  timeLineData: [],
  getData() {
    window.ceobeRequest.getCardList().then(res => {
      let data = res.data.data;
      home.timeLineData = Object.values(data).flat().sort((x, y) => y.timeForSort - x.timeForSort)
      home.timeLineData.forEach(item => {
        item.parent = sourceInfo.find(x => x.name == item.dataSource)
      })
    })
  }
});
const component = reactive({
  getComponentName(item) {
    if (item.dataSource == "塞壬唱片网易云音乐") {
      return Music
    } else {
      return Info
    }
  }
})
onMounted(() => {
  home.getData();
})
</script>

<style rel="stylesheet/scss" lang="scss">
.time-line {
  height: 100vh;
  overflow: auto;
  min-width: 500px;

  .v-timeline {
    grid-template-columns: 8px min-content 8px;

    .v-timeline-item {
      .v-timeline-item__body {
        -webkit-padding-start: 8px !important;
        padding-inline-start: 8px !important;

        .v-card-title {
          text-shadow: 0 0 5px black;
        }
      }
    }
  }

}
</style>
