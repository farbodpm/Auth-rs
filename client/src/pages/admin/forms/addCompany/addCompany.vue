<template>
<div class="markup-tables flex">
    <va-card :title="$t('tables.basic')" class="flex mb-4">
      <va-card-content>
  <form @submit.prevent="onsubmit">
    <va-input
      class="mb-3"
      v-model="name"
      :label="$t('company.name')"
      :error="!!nameErrors.length"
      :error-messages="nameErrors"
    />

    <va-input
      class="mb-3"
      v-model="address"
      :label="$t('company.address')"
      :error="!!addressErrors.length"
      :error-messages="addressErrors"
    />

    <va-input
      class="mb-3"
      v-model="phone"
      :label="$t('company.phone')"
      :error="!!phoneErrors.length"
      :error-messages="phoneErrors"
    />

    <va-card
          class="leaflet-maps-page__widget"
          title="Leaflet Maps"
        >
        <leaflet-map style="height: 65vh;" @latlngChange="latlngChange" />
    </va-card>
    <div class="d-flex justify--center mt-3">
      <va-button @click="onsubmit" class="my-0">{{ $t('company.add') }}</va-button>
    </div>
  </form>
   </va-card-content>
    </va-card>
  </div>
</template>

<script>
import axios from 'axios';
import LeafletMap from '../../maps/leaflet-maps/LeafletMap'
 
export default {
  name: 'addCompany',
  components: {
    LeafletMap,
  },
  data () {
    return {
      name: '',
      address: '',
      lat: '',
      lng : '',
      phone : '',
      nameErrors: [],
      addressErrors: [],
      phoneErrors: []
    }
  },
  computed: {
    formReady () {
      return !this.nameErrors.length && !this.addressErrors.length
    },
  },
  methods: {
      latlngChange(e){
          this.lat = String(e.lat);
          this.lng = String(e.lng);
          console.log(this.lat);
      },
    onsubmit () {
      this.nameError = this.name ? [] : ['name is required']
      this.addressErrors = this.address ? [] : ['address is required']

      const BASE_URL = process.env.VUE_APP_APP_BASE_URL;
      console.log(BASE_URL);
      axios.post(BASE_URL + "/api/company/create",
      {
        name : this.name, 
        address :  this.address, 
        lat : this.lat,  
        lng :  this.lng, 
        phone  :  this.phone,

      },{
      headers: {
    // 'application/json' is the modern content-type for JSON, but some
    // older servers may use 'text/json'.
    // See: http://bit.ly/text-json
    'auth': localStorage.getItem('token')
  }}).then((data) => {
        if (data.data.message == "Now you are logged in") {
          this.$router.push({ name:"dashboard"});
          console.log(data.data.token)
          localStorage.setItem("token",data.data.token);
        }
        else{
        this.passwordErrors.push(data.data);
        }
      })
      if (!this.formReady) {
        return
      }
    },
  },
}
</script>