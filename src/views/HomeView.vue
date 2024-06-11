<template>
  <v-main>
    <v-container>
      <v-row no-gutters>
        <v-col align="center">
          <v-btn @click="btnToSpace()">工作区</v-btn>
        </v-col>
        <v-col align="center">
          <v-btn>设置</v-btn>
        </v-col>
      </v-row>
    </v-container>
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