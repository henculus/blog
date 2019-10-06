<template>
    <transition name="component-load" mode="out-in">
        <div id="content-wrapper">
            <input placeholder="Название статьи" class="title" v-model="article.title">
            <div id="content" :class="{ 'disabled': !article.title }">
                <textarea ref="area"></textarea>
            </div>
            <transition name="component-load" mode="out-in">
                <button class="publish" v-if="article.title" @click="publishArticle">Опубликовать</button>
            </transition>
        </div>
    </transition>
</template>

<script>
    import SimpleMDE from 'simplemde'
    import 'simplemde/dist/simplemde.min.css'
    import marked from 'marked'
    import api from "../../api"

    export default {
        name: "ArticleEditorTextEditor",

        data() {
            return {
                simplemde: '',
                article: {
                    title: '',
                    body: '',
                    id: ''
                },
                delayedSave: ''

            }
        },
        mounted() {
            this.createMd()
        },
        computed: {
            htmlText: function () {
                return marked(this.simplemde.value())
            },
            mdText: function () {
                this.saveArticle(this.simplemde.value())
                return this.simplemde.value()
            },
        },


        methods: {
            createMd: async function () {
                if (this.$route.params.id) {
                    await api.getPost(this.$route.params.id).then(
                        response => {
                            this.article.title = response.data.title
                            this.article.body = response.data.body
                            this.article.id = response.data.id
                        }
                    )
                }
                this.simplemde = new SimpleMDE({
                    element: this.$refs.area,
                    spellChecker: false,
                    initialValue: this.article.body
                })
            },
            saveArticle: function (text) {
                this.article.body = text
                clearTimeout(this.delayedSave)
                this.delayedSave = setTimeout(() => {
                    api.sendPost(this.article).then(
                        response => {
                            this.article.id = response.id
                        }
                    )
                }, 1000)
            },
            publishArticle: function () {
                // let id = this.article.id
                // delete this.article.id
                api.patchPost({...this.article, published: true}, this.article.id).then(
                    response => {
                        this.$router.push(`/articles/${this.article.id}`)
                    }
                )
            }
        },
    }
</script>

<style lang="sass">


    .CodeMirror
        z-index: 0 !important

        .CodeMirror-fullscreen
            z-index: 10 !important
</style>

<!--styles for current component-->

<style lang="sass" scoped>
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

</style>
