<script setup>
import CrabButton from './CrabButton.vue'
import RandForex from './randForex.vue'
import CrabBreeding from './CrabBreeding.vue'
import settings from '../data/settings.json'
</script>

<script>
export default {
  emits: ['crab', 'buy', 'sell', 'foodBought', 'crabsBreed'],
  data() {
    return {
      crabs: 0,
      dollars: 0,
      doge: 0,
      crabFood: 0,
      crabSpeedUpgrade: 1,
      crabWaitingUpgrade: 1,
      crabChanceUpgrade: 1
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
    },
    foodBought(foodCost){
      if(this.dollars >= foodCost){
        this.dollars -= foodCost;
        this.crabFood++;
      }
    },
    crabsBreed(amount){
      if(this.crabFood >= amount){
        this.crabFood -= amount;
        this.crabs += amount;
      }
    }
  },
  computed: {
    crabWaitingTime(){
      return settings.crabWaitingTime * this.crabWaitingUpgrade;
    },
    crabChance(){
      return settings.crabChance * this.crabChanceUpgrade;
    },
    crabSpeed(){
      return settings.crabSpeed * this.crabSpeedUpgrade;
    }
  }
}
</script>

<template>
<p>Crabs: {{this.crabs}}</p>
<p>${{this.dollars.toFixed(2)}}</p>
<p>DogeCoin: {{this.doge}}</p>
<p>CrabFood: {{this.crabFood}}</p>
<CrabButton @crab="crabFound" :waitingTicks="this.crabWaitingTime" :successChance="this.crabChance" :millisPerTick="this.crabSpeed"></CrabButton>
<br>
<crabBreeding 
:crabFood="this.crabFood"
:crabs="this.crabs"
:dollars="this.dollars"
:breedRate="1.25"
:millisPerTick="100"
:ticksPerBreed="100"
:crabFoodCost="1.5"
@foodBought="this.foodBought"
@crabsBreed="this.crabsBreed"
></crabBreeding>
<RandForex 
@sell="crabSold" 
@buy="crabBought" 
currencyFrom="Crab" 
currencyTo="Dollars" 
:minPrice="Number(settings.crabMinPrice)" 
:maxPrice="Number(settings.crabMaxPrice)" 
:maxVolitlity="Number(settings.crabMaxVolitility)"
:priceHistoryLength="10"
:startingPrice="Number(settings.crabStartingPrice)"
></RandForex>
<RandForex 
@sell="dogeSold" 
@buy="dogeBought" 
currencyFrom="DogeCoin" 
currencyTo="Dollars" 
:minPrice="settings.dogeMinPrice" 
:maxPrice="settings.dogeMaxPrice" 
:maxVolitlity="settings.dogeMaxVolitlity"
:minVolitlity="settings.dogeMinVolitlity"
:precision="2"
:startingPrice="settings.dogeStartingPrice"
:priceHistoryLength="100"
></RandForex>
</template>

<style scoped>

</style>
