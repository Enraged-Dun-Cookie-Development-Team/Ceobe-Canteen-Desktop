<template>
  <div class="terra-window">
    <v-card
        class="mx-auto"
        width="400"
        @click="terra.show = !terra.show"
    >
      <v-img
          v-if="info.coverImage"
          referrerpolicy="no-referrer"
          class="align-end text-white"
          :src="terra.imgUrl"
          cover
      >
      </v-img>

      <v-card-title class="text-shadow-none white-space-normal">
        {{info.componentData.title}}
      </v-card-title>
      <v-card-subtitle>最新话：{{info.componentData.episodes[0].title}}</v-card-subtitle>
      <v-card-actions>
        <slot :info="info"></slot>
      </v-card-actions>
      <v-expand-transition>
        <div v-show="terra.show">
          <v-divider></v-divider>
          <v-card-subtitle class="white-space-normal mt-1">
            {{ info.componentData.introduction }}
          </v-card-subtitle>
          <v-card-text>
            <v-btn @click.stop="terra.openUrl(item.cid)"
                   class="mr-1 mb-1"
                   v-for="item in info.componentData.episodes">
              {{ item.title }}
            </v-btn>
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
  show: false,

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
      url: `https://terra-historicus.hypergryph.com/comic/${props.info.componentData.cid}/episode/${cid}`,
      source: props.info.dataSource,
      icon: props.info.parent.img,
      width: '428px',
      useragent: "Mozilla/5.0 (iPhone; CPU iPhone OS 12_0 like Mac OS X) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/12.0 Mobile/15E148 Safari/604.1"
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
