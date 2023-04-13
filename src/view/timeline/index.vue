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
        <v-card
            class="mx-auto"
            max-width="400"
        >
          <v-img
              v-if="false"
              class="align-end text-white"
              height="200"
              :src="item.coverImage"
              cover
          >
          </v-img>
          <v-card-text>
            {{ item.content }}
          </v-card-text>

          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn size="small" icon="fas fa-copy"></v-btn>
            <v-btn size="small" icon="fas fa-share-nodes"></v-btn>
            <v-btn size="small" icon="fas fa-link"></v-btn>
          </v-card-actions>
        </v-card>
      </v-timeline-item>
    </v-timeline>
  </div>
</template>

<script setup name="timeLine">
import {getCurrentInstance, onMounted, reactive} from "vue";
import {sourceInfo} from "@/constant"

const {proxy} = getCurrentInstance();
const home = reactive({
  data: [],
  timeLineData: [],
  getData() {
    window.ceobeRequest.getCardList().then(res => {
      let data = res.data.data;
      home.timeLineData = Object.values(data).flat().sort((x, y) => x.timeForSort - y.timeForSort)
      home.timeLineData.forEach(item => {
        item.parent = sourceInfo.find(x => x.name == item.dataSource)
      })
      console.log(home.timeLineData)
    })
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