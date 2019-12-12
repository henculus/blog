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
                    <section class="article-content">
                        <div class="article-content-element"
                             v-for="(element, key) in articleBody.ops"
                             :key="key">
                            <p class="paragraph"
                               v-if="!element.insert.hasOwnProperty('lazyImage')"
                               :style="isBold(element) ? 'font-weight: 700' : 'font-weight: 400'"
                               v-html="element.insert.replace(/\n/g, '<br />').replace(/ /g, '&nbsp;')"
                            >

                            </p>
                            <lazy-image
                                    v-if="element.insert.hasOwnProperty('lazyImage')"
                                    :img-padding="56.25"
                                    :low-res-img-path="element.insert.lazyImage.lowResUrl"
                                    :high-res-img-path="element.insert.lazyImage.highResUrl"
                            ></lazy-image>
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

    export default {
        name: "ArticleComponent",
        components: {
            LazyImage,
            ComponentLoading,
        },
        data() {
            return {
                article: {},
                isLoading: true,
            }
        },
        computed: {
            articleBody : function () {
                return JSON.parse(this.article.body)
            }
        },
        methods: {
            isBold: function (str_element) {
                if (str_element.hasOwnProperty('attributes')) {
                    return !!str_element.attributes.bold === true
                } else
                    return false
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
