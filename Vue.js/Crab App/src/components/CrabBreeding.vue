<script>
export default {
  props: ['crabFood', 'crabs', 'dollars', 'breedRate', 'millisPerTick', 'ticksPerBreed', 'crabFoodCost'],
  emits: ['foodBought', 'crabsBreed'],
  data() {
    return {
      ticks: 0
    }
  },
  methods:{
    breed(){
      let crabsBreed = this.crabs * this.breedRate;
      if(crabsBreed > this.crabFood){
        crabsBreed = this.crabFood;
      }
      this.$emit('crabsBreed', crabsBreed);
    },
    buyFood(){
      if(this.dollars >= this.crabFoodCost){
        this.$emit('foodBought', this.crabFoodCost);
      }
    }
  },
  computed:{
    progress(){
      return (this.ticks / this.ticksPerBreed * 100) + '%'
    },
    disableBuying(){
      if(this.dollars >= this.crabFoodCost){
        return false;
      }
      return true;
    }
  },
  mounted(){
    setInterval(() =>{
      if(this.ticks >= this.ticksPerBreed){
        this.breed();
        this.ticks = 0;
      }
      this.ticks++;
    }, this.millisPerTick)
  }
}
</script>

<template>
<div>
  <button @click="buyFood" :disabled="disableBuying">Buy 1 Crab Food For ${{crabFoodCost.toFixed(2)}}</button>
  <br>
  <span>Your Crabs Are Making More Crabs</span>
</div>
</template>

<style scoped>
span{
  background : linear-gradient(to right, blue v-bind('progress'), rgba(0,0,0,0) v-bind('progress'));
}
</style>
