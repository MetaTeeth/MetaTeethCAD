<template>
  <v-main class="d-flex align-center justify-center">
    <p>Hello World</p>
    <v-btn @click="btnToSpace()">
      Button
    </v-btn>
  </v-main>
</template>

<script>
export default {
  methods: {
    btnToSpace() {
      this.$router.push("/space");
    }
  }
}
</script>