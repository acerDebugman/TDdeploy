<script setup>
import FancyButton from './FancyButton.vue'
import AwesomeIcon from './AwesomeIcon.vue'
import BaseLayout from './BaseLayout.vue'
</script>

<template>
  <FancyButton>
    Click me
 	</FancyButton>
  <FancyButton>
    <span style="color:red">Click me! </span>
    <AwesomeIcon />
  </FancyButton>


  <BaseLayout>
    <template #header>
      <h1>Here might be a page title</h1>
    </template>

    <template #default>
      <p>A paragraph for the main content.</p>
      <p>And another one.</p>
    </template>

    <template #footer>
      <p>Here's some contact info</p>
    </template>
  </BaseLayout>
</template>