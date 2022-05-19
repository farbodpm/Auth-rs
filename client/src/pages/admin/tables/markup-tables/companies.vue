<template>
  <div class="markup-tables flex">
    <va-card :title="$t('tables.basic')" class="flex mb-4">
      <va-card-content>
        <div class="table-wrapper">
          <table class="va-table">
            <thead>
              <tr>
                <th>{{ $t('tables.headings.name') }}</th>
                <th>{{ $t('tables.headings.address') }}</th>
                <th>{{ $t('tables.headings.phone') }}</th>
              </tr>
            </thead>

            <tbody>
              <tr v-for="company in companies" :key="company.id">
                <td>{{ company.name }}</td>
                <td>{{ company.location }}</td>
                <td>{{ company.phone }}</td>

              </tr>
            </tbody>
          </table>          
        </div>
      </va-card-content>
    </va-card>
  </div>
</template>

<script>
import axios from 'axios'
export default {
  data () {
    return {
      companies: [],
    }
  },
  mounted () {
    const BASE_URL = process.env.VUE_APP_APP_BASE_URL;
      console.log(BASE_URL);
      axios.post(BASE_URL + "/api/company/list",
      { user_id_filter: 0 }, {
  headers: {
    // 'application/json' is the modern content-type for JSON, but some
    // older servers may use 'text/json'.
    // See: http://bit.ly/text-json
    'auth': localStorage.getItem('token')
  }
}).then((response) => {this.companies = response.data.companies;});
  },
  methods: {
    getStatusColor (status) {
      if (status === 'paid') {
        return 'success'
      }

      if (status === 'processing') {
        return 'info'
      }

      return 'danger'
    },
  },
}
</script>

<style lang="scss">
  .markup-tables {
    .table-wrapper {
      overflow: auto;
    }

    .va-table {
      width: 100%;
    }
  }
</style>
