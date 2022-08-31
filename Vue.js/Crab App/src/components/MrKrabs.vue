<script setup>
import CrabButton from './CrabButton.vue'
import RandForex from './randForex.vue'
import CrabBreeding from './CrabBreeding.vue'
import settings from '../data/settings.json'
</script>

<script>
export default {
  emits: ['crab', 'buy', 'sell', 'foodBought', 'crabsBreed', 'noCrab'],
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
      startingBreedingrate: 1.25,
      crabMessage: ""
    }
  },
  methods:{
    crabFound(){
      this.crabs += this.trappingAdjustemnt;
      this.addTrappingXP();
      this.crabMessage = "You Found a Crab!";
    },
    crabNotFound(){
      this.crabMessage = "You failed to find any crabs...";
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
<div class="center">
<h2>Items</h2>
<p>Crabs: {{crabs}}</p>
<p>${{dollars.toFixed(2)}}</p>
<p>DogeCoin: {{doge}}</p>
<p>CrabFood: {{crabFood}}</p>
<h2>Skills</h2>
<p>Trapping: {{trappingLevel}}</p>
<p>Husabndry: {{husbandryLevel}}</p>
<p>Barter: {{barterLevel}}</p>
<p>Investing: {{investingLevel}}</p>
<h2>Trapping</h2>
<CrabButton @crab="crabFound" @noCrab="crabNotFound" :waitingTicks="crabWaitingTime" :successChance="crabChance" :millisPerTick="crabSpeed"></CrabButton>
<p>{{crabMessage}}</p>
<h2>Breeding</h2>
<crabBreeding 
:crabFood="crabFood"
:crabs="crabs"
:dollars="dollars"
:breedRate="adjustedBreedingRate"
:millisPerTick="100"
:ticksPerBreed="100"
:crabFoodCost="crabFoodCost"
@foodBought="foodBought"
@crabsBreed="crabsBreed"
></crabBreeding>
<h2>Trading</h2>
<p>Your current trading fee is {{investingFeeStr}}</p>
<div class="center">
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
</div>
</div>
</template>

<style scoped>
div.center{
  margin: auto;
  width: 50%;
  padding: 10px;
  text-align: center;
}
</style>
