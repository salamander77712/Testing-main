<script>
export default {
  props: ['waitingTicks', 'successChance', 'millisPerTick'],
  emits: ['crab', 'noCrab'],
  data() {
    return {
      ticksLocked : 0
    }
  },
  methods:{
    onClick(){
      let waitingLoop = setInterval(wait => {
        this.ticksLocked++;
      if(this.ticksLocked >= this.waitingTicks){
        this.ticksLocked = 0;
        if(Math.floor(Math.random() * this.successChance) == 0){
          this.$emit('crab');
        }
        else{
          this.$emit('noCrab');
        }
        clearInterval(waitingLoop);
      }
    }, this.millisPerTick);
    }
  },
  computed:{
    progress(){
        return (this.ticksLocked / this.waitingTicks * 100) + '%'
      },
      isDisabled(){
        if(this.ticksLocked == 0){
          return false;
        }
        else{
          return true;
        }
      }
  }
}
</script>

<template>
<div>
  <button :disabled="isDisabled"  @click="onClick()">Search For Crabs</button>
</div>
</template>

<style scoped>
button{
  background : linear-gradient(to right, blue v-bind('progress'), rgba(0,0,0,0) v-bind('progress'));
}
</style>
