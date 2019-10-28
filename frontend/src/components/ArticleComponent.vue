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
                        <p class="paragraph">{{article.body}}</p>
                    </section>
                    <lazy-image :img-padding="56.25" :low-res-img-path="`https://i.imgur.com/xK5T9H0.jpg`"
                                :high-res-img-path="`https://i.imgur.com/MOhedrY.jpg`"></lazy-image>
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
