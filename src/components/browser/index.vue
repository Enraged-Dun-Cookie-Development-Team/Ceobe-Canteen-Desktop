<template>
<!--  <div>{{a}}</div>-->
  <div @click="back">back</div>
<!--  <iframe src="https://www.baidu.com"></iframe>-->
</template>

<script setup name="index">
// 假设你想获取名为 "data" 的元信息
import {useRoute} from "vue-router";
import {onMounted, ref} from "vue";
import router from "@/router";

const route = useRoute();
const a = ref(null)

function back(){
  router.back();
}
onMounted(()=>{
  a.value = route.meta
})

</script>

<style rel="stylesheet/scss" lang="scss">
.index {
}
</style>