<template>
  <div class="tool">
    <v-card class="mt-2 position-relative">
      <v-card-text class="d-flex flex-wrap justify-start">
        <v-btn  class="mr-2 mb-2" v-for="(item,index) in props.list">
          <template #prepend>
            <v-img width="20" class="btn-img" :src="getImage(item.img)"></v-img>
          </template>
          {{ item.name }}
        </v-btn>
      </v-card-text>
      <span class="card-title position-absolute">{{props.title}}</span>
    </v-card>
  </div>
</template>

<script setup name="tool">
import {getImage} from "@/utils/imageUtil"

const props = defineProps(['title', 'list'])
const emits = defineEmits();

function clickBtn(data) {
  emits('click', data)
}
</script>

<style rel="stylesheet/scss" lang="scss">
.tool {
  .btn-img {
    border-radius: 50%
  }
  .card-title{
    display: none;
    bottom: -20px;
    right: 10px;
    font-size: 36px;
    color: #000;
    opacity: 0.1;
  }
  .v-card-text{
    padding-bottom: calc(1rem - 8px);
  }
}
</style>