<template>
  <div class="leaflet-map fill-height">
  </div>
</template>

<script>
import 'leaflet-map'
import * as Leaflet from 'leaflet'

export default {
  name: 'leaflet-map',
  data() {
    return {

    }
  },
  mounted () {
    Leaflet.Icon.Default.imagePath = process.env.BASE_URL + 'img/vendor/leaflet/'

    const map = Leaflet.map(this.$el).setView([32.0, 51.0], 13)
    map.on('click', (e) => {
    alert(e.latlng.lat);
    this.$emit('latlngChange', e.latlng);
    } );
    Leaflet.tileLayer('https://{s}.tile.osm.org/{z}/{x}/{y}.png', {
      attribution: '&copy; <a href="https://osm.org/copyright">OpenStreetMap</a> contributors',
    }).addTo(map)
  },
}
</script>

<style lang="scss">
@import "~leaflet/dist/leaflet.css";
</style>
