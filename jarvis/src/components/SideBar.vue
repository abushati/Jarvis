<script setup lang="ts">
import WelcomeItem from './WelcomeItem.vue'
import DocumentationIcon from './icons/IconDocumentation.vue'
import ToolingIcon from './icons/IconTooling.vue'
import EcosystemIcon from './icons/IconEcosystem.vue'
import CommunityIcon from './icons/IconCommunity.vue'
import SupportIcon from './icons/IconSupport.vue'
import SideBarElement from './SideBarElement.vue'
import SideBar from "./sidebar.ts"
</script>

<template>
  <div id="sidebar" style="display: flex;flex-direction: column;width: 10%;height: 100%;">
    <SideBarElement>
      <template #icon>
        <DocumentationIcon />
      </template>
    </SideBarElement>
    <SideBarElement>
      <template #icon>
        <DocumentationIcon />
      </template>
    </SideBarElement>
    <SideBarElement>
      <template #icon>
        <DocumentationIcon />
      </template>
    </SideBarElement>
  </div>
</template>

<style scoped>

#sidebar {
  background-color: aqua;
}

</style>
