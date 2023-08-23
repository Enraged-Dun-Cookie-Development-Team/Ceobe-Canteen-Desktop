<template>
  <div class="time-line">
    <div class="fix-btn">
      <v-btn
        class="refresh-btn"
        prepend-icon="fas fa-arrows-rotate"
        color="#ffba4b"
        elevation="0"
        :ripple="false"
        :class="
          scroll.scrollShow && timeline.refreshTimelineData && timeline.refreshTimelineData?.length !== 0
            ? 'refresh-btn-show'
            : ''
        "
        density="default"
        rounded="xl"
        @click.stop="timeline.refreshTimeline"
      >
        <template #prepend>
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
    <div v-if="!timeline.timelineData" class="loading-image">
      <img :src="getImage('/assets/image/load/list-load.gif')"  alt=""/>
    </div>
    <v-timeline v-else ref="timeline_area" align="start" side="end" truncate-line="start">
      <v-timeline-item
        v-for="cookie in timeline.timelineData"
        :key="cookie.item.id"
        :left="true"
        fill-dot="fill-dot"
        dot-color="#fff"
        size="50"
      >
        <template #icon>
          <v-avatar rounded :image="cookie.icon"></v-avatar>
        </template>
        <component
          :is="component.getComponentName(cookie)"
          :id="cookie.item.id"
          :info="cookie"
          @open-url="card.openUrlInThis"
        >
          <template v-if="card.isCopyImage && cookie.item.id == card.copyImageId" #default="info">
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
          <template v-else #default="info">
            <span class="font-weight-bold pl-2">{{
              new Date(info.info.timestamp?.platform ?? info.info.timestamp?.fetcher ?? '').toLocaleString()
            }}</span>
            <v-spacer></v-spacer>
            <v-btn
              size="small"
              icon="fas fa-copy"
              title="复制链接"
              @click.stop="card.copy(info.info.item.url ?? '')"
            ></v-btn>
            <v-btn
              size="small"
              icon="fas fa-share-nodes"
              title="生成卡片"
              @click.stop="card.copyImage(info.info.item.id)"
            ></v-btn>
            <v-btn
              size="small"
              icon="fas fa-link"
              title="使用浏览器打开"
              @click.stop="card.openUrlInBrowser(info.info.item.url)"
            ></v-btn>
          </template>
        </component>
      </v-timeline-item>
    </v-timeline>
    <div v-if="timeline.nextPageId" class="loading-more">这是精美的加载动画</div>
    <div v-else class="loading-more">没有更多饼了，小刻很满足！！！</div>
  </div>
</template>

<script setup name="timeLine">
import { getCurrentInstance, nextTick, onMounted, reactive } from "vue";
import { getImage } from "@/utils/imageUtil.ts";
import Info from "@/components/Card/Common.vue";
import { useRouter } from "vue-router";
import * as htmlToImage from "html-to-image";
import { getCookieList, getCookieSearchList } from "@/api/list.ts";

const router = useRouter();
getCurrentInstance();

// 卡片数据
const timeline = reactive({
    timelineData: null, // 饼列表
    refreshTimelineData: null, // 刷新临时存的饼列表
    tempTimelineData: null, // 用于搜索状态，临时存的饼列表
    nextPageId: null, // 下一页id
    refreshNextPageId: null, // 刷新临时存的下一页id
    tempNextPageId: null, // 用于搜索状态，临时存的下一页id
    combId: null, // 数据源组合id
    refreshCombId: null, // 刷新临时存的数据源组合id
    updateCookieId: null, // 更新饼id
    refreshUpdateCookieId: null, // 刷新临时存的更新饼id
    searchStatus: false, // 搜索状态，列表展示搜索内容就为true
    searchWord: null,
    async getData() {
        window.newestTimeline.getTimeline((_, arg) => {
            if (arg == null) {
                return;
            }
            if (timeline.searchStatus) {
                // 搜索状态直接更新临时
                timeline.tempTimelineData = arg.cookies;
                timeline.combId = arg.comb_id;
                timeline.updateCookieId = arg.update_cookie_id;
                timeline.tempNextPageId = arg.next_page_id;
            } else if (!timeline.timelineData || !scroll.scrollShow) {
                timeline.timelineData = arg.cookies;
                timeline.combId = arg.comb_id;
                timeline.updateCookieId = arg.update_cookie_id;
                timeline.nextPageId = arg.next_page_id;
                document.querySelector(".time-line").scrollTop = 0;
            } else {
                timeline.refreshTimelineData = arg.cookies;
                timeline.refreshCombId = arg.comb_id;
                timeline.refreshUpdateCookieId = arg.update_cookie_id;
                timeline.refreshNextPageId = arg.next_page_id;
            }
        });
    },
    refreshTimeline() {
        if (!timeline.refreshTimelineData || timeline.refreshTimelineData.length == 0) {
            return;
        }
        timeline.timelineData = timeline.refreshTimelineData.slice(0);
        timeline.refreshTimelineData = null;
        timeline.combId = timeline.refreshCombId;
        timeline.refreshCombId = null;
        timeline.updateCookieId = timeline.refreshUpdateCookieId;
        timeline.refreshUpdateCookieId = null;
        timeline.nextPageId = timeline.refreshNextPageId;
        timeline.refreshNextPageId = null;
        document.querySelector(".time-line").scrollTop = 0;
    },
    searchTimeline() {
        window.searchWordEvent.getSearchWord((_, searchWord) => {
            timeline.searchWord = searchWord;
            if (searchWord !== "" && searchWord !== null) {
                // 先确保数据更新到显示
                timeline.refreshTimeline();
                // 如果之前不是在搜索状态，才转移列表
                if (!timeline.searchStatus) {
                    // 把列表存到临时列表
                    timeline.tempTimelineData = timeline.timelineData?.slice(0);
                    timeline.tempNextPageId = timeline.nextPageId;
                    timeline.timelineData = null;
                    timeline.nextPageId = null;
                    timeline.searchStatus = true;
                    document.querySelector(".time-line").scrollTop = 0;
                }
                getCookieSearchList(null, timeline.combId, searchWord).then((data) => {
                    if (data.status == 200) {
                        let respData = data.data.data;
                        timeline.timelineData = respData.cookies;
                        timeline.nextPageId = respData.next_page_id;
                    }
                });
            } else {
                // 如果之前是在搜索状态，才需要回归
                if (timeline.searchStatus) {
                    // 回归普通列表
                    timeline.searchStatus = false;
                    timeline.timelineData = timeline.tempTimelineData?.slice(0);
                    timeline.nextPageId = timeline.tempNextPageId;
                    document.querySelector(".time-line").scrollTop = 0;
                }
            }
        });
    },
});

// 卡片操作
const card = reactive({
    isCopyImage: false, // 当前是否在截图
    copyImageId: null,
    openUrlInThis(data) {
        router.push({
            path: "/home/Browser",
            query: data,
        });
    },
    copyImage(id) {
        card.copyImageId = id;
        card.isCopyImage = true;
        setTimeout(() => {
            nextTick(() => {
                htmlToImage.toJpeg(document.getElementById(id), { quality: 0.95 }).then(function (dataUrl) {
                    card.isCopyImage = false;
                    window.operate.copy({ type: "img", data: dataUrl });
                });
            });
        }, 500);
    },
    copy(url) {
    // show.value = true;
        window.operate.copy({ type: "text", data: url });
    },
    openUrlInBrowser(url) {
        window.operate.openUrlInBrowser(url);
    },
});

// 滚动操作
const scroll = reactive({
    scrollShow: false,

    bindHandleScroll(e) {
        if (document.querySelector(".time-line") !== e.target) {
            return;
        }
        scroll.scrollShow = e.target.scrollTop > 600 ? true : false;

        // 因为有些情况会导致高度不能正好相等，给个差值小于5来扩大判断范围
        if (Math.abs(e.target.scrollTop + e.target.clientHeight - e.target.scrollHeight) < 5) {
            if (!timeline.nextPageId) {
                return;
            }
            if (timeline.searchStatus) {
                getCookieSearchList(timeline.nextPageId, timeline.combId, timeline.searchWord).then((data) => {
                    if (data.status == 200) {
                        let respData = data.data.data;
                        timeline.timelineData.push(...respData.cookies);
                        timeline.nextPageId = respData.next_page_id;
                    }
                });
            } else {
                getCookieList(timeline.combId, timeline.nextPageId, timeline.updateCookieId)
                    .then((resp) => {
                        let cookies_info = resp.data.data;
                        timeline.timelineData.push(...cookies_info.cookies);
                        timeline.nextPageId = cookies_info.next_page_id;
                    })
                    .catch(() => {
                        // TODO：弹窗处理一下
                    });
            }
        }
    },
    scrollToTop() {
        let top = document.querySelector(".time-line").scrollTop;
        let changeTop = top / 10;
        const timeTop = setInterval(() => {
            document.querySelector(".time-line").scrollTop = top -= changeTop;
            if (top <= 0) {
                clearInterval(timeTop);
            }
        }, 10);
    },
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
    getComponentName() {
        return Info;
    },
});
onMounted(() => {
    timeline.getData();
    // 如果没有数据，让后台发一份过来
    if (!timeline.timelineData) {
        window.newestTimeline.needTimeline();
    }
    window.addEventListener("scroll", throttle(scroll.bindHandleScroll, 500), true);
    timeline.searchTimeline();
});
</script>

<style rel="stylesheet/scss" lang="scss">
.time-line {
  overflow: auto;
  width: 500px;
  min-width: 500px;

  .fix-btn {
    .refresh-btn {
      position: fixed;
      bottom: -40px;
      left: 300px;
      z-index: 1;
      opacity: 1;
      transition: 0.3s all;
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
      bottom: -40px;
      left: 410px;
      z-index: 1;
      opacity: 1;
      transition: 0.3s all;
      &.top-btn-show {
        bottom: 18px;
        opacity: 0.4;

        &:hover {
          opacity: 1;
        }
      }
    }
  }

  .loading-image {
    height: calc(100vh - 70px);
    line-height: calc(100vh - 70px);
    img {
      width: 100%;
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
  .v-timeline--vertical.v-timeline {
    height: auto;
  }
  .loading-more {
    margin-top: 20px;
    margin-bottom: 20px;
    width: 100%;
    text-align: center;
  }
}
</style>
