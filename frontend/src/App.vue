<template>
  <div id="app">
    <Selector v-bind:data="selection" />
  </div>
</template>

<script>
import Selector from "./components/Selector.vue";

export default {
  name: "App",
  components: {
    Selector
  },
  data: () => {
    return {
      selection: {
        Japanese: {
          Vocabulary: 1,
          Grammar: 2
        },
        "Ancient Greek": {
          Vocabulary: 3,
          Grammar: 4
        }
      }
    };
  }
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
