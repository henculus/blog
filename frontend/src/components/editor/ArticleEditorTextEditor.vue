<template>
    <transition name="component-load" mode="out-in">
        <div id="content-wrapper">
            <label for="article_title"></label>
            <input placeholder="Название статьи" class="title" v-model="article.title" id="article_title">
            <div id="content" :class="{ 'disabled': !article.title }">
                <div id="toolbar">
                    <button class="ql-bold"></button>
                    <button class="ql-image"></button>
                </div>
                <!-- Create the editor container -->
                <div id="editor">
                </div>
            </div>
            <transition name="component-load" mode="out-in">
                <button class="publish" v-if="article.title && article.body">Опубликовать
                </button>
            </transition>
            {{delta}}
        </div>
    </transition>
</template>

<script>
    import Quill from 'quill'

    export default {
        name: "ArticleEditorTextEditor",

        data() {
            return {
                article: {
                    title: '',
                    body: '',
                    id: '',
                    published: false
                },
                editor: null,
                options: null,
                delta: null
            }
        },
        mounted() {
            this.options = {
                debug: 'info',
                placeholder: 'Compose an epic...',
                modules: {
                    toolbar: {
                        container: '#toolbar',
                        handlers: {
                            'image': function (value) {
                                console.log('IMAGE VALUE:', value)
                            }
                        }
                    }
                },
                bounds: '#content',
                theme: 'snow'
            }
            this.editor = new Quill('#editor', this.options)
            let self = this
            this.editor.on('text-change', function () {
                self.delta = self.editor.getContents()
            })
        },
        computed: {},


        methods: {}
    }
</script>
<!--styles for current component-->

<style lang="sass" scoped>
    @import '~quill/dist/quill.core.css'
    @import '~quill/dist/quill.bubble.css'
    @import '~quill/dist/quill.snow.css'
    @import ../../variables

    .disabled
        pointer-events: none
        opacity: 0.4

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
        transition: all .2s ease-out

        &:hover
            background: #00d777

        &:active
            background: #00c362

</style>
