<template>
  <div class="col-md-6 offset-md-3 mt-3">
    <div class="card mt-2">
      <h4 class="card-header">Login</h4>
      <div class="card-body">

        <Form @submit="check" :validation-schema="schema" v-slot="{ errors, loading }">
          <div class="form-group">
            <label for="email">Email</label>
            <Field
              id="email"
              name="email"
              type="email"
              autocomplete="on"
              class="form-control"
              :class="{ 'is-invalid': errors.email }"
            />
            <div class="invalid-feedback">{{ errors.email }}</div>
          </div>

          <div class="form-group">
            <button :disabled="loading" class="btn btn-primary mt-4">
              <span v-if="loading" class="spinner-border spinner-border-sm mr-1"></span>
              Check
            </button>
          </div>

          <div v-if="errors.apiError" class="alert alert-danger mt-3 mb-0">{{ errors.apiError }}</div>
        </Form>

        <div v-if="response.message" :class="['alert', response.status ? 'alert-success' : 'alert-danger', 'mt-3']">
          {{ response.message }}
        </div>

      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';
import * as yup from 'yup';
import { mailService } from '@/services';


const loading = ref(false);
const message = ref('');
const response = ref('');

const schema = yup.object().shape({
  email: yup.string().required('Email is required!')
});

const check = (data, { resetForm }) => {
  response.value = { status: false, message: '' };
  loading.value = true;

  mailService.check(data).then(
    (res) => {
      response.value = res;
      console.log(res);
      loading.value = false;
      // resetForm();
    },
    (error) => {
      loading.value = false;
      message.value =
        (error.response && error.response.data && error.response.data.message) ||
        error.message ||
        error.toString();
    }
  );
}
</script>

<style scoped>

</style>
