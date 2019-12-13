<template>
    <transition name="component-load" mode="out-in">
        <component-loading v-if="isLoading"></component-loading>
        <div v-else id="content-wrapper">
            <div id="top-card-wrapper">
                <lazy-image :img-padding="25" :low-res-img-path="`https://i.imgur.com/w9sSofn.jpg`"
                            :high-res-img-path="`https://i.imgur.com/SkWrPcM.jpg`"></lazy-image>
            </div>
            <div id="content">
                <article class="article">
                    <section class="headline">
                        <div class="title">{{article.title}}</div>
                        <div class="subtitle">{{article.subtitle}}</div>
                        <div class="author">{{article.author}}</div>
                    </section>
                    <section class="article-content"
                             v-if="isIncludeImage"
                    >
                        <div class="article-content-element"
                             v-for="(element, key) in imageIndices"
                             :key="key"
                        >
                               <p class="paragraph"
                                   v-if="element > 0"
                                   v-html="deltaToHtml(articleBody.ops.slice(imageIndices[key - 1], element))"
                                >
                                </p>
                                <lazy-image
                                        :img-padding="56.25"
                                        :low-res-img-path="articleBody.ops[element].insert.lazyImage.lowResUrl"
                                        :high-res-img-path="articleBody.ops[element].insert.lazyImage.highResUrl"
                                ></lazy-image>
                                <p class="paragraph"
                                   v-if="element === 0 && imageIndices.length === 1"
                                   v-html="deltaToHtml(articleBody.ops.slice(element + 1, imageIndices[key+1]))"
                                >
                                </p>
                                <p class="paragraph"
                                   v-if="element === imageIndices[imageIndices.length - 1]
                                   && element !== articleBody.ops[articleBody.ops.length - 1] && element !== 0"
                                   v-html="deltaToHtml(articleBody.ops.slice(element))"
                                >
                                </p>
                        </div>
                    </section>
                    <section class="article-content"
                             v-if="!isIncludeImage"
                    >
                        <p class="paragraph"
                                v-html="deltaToHtml(articleBody.ops)"
                        >
                        </p>
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
            isIncludeImage: function () {
                for (let op of this.articleBody.ops) {
                    if (op.insert.hasOwnProperty('lazyImage')) {
                        return true
                    }
                }
                return false
            },
            imageIndices: function () {
                let imageCoords = []
                for (let op of this.articleBody.ops) {
                    if (op.insert.hasOwnProperty('lazyImage')) {
                            imageCoords.push(this.articleBody.ops.indexOf(op))
                    }
                }
                return imageCoords
            },
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
