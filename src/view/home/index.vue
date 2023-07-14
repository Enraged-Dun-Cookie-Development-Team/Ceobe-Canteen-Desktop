<template>
  <div class="home d-flex">
    <time-line class="pt-2"></time-line>
    <keep-alive>
      <transition name="fade" mode="out-in">
        <router-view/>
      </transition>
    </keep-alive>
  </div>
</template>

<script setup name="home">
import TimeLine from "@/view/home/timeLine";

</script>

<style rel="stylesheet/scss" lang="scss">
.home {
  min-width: 100vw;
  overflow: hidden;

  .fade-enter-active,
  .fade-leave-active {
    transition: opacity 0.5s;
  }

  .fade-enter,
  .fade-leave-to {
    opacity: 0;
  }
}
</style>