<template>
  <div class="home d-flex">
    <time-line class="pt-2"></time-line>
    <info-and-tool class="pt-2"></info-and-tool>
  </div>
</template>

<script setup name="home">

import TimeLine from "@/view/home/timeLine";
import InfoAndTool from "@/view/home/infoAndTool";
</script>

<style rel="stylesheet/scss" lang="scss">
.home {
  min-width:1200px;
  overflow: hidden;
}
</style>