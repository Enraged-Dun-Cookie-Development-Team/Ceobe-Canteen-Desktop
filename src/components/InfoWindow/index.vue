<template>
  <v-card
      class="mx-auto"
      max-width="400"
  >
    <v-img
        v-if="false"
        class="align-end text-white "
        height="200"
        :src="info.coverImage"
        cover
    >
    </v-img>
    <v-card-text>
      {{ info.content }}
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
import {defineProps} from "vue";

const props = defineProps(['item'])

const info = props.item
</script>

<style rel="stylesheet/scss" lang="scss">

</style>