<template>
    <div class="article-feed-wrapper">
        <article-list :articles="articles" :mainFeed = true></article-list>
    </div>
</template>

<script>
    import ArticleList from "./ArticleList"
    import api from "../api"

    export default {
        name: "ArticleFeed",
        components: {ArticleList},
        data() {
            return{
                articles: undefined
            }
        },
        created() {
            this.getPosts()
        },
        watch: {
          '$store.state.AuthorizationStore.sub': function () {
              this.getPosts()
              console.log('User data updated')
          }
        },
        methods:{
            getPosts: function () {
                api.getPosts().then(
                    response => {
                        this.articles = response.data
                    }
                )
            }
        }
    }
</script>

<style lang="sass" scoped>
    .article-feed-wrapper
        width: 100%
</style>
