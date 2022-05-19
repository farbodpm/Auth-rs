<template>
  <form @submit.prevent="onsubmit">
    <va-input
      class="mb-3"
      v-model="username"
      :label="$t('auth.username')"
      :error="!!usernameErrors.length"
      :error-messages="usernameErrors"
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
      username: '',
      password: '',
      keepLoggedIn: false,
      usernameErrors: [],
      passwordErrors: [],
    }
  },
  computed: {
    formReady () {
      return !this.usernameErrors.length && !this.passwordErrors.length
    },
  },
  methods: {
    onsubmit () {
      this.usernameErrors = this.username ? [] : ['username is required']
      this.passwordErrors = this.password ? [] : ['Password is required']
      const BASE_URL = process.env.VUE_APP_APP_BASE_URL;
      console.log(BASE_URL);
      axios.post(BASE_URL + "/api/login",
      {
        "login_type": "Username",
        "login_input": this.username,
        "login_credentials": this.password,
      }).then((data) => {
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
