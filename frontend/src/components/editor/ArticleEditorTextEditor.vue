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
                <button class="publish" v-if="article.title && article.body"
                        @click="$emit('sendPost', { title: article.title, body: JSON.stringify(article.body)})"
                >
                    Опубликовать
                </button>
            </transition>
        </div>
    </transition>
</template>

<script>
    import Quill from 'quill'
    import imageCompressor from '../../../quill.imageCompressor.js'
    let EmbedBlot = Quill.import('blots/block/embed')

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
            class LazyImage extends EmbedBlot {
                static create(data) {
                    const node = super.create()
                    node.setAttribute('src', data.highResImg)
                    node.setAttribute('data-low-res', data.lowResImg)
                    return node
                }
                static value(node) {
                    return {
                        highResUrl: node.getAttribute('src'),
                        lowResUrl: node.getAttribute('data-low-res')
                    }
                }
            }
            LazyImage.blotName = 'lazyImage'
            LazyImage.tagName = 'img'
            Quill.register('modules/imageCompressor', imageCompressor)
            Quill.register('formats/lazyImage', LazyImage)
            this.options = {
                debug: 'warn',
                placeholder: 'Compose an epic...',
                modules: {
                    imageCompressor: {
                        quality: 0.1,
                        imageType: 'image/*'
                    },
                    toolbar: {
                        container: '#toolbar',
                    }
                },
                bounds: '#content',
                theme: 'snow'
            }
            this.editor = new Quill('#editor', this.options)
            let self = this
            this.editor.on('text-change', function () {
                self.article.body = self.editor.getContents()
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
