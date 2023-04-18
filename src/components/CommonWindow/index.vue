<template>
  <div class="common-window">
    <v-card
        class="mx-auto"
        max-width="400"
        min-width="400"
    >
      <v-img
          v-if="info.coverImage"
          referrerpolicy="no-referrer"
          class="align-end text-white header-image"
          :src="common.imgUrl"
          max-height="320"
          cover
      >
        <template #default>
          <v-card-title class="text-white text-right">{{info.imageList.length>1?`还有${info.imageList.length - 1}张图`:""}}</v-card-title>
        </template>
      </v-img>
      <!--    <v-row v-if="common.imgUrl.length>0">-->
      <!--      <v-col-->
      <!--          v-for="n in common.imgUrl"-->
      <!--          :key="n"-->
      <!--          class="d-flex child-flex"-->
      <!--          :cols="common.imgUrl.length == 1?12:common.imgUrl.length == 2?6:4"-->
      <!--      >-->
      <!--        <v-img v-for="(item,index) in common.imgUrl"-->
      <!--               class="align-end text-white"-->
      <!--               :max-height="common.imgHeight[index]"-->
      <!--               :src="item"-->
      <!--               cover></v-img>-->
      <!--      </v-col>-->
      <!--    </v-row>-->
      <v-card-text>
        <div v-html="info.content.replace(/(\r\n|\n)/g, '<br>')"></div>
      </v-card-text>

      <v-card-actions>
        <span class="font-weight-bold pl-2">{{ info.timeForDisplay }}</span>
        <v-spacer></v-spacer>
        <v-btn size="small" icon="fas fa-copy"></v-btn>
        <v-btn size="small" icon="fas fa-share-nodes"></v-btn>
        <v-btn size="small" icon="fas fa-link"></v-btn>
      </v-card-actions>
    </v-card>
  </div>
</template>

<script setup>
import {defineProps, onMounted, reactive} from "vue";
import {getImageFromBase64} from "@/utils/imageUtil";


const props = defineProps(['info'])

const common = reactive({
  imgUrl: [],
  getImg() {
    if (props.info.dataSource.includes("微博")) {
      window.ceobeRequest.getHasRefererImageBase64(props.info.coverImage).then(res => {
        common.imgUrl = 'data:image/jpeg;base64,' + res;
      })

      // 留着以后用
      // let promiseList = [];
      // props.info.imageList.forEach(async item => {
      //   promiseList.push(window.ceobeRequest.getWeiboImageBase64(item))
      // })
      // Promise.all(promiseList).then(res => {
      //   if (res && res.length > 0) {
      //     common.imgUrl = res.map(x => `data:image/jpeg;base64,${x}`)
      //   }
      // });
    } else {
      common.imgUrl = props.info.coverImage
    }
  },
  async getHeight(event) {
    common.imgHeight = [];
    let imageData = await getImageFromBase64(event);
    let height = imageData.height > 400 ? 400 : imageData.height
    common.imgHeight.push(height)
  }
})

onMounted(() => {
  if (props.info.coverImage) {
    common.getImg(props.info.coverImage);
  }
})
</script>

<style rel="stylesheet/scss" lang="scss">
.common-window {
  .header-image {
    img {
      object-position: top;
    }
  }
}
</style>
