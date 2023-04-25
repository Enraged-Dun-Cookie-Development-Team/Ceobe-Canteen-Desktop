<template>
  <div class="common-window">
    <v-card
        class="mx-auto cursor-pointer"
        @click="common.openUrl"
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
      </v-img>

      <v-card-text>
        <div v-html="info.content.replace(/(\r\n|\n)/g, '<br>')"></div>
      </v-card-text>

      <v-card-actions>
        <slot :info="info"></slot>
      </v-card-actions>

    </v-card>
  </div>
</template>

<script setup>
import {defineProps, onMounted, reactive} from "vue";

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
  openUrl() {
    // 统一格式 只需要标题和url和icon
    let data = {
      url:props.info.jumpUrl,
      source:props.info.dataSource,
      icon:props.info.parent.img
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
