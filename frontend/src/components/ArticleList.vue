<template>
    <div class="article-list-wrapper">
        <div class="article-block-name">Последние статьи</div>
        <transition name="component-load" mode="out-in">
            <component-loading v-if="isLoading"></component-loading>
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
    import {HTTP} from "../server_defaults"

    export default {
        name: "ArticleList",
        components: {
            ArticleListItem,
            ComponentLoading
        },
        data() {
            return {
                isLoading: true,
                articles: []
            }
        },
        mounted() {
            HTTP.get('/posts', {withCredentials: true}).then(
                response => {
                    this.isLoading = false
                    this.articles = response.data
                }
            )
        }
    }
</script>

<style lang="sass" scoped>
    @import "../variables"
    .article-list-wrapper
        display: flex
        flex-direction: column
        margin: 0 auto
        padding: $content-padding-mobile
        width: 100%

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
            position: relative
            width: $content-width
            padding: 20px 0

            .article-block-name
                font-size: 2em
</style>
