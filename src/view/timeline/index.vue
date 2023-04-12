<template>
  <div class="time-line">
    <v-timeline
        align="start"
        side="end"
    >
      <v-timeline-item
          v-for="n in 3"
          :key="n"
          icon="mdi-star"
          :left="true"
      >
        <template v-slot:icon>
          <v-avatar image="https://i.pravatar.cc/64"></v-avatar>
        </template>
          <v-card
              v-for="item in home.timeLineData"
              class="mx-auto"
              max-width="400"
          >
            <v-img
                class="align-end text-white"
                height="200"
                src="https://cdn.vuetifyjs.com/images/cards/docks.jpg"
                cover
            >
            </v-img>
            <v-card-text>

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
import {onMounted, reactive} from "vue";
import {sourceInfo} from "@/constant"

const home = reactive({
  data: [],
  timeLineData: [],
  getData() {
    window.ceobeRequest.getCardList().then(res => {
      let data = res.data.data;
      home.timeLineData = Object.values(data).flat().sort((x, y) => x.timeForSort - y.timeForSort)
    })
  }
})
onMounted(() => {
  home.getData();
})
</script>

<style rel="stylesheet/scss" lang="scss">
.time-line {
  height: 100vh;
}
</style>