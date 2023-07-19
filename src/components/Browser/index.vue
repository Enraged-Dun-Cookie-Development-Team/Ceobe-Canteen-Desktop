<template>
  <div class="browser w-100">
    <v-toolbar :title="query.source">
      <template #prepend>
        <v-img :src="getImage(query.icon)" width="26" class="border-radius-50"></v-img>
      </template>
      <v-btn size="small" @click="back" icon="fas fa-circle-xmark" title="关闭"></v-btn>
    </v-toolbar>
    <webview
      :src="query.url"
      style="margin: auto"
      :style="{ width: query.width ? query.width : '100%' }"
      :useragent="query.useragent ? query.useragent : null"
    ></webview>
  </div>
</template>

<link rel="stylesheet" type="text/css" href="" />
