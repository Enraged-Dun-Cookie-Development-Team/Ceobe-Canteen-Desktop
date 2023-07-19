<template>
  <div class="app-top d-flex justify-space-between align-center w-100 pa-2">
    <div class="h-100 d-flex align-center">
      <v-img src="../../assets/image/logo/icon.png" width="40"></v-img>
      <span class="title ml-2 mt-1">小刻食堂</span>
    </div>

    <div class="h-100 no-drag">
      <v-menu v-model="search.show" :close-on-content-click="false" location="bottom" transition="slide-y-transition">
        <template v-slot:activator="{ props }">
          <v-btn variant="text" icon="fas fa-magnifying-glass" v-bind="props"></v-btn>
        </template>
        <v-card class="mx-auto" color="grey-lighten-3" min-width="400">
          <v-text-field
            :loading="search.searchLoading"
            v-model="search.model"
            density="compact"
            variant="solo"
            label="查找饼仓"
            autofocus
            append-inner-icon="fa fa-magnifying-glass"
            hide-details
            @click:append-inner="search.searchIng"
            clearable
          ></v-text-field>
        </v-card>
      </v-menu>

      <v-menu v-model="menuShow.show" :close-on-content-click="false" location="bottom" transition="slide-y-transition">
        <template v-slot:activator="{ props }">
          <v-btn variant="text" icon="fas fa-database" v-bind="props"></v-btn>
        </template>
        <v-list density="compact">
          <v-list-item
            class="menuShow-item"
            :class="item.check ? '' : 'not'"
            v-for="(item, i) in sourceList"
            :key="i"
            :value="item"
            color="primary"
            @click="menuShow.changeSelectSource(item)"
            :prepend-avatar="getImage(item.img)"
          >
            <v-list-item-title v-text="item.name"></v-list-item-title>
          </v-list-item>
        </v-list>
      </v-menu>
    </div>

    <div class="h-100 no-drag">
      <v-btn variant="text" icon="fas fa-gear"></v-btn>
      <v-btn variant="text" icon="fas fa-minus" @click="handleWindow.minus"></v-btn>
      <v-btn
        variant="text"
        :icon="winMax ? 'fas fa-minimize' : 'fas fa-maximize'"
        @click="handleWindow.maximize"
      ></v-btn>
      <v-btn variant="text" icon="fas fa-circle-xmark" @click="handleWindow.close" color="error"></v-btn>
    </div>
  </div>
</template>

<script setup name="index">
import { reactive, ref } from 'vue';
import { sourceInfo } from '@/constant';
import { getImage } from '@/utils/imageUtil';

const sourceList = ref(sourceInfo);
const winMax = ref(false);
const menuShow = reactive({
  show: false,
  changeSelectSource(data) {
    data.check = !data.check;
  }
});
const search = reactive({
  show: false,
  searchLoading: false,
  model: null,
  searchIng(event) {
    search.searchLoading = true;
    console.log(search.model);
    setTimeout(() => {
      search.searchLoading = false;
      search.show = false;
    }, 1000);
  }
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
  }
});
</script>

<style rel="stylesheet/scss" lang="scss">
.app-top {
  height: 60px;
  background: #fdba4b;
  position: relative;
  -webkit-app-region: drag;

  .no-drag {
    -webkit-app-region: no-drag;
  }

  .title {
    color: #707070;
    font-family: 'DOUYUfont', sans-serif;
    font-size: 26px;
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
