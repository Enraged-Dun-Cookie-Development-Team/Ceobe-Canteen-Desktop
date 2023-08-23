<template>
  <div class="setting">
    <v-card>
      <v-toolbar color="#e6a23c">
        <v-toolbar-title>设置</v-toolbar-title>
        <v-spacer></v-spacer>
        <v-btn variant="text" icon="fa-solid fa-xmark" @click="setting.close"></v-btn>
      </v-toolbar>
      <v-card-item>
        <div class="d-flex justify-space-between align-center w-100">
          <div>
            <v-card-title>开机自启</v-card-title>
            <v-card-subtitle>每次开机就能自动蹲饼呢~</v-card-subtitle>
          </div>
          <div>
            <v-switch v-model="setting.isBoot" color="#ffba4b" @change="setting.changeBoot"></v-switch>
          </div>
        </div>
      </v-card-item>
      <v-card-item>
        <div class="d-flex justify-space-between align-center w-100">
          <div>
            <v-card-title>版本</v-card-title>
            <v-card-subtitle>当前版本 x.xx</v-card-subtitle>
          </div>
          <div>
            <v-btn color="#ffba4b" @click="setting.checkVersion">检查更新</v-btn>
          </div>
        </div>
      </v-card-item>
    </v-card>
    <v-snackbar v-model="setting.showDownload" >
      检测到了新版本，即将跳转到下载网页
    </v-snackbar>
  </div>

</template>

<script setup name="setting">
import {onMounted, reactive} from "vue";
const emits = defineEmits({
  close: null
})

const setting = reactive({
  isBoot:false,
  changeBoot(){
    window.operate.bootSetting(setting.isBoot).then(res=>{
      
    });
  },
  initBoot() {
    window.operate.getBootSetting().then(res=>{
      setting.isBoot = res;
    });
  },
  close(){
    emits('close')
  },

  showDownload:false,
  checkVersion(){
    setting.showDownload = true;
    setTimeout(()=>{
      // window.operate.openUrlInBrowser('')
      setting.showDownload = false
    },3000)
  }
})

onMounted(() => {
  setting.initBoot();
});
</script>

<style rel="stylesheet/scss" lang="scss">
</style>