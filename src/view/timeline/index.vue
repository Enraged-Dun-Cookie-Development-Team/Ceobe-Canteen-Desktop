<template>
  <div class="time-line">
    <v-timeline
        align="start"
        side="end"
    >
      <v-timeline-item
          v-for="item in home.timeLineData"
          :key="item.id"
          :left="true"
      >
        <template v-slot:icon>
          <v-avatar :image="proxy.getImg('..'+item.parent.img)"></v-avatar>
        </template>
        <component :is="component.getComponentName(item)" :info="item" ></component>
      </v-timeline-item>
    </v-timeline>
  </div>
</template>

<script setup name="timeLine">
import {getCurrentInstance,defineAsyncComponent, onMounted, reactive} from "vue";
import {sourceInfo} from "@/constant"

import Music from "@/components/MusicWindow"
import Info from "@/components/InfoWindow"
import Terra from "@/components/TerraWindow"

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
  getComponentName(item){
    if(item.dataSource == "塞壬唱片网易云音乐"){
      return Music
    }else{
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
  //height: 100vh;
  //overflow: auto;
}
</style>