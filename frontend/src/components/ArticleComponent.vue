<template>
    <transition name="component-load" mode="out-in">
        <component-loading v-if="isLoading"/>
        <div v-else id="content-wrapper">
            <div id="top-card-wrapper">
                <lazy-image :img-padding="25" :low-res-img-path="`https://i.imgur.com/w9sSofn.jpg`"
                            :high-res-img-path="`https://i.imgur.com/SkWrPcM.jpg`"/>
            </div>
            <div id="content">
                <article class="article">
                    <section class="headline">
                        <div class="title">{{article.title}}</div>
                        <div class="subtitle">{{article.subtitle}}</div>
                        <div class="author">{{article.author}}</div>
                    </section>
                    <section class="article-content">
                        <div class="article-content-element"
                             v-for="(element, key) in deltaToNewFormat"
                             :key="key"
                        >
                            <p class="paragraph"
                               v-if="element"
                               v-html="deltaToHtml(element)"
                            >
                            </p>
                            <lazy-image
                                v-if="element.hasOwnProperty('insert')"
                                :img-padding="56.25"
                                :low-res-img-path="element.insert.lazyImage.lowResUrl"
                                :high-res-img-path="element.insert.lazyImage.highResUrl"
                            >
                            </lazy-image>
                        </div>
                    </section>
                </article>

            </div>
        </div>
    </transition>
</template>

<script>
    import api from "../api"
    import ComponentLoading from "./ComponentLoading"
    import LazyImage from "./LazyImage"
    import {QuillDeltaToHtmlConverter} from 'quill-delta-to-html'

    export default {
        name: "ArticleComponent",
        components: {
            LazyImage,
            ComponentLoading,
        },
        data() {
            return {
                article: {body: {ops: [{insert: {lazyImage: {highResUrl: '', lowResUrl: ''}}}]}},
                isLoading: true,
            }
        },
        computed: {
            articleBody: function () {
                return JSON.parse(this.article.body)
            },
            deltaToNewFormat: function () {
                let new_format = []
                let textArr = []
                for (let op of this.articleBody.ops) {
                    if (op.insert.hasOwnProperty('lazyImage')) {
                        if (textArr.length > 0) {
                            new_format.push(textArr)
                            textArr = []
                        }
                        new_format.push(op)
                    } else {
                        textArr.push(op)
                    }
                }
                new_format.push(textArr)
                return new_format
            }
        },
        methods: {
            deltaToHtml: function (ops_arr) {
                const deltaConverter = new QuillDeltaToHtmlConverter(ops_arr)
                return deltaConverter.convert()
            }
        },
        mounted: function () {
            api.getPost(this.$route.params.id).then(
                response => {
                    this.article = response.data
                    this.isLoading = false
                }
            )
            this.$route.params
        }
    }
</script>

<style scoped lang="sass">
    @import "../article_style"
    @import "../variables.sass"
    #top-card-wrapper
        min-width: 800px
</style>
