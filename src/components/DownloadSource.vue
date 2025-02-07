<script setup lang="ts">
import { computed, ref } from "vue";

import operate from "@/api/operations/operate";
import {
  DownloadSource,
  primaryToSpare,
  PrimaryUrl,
  SpareUrl,
} from "@/api/resourceFetcher/version";

const props = defineProps<{
  source: DownloadSource;
}>();

const downloadSourceCombine = (primary: PrimaryUrl, spares?: SpareUrl[]) => {
  const urls: SpareUrl[] = [];
  urls.push(primaryToSpare(primary));
  if (spares) urls.push(...spares);
  return urls;
};
const urls = downloadSourceCombine(
  props.source.primary_url,
  props.source.spare_urls,
).map((url) => {
  return url.url;
});
const sourceItem = computed(() =>
  downloadSourceCombine(props.source.primary_url, props.source.spare_urls),
);

const itemProp = (item: SpareUrl) => {
  return {
    title: item.name === "Primary" ? "主链接" : item.name,
    subtitle: `手动下载：${item.manual} | 支持平台：${item.support_platforms ?? [].join(", ")}`,
  };
};

const select = ref(sourceItem.value[0]);
</script>

<template>
  <v-col>
    <p>{{ source.name }}</p>
    <p v-if="source.description">{{ source.description }}</p>
    <v-row class="pa-4">
      <VSelect
        v-model="select"
        density="compact"
        label="下载源"
        :items="sourceItem"
        :item-props="itemProp"
        :hint="`手动下载：${select.manual} | 支持平台：${select.support_platforms ?? [].join(', ')}`"
        single-line
      >
      </VSelect>
      <v-btn
        class="ml-4"
        color="#e6a23c"
        icon="mdi mdi-arrow-right"
        @click="operate.openUrlInBrowser(select.url)"
      />
    </v-row>
  </v-col>
</template>

<style lang="scss" scoped></style>
