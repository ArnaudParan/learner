import Vue from "vue";
import Test from "./Test.vue";
import VueHotkey from "v-hotkey";

Vue.config.productionTip = false;

Vue.use(VueHotkey);

new Vue({
  render: h => h(Test)
}).$mount("#app");
