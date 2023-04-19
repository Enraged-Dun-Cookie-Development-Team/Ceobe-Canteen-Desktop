<template>
  <div class="app-main">
    <keep-alive>
      <transition>
        <router-view/>
      </transition>
    </keep-alive>
  </div>
</template>

<script setup name="AppMain">

</script>

<style rel="stylesheet/scss" lang="scss">
.app-main {
}
</style>