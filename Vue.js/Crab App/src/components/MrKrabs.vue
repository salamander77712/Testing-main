<script setup>
import CrabButton from './CrabButton.vue'
</script>

<script>
export default {
  emits: ['crab'],
  data() {
    return {
      crabs: 0
    }
  },
  methods:{
    crabFound(){
      this.crabs++;
    }
  }
}
</script>

<template>
<p>Crabs: {{this.crabs}}</p>
<CrabButton @crab="crabFound" waitingTicks="10" successChance="3" millisPerTick="100"></CrabButton>
</template>

<style scoped>

</style>
