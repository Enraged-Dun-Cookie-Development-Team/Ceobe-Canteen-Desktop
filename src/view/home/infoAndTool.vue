<template>
  <div class="info-and-tool">
    <!--常用信息-->
    <weekly-quest></weekly-quest>
    <!--公告-->
    <announcement v-model="announcementInfo.data"></announcement>
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

const announcementInfo = reactive({
  data: [],
  getData() {
    window.ceobeRequest.getAnnouncementInfo().then(res => {
      announcementInfo.data = res.data.filter(
          (x) =>
              new Date(x.start_time) <= changeToCCT(new Date()) &&
              new Date(x.over_time) >= changeToCCT(new Date())
      );
    })
  }
})
</script>

<style rel="stylesheet/scss" lang="scss">
.info-and-tool {
  width: 500px;
  max-width: 500px;
}
</style>