<template>
    <div class="article-feed-wrapper">
        <article-list :main-feed="false"></article-list>
    </div>
</template>

<script>
    import ArticleList from "./ArticleList"
    export default {
        name: "UserPublications",
        components: {ArticleList}
    }
</script>

<style scoped>

</style>
