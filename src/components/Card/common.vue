<template>
  <div class="common-window">
    <v-card
        class="mx-auto cursor-pointer"
        @click="common.openUrl"
        width="400"
    >
      <v-img
          v-if="info.default_cookie.images"
          referrerpolicy="no-referrer"
          class="align-end text-white header-image"
          :src="common.getImg()"
          max-height="320"
          cover
      >
      </v-img>

      <v-card-text>
        <div v-html="info.default_cookie.text.replace(/(\r\n|\n)/g, '<br>')"></div>
      </v-card-text>

      <v-card-actions>
        <slot :info="info"></slot>
      </v-card-actions>

    </v-card>
  </div>
</template>

<script setup>
import {onMounted, reactive, toRaw} from "vue";

const props = defineProps(['info'])
const emits = defineEmits();
const cookie = toRaw(props.info)

const common = reactive({
  imgUrl: [],
  getImg() {
    if (!cookie.default_cookie.images) {
      common.imgUrl = [];
    } else if (cookie.datasource.includes("微博")) {
      window.ceobeRequest.getHasRefererImageBase64(cookie.default_cookie.images[0].compress_url).then(res => {
        common.imgUrl = 'data:image/jpeg;base64,' + res;
      })
    } else {
      common.imgUrl = cookie.default_cookie.images[0].compress_url;
    }
    return common.imgUrl;
  },
  openImage() {
  },
  openUrl() {
    // 统一格式 只需要标题和url和icon
    let data = {
      url:props.info.item.url,
      source:props.info.dataSource,
      icon:props.info.icon
    }
    emits('openUrl', data)
  }
})

onMounted(() => {
  if (props.info.default_cookie.image) {
    console.log(props.info)
    common.getImg(props.info.default_cookie.image);
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
