<template>
    <div class="background">
    </div>
</template>

<script setup name="index">
import { onMounted, onBeforeMount } from "vue";
import { backgroundInit } from "@/utils/backgroundUtil"

onBeforeMount(() => {
    backgroundInit().then(() => {})
})
</script>