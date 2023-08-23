<template>
  <div class="info-and-tool">
    <!--常用信息-->
    <weekly-quest></weekly-quest>
    <!--公告-->
    <announcement></announcement>
    <!--广告-->
    <!--快捷工具-->
    <tool-or-source title="快捷工具" :list="tool.list"></tool-or-source>
    <!--发源地-->
    <tool-or-source title="饼的发源地" :imgLocal="false" :list="source.list"></tool-or-source>
    <!--推荐视频-->
    <video-list></video-list>
    <div class="d-flex justify-center ma-2">
      Powered By &nbsp;<v-btn class='pl-0' variant="plain" density="compact" @click="openUrlInBrowser('https://www.ceobecanteen.top/#/')">CeobeCanteen</v-btn>
    </div>
  </div>
</template>

<script setup name="infoAndTool">
import { onMounted, reactive } from 'vue';
import { changeToCCT } from '@/utils/timeUtil';
import Announcement from '@/components/InfoAndTool/announcement';
import WeeklyQuest from '@/components/InfoAndTool/weeklyQuest';
import ToolOrSource from '@/components/InfoAndTool/toolOrSource';
import { sourceInfo, toolInfo } from '@/constant';
import VideoList from '@/components/InfoAndTool/videoList';
import { getResourceList } from '@/api/list';

const source = reactive({
  list: [],
  getSource() {
    getResourceList().then((res) => {
      let sourceList = res.data.data;
      source.list = sourceList.map((datasource) => {
        return {
          url: datasource.jump_url,
          name: datasource.nickname,
          img: datasource.avatar
        }
      })
    })
  }
});

const tool = reactive({
  list: toolInfo,
});

const openUrlInBrowser = (url) => {
  window.operate.openUrlInBrowser(url);
}

onMounted(() => {
  source.getSource();
})
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
