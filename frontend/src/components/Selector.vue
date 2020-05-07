<template>
  <div>
    <div v-if="selected === undefined">
      <button
        class="mdc-button mdc-button--raised foo-button"
        v-for="(category, catName) in data"
        :key="catName"
        v-on:click="selected = catName"
      >
        <span class="mdc-button__label">{{ catName }}</span>
      </button>
    </div>
    <div v-if="selected !== undefined">
      <button
        class="mdc-button mdc-button--raised foo-button"
        v-on:click="selected = undefined"
      >
        <span class="mdc-button__label">Back</span>
      </button>
      <a
        style="all: unset;"
        v-bind:href="'/test/' + category"
        v-for="(category, catName) in data[selected]"
        :key="catName"
      >
        <button class="mdc-button mdc-button--raised foo-button">
          <span class="mdc-button__label">{{ catName }}</span>
        </button>
      </a>
    </div>
  </div>
</template>

<script>
export default {
  name: "Selector",
  props: ["data"],
  data: () => {
    return {
      selected: undefined
    };
  }
};
</script>

<style lang="scss">
@use "@material/button";
@include button.core-styles;

.foo-button {
  padding-top: 20px;
  padding-bottom: 20px;
  @include button.container-fill-color(lightsalmon);
}
</style>
