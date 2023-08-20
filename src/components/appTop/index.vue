<template>
  <div class="app-top d-flex justify-space-between align-center w-100 pa-2">
    <div class="h-100 d-flex align-center">
      <v-img src="../../assets/image/logo/icon.png" width="40"></v-img>
      <span class="title ml-2 mt-1">小刻食堂</span>
    </div>

    <div class="h-100 no-drag">
      <v-menu v-model="search.show" :close-on-content-click="false" location="bottom" transition="slide-y-transition">
        <template #activator="{ props }">
          <v-btn variant="text" icon="fas fa-magnifying-glass" v-bind="props"></v-btn>
        </template>
        <v-card class="mx-auto" color="grey-lighten-3" min-width="400">
          <v-text-field
            v-model="search.searchWord"
            density="compact"
            variant="solo"
            label="查找饼仓"
            autofocus
            append-inner-icon="fa fa-magnifying-glass"
            hide-details
            clearable
            @click:append-inner="search.searching"
            @update:model-value="search.searchChange"
          ></v-text-field>
        </v-card>
      </v-menu>
      <span v-if="search.wordShow !== ''">{{ search.wordShow }}</span>
      <v-menu
        v-model="menuShow.show"
        :close-on-content-click="false"
        location="bottom"
        transition="slide-y-transition"
        :on-update:model-value="menuShow.changeDatasourceOpen(menuShow.show)"
      >
        <template #activator="{ props }">
          <v-btn variant="text" icon="fas fa-database" v-bind="props"></v-btn>
        </template>
        <v-list density="compact">
          <v-list-item
            v-for="(item, i) in menuShow.datasourceList"
            :key="i"
            class="menuShow-item"
            :class="item.check ? '' : 'not'"
            :value="item"
            color="primary"
            :title="item.nickname"
            :prepend-avatar="item.avatar"
            @click="menuShow.changeSelectSource(item)"
          >
          </v-list-item>
        </v-list>
      </v-menu>
    </div>

    <div class="h-100 no-drag">
      <v-btn variant="text" icon="fas fa-comments-dollar" @click="donate.show = true"></v-btn>
      <v-btn variant="text" icon="fas fa-gear" @click="setting.show = true"></v-btn>
      <v-btn variant="text" icon="fas fa-minus" @click="handleWindow.minus"></v-btn>
      <v-btn
        variant="text"
        :icon="winMax ? 'fas fa-minimize' : 'fas fa-maximize'"
        @click="handleWindow.maximize"
      ></v-btn>
      <v-btn variant="text" icon="fas fa-circle-xmark" color="error" @click="handleWindow.close"></v-btn>
    </div>
  </div>

  <v-dialog width="800" persistent v-model="donate.show" transition="dialog-top-transition">
    <donate-page  @close="donate.show = false"></donate-page>
  </v-dialog>

  <v-dialog width="400" persistent v-model="setting.show"  transition="dialog-top-transition">
    <setting-page @close="setting.show = false"></setting-page>
  </v-dialog>
</template>

<script setup name="index">
import { reactive, ref } from 'vue';
import { getImage } from '@/utils/imageUtil';
import { getAllDatasources, getDatasourceComb } from '@/api/list';
import donatePage from './donate'
import settingPage from './setting'

const winMax = ref(false);

const menuShow = reactive({
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
      let datasourceConfig = JSON.parse(window.localStorage.getItem('datasource-config'));
      // 打开列表
      getAllDatasources().then((data) => {
        if (data.status == 200) {
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
      let datasourceComb = window.localStorage.getItem('datasource-comb');
      // 如果组合id和之前一样，不进行刷新
      if (datasourceComb == comb_id) {
        return;
      }
      window.localStorage.setItem('datasource-config', JSON.stringify(datasourceConfig));
      window.localStorage.setItem('datasource-comb', comb_id);
      window.datasourceConfig.updateDatasourceComb();
    }
  },
});
const search = reactive({
  show: false,
  wordShow: '',
  searchWord: null,
  searching() {
    // 将搜索词发送给timeline进行处理
    window.searchWordEvent.sendSearchWord(search.searchWord);
    search.wordShow = search.searchWord;
  },
  searchChange() {
    // 如果为空 同步到timeline
    if (search.searchWord === '' || !search.searchWord) {
      window.searchWordEvent.sendSearchWord(search.searchWord);
      search.wordShow = '';
    }
  },
});

const handleWindow = reactive({
  // 最小化
  minus() {
    window.operate.minus();
  },
  // 最大化 重置大小
  maximize() {
    winMax.value = !winMax.value;
    window.operate.maximize();
  },
  // 关闭
  close() {
    window.operate.close();
  },
});

const donate = reactive({
  show:false
})

const setting = reactive({
  show:false,
})
</script>

<style rel="stylesheet/scss" lang="scss">
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
    font-family: 'DOUYUfont', sans-serif;
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
