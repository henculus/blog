<template>
    <transition name="component-load" mode="out-in">
        <div id="content-wrapper">
            <div id="content">
                <textarea ref="area" id="area"></textarea>
            </div>
        </div>
    </transition>
</template>

<script>
    import SimpleMDE from 'simplemde'
    import 'simplemde/dist/simplemde.min.css'

    export default {
        name: "ArticleEditorTextEditor",
        data() {
            return {
                simplemde: ''
            }
        },
        mounted() {
            this.simplemde = new SimpleMDE({element: this.$refs.area, spellChecker: false})
        },
        methods: {},
    }
</script>

<style lang="sass" scoped>

    #content-wrapper
        #content
            .CodeMirror
                z-index: 0 !important
</style>
