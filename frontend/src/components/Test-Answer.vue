<template>
  <div>
    <button
      v-if="!revealed"
      v-on:click="revealed = true"
      class="line button data-button mdc-button mdc-button--raised"
    >
      <span class="mdc-button__label">reveal</span>
    </button>
    <button
      v-if="revealed"
      class="line button data-button mdc-button mdc-button--raised"
    >
      <span class="mdc-button__label">{{ data.val }}</span>
    </button>
    <div class="line">
      <button
        v-if="revealed"
        v-bind:disabled="validated"
        v-on:click="
          validated = true;
          $emit('ok');
        "
        class="ok-button mdc-button mdc-button--raised button"
      >
        <span class="mdc-button__label">ok</span>
      </button>
      <button
        v-if="revealed"
        v-bind:disabled="validated"
        v-on:click="
          validated = true;
          $emit('error');
        "
        class="error-button mdc-button mdc-button--raised button"
      >
        <span class="mdc-button__label">error</span>
      </button>
    </div>
  </div>
</template>

<script>
export default {
  name: "TestAnswer",
  props: ["data"],
  data: function() {
    return {
      revealed: false,
      validated: false
    };
  },
  methods: {
    reset: function() {
      this.revealed = false;
      this.validated = false;
    }
  }
};
</script>

<style lang="scss">
@use "@material/button";
@include button.core-styles;

.mdc-button__label {
  font-size: xx-large;
}

.line {
  width: 100%;
  margin-top: 10px;
  margin-bottom: 10px;
}

.button {
  height: 3em;
}

.data-button {
  @include button.container-fill-color(lightgray);
  @include button.ink-color(black);
}

.reveal-button {
  width: 100%;
  @include button.container-fill-color(cyan);
  @include button.ink-color(black);
}

.ok-button {
  width: 50%;
  @include button.container-fill-color(lawngreen);
  @include button.ink-color(black);
}

.error-button {
  width: 50%;
  @include button.container-fill-color(crimson);
  @include button.ink-color(black);
}
</style>
