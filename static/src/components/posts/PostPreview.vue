<template>
    <div id="post_preview">
        <router-link class="title" :to="'/post/' + post.id">
            <PostTitle v-on:click="open_post" v-bind:title = "post.title"></PostTitle>
        </router-link>
        <post-body v-bind:body = "post.body | truncate(500)"></post-body>
        <button v-on:click="open_post">Read more...</button>
    </div>
</template>

<script>
    import PostTitle from "@/components/posts/PostTitle";
    import PostBody from "@/components/posts/PostBody";

    export default {
        name: "PostPreview",
        components: {PostTitle, PostBody},
        props: ['post'],
        methods: {
            open_post: function () {
                this.$router.push(`/post/${this.post.id}`)
            }
        },
        filters: {
            truncate: function (value, limit) {
                if (value.length > limit) {
                    value = value.substring(0, (limit - 3)) + '...';
                }
                return value
            }
        }
    }
</script>

<style scoped>
    .title {
        color: #9C89B8;
        text-decoration: none;
    }

    .title:hover {
        color: #F0A6CA;
    }

    #post_preview {
        margin: auto auto 4rem;
        max-width: 44rem;
        display: flex;
        flex-direction: column;
        align-items: flex-start;
    }
</style>