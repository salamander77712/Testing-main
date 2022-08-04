<script>
export default {
  props: ['waitingTicks', 'successChance', 'ticks'],
  data() {
    return {
      ticksLocked : 0
    }
  },
  methods:{
    onClick(){
      
    }
  },
  computed:{
    buttonText(){
      if (this.ticksLocked == 0) {
        return "Search for crabs";
      }
      else{
        let message = "Searching for crabs"
        for(let i = 0; i < this.ticksLocked % 4; i++){
          message += '.';
        }
        return message;
      }
    },
    buttonClass(){
      if(this.ticksLocked == 0){
        return "active";
      }
      else{
        return "inactive";
      }
    },
    progress(){
      return (String(this.ticksLocked / this.watingTicks * 100) + '%');
    }
  }
}
</script>

<template>
<button :class="buttonClass" @click="onClick()">{{buttonText}}</button>
</template>

<style scoped>
.inactive{
  background : linear-gradient(to right, blue v-bind('progress'), rgba(0,0,0,0) v-bind('progress'));
}
</style>
