<template>
  <div class="info-and-tool">
    <!--常用信息-->
<!--    <weekly-quest></weekly-quest>-->
    <!--公告-->
<!--    <annoukncement ></annoukncement>-->
    <!--广告-->
    <!--发源地-->
    <tool-or-source title="饼的发源地" :list="source.list"></tool-or-source>
    <!--快捷工具-->
    <tool-or-source title="快捷工具" :list="tool.list"></tool-or-source>
    <!--推荐视频-->
    <!--Powered By 蓝芷怡 & 洛梧藤 & 云闪-->
  </div>
</template>

<script setup name="infoAndTool">
import {reactive} from "vue";
import {changeToCCT} from "@/utils/timeUtil";
import Announcement from "@/components/InfoAndTool/announcement";
import WeeklyQuest from "@/components/InfoAndTool/weeklyQuest";
import ToolOrSource from "@/components/InfoAndTool/toolOrSource";
import {sourceInfo,toolInfo} from "@/constant";

const source = reactive({
  list:sourceInfo
})

const tool = reactive({
  list:toolInfo
})

</script>

<style rel="stylesheet/scss" lang="scss">
.info-and-tool {
  width: 600px;
  max-width: 600px;
  user-select: none;
}
</style>