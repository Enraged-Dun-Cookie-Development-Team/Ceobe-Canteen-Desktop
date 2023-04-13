<template>
  <div class="music-window">
    <v-card
        theme="dark"
        max-width="400"
        min-width="400"
    >
      <template #image>
        <v-img
            :src="info.coverImage"
            width="130%"
            class="back-image"
            cover
        ></v-img>
      </template>
      <div class="d-flex flex-no-wrap justify-space-between">
        <div>
          <v-card-title :title="info.componentData.name"
                        class="text-h6 text-truncate user-select-none"
                        style="width: 250px">
            {{ info.componentData.name }}
          </v-card-title>

          <v-card-subtitle>共{{ info.componentData.size }}首</v-card-subtitle>

          <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn
                class="ms-2"
                icon="mdi: mdi-play"
                variant="text"
            ></v-btn>
            <v-spacer></v-spacer>
          </v-card-actions>
        </div>
        <v-avatar
            class="ma-3"
            size="125"
            rounded="0"
        >
          <v-img :src="info.coverImage"></v-img>
        </v-avatar>
      </div>
      <v-card-actions>
        <v-span>{{ info.timeForDisplay }}</v-span>
        <v-spacer></v-spacer>
        <v-btn size="small" icon="fas fa-copy"></v-btn>
        <v-btn size="small" icon="fas fa-share-nodes"></v-btn>
        <v-btn size="small" icon="fas fa-link"></v-btn>
      </v-card-actions>
    </v-card>
  </div>

</template>

<script setup>
import {defineProps, onMounted} from "vue";

const props = defineProps(['info'])
console.log(props.info)
</script>

<style rel="stylesheet/scss" lang="scss">
.music-window {
  .back-image {
    filter: blur(10px) brightness(0.5);
  }
}
</style>