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
      dollars: 0,
      doge: 0
    }
  },
  methods:{
    crabFound(){
      this.crabs++;
    },
    crabSold(dollarsPerCrab){
      if(this.crabs >= 1){
        this.crabs--;
        this.dollars += Number(dollarsPerCrab);
      }
    },
    crabBought(dollarsPerCrab){
      if(this.dollars >= dollarsPerCrab){
        this.dollars -= dollarsPerCrab;
        this.crabs++;
      }
    },
    dogeSold(dollarsPerDoge){
      if(this.doge >= 1){
        this.doge--;
        this.dollars += Number(dollarsPerDoge);
      }
    },
    dogeBought(dollarsPerDoge){
      if(this.dollars >= dollarsPerDoge){
        this.dollars -= dollarsPerDoge;
        this.doge++;
      }
    }
  }
}
</script>

<template>
<p>Crabs:{{this.crabs}} $:{{this.dollars.toFixed(2)}} DogeCoin:{{this.doge}}</p>
<CrabButton @crab="crabFound" waitingTicks="10" successChance="1" millisPerTick="100"></CrabButton>
<RandForex 
@sell="crabSold" 
@buy="crabBought" 
currencyFrom="Crab" 
currencyTo="Dollars" 
:minPrice="Number(0.5)" 
:maxPrice="Number(5)" 
:maxVolitlity="Number(0.5)"
:priceHistoryLength="10"
></RandForex>
<RandForex 
@sell="dogeSold" 
@buy="dogeBought" 
currencyFrom="DogeCoin" 
currencyTo="Dollars" 
:minPrice="Number(0)" 
:maxPrice="Number(10000)" 
:maxVolitlity="Number(1)"
:minVolitlity="Number(0.1)"
:precision="2"
:startingPrice="1"
:priceHistoryLength="100"
></RandForex>
</template>

<style scoped>

</style>
