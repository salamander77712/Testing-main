<script setup>
import CrabButton from './components/CrabButton.vue'
import TickTimer from './components/TickTimer.vue'
import MrKrabs from './components/MrKrabs.vue'
</script>

<template>
<MrKrabs @tick="updateTicks()"></MrKrabs>
</template>

<style scoped>

</style>
