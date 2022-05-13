<template>
  <form @submit.prevent="onsubmit" class="login">
    <div class="row mb-2">
      <!-- <va-input
        class="mb-3"
        v-model="email"
        type="email"
        :label="$t('auth.email')"
        :error="!!emailErrors.length"
        :error-messages="emailErrors"
      /> -->
    </div>
    <div class="row justify--center">
      <!-- <va-button type="submit" class="my-0">{{ $t('auth.reset_password') }}</va-button> -->
    </div>
  </form>
</template>

<script>
import axios from 'axios';

export default {
  name: 'recover-password',
  data () {
    return {
      email: '',
      emailErrors: [],
    }
  },
  methods: {
    onsubmit () {
      if (!this.email) {
        this.emailErrors = ['Email is required']
      } else {
        axios.post("http://localhost:8000/api/signup",
      {
        "login_type": "Username",
        "login_input": this.email,
        "login_credentials": this.password,
      }).then((data) => {
        console.log(data);
        this.passwordErrors.push(data.data);
      })
      }
    },
  },
}
</script>

<style lang="scss">
</style>
