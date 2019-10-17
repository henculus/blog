<template>
    <div>
        <div class="editor-wrapper">
<!--            <span class="component-name">Редактор</span>-->
            <text-editor></text-editor>
        </div>
    </div>
</template>

<script>
    import TextEditor from './ArticleEditorTextEditor'
    import api from "../../api"
    export default {
        name: "ArticlesEditor",
        components: {TextEditor},
        methods: {
            sendPost: function (data) {
                api.sendPost(data).then(
                    response => console.log('Successfully sent post', response),
                    error => console.error('Error on sending post:', error)
                )
            }
        }
    }
</script>

<style lang="sass" scoped>
    @import "../../variables"
    .editor-wrapper
        height: 100%
        padding: 20px 0
        width: $content-width
        .component-name
            font-size: 2em
</style>
