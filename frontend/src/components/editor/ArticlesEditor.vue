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
    export default {
        name: "ArticlesEditor",
        components: {TextEditor}
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
