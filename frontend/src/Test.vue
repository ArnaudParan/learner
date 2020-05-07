<template>
  <div id="app">
    <Test-Panel
      ref="panel"
      v-bind:data="testData"
      v-on:data-validated="onValidation"
    />
  </div>
</template>

<script>
import TestPanel from "./components/Test-Panel.vue";

export default {
  name: "Test",
  components: {
    TestPanel
  },
  methods: {
    fetchData: function() {
      const req = new XMLHttpRequest();
      const component = this; // eslint-disable-line @typescript-eslint/no-this-alias
      req.onreadystatechange = function() {
        if (this.readyState == 4 && this.status == 200) {
          component.testData = JSON.parse(this.responseText).data;
        }
      };
      req.open("GET", "/api/test?testid=" + String(this.testId), true);
      req.setRequestHeader("Content-Type", "application/json");
      req.send();
    },
    sendData: function(data) {
      const req = new XMLHttpRequest();
      req.open("PUT", "/api/test_result", true);
      req.setRequestHeader("Content-Type", "application/json");
      req.send(JSON.stringify({testid: this.testId, val: data}));
    },
    onValidation: function(data) {
      this.sendData(data);
      this.fetchData();
    }
  },
  data: function() {
    return {
      testData: []
    };
  },
  computed: {
    testId: function() {
      return parseInt(window.location.href.split("/").pop());
    }
  },
  created: function() {
    this.fetchData();
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
