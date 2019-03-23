<template>
    <div id="posts">
        <post-preview
                v-for = "post in posts"
                v-bind:key = "post.id"
                v-bind:post = "post"
        ></post-preview>

        <ul v-if="errors && errors.length">
            <li v-for="error of errors" v-bind:key="error">
                {{error.message}}
            </li>
        </ul>
    </div>
</template>

<script>
    import {HTTP} from '@/http-common';
    import PostPreview from "@/components/posts/PostPreview";

    export default {
        name: "Posts",
        components: {PostPreview},
        data() {
            return {
                posts: [],
                errors: []
            }
        },

        created() {
            HTTP.get('/api/posts')
                .then(
                    response => {
                        this.posts = response.data
                    }
                )
                .catch(e => {
                    this.errors.push(e)
                })
        }
    }
</script>

<style scoped>
    #posts {
        margin: 1rem;
    }
    #posts > ul {
        list-style-type: none;
    }
</style>