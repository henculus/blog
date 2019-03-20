<template>
    <div id="post">
        <PostTitle v-bind:title="post.title"></PostTitle>
        <PostBody v-bind:body="post.body"></PostBody>

        <ul v-if="errors && errors.length">
            <li v-for="error of errors" v-bind:key="error">
                {{error.message}}
            </li>
        </ul>
    </div>
</template>

<script>
    import {HTTP} from '@/http-common'
    import PostTitle from "@/components/PostTitle";
    import PostBody from "@/components/PostBody";

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
            HTTP.get(`/api/posts/${this.$route.params.id}`)
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