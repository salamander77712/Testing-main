<script setup>
import LineGraph from './LineGraph.vue';
</script>


<script>
export default {
  props: {
    currencyFrom:{
      required: true
    },
    currencyTo:{
      required: true
    },
    startingPrice:{
      type: Number,
      default: 1
    },
    maxPrice:{
      type: Number,
      default: 0
    },
    minPrice:{
      type: Number,
      default: 0
    },
    minVolitlity:{
      type: Number,
      default: 0
    },
    maxVolitlity:{
      type: Number,
      required: true
    },
    millisPerTick:{
      type: Number,
      default: 1000
    },
    average:{
      type: Number,
      default: -1
    },
    precision:{
      type: Number,
      default: 2
    },
    priceHistoryLength:{
      type: Number,
      default: 4
    }
  },
  emits:['buy', 'sell'],
  data() {
    return {
      currentPrice: this.startingPrice,
      priceHistory: []
    }
  },
  methods:{
    tick(){
      let priceIncrease = (Math.random() * this.maxVolitlity) + this.minVolitlity;
      if(Math.random() < 0.5){
        priceIncrease *= -1;
      }
      let newPrice = Number(priceIncrease + this.currentPrice);
      if(newPrice < this.minPrice){
        this.currentPrice = this.minPrice;
      }
      else{
        if(this.maxPrice != 0){
          if(newPrice > this.maxPrice){
            this.currentPrice = this.maxPrice;
          }
          else{
            this.currentPrice = newPrice;
          }
        }
        else{
          this.currentPrice = newPrice;
        }
      }
      this.priceHistory.push(this.priceRounded);
      if(this.priceHistory.length > this.priceHistoryLength){
        this.priceHistory.splice(0, 1);
      }
    }
  },
  mounted(){
    setInterval(() => {this.tick()}, this.millisPerTick);
  },
  computed:{
    priceRounded(){
      return this.currentPrice.toFixed(this.precision);
    },
    chartData(){
      return this.priceHistory;
    },
    labels(){
      let output = [];
      for (let i = -this.priceHistoryLength + 1; i <= 0; i++) {
        output.push(i);
      }
      return output;
    }
  }
}
</script>

<template>
<div>
  <h3>Convert {{currencyFrom}} To {{currencyTo}}</h3>
  <p>1 {{currencyFrom}} is currently worth {{priceRounded}} {{currencyTo}}</p>
  <button @click="$emit('buy', priceRounded)">Buy 1 {{currencyFrom}}</button>
  <button @click="$emit('sell', priceRounded)">Sell 1 {{currencyFrom}}</button>
  <div class="graph">
  <LineGraph
  :labels="labels" 
  :values="chartData"
  :title="currencyFrom + ' To ' + currencyTo"
  :min="0"
  :max="5"
  >
  </LineGraph></div>
</div>
</template>

<style scoped>
.graph{
  max-width: 200px;
  max-height: 200px;
}
</style>
