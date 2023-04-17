<template>
  <div class="info-and-tool">
    <!--常用信息-->
<!--    <weekly-quest></weekly-quest>-->
    <!--公告-->
    <announcement ></announcement>
    <!--广告-->
    <!--发源地-->
    <!--快捷工具-->
    <!--推荐视频-->
    <!--Powered By 蓝芷怡 & 洛梧藤 & 云闪-->
  </div>
</template>

<script setup name="infoAndTool">
import {reactive} from "vue";
import {changeToCCT} from "@/utils/timeUtil";
import Announcement from "@/components/InfoAndTool/announcement";
import WeeklyQuest from "@/components/InfoAndTool/weeklyQuest";

</script>

<style rel="stylesheet/scss" lang="scss">
.info-and-tool {
  width: 600px;
  max-width: 600px;
  user-select: none;
}
</style>