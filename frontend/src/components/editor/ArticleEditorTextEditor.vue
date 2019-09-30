<template>
    <transition name="component-load" mode="out-in">
        <div id="content-wrapper">
            <div class="title">{{title}}</div>
            <div id="content">
                <label for="area"></label>
                <textarea ref="area" id="area"></textarea>
            </div>
            <span v-if="simplemde">{{htmlText}}</span>
            <transition name="component-load" mode="out-in" appear>
                <article-title-modal @setTitle="setTitle" v-if="!title"/>
            </transition>
        </div>
    </transition>
</template>

<script>
    import SimpleMDE from 'simplemde'
    import 'simplemde/dist/simplemde.min.css'
    import marked from 'marked'
    import ArticleTitleModal from "./ArticleTitleModal"

    export default {
        name: "ArticleEditorTextEditor",

        components: {
            ArticleTitleModal
        },

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
        padding: 20px 0
    .CodeMirror
        z-index: 0 !important
        .CodeMirror-fullscreen
            z-index: 10 !important
</style>
