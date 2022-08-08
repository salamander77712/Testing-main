<script setup>
import CrabButton from './CrabButton.vue'
import RandForex from './randForex.vue'
</script>

<script>
export default {
  emits: ['crab', 'buy', 'sell'],
  data() {
    return {
      crabs: 0,
      dollars: 0
    }
  },
  methods:{
    crabFound(){
      this.crabs++;
    },
    crabSold(dollarsPerCrab){
      if(this.crabs >= 1){
        console.log(dollarsPerCrab);
        this.crabs--;
        this.dollars += Number(dollarsPerCrab);
      }
    },
    crabBought(dollarsPerCrab){
      if(this.dollars >= dollarsPerCrab){
        this.dollars -= dollarsPerCrab;
        this.crabs++;
      }
    }
  }
}
</script>

<template>
<p>Crabs:{{this.crabs}} $:{{this.dollars.toFixed(2)}}</p>
<CrabButton @crab="crabFound" waitingTicks="10" successChance="1" millisPerTick="100"></CrabButton>
<RandForex @sell="crabSold" @buy="crabBought" currencyFrom="Crab" currencyTo="Dollars" :minPrice="Number(0.5)" :maxPrice="Number(5)" :maxVolitlity="Number(0.5)"></RandForex>
</template>

<style scoped>

</style>
