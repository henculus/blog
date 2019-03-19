<template>
    <div id="post_preview">
        <PostTitle v-bind:title = "post.title"></PostTitle>
        <PostBody v-bind:body = "post.body | truncate(500)"></PostBody>
        <button v-on:click="open_post">Read more...</button>
    </div>
</template>

<script>
    import PostTitle from "@/components/PostTitle";
    import PostBody from "@/components/PostBody";

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
    #post_preview {
        margin: auto auto 4rem;
        max-width: 44rem;
    }
    button {
        cursor: pointer;
        float: left;
        margin-top: 1rem;
        margin-left: 2rem;
        border: 2px solid gray;
        border-radius: 3px;
        padding: 10px 32px;
        text-align: center;
        text-decoration: none;
        display: inline-block;
        font-size: 16px;
    }
</style>