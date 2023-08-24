<template>
  <div class="info-and-tool">
    <!--常用信息-->
    <weekly-quest></weekly-quest>
    <!--公告-->
    <announcement></announcement>
    <!--广告-->
    <!--发源地-->
    <tool-or-source title="饼的发源地" :list="source.list"></tool-or-source>
    <!--快捷工具-->
    <tool-or-source title="快捷工具" :list="tool.list"></tool-or-source>
    <!--推荐视频-->
    <video-list></video-list>
    <div class="d-flex justify-center ma-2">
      Powered By &nbsp;<a href="https://www.ceobecanteen.top/#/">
        CeobeCanteen</a
      >
    </div>
  </div>
</template>

<script setup name="infoAndTool" lang="ts">
import { reactive } from "vue";
import { sourceInfo, toolInfo } from "../../constant";
import ToolOrSource from "../../components/InfoAndTool/ToolOrSource.vue";
import VideoList from "../../components/InfoAndTool/VideoList.vue";
import Announcement from "../../components/InfoAndTool/Announcement.vue";
import WeeklyQuest from "../../components/InfoAndTool/WeeklyQuest.vue";

const source = reactive({
  list: sourceInfo,
});

const tool = reactive({
  list: toolInfo,
});
</script>

<style rel="stylesheet/scss" lang="scss">
.info-and-tool {
  overflow: auto;
  padding: 10px 16px 0;
  min-width: 600px;
  user-select: none;
}
a {
  text-decoration: none;
}
</style>
