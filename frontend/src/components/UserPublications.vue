<template>
    <div class="article-feed-wrapper">
        <article-list :articles="articles" :mainFeed = false></article-list>
    </div>
</template>

<script>
    import ArticleList from "./ArticleList"
    import api from "../api"

    export default {
        name: "UserPublications",
        components: {ArticleList},
        data() {
            return{
                articles: undefined
            }
        },
        created() {
            api.getAuthorPosts(this.$store.state.AuthorizationStore.sub).then(
                response => {
                    this.articles = response.data
                }
            )
        }
    }
</script>

<style scoped>

</style>
