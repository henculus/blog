<template>
    <div class="article-list-wrapper">
        <div class="article-block-name" v-if="mainFeed">Последние статьи</div>
        <div class="article-block-name" v-else>Ваши статьи</div>
        <transition name="component-load" mode="out-in">
            <component-loading v-if="!this.articles"></component-loading>
            <div class="article-list" v-else>
                <article-list-item v-for="article in articles" :key="article.id"
                                   :article="article"></article-list-item>
            </div>
        </transition>
    </div>
</template>

<script>
    import ComponentLoading from "./ComponentLoading"
    import ArticleListItem from "./ArticleListItem"

    export default {
        name: "ArticleList",
        components: {
            ArticleListItem,
            ComponentLoading
        },
        props: {
            articles: Array,
            mainFeed: Boolean
        },
    }
</script>

<style lang="sass" scoped>
    @import "../variables"
    .article-list-wrapper
        display: flex
        flex-direction: column
        margin: 0 auto
        width: 100%
        padding: $content-padding-mobile

        .article-block-name
            +deselect
            font-size: 1.8em
            padding-bottom: 20px

        .article-list
            //width: 50%
            position: relative
            display: block

    +media_screensize_mobile
        .article-list-wrapper
            width: $content-width !important
            padding: 20px 0

            .article-block-name
                font-size: 2em
</style>
