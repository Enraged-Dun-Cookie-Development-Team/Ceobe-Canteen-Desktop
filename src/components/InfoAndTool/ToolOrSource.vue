<template>
  <div class="tool">
    <v-card class="mt-2 position-relative">
      <v-card-text v-if="listLoaded && toolsLinkList.length >0" class="d-flex flex-wrap justify-start">
        <v-btn
            v-for="(item, index) in toolsLinkList"
            :key="index"
            class="mr-2 mb-2"
            @click.stop="openUrl(item.jump_url)"
        >
          <template #prepend>
            <v-img width="20" class="btn-img" :src="getImage(item.avatar)"></v-img>
          </template>
          {{ item.nickname }}
        </v-btn>
      </v-card-text>
      <v-card-text v-else>

        <v-progress-linear indeterminate color="#e6a23c"></v-progress-linear>
      </v-card-text>
      <span class="card-title position-absolute">{{ props.title }}</span>
    </v-card>
  </div>
</template>

<script setup name="tool" lang="ts">
import {getImage} from "@/utils/imageUtil";
import operate from "@/api/operations/operate";
import {onMounted, ref} from "vue";
import {Datum} from "@/api/resourceFetcher/toolkits.js";

const props = defineProps<{
  title: string; getList: ()=> Promise<Datum[]>
}>();
const listLoaded = ref(false)
const toolsLinkList = ref<Datum[]>([])

onMounted(() => {
  props.getList().then((list)=>{
    toolsLinkList.value = list
    listLoaded.value = true
  })
})

function openUrl(url: string) {
  operate.openUrlInBrowser(url);
}
</script>

<style rel="stylesheet/scss" lang="scss">
.tool {
  .btn-img {
    border-radius: 50%;
  }

  .card-title {
    right: 10px;
    bottom: -20px;
    display: none;
    font-size: 36px;
    color: #000;
    opacity: 0.1;
  }

  .v-card-text {
    padding-bottom: calc(1rem - 8px);
  }
}
</style>
