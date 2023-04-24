<template>
  <div class="terra-window">
    <v-card
        class="mx-auto"
        min-width="400"
    >
      <v-img
          v-if="info.coverImage"
          referrerpolicy="no-referrer"
          class="align-end text-white"
          :src="terra.imgUrl"
          cover
      >
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
        <v-btn size="small" :icon="terra.show ? 'mdi:mdi-chevron-up' : 'mdi: mdi-chevron-down'"
               @click="terra.show = !terra.show"></v-btn>
      </v-card-actions>
      <v-expand-transition>
        <div v-show="terra.show">
          <v-divider></v-divider>
          <v-card-text>
            {{info.componentData.title}}
          </v-card-text>
          <v-card-subtitle>
            {{info.componentData.introduction}}
          </v-card-subtitle>
          <v-card-text>
            <v-btn @click="terra.openUrl(item.cid)"
                   v-for="item in info.componentData.episodes">
              {{item.title}}</v-btn>
          </v-card-text>
        </div>
      </v-expand-transition>
    </v-card>
  </div>
</template>

<script setup>
import {defineProps, onMounted, reactive} from "vue";


const props = defineProps(['info'])
const emits = defineEmits();

const terra = reactive({
  imgUrl: [],
  show:false,

  getImg() {
    if (props.info.dataSource.includes("微博")) {
      window.ceobeRequest.getHasRefererImageBase64(props.info.coverImage).then(res => {
        terra.imgUrl = 'data:image/jpeg;base64,' + res;
      })
    } else {
      terra.imgUrl = props.info.coverImage
    }
  },
  openImage() {
  },
  openUrl(cid) {
    // 统一格式 只需要标题和url和icon
    let data = {
      url:`https://terra-historicus.hypergryph.com/comic/${props.info.componentData.cid}/episode/${cid}`,
      source:props.info.dataSource,
      icon:props.info.parent.img,
      useragent:"Mozilla/5.0 (iPhone; CPU iPhone OS 12_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.0 Mobile/15E148 Safari/604.1"
    }
    emits('openUrl', data)
  }
})

onMounted(() => {
  if (props.info.coverImage) {
    terra.getImg(props.info.coverImage);
  }
})
</script>

<style rel="stylesheet/scss" lang="scss">
.terra-window {
}
</style>
