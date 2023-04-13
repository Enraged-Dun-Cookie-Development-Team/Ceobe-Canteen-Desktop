<template>
  <v-card
      class="mx-auto"
      max-width="400"
      min-width="400"
  >
    <v-img
        v-if="info.coverImage"
        referrerpolicy="no-referrer"
        class="align-end text-white "
        height="200"
        :src="common.imgUrl"
        cover
    >
    </v-img>
    <v-card-text>
      <div v-html="info.content.replace(/(\r\n|\n)/g, '<br>')"></div>
    </v-card-text>

    <v-card-actions>
      <v-spacer></v-spacer>
      <v-btn size="small" icon="fas fa-copy"></v-btn>
      <v-btn size="small" icon="fas fa-share-nodes"></v-btn>
      <v-btn size="small" icon="fas fa-link"></v-btn>
    </v-card-actions>
  </v-card>
</template>

<script setup>
import {defineProps, onMounted, reactive} from "vue";


const props = defineProps(['info'])

const common = reactive({
  imgUrl: null,
  getImg() {
    if (props.info.dataSource.includes("微博")) {
      window.ceobeRequest.getWeiboImageBase64(props.info.coverImage).then(res => {
        common.imgUrl = `data:image/jpeg;base64,${res}`;
      })
    } else {
      common.imgUrl = props.info.coverImage
    }
  }
})

onMounted(() => {
  common.getImg(props.info.coverImage);
})
</script>

<style rel="stylesheet/scss" lang="scss">

</style>
