<template>
  <div class="tool">
    <v-card class="mt-2 position-relative">
      <v-card-text class="d-flex flex-wrap justify-start">
        <v-btn :class="index>props.notMarginTop?'mt-2':''" class="mr-2 " v-for="(item,index) in props.list">
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
import {reactive, defineProps} from "vue";
import {getImage} from "@/utils/imageUtil"

// notMarginTop是数字 代表从第几个开始就带margin-top 从0开始算
const props = defineProps(['title', 'list','notMarginTop'])
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
}
</style>