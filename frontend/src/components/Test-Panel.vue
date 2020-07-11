<template>
  <div id="panel">
    <div class="question" v-if="0 in data">
      {{ data[0].val }}
    </div>
    <div class="question" v-else>
      Waiting for data ...
    </div>
    <div>
      <span class="answer" v-for="(item, index) in data.slice(1)" :key="index">
        <Test-Answer
          ref="answer"
          v-on:ok="cache[index] = true; cache.push()"
          v-on:error="cache[index] = false; cache.push()"
          v-bind:data="item"
        />
      </span>
    </div>
    <div>
      <button
        class="mdc-button mdc-button--raised"
        v-on:click="$emit('data-validated', validationData())"
        v-bind:disabled="!filled"
      >
        <span class="mdc-button__label">Continue</span>
      </button>
    </div>
  </div>
</template>

<script>
import TestAnswer from "./Test-Answer.vue";

export default {
  name: "TestPanel",
  components: {
    TestAnswer
  },
  props: ["data"],
  data: function() {
    return {
      cache: []
    };
  },
  computed: {
    filled: function() {
      return this.cache.filter(x => x !== undefined).length === this.data.length - 1;
    }
  },
  watch: {
    data: function(newVal, oldVal) {
      this.reset();
    }
  },
  methods: {
    reset: function() {
      if (Array.isArray(this.$refs.answer)) {
        this.$refs.answer.forEach(function(item) {
          item.reset();
        });
      }
      else if (this.$refs.answer !== undefined) {
        this.$refs.answer.reset();
      }
      this.cache = [];
    },
    validationData: function() {
      return this.data.slice(1).map((val, id) => {
        return { label: val.label, val: val.val, validAnswer: this.cache[id] };
      });
    }
  }
};
</script>

<style>
.question {
  font-size: xx-large;
}

.answer {
  display: inline-block;
  vertical-align: top;
  margin: 30px;
  width: 20%;
}
</style>
