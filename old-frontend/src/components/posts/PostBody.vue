<template>
    <p id="body">{{body}}</p>
</template>

<script>
    export default {
        props: ['body'],
        name: "PostBody"
    }
</script>

<style scoped>
    #body {
        text-align: justify;
        max-width: 40rem;
    }
</style>