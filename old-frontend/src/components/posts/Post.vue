<template>
    <div id="post">
        <post-title v-bind:title="post.title"></post-title>
        <post-body v-bind:body="post.body"></post-body>

        <ul v-if="errors && errors.length">
            <li v-for="error of errors" v-bind:key="error">
                {{error.message}}
            </li>
        </ul>
    </div>
</template>

<script>
    import {HTTP} from '@/http-common'
    import PostTitle from "@/components/posts/PostTitle";
    import PostBody from "@/components/posts/PostBody";

    export default {
        name: "Post",
        components: {PostBody, PostTitle},
        data() {
            return {
                post: {title: '', body: ''},
                errors: []
            }
        },

        created() {
            HTTP.get(`/posts/${this.$route.params.id}`)
                .then(
                    response => {
                        this.post = response.data
                    }
                )
                .catch(e => {
                    this.errors.push(e)
                })
        }
    }
</script>

<style scoped>
#post {
    margin: 1rem;
    display: flex;
    flex-direction: column;
    align-items: center;
}
</style>