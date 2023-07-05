<template>
  <div class="time-line">
    <v-timeline
        align="start"
        side="end"
        truncate-line="start"
    >
      <v-timeline-item
          v-for="cookie in home.timeLineData"
          :key="cookie.item.id"
          :left="true"
          fill-dot="fill-dot"
          dot-color="#fff"
          size="50"
      >
        <template v-slot:icon>
          <v-avatar rounded :image="cookie.icon">
          </v-avatar>
        </template>
        <component :is="component.getComponentName(cookie)" :id="cookie.item.id" :info="cookie" @openUrl="card.openUrlInThis">
          <template #default="info" v-if="card.isCopyImage && cookie.id == card.copyImageId">
            <div class="h-100 w-100 d-flex flex-column">
              <v-divider class="my-2"></v-divider>
              <div class="h-100 w-100 d-flex justify-space-between align-center print px-2">
                <div class="d-flex flex-column">
                  <div class="font-weight-bold title">{{ info.info.dataSource }}</div>
                  <div class="font-weight-light subtitle">{{ new Date(info.info.timestamp.platform).toLocaleString() }}</div>
                </div>
                <div class="d-flex align-center">
                  <img :src="getImage('/assets/image/logo/icon.png')" width="35">
                  <v-divider class="mx-2" vertical></v-divider>
                  <div>
                    <div class="font-weight-bold title">小刻终于吃到饼啦！</div>
                    <div class="font-weight-light subtitle">分享来自小刻食堂</div>
                  </div>
                </div>
              </div>
            </div>
          </template>
          <template #default="info" v-else>
            <span class="font-weight-bold pl-2">{{ new Date(info.info.timestamp.platform).toLocaleString() }}</span>
            <v-spacer></v-spacer>
            <v-btn size="small" icon="fas fa-copy" title="复制链接" @click.stop="card.copy(info.info.item.url)"></v-btn>
            <v-btn size="small" icon="fas fa-share-nodes" title="生成卡片"
                  @click.stop="card.copyImage(cookie.item.id)"></v-btn>
            <v-btn size="small" icon="fas fa-link" title="使用浏览器打开"
                  @click.stop="card.openUrlInBrowser(info.info.item.url)"></v-btn>
          </template>

        </component>
      </v-timeline-item>
    </v-timeline>
  </div>
</template>

<script setup name="timeLine">
import {getCurrentInstance, nextTick, onMounted, reactive} from "vue";
import {sourceInfo} from "@/constant"
import {getImage} from "@/utils/imageUtil"
//import Music from "@/components/Card/music"
import Info from "@/components/Card/common"
//import Terra from "@/components/Card/terra"
import {useRouter} from "vue-router";
import * as htmlToImage from "html-to-image";

const router = useRouter();
const {proxy} = getCurrentInstance();

// 卡片数据
const home = reactive({
  data: [],
  timeLineData: [],
  nextPageId: null,
  async getData() {
    window.newestTimeline.getTimeline((_, arg) => {
      home.timeLineData = Object.values(arg.cookies);
      home.nextPageId = arg.next_page_id;
    })
    /* 
      "cookies": [
            {
                "datasource": "明日方舟官网",
                "icon": "http://cdn-dev.ceobecanteen.top/datasource-avatar/f5f6b090-8def-444b-8f5e-fae1b38cfb8c",
                "timestamp": {
                    "platform": 1687449600000,
                    "platform_precision": "day",
                    "fetcher": 1687530936977
                },
                "default_cookie": {
                    "text": "【明日方舟×中国航天神舟传媒】“宿于繁星”限时活动即将开启",
                    "images": null
                },
                "item": {
                    "id": "news/2023065436",
                    "url": "https://ak.hypergryph.com/news/2023065436.html",
                    "cate": "活动",
                    "is_top": true
                }
            },
            ...
          ]
    */
  }
});

// 卡片操作
const card = reactive({
  isCopyImage: false,// 当前是否在截图
  copyImageId: null,
  openUrlInThis(data) {
    router.push({
      path: '/home/Browser',
      query: data
    })

    // window.operate.openNotificationWindow({
    //   dataSource:home.timeLineData[0].dataSource,
    //   coverImage:home.timeLineData[0].coverImage,
    //   content:home.timeLineData[0].content,
    //   timeForDisplay:home.timeLineData[0].timeForDisplay});
  },
  copyImage(id) {
    card.copyImageId = id;
    card.isCopyImage = true;

    nextTick(() => {
      htmlToImage.toJpeg(document.getElementById(id), {quality: 0.95})
          .then(function (dataUrl) {
            card.isCopyImage = false;
            window.operate.copy({type: 'img', data: dataUrl})
          });
    })
  },
  copy(url) {
    window.operate.copy({type: 'text', data: url})
  },
  openUrlInBrowser(url) {
    window.operate.openUrlInBrowser(url)
  }
})

const component = reactive({
  getComponentName(item) {
    return Info
  }
})
onMounted(() => {
  home.getData();
})
</script>

<style rel="stylesheet/scss" lang="scss">
.time-line {
  height: 100vh;
  overflow: auto;
  min-width: 500px;

  .v-timeline {
    grid-template-columns: 8px min-content 8px;

    .v-timeline-item {
      .v-timeline-item__body {
        -webkit-padding-start: 8px !important;
        padding-inline-start: 8px !important;

        .print {
          .title {
            font-size: 16px;
          }

          .subtitle {
            font-size: 12px;
            opacity: var(--v-medium-emphasis-opacity);
          }
        }

        .v-card-title {
          text-shadow: 0 0 5px black;
        }
      }
    }
  }

}
</style>
