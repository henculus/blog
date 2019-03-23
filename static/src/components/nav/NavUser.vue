<template>
    <router-link id="link" :to="`/auth`">
        <p id="">Login</p>
    </router-link>
</template>

<script>
    export default {
        name: "NavUser"
    }
</script>

<style scoped>
    #link {
        margin-left: auto;
    }
    #link p {
        margin: 1rem;
    }
</style>