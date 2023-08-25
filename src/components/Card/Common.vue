<template>
  <div class="common-window">
    <v-card class="mx-auto cursor-pointer" width="400" @click="common.openUrl">
      <v-img
        v-if="info.default_cookie.images"
        referrerpolicy="no-referrer"
        class="align-end text-white header-image"
        :src="common.getImg()"
        max-height="320"
        cover
      ></v-img>

      <v-card-text>
        <div class="cookie-text">{{ info.default_cookie.text }}</div>
        <div v-if="info.item.retweeted" class="retweet-area">
          <div>转发自：{{ info.item.retweeted.author_name }}</div>
          <div class="retweet-text">{{ info.item.retweeted.text }}</div>
        </div>
      </v-card-text>

      <v-card-actions>
        <slot :info="info"></slot>
      </v-card-actions>
    </v-card>
  </div>
</template>

<script setup lang="ts">
import { onMounted, reactive } from "vue";
import ceobeRequest from "@/api/operations/ceobeRequest";

const props = defineProps({
  info: {
    type: Object,
    default: () => {},
  },
});
const emits = defineEmits({
  openUrl: null,
});

const common = reactive({
  imgUrl: [],
  getImg() {
    if (!props.info.default_cookie.images) {
      common.imgUrl = [];
    } else if (props.info.datasource.includes("微博")) {
      ceobeRequest
        .getHasRefererImageBase64(
          props.info.default_cookie.images[0].origin_url,
        )
        .then((res) => {
          common.imgUrl = "data:image/jpeg;base64," + res;
        });
    } else {
      common.imgUrl = props.info.default_cookie.images[0].origin_url;
    }
    return common.imgUrl;
  },
  openImage() {},
  openUrl() {
    // 统一格式 只需要标题和url和icon
    let data = {
      url: props.info.item.url,
      source: props.info.datasource,
      icon: props.info.icon,
    };
    console.log(data);
    emits("openUrl", data);
  },
});

onMounted(() => {
  if (props.info.default_cookie.images) {
    common.getImg(props.info.default_cookie.images);
  }
});
</script>

<style rel="stylesheet/scss" lang="scss">
.common-window {
  .header-image {
    img {
      object-position: top;
    }
  }
  .cookie-text {
    /* 盒子模型 */
    display: -webkit-box;
    /* 超出范围隐藏 */
    overflow: hidden;
    /* 文字超出用省略号 */
    text-overflow: ellipsis;
    white-space: pre-wrap;
    /* 显示的文本行数 */
    -webkit-line-clamp: 10;
    /* 子元素的垂直排列方式 */
    -webkit-box-orient: vertical;
  }
  .retweet-area {
    padding: 10px;
    background-color: #e7e7e7;
    .retweet-text {
      /* 盒子模型 */
      display: -webkit-box;
      /* 超出范围隐藏 */
      overflow: hidden;
      /* 文字超出用省略号 */
      text-overflow: ellipsis;
      white-space: pre-wrap;
      /* 显示的文本行数 */
      -webkit-line-clamp: 7;
      /* 子元素的垂直排列方式 */
      -webkit-box-orient: vertical;
    }
  }
}
</style>
