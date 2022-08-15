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
      crabChanceUpgrade: 1,
      trappingLevel: 0,
      trappingXP: 0,
      trappingXpToLevel: 10,
      trappingXpIncrease: 1.1,
      husbandryLevel: 0,
      husbandryXP: 0,
      husbandryXpToLevel: 10,
      husbandryXpIncrease: 1.25,
      barterLevel: 0,
      barterXP: 0,
      barterXpToLevel: 10,
      barterXpIncrease: 1.25,
      investingLevel: 0,
      investingXP: 0,
      investingXpToLevel: 100,
      investingXpIncrease: 1.1,
      startingInvestingFee: 0.1,
      startingFoodPrice: 1,
      startingBreedingrate: 1.25
    }
  },
  methods:{
    crabFound(){
      this.crabs += this.trappingAdjustemnt;
      this.addTrappingXP();
    },
    crabSold(dollarsPerCrab){
      let fee = (this.investingFee * dollarsPerCrab) + dollarsPerCrab
      if(this.crabs >= 1){
        this.crabs--;
        this.dollars += dollarsPerCrab - fee;
        this.addInvestingXP(dollarsPerCrab);
      }
    },
    crabBought(dollarsPerCrab){
      let fee = (this.investingFee * dollarsPerCrab) + dollarsPerCrab
      if(this.dollars >= dollarsPerCrab + fee){
        this.dollars -= dollarsPerCrab + fee;
        this.crabs++;
        this.addInvestingXP(dollarsPerCrab);
      }
    },
    dogeSold(dollarsPerDoge){
      let fee = (this.investingFee * dollarsPerDoge) + dollarsPerDoge
      if(this.doge >= 1){
        this.doge--;
        this.dollars += dollarsPerDoge - fee;
        this.addInvestingXP(dollarsPerDoge);
      }
    },
    dogeBought(dollarsPerDoge){
      let fee = (this.investingFee * dollarsPerDoge) + dollarsPerDoge
      if(this.dollars >= dollarsPerDoge + fee){
        this.dollars -= dollarsPerDoge + fee;
        this.doge++;
        this.addInvestingXP(dollarsPerDoge);
      }
    },
    foodBought(foodCost){
      if(this.dollars >= foodCost){
        this.dollars -= foodCost;
        this.crabFood++;
        this.addBarterXP();
      }
    },
    crabsBreed(amount){
      if(this.crabFood >= amount){
        this.crabFood -= amount;
        this.crabs += amount;
        this.addHusbandryXP(amount);
      }
    },
    addTrappingXP(amount=1){
      this.trappingXP += amount;
      while(this.trappingXP >= this.trappingXpToLevel){
        this.trappingXP -= this.trappingXpToLevel;
        this.trappingLevel++;
        this.trappingXpToLevel *= this.trappingXpIncrease;
      }
    },
    addHusbandryXP(amount=1){
      this.husbandry += amount;
      while(this.husbandryXP >= this.husbandryXpToLevel){
        this.husbandryXP -= this.husbandryXpToLevel;
        this.husbandryLevel++;
        this.husbandryXpToLevel *= this.husbandryXpIncrease;
      }
    },
    addBarterXP(amount=1){
      this.barterXP += amount;
      while(this.barterXP >= this.barterXpToLevel){
        this.barterXP -= this.barterXpToLevel;
        this.barterLevel++;
        this.barterXpToLevel *= this.barterXpIncrease;
      }
    },
    addInvestingXP(amount=1){
      this.investingXP += amount;
      while(this.investingXP >= this.investingXpToLevel){
        this.investingXP -= this.investingXpToLevel;
        this.investingLevel++;
        this.investingXpToLevel *= this.investingXpIncrease;
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
    },
    investingFee(){
      if(this.investingLevel < 100){
        return this.startingInvestingFee - (this.startingInvestingFee * this.investingLevel / 100);
      }
      else{
        return 0;
      }
    },
    investingFeeStr(){
      return (this.investingFee * 100) + '%'
    },
    crabFoodCost(){
      if(this.barterLevel < 100){
        return this.startingFoodPrice - (this.startingFoodPrice * this.barterLevel / 100) + 0.5;
      }
      else{
        return 0.5;
      }
    },
    breedingAdjustment(){
      if(this.husbandryLevel < 100){
        return 1 + this.husbandryLevel / 100;
      }
      else{
        return 2;
      }
    },
    adjustedBreedingRate(){
      return this.startingBreedingrate * this.breedingAdjustment;
    },
    trappingAdjustemnt(){
      if(this.trappingLevel < 100){
        return 1 + this.trappingLevel / 10;
      }
      else{
        return 11;
      }
    }
  }
}
</script>

<template>
<h2>Items</h2>
<p>Crabs: {{this.crabs}}</p>
<p>${{this.dollars.toFixed(2)}}</p>
<p>DogeCoin: {{this.doge}}</p>
<p>CrabFood: {{this.crabFood}}</p>
<br>
<h2>Skills</h2>
<p>Trapping: {{this.trappingLevel}}</p>
<p>Husabndry: {{this.husbandryLevel}}</p>
<p>Barter: {{this.barterLevel}}</p>
<p>Investing: {{this.investingLevel}}</p>
<CrabButton @crab="crabFound" :waitingTicks="this.crabWaitingTime" :successChance="this.crabChance" :millisPerTick="this.crabSpeed"></CrabButton>
<br>
<crabBreeding 
:crabFood="this.crabFood"
:crabs="this.crabs"
:dollars="this.dollars"
:breedRate="this.adjustedBreedingRate"
:millisPerTick="100"
:ticksPerBreed="100"
:crabFoodCost="this.crabFoodCost"
@foodBought="this.foodBought"
@crabsBreed="this.crabsBreed"
></crabBreeding>
<h2>Trading</h2>
<p>Your current trading fee is {{this.investingFeeStr}}</p>
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
