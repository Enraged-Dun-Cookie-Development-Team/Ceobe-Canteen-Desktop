<template>
  <meta name="referrer" content="no-referrer" />
  <router-view />
</template>

<script lang="ts" setup>
import {onMounted} from "vue";
import {getCurrent} from "@tauri-apps/api/window";
import updater from "./api/operations/updater";
import {notification} from "@tauri-apps/api";

onMounted(()=>{
  if (getCurrent().label == "main"){
    console.log("checking update")
    updater.checkUpdate()
        .then(async (hasNewer)=>{
          if (hasNewer){
            if (!await notification.isPermissionGranted()){
              await notification.requestPermission()
            }

            notification.sendNotification({
              title:"新版小刻食堂可用",
              icon:"/assert/icon.png",
            })
          }
        }).catch(async (err)=>{
        if (!await notification.isPermissionGranted()){
          await notification.requestPermission()
        }

        notification.sendNotification({
          title:"获取更新失败",
          icon:"/assert/icon.png",
          body:err.toString()
        })
    })

  }
})

</script>
