<template>
  <form @submit.prevent="onsubmit">
    <va-input
      class="mb-3"
      v-model="email"
      type="email"
      :label="$t('auth.email')"
      :error="!!emailErrors.length"
      :error-messages="emailErrors"
    />

    <va-input
      class="mb-3"
      v-model="password"
      type="password"
      :label="$t('auth.password')"
      :error="!!passwordErrors.length"
      :error-messages="passwordErrors"
    />

    <div class="auth-layout__options d-flex align--center justify--space-between">
      <va-checkbox v-model="keepLoggedIn" class="mb-0" :label="$t('auth.keep_logged_in')"/>
      <router-link class="ml-1 link" :to="{name: 'recover-password'}">{{$t('auth.recover_password')}}</router-link>
    </div>

    <div class="d-flex justify--center mt-3">
      <va-button @click="onsubmit" class="my-0">{{ $t('auth.login') }}</va-button>
    </div>
  </form>
</template>

<script>
import axios from 'axios';

export default {
  name: 'login',
  data () {
    return {
      email: '',
      password: '',
      keepLoggedIn: false,
      emailErrors: [],
      passwordErrors: [],
    }
  },
  computed: {
    formReady () {
      return !this.emailErrors.length && !this.passwordErrors.length
    },
  },
  methods: {
    onsubmit () {
      
      this.emailErrors = this.email ? [] : ['Email is required']
      this.passwordErrors = this.password ? [] : ['Password is required']
      axios.post("http://localhost:8000/api/login",
      {
        "login_type": "Username",
        "login_input": this.email,
        "login_credentials": this.password,
      }).then((data) => {
        console.log(data);
        if (data.data == "Now you are logged in") {
          this.$router.push({ name:"dashboard"});
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
