<template>
    <div id="post_preview">
        <router-link class="title" :to="'/post/' + post.id">
            <PostTitle v-on:click="open_post" v-bind:title = "post.title"></PostTitle>
        </router-link>
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
    button {
        background-color: #F0E6EF;
        color: #F0A6CA;
        justify-content: flex-start;
        cursor: pointer;
        border: 2px solid #9C89B8;
        border-radius: 3px;
        padding: 10px 32px;
        text-align: center;
        text-decoration: none;
        font-size: 16px;
    }

    button:hover {
        background-color: #9C89B8;
        color: #F0A6CA;
    }
</style>