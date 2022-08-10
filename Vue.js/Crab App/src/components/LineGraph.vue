<script>
import { Line } from 'vue-chartjs';
import {
  Chart as ChartJS,
  Title,
  Tooltip,
  Legend,
  LineElement,
  LinearScale,
  PointElement,
  CategoryScale
} from 'chart.js';
ChartJS.register(
  Title,
  Tooltip,
  Legend,
  LineElement,
  LinearScale,
  PointElement,
  CategoryScale
);
export default {
  props: ['values', 'labels', 'title', 'min', 'max'],
  components: { Line },
  data() {
    return {
    }
  },
  computed: {
      chartData() { return {
        labels: this.labels,
        datasets: [{
          label: this.title,
          data: this.copy(this.values),
          fill: false,
          borderColor: 'rgb(75, 192, 192)',
          tension: 0.1
        }]
      } },
      chartOptions(){ return{
        scales: {
            y: {
                suggestedMin: this.min,
                suggestedMax: this.max
            }
        }
    }
      }
    },
  methods:{
    copy(array){
      let identity = (x) => x;
      return array.map(identity);
    }
  }
}
</script>

<template>

<Line 
:chart-data = "this.chartData"
:chart-options = "this.chartOptions"
/>

</template>

<style scoped>

</style>
