<template>
    <transition name="component-load" mode="out-in">
        <div id="content-wrapper">
            <div id="content">
                <label for="area">
                </label><textarea ref="area" id="area"></textarea>
            </div>
            <span v-if="simplemde">{{htmlText}}</span>
        </div>
    </transition>
</template>

<script>
    import SimpleMDE from 'simplemde'
    import 'simplemde/dist/simplemde.min.css'
    import marked from 'marked'

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
        computed: {
            htmlText: function () {
                return marked(this.simplemde.value())
            }
        }
    }
</script>

<style lang="sass">
    .CodeMirror
        z-index: 0 !important
</style>
