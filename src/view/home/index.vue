<template>
  <div class="home d-flex">
    <time-line class="pt-2" @openUrl="handle.openUrl"></time-line>
    <router-view v-slot="{Component,route}">
      <transition name="fade-transform">
        <component v-if="!route.meta.link" :is="Component" :key="route.path"/>
      </transition>
    </router-view>
  </div>
</template>

<script setup name="home">
import TimeLine from "@/view/home/timeLine";
import {reactive, ref} from "vue";

const handle = reactive({
  openUrl(data){

  }
})
</script>

<style rel="stylesheet/scss" lang="scss">
.home {
  min-width:1200px;
  overflow: hidden;
}
</style>