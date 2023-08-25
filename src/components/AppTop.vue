<template>
  <div
    class="app-top d-flex justify-space-between align-center w-100 pa-2"
    data-tauri-drag-region
  >
    <div class="h-100 d-flex align-center">
      <v-img src="icon.png" width="40"></v-img>
      <span class="title ml-2 mt-1">小刻食堂</span>
    </div>

    <div class="h-100 no-drag">
      <v-menu
        v-model="search.show"
        :close-on-content-click="false"
        location="bottom"
        transition="slide-y-transition"
      >
        <template #activator="{ props }">
          <v-btn
            icon="fas fa-magnifying-glass"
            v-bind="props"
            variant="text"
          ></v-btn>
        </template>
        <v-card class="mx-auto" color="grey-lighten-3" min-width="400">
          <v-text-field
            v-model="search.searchWord"
            append-inner-icon="fa fa-magnifying-glass"
            clearable
            density="compact"
            label="查找饼仓"
            variant="solo"
            autofocus
            hide-details
            @click:append-inner="search.searching"
            @update:model-value="search.searchChange"
          ></v-text-field>
        </v-card>
      </v-menu>
      <span v-if="search.wordShow !== ''">{{ search.wordShow }}</span>
      <v-menu
        v-model="menuShow.show"
        :close-on-content-click="false"
        :on-update:model-value="menuShow.changeDatasourceOpen"
        location="bottom"
        transition="slide-y-transition"
      >
        <template #activator="{ props }">
          <v-btn icon="fas fa-database" v-bind="props" variant="text"></v-btn>
        </template>
        <v-list density="compact">
          <v-list-item
            v-for="(item, i) in menuShow.datasourceList"
            :key="i"
            :prepend-avatar="item.avatar"
            :title="item.nickname"
            :value="item"
            class="menuShow-item"
            color="primary"
            @click="menuShow.changeSelectSource(item)"
          >
          </v-list-item>
        </v-list>
      </v-menu>
    </div>
    <!-- right control panel -->
    <div class="h-100 no-drag">
      <v-btn
        variant="text"
        icon="fas fa-comments-dollar"
        @click="donate.show = true"
      ></v-btn>
      <v-btn
        variant="text"
        icon="fas fa-gear"
        @click="setting.show = true"
      ></v-btn>
      <v-btn
        variant="text"
        icon="fas fa-minus"
        @click="handleWindow.minus"
      ></v-btn>
      <v-btn
        variant="text"
        :icon="winMax ? 'fas fa-minimize' : 'fas fa-maximize'"
        @click="handleWindow.maximize"
      ></v-btn>
      <v-btn
        color="error"
        icon="fas fa-circle-xmark"
        variant="text"
        @click="handleWindow.close"
      ></v-btn>
    </div>
  </div>

  <v-dialog
    v-model="donate.show"
    width="800"
    persistent
    transition="dialog-top-transition"
  >
    <donate-page @close="donate.show = false"></donate-page>
  </v-dialog>

  <v-dialog
    v-model="setting.show"
    width="400"
    persistent
    transition="dialog-top-transition"
  >
    <setting-page @close="setting.show = false"></setting-page>
  </v-dialog>
</template>

<script lang="ts" name="index" setup>
import { reactive, ref } from "vue";
import storage from "../api/operations/localStorage";
import {
  DatasourceItem,
  getConfigDatasourceList,
} from "../api/resourceFetcher/datasourceList";
import { getDatasourceComb } from "../api/resourceFetcher/datasourceCombine";
import datasourceConfigOperate from "../api/operations/datasourceConfig";
import searchWordEvent from "../api/operations/searchWordEvent";
import operate from "../api/operations/operate";
import DonatePage from "./DonatePage.vue";
import SettingPage from "./SettingPage.vue";

const winMax = ref(false);

const menuShow = reactive<{
  notOpened: boolean;
  show: boolean;
  datasourceList: DatasourceItem[];
  changeSelectSource: (DatasourceItem) => void;
  changeDatasourceOpen: (DatasourceItem) => void;
}>({
  notOpened: true, // 没有打开过
  show: false,
  datasourceList: [],
  changeSelectSource(data) {
    data.check = !data.check;
  },
  async changeDatasourceOpen(value) {
    if (value) {
      if (menuShow.notOpened) {
        menuShow.notOpened = false;
      }
      let datasourceConfig = JSON.parse(
        <string>storage.getItem("datasource-config"),
      );
      // 打开列表
      getConfigDatasourceList().then((data) => {
        if (data.status == 200) {
          console.log(data);
          menuShow.datasourceList = data.data.data;
          if (datasourceConfig) {
            let datasourceConfigUuidMap = {};
            for (let datasource of datasourceConfig) {
              datasourceConfigUuidMap[datasource] = true;
            }
            menuShow.datasourceList.forEach((element) => {
              if (datasourceConfigUuidMap[element.unique_id]) {
                element.check = true;
              }
            });
          } else {
            menuShow.datasourceList.forEach((element) => {
              element.check = true;
            });
          }
        }
      });
    } else {
      if (menuShow.notOpened) {
        return;
      }
      // 关闭列表
      let datasourceConfig = menuShow.datasourceList
        .filter((element) => {
          return element.check;
        })
        .map((element) => {
          return element.unique_id;
        });
      let comb_resp = await getDatasourceComb(datasourceConfig);
      let comb_id = comb_resp.data.data.datasource_comb_id;
      let datasourceComb = storage.getItem("datasource-comb");
      // 如果组合id和之前一样，不进行刷新
      if (datasourceComb == comb_id) {
        return;
      }
      storage.setItem("datasource-config", JSON.stringify(datasourceConfig));
      storage.setItem("datasource-comb", comb_id);
      await datasourceConfigOperate.updateDatasourceComb();
    }
  },
});
const search = reactive({
  show: false,
  wordShow: "",
  searchWord: null,
  searching() {
    if (search.searchWord) {
      // 将搜索词发送给timeline进行处理
      searchWordEvent.sendSearchWord(search.searchWord);
      search.wordShow = search.searchWord;
    }
  },
  searchChange() {
    // 如果为空 同步到timeline
    if (search.searchWord === "" || !search.searchWord) {
      search.wordShow = "";
      searchWordEvent.sendSearchWord("");
    }
  },
});

const handleWindow = reactive({
  // 最小化
  minus() {
    operate.minus();
  },
  // 最大化 重置大小
  maximize() {
    winMax.value = !winMax.value;
    operate.maximize();
  },
  // 关闭
  close() {
    operate.close();
  },
});

const donate = reactive({
  show: false,
});

const setting = reactive({
  show: false,
});
</script>

<style lang="scss" rel="stylesheet/scss">
.app-top {
  position: relative;
  height: 60px;
  background: #fdba4b;
  -webkit-app-region: drag;

  .no-drag {
    -webkit-app-region: no-drag;
  }

  .title {
    font-size: 26px;
    font-family: "DOUYUfont", sans-serif;
    color: #707070;
  }
}

.menuShow-item {
  position: relative;
  transition: all 0.3s;

  &.not {
    filter: grayscale(100%) opacity(25%);
  }
}
</style>
