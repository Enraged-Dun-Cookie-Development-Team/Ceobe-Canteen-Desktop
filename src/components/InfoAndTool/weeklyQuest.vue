<template>
  <div class="weekly-quest flex-column">
    <v-card>
      <v-card-text>
        <div class="d-flex flex-row">
          <div>
            <div
                v-for="(item, index) in quest.countdown"
                :key="index"
            >
              <div>
                距离
                <v-tooltip location="bottom" :text="item.remark" v-if="item.remark">
                  <template v-slot:activator="{ props }">
                    <span v-bind="props" class="online-blue cursor-pointer">{{ item.text }}</span>
                  </template>
                </v-tooltip>
                <span v-else class="online-orange">{{ item.text }}</span>
                <span title="国服 UTC-8">{{ ' ' + quest.calcActivityDiff(item.time) }}</span>
              </div>
            </div>
          </div>
        </div>
        <div class="d-flex flex-row justify-space-around mt-2">
          <span v-for="item in quest.resources" :key="item.name">
            <v-tooltip location="bottom" :text="`${item.name} - 开放日期： ${quest.calcResourceOpenDay(item.day)}`">
              <template v-slot:activator="{ props }">
                <v-img
                    class="cursor-pointer"
                    :aspect-ratio="1"
                    v-bind="props"
                    :src="getImage(item.src)"
                    width="40"
                    :class="item.notToday ? 'opacity-2' : ''"/>
              </template>
            </v-tooltip>
          </span>
        </div>
      </v-card-text>
    </v-card>
  </div>
</template>

<script setup name="weeklyQuest">
import {reactive, defineProps, onMounted} from "vue";
import {dayInfo} from "@/constant";
import {getImage} from '@/utils/imageUtil'
import {changeToCCT, numberToWeek, calcDiff} from "@/utils/timeUtil";

const quest = reactive({
  dayInfo: dayInfo,
  openResources: false,
  resourceInfo: {},
  resources: [],
  countdown: [],
  getData() {
    window.ceobeRequest.getResourceInfo().then(res => {
      quest.resourceInfo = res.data.data
      quest.resourcesNotToday()
      quest.calcCountdown()
    })
  },
  calcCountdown() {
    // 倒计时
    quest.countdown = quest.resourceInfo.countdown.filter(
        (x) =>
            new Date(x.start_time) <= changeToCCT(new Date()) &&
            new Date(x.over_time) >= changeToCCT(new Date())
    );
  },
  // 今天有没有该资源可以刷
  resourcesNotToday() {
    let date = changeToCCT(new Date());
    // 如果日期在里面
    if (quest.resourceInfo) {
      let starTime = new Date(quest.resourceInfo.resources.start_time);
      let overTime = new Date(quest.resourceInfo.resources.over_time);
      if (date >= starTime && date <= overTime) {
        quest.resources = quest.dayInfo.map((item) => {
          return {
            ...item,
            notToday: false
          }
        });
        quest.openResources = true;
        return;
      }
    }
    // 如果不在里面
    let week = date.getDay();
    // 判断4点更新
    week = date.getHours() >= 4 ? week : week - 1;
    week = week == -1 ? 6 : week;
    quest.resources = quest.dayInfo.map((item) => {
      return {
        ...item,
        notToday: !item.day.includes(week),
      }
    });
    quest.openResources = false;
  },
  calcActivityDiff(endDate) {
    let startDate = changeToCCT(new Date());
    const diff = calcDiff(endDate, startDate);
    if (diff) {
      return '剩' + diff;
    } else {
      return '已结束';
    }
  },
  // 计算资源关卡开启时间
  calcResourceOpenDay(days) {
    if (quest.openResources) {
      return '活动期间，“资源收集”限时全天开放';
    } else {
      return days.map((x) => numberToWeek(x)).join();
    }
  },
})

onMounted(() => {
  quest.getData()
})
</script>

<style rel="stylesheet/scss" lang="scss">
.weekly-quest {
  width: 100%;
}
</style>