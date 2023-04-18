<template>
  <div class="app-main">
    <v-app>
      <v-main>
        <keep-alive>
          <transition>
            <router-view/>
          </transition>
        </keep-alive>
      </v-main>
    </v-app>
  </div>
</template>

<script setup name="AppMain">
import {computed} from "vue";
import {useRoute} from "vue-router";
import Home from './home/index'

const route = useRoute();

</script>

<style rel="stylesheet/scss" lang="scss">
.app-main {
}
</style>