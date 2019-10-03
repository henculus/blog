<template>
    <transition name="component-load" mode="out-in">
        <div id="content-wrapper">
            <input placeholder="Название статьи" class="title">{{title}}</input>
            <div id="content">
                <label for="area"></label>
                <textarea ref="area" id="area"></textarea>
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
                simplemde: '',
                title: ''
            }
        },
        mounted() {
            this.simplemde = new SimpleMDE({element: this.$refs.area, spellChecker: false})
        },
        computed: {
            htmlText: function () {
                return marked(this.simplemde.value())
            }
        },

        methods: {
            setTitle: function (title) {
                this.title = title
            }
        }
    }
</script>

<style lang="sass">
    .title
        font-size: 1.8em
        font-family: 'Comfortaa', sans-serif
        padding: 10px
        margin-bottom: 20px
        width: 100%
        border-radius: 4px
        border: 1px solid lightgray
    .CodeMirror
        z-index: 0 !important
        .CodeMirror-fullscreen
            z-index: 10 !important
</style>
