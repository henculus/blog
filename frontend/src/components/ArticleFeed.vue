<template>
    <div class="article-feed-wrapper">
        <article-list :main-feed="true"></article-list>
    </div>
</template>

<script>
    import ArticleList from "./ArticleList"

    export default {
        name: "ArticleFeed",
        components: {ArticleList}
    }
</script>

<style lang="sass" scoped>
    .article-feed-wrapper
        width: 100%
</style>
