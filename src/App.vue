<template>
  <div>{{ data }}</div>
</template>

<script setup>
import {onMounted, ref} from "vue";

const data = ref(null);
debugger
onMounted(()=>{

  window.ceobeRequest.getCardList1().then(res => {
    data.value = res;
  })
})
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
