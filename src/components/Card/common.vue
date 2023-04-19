<template>
  <div class="common-window">
    <v-card
        class="mx-auto cursor-pointer"
        @click="common.openUrl(info)"
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
        <template #default v-if="info.imageList">
          <v-card-title class="text-white text-right">
            {{ info.imageList.length > 1 ? `还有${info.imageList.length - 1}张图 →` : "" }}
          </v-card-title>
        </template>
      </v-img>

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
import {getImage, getImageFromBase64} from "@/utils/imageUtil";


const props = defineProps(['info'])
const emits = defineEmits();

const common = reactive({
  imgUrl: [],
  getImg() {
    if (props.info.dataSource.includes("微博")) {
      window.ceobeRequest.getHasRefererImageBase64(props.info.coverImage).then(res => {
        common.imgUrl = 'data:image/jpeg;base64,' + res;
      })
    } else {
      common.imgUrl = props.info.coverImage
    }
  },
  openImage() {
  },
  openUrl(item) {
    // 统一格式 只需要标题和url和icon
    let data = {
      url:item.jumpUrl,
      source:item.dataSource,
      icon:item.parent.img
    }
    emits('openUrl', data)
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
