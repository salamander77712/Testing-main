<script>
export default {
  props: ['millisPerTick'],
  emits: ['tick'],
  mounted() {
    this.go();
  },
  data() {
    return {
    }
  },
  methods: {
    go(){
      setInterval(this._go, this.millisPerTick);
    },
    _go(){
      this.$emit('tick');
      console.log('tick');
    }
  }
}
</script>

<template>
</template>

<style scoped>

</style>
