<script setup>
import CrabButton from './CrabButton.vue'
import TickTimer from './TickTimer.vue'
</script>

<script>
export default {
  emits: ['tick'],
  data() {
    return {
      ticks: 0
    }
  },
  methods:{
    updateTicks(){
      this.ticks++;
    }
  }
}
</script>

<template>
<TickTimer millisPerTick = "100"></TickTimer>
<CrabButton waitingTicks="10" successChance="100" :ticks="ticks"></CrabButton>
</template>

<style scoped>

</style>
