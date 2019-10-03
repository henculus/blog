<template>
    <transition name="component-load" mode="out-in">
        <div id="content-wrapper">
            <input placeholder="Название статьи" class="title" v-model="title">
            <div id="content">
                <label for="area"></label>
                <textarea ref="area" id="area"></textarea>
            </div>
            <transition name="component-load" mode="out-in">
                <button class="publish" v-if="title" @click.prevent>Опубликовать</button>
            </transition>
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
    @import ../../variables

    .title, .publish
        font-family: 'Comfortaa', sans-serif
        border-radius: 4px

    .title
        padding: 10px
        font-size: 1.8em
        margin-bottom: 20px
        width: 100%
        border: 1px solid lightgray

    .publish
        position: relative
        float: right
        margin-top: 20px
        color: white
        background: $rnt_green
        border: none
        padding: 20px !important


        .CodeMirror
            z-index: 0 !important

            .CodeMirror-fullscreen
                z-index: 10 !important
</style>
