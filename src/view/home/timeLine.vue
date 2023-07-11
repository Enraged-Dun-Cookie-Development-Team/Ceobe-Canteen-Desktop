<template>
  <div class="time-line">
    <div class="fix-btn">
      <v-btn
        class="refresh-btn"
        prepend-icon="fas fa-arrows-rotate"
        color="#ffba4b"
        elevation="0"
        :ripple="false"
        :class="scroll.scrollShow && timeline.refreshTimelineData.length !== 0 ? 'refresh-btn-show' : ''"
        density="default"
        rounded="xl"
        @click.stop="timeline.refreshTimeline"
      >
        <template v-slot:prepend>
          <v-icon color="#fff"></v-icon>
        </template>
        <span style="color: #fff">有新饼</span>
      </v-btn>
      <v-btn
        class="top-btn"
        size="small"
        icon="fas fa-arrow-up"
        color="#ffba4b"
        elevation="0"
        style="color: #fff"
        density="default"
        :class="scroll.scrollShow ? 'top-btn-show' : ''"
        @click.stop="scroll.scrollToTop"
      ></v-btn>
    </div>
    <v-timeline ref="timeline_area" align="start" side="end" truncate-line="start">
      <v-timeline-item
        v-for="cookie in timeline.timeLineData"
        :key="cookie.item.id"
        :left="true"
        fill-dot="fill-dot"
        dot-color="#fff"
        size="50"
      >
        <template v-slot:icon>
          <v-avatar rounded :image="cookie.icon"></v-avatar>
        </template>
        <component
          :is="component.getComponentName(cookie)"
          :id="cookie.item.id"
          :info="cookie"
          @openUrl="card.openUrlInThis"
        >
          <template #default="info" v-if="card.isCopyImage && cookie.item.id == card.copyImageId">
            <div class="h-100 w-100 d-flex flex-column">
              <v-divider class="my-2"></v-divider>
              <div class="h-100 w-100 d-flex justify-space-between align-center print px-2">
                <div class="d-flex flex-column">
                  <div class="font-weight-bold title">
                    {{ info.info.datasource }}
                  </div>
                  <div class="font-weight-light subtitle">
                    {{ new Date(info.info.timestamp.platform).toLocaleString() }}
                  </div>
                </div>
                <div class="d-flex align-center">
                  <img :src="getImage('/assets/image/logo/icon.png')" width="35" />
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
            <span class="font-weight-bold pl-2">{{ new Date(cookie.timestamp.platform).toLocaleString() }}</span>
            <v-spacer></v-spacer>
            <v-btn size="small" icon="fas fa-copy" title="复制链接" @click.stop="card.copy(cookie.item.url)"></v-btn>
            <v-btn
              size="small"
              icon="fas fa-share-nodes"
              title="生成卡片"
              @click.stop="card.copyImage(cookie.item.id)"
            ></v-btn>
            <v-btn
              size="small"
              icon="fas fa-link"
              title="使用浏览器打开"
              @click.stop="card.openUrlInBrowser(cookie.item.url)"
            ></v-btn>
          </template>
        </component>
      </v-timeline-item>
    </v-timeline>
  </div>
</template>

<script setup name="timeLine">
import { getCurrentInstance, nextTick, onMounted, reactive, ref } from 'vue';
import { sourceInfo } from '@/constant';
import { getImage } from '@/utils/imageUtil';
//import Music from "@/components/Card/music"
import Info from '@/components/Card/common';
//import Terra from "@/components/Card/terra"
import { useRouter } from 'vue-router';
import * as htmlToImage from 'html-to-image';
import { getCookieList } from '@/api/list';

const router = useRouter();
const { proxy } = getCurrentInstance();

// 卡片数据
const timeline = reactive({
  timeLineData: [],
  refreshTimelineData: [],
  nextPageId: null,
  refreshNextPageId: null,
  combId: null,
  refreshCombId: null,
  updateCookieId: null,
  refreshUpdateCookieId: null,
  async getData() {
    window.newestTimeline.getTimeline((_, arg) => {
      if (timeline.timeLineData.length === 0 || !scroll.scrollShow) {
        timeline.timeLineData = arg.cookies;
        timeline.combId = arg.comb_id;
        timeline.updateCookieId = arg.update_cookie_id;
        timeline.nextPageId = arg.next_page_id;
        document.querySelector('.time-line').scrollTop = 0;
      } else {
        timeline.refreshTimelineData = arg.cookies;
        timeline.refreshCombId = arg.comb_id;
        timeline.refreshUpdateCookieId = arg.update_cookie_id;
        timeline.refreshNextPageId = arg.next_page_id;
      }
    });
  },
  refreshTimeline() {
    if (timeline.refreshTimelineData.length == 0) {
      return;
    }
    timeline.timeLineData = timeline.refreshTimelineData.slice(0);
    timeline.refreshTimelineData = [];
    timeline.combId = timeline.refreshCombId;
    timeline.refreshCombId = null;
    timeline.updateCookieId = timeline.refreshUpdateCookieId;
    timeline.refreshUpdateCookieId = null;
    timeline.nextPageId = timeline.refreshNextPageId;
    timeline.refreshNextPageId = null;
    document.querySelector('.time-line').scrollTop = 0;
  }
});

// 卡片操作
const card = reactive({
  isCopyImage: false, // 当前是否在截图
  copyImageId: null,
  openUrlInThis(data) {
    router.push({
      path: '/home/Browser',
      query: data
    });
  },
  copyImage(id) {
    card.copyImageId = id;
    card.isCopyImage = true;
    setTimeout(() => {
      nextTick(() => {
        htmlToImage.toJpeg(document.getElementById(id), { quality: 0.95 }).then(function (dataUrl) {
          card.isCopyImage = false;
          window.operate.copy({ type: 'img', data: dataUrl });
        });
      });
    }, 500);
  },
  copy(url) {
    // show.value = true;
    window.operate.copy({ type: 'text', data: url });
  },
  openUrlInBrowser(url) {
    debugger
    window.operate.openUrlInBrowser(url);
  }
});

// 滚动操作
const scroll = reactive({
  scrollShow: false,

  bindHandleScroll(e) {
    scroll.scrollShow = e.target.scrollTop > 600 ? true : false;

    if (e.target.scrollTop + e.target.clientHeight == e.target.scrollHeight) {
      if (!timeline.nextPageId) {
        return;
      }
      getCookieList(timeline.combId, timeline.nextPageId, timeline.updateCookieId)
        .then(resp => {
          let cookies_info = resp.data.data;
          timeline.timeLineData.push(...cookies_info.cookies);
          timeline.nextPageId = cookies_info.next_page_id;
        })
        .catch(() => {
          // TODO：弹窗处理一下
        });
    }
  },
  scrollToTop() {
    let top = document.querySelector('.time-line').scrollTop;
    let changeTop = top / 10;
    const timeTop = setInterval(() => {
      document.querySelector('.time-line').scrollTop = top -= changeTop;
      if (top <= 0) {
        clearInterval(timeTop);
      }
    }, 10);
  }
});

// 简单的节流函数
const throttle = (fn, t) => {
  let last;
  let timer;
  let interval = t || 500;
  return function () {
    let args = arguments;
    let now = +new Date();
    if (last && now - last < interval) {
      clearTimeout(timer);
      timer = setTimeout(() => {
        last = now;
        fn.apply(this, args);
      }, interval);
    } else {
      last = now;
      fn.apply(this, args);
    }
  };
};

const component = reactive({
  getComponentName(item) {
    return Info;
  }
});
onMounted(() => {
  timeline.getData();
  window.addEventListener('scroll', throttle(scroll.bindHandleScroll, 500), true);
});
onMounted(() => {
    window.newestTimeline.getTimeline((_, args) => {
        // 接收到的是详细的饼列表
        console.log(args);
        const cookies = args.cookies
        console.log(cookies);
        let cookies_len = args.length;
        let count = 0;
        window.operate.openNotificationWindow(cookies[0]);
        // cookies.forEach(data => {
        //     count++;
        //     window.operate.openNotificationWindow(data);
        //     if (count !== cookies_len) {
        //         setTimeout(() => {
        //             window.notification.closeWindow();
        //         }, 10000)
        //     }
        // });
    })
})
</script>

<style rel="stylesheet/scss" lang="scss">
.time-line {
  height: 100vh;
  overflow: auto;
  min-width: 500px;

  .fix-btn {
    .refresh-btn {
      position: fixed;
      left: 300px;
      bottom: -40px;
      z-index: 1;
      transition: 0.3s all;
      opacity: 1;
      &.refresh-btn-show {
        bottom: 20px;
        opacity: 0.4;

        &:hover {
          opacity: 1;
        }
      }
    }
    .top-btn {
      position: fixed;
      left: 410px;
      bottom: -40px;
      z-index: 1;
      transition: 0.3s all;
      opacity: 1;
      &.top-btn-show {
        bottom: 18px;
        opacity: 0.4;

        &:hover {
          opacity: 1;
        }
      }
    }
  }

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
