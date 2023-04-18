<script setup lang="ts">
  import test from "./sidebar.ts"
</script>

<template>
  <div class="item">
    <i onclick="test()">
      <slot name="icon"></slot>
    </i>
  </div>
</template>










<style scoped>
.item {
  margin-top: 2rem;
  display: flex;
  background: black;
      justify-content: center;
}

.details {
  flex: 1;
  margin-left: 1rem;
}

i {
  display: flex;
  place-items: center;
  place-content: center;
  width: 32px;
  height: 32px;

  color: var(--color-text);
}

h3 {
  font-size: 1.2rem;
  font-weight: 500;
  margin-bottom: 0.4rem;
  color: var(--color-heading);
}

</style>
