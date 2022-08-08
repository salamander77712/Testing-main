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
    }
  },
  emits:['buy', 'sell'],
  data() {
    return {
      currentPrice: this.startingPrice
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
    }
  },
  mounted(){
    setInterval(() => {this.tick()}, this.millisPerTick);
  },
  computed:{
    priceRounded(){
      return this.currentPrice.toFixed(this.precision);
    }
  }
}
</script>

<template>
<div>
  <h2>Convert {{this.currencyFrom}} To {{this.currencyTo}}</h2>
  <p>1 {{this.currencyFrom}} is currently worth {{this.priceRounded}} {{this.currencyTo}}</p>
  <button @click="$emit('buy', this.priceRounded)">Buy 1 {{this.currencyFrom}}</button>
  <button @click="$emit('sell', this.priceRounded)">Sell 1 {{this.currencyFrom}}</button>
</div>
</template>

<style scoped>

</style>
