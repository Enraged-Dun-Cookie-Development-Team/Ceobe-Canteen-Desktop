<template>
  <index></index>
</template>

<script setup>
import Index from "./view/home"
import {onMounted, ref} from "vue";

const data = ref(null);
onMounted(()=>{
  window.ceobeRequest.getCardList().then(res => {
    data.value = res.data;
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
