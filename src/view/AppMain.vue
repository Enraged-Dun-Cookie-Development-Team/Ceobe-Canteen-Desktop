<template>
  <div class="app-main">
    <v-app>
      <v-main>
        <router-view v-slot="{Component,route}">
          <transition name="fade-transform">
            <component v-if="!route.meta.link" :is="Component" :key="route.path"/>
          </transition>
        </router-view>
      </v-main>
    </v-app>
  </div>
</template>

<script setup name="AppMain">

</script>

<style rel="stylesheet/scss" lang="scss">
.app-main {
}
</style>