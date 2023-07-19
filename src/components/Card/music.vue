<template>
  <div class="music-window">
    <v-card theme="dark" width="400">
      <template #image>
        <v-img :src="info.coverImage" width="130%" class="back-image" cover></v-img>
      </template>
      <div class="d-flex flex-no-wrap justify-space-between">
        <div>
          <v-card-title
            :title="info.componentData.name"
            class="text-h6 text-truncate user-select-none"
            style="width: 250px"
          >
            {{ info.componentData.name }}
          </v-card-title>

          <v-card-subtitle>共{{ info.componentData.size }}首</v-card-subtitle>

          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn
              class="ms-2"
              :icon="!showPlayer ? 'mdi: mdi-play' : 'mdi: mdi-stop'"
              variant="text"
              @click.stop="
                showPlayer = !showPlayer;
                clickIndex++;
              "
            ></v-btn>
            <v-spacer></v-spacer>
          </v-card-actions>
        </div>
        <v-avatar class="ma-3" size="125" rounded="0">
          <v-img :src="info.coverImage"></v-img>
        </v-avatar>
      </div>
      <v-card-actions>
        <slot :info="info"></slot>
      </v-card-actions>
      <v-expand-transition>
        <div v-show="showPlayer">
          <v-divider></v-divider>
          <div class="d-flex justify-center ma-1">此插件来源于网易云音乐</div>
          <webview
            v-if="showPlayer"
            :style="{ height: 150 + info.componentData.size * 30 + 'px' }"
            :id="`webview-${id}`"
            :src="`http://music.163.com/outchain/player?type=1&id=${id}&auto=0`"
          ></webview>
          <div v-else :style="{ height: 150 + info.componentData.size * 30 + 'px' }"></div>
        </div>
      </v-expand-transition>
    </v-card>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue';

const props = defineProps(['info']);
const emits = defineEmits();
const id = props.info.jumpUrl.split('=')[1];
const showPlayer = ref(false);

onMounted(() => {});
</script>

<style rel="stylesheet/scss" lang="scss">
.music-window {
  .back-image {
    filter: blur(10px) brightness(0.5);
  }
}
</style>
