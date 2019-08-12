<template>
    <transition name="component-load" mode="out-in">
        <component-loading v-if="isLoading"></component-loading>
        <div v-else id="content-wrapper">
            <div id="top-card-wrapper">
                <lazy-image :img-padding="30" :low-res-img-path="`https://i.imgur.com/QWbnFgn.jpg`" :high-res-img-path="`https://i.imgur.com/cpo8HZe.jpg`"></lazy-image>
            </div>
            <div id="content">
                <article class="article">
                    <section class="headline">
                        <div class="title">{{article.title}}</div>
                        <div class="subtitle">{{article.subtitle}}</div>
                        <div class="author">{{article.author}}</div>
                    </section>
                    <section class="article-content">
                        {{article.body}}
                    </section>
                </article>

            </div>
        </div>
    </transition>
</template>

<script>
    import {HTTP} from '../server_defaults'
    import ComponentLoading from "./ComponentLoading"
    import LazyImage from "./LazyImage"

    export default {
        name: "ArticleComponent",
        components: {
            LazyImage,
            ComponentLoading
        },
        data() {
            return {
                article: {},
                isLoading: true,
            }
        },
        mounted: function () {
            HTTP.get(`/posts/${this.$route.params.id}`, {withCredentials: true}).then(
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
    @import "../variables.sass"

    #content-wrapper
        display: flex
        flex-direction: column
        align-items: center
        width: 100%
        position: relative

        #top-card-wrapper
            display: none
            position: relative
            background: lightgray
            width: 100%

        #content
            width: 100%
            margin: 20px auto
            padding: $content-padding-mobile

            .article
                word-wrap: break-word
                font-size: 13px

                .headline
                    .title
                        font-size: 3em
                        letter-spacing: 2px
                        //font-weight: 600 !important
                        font-family: $title_font
                        margin-bottom: 10px

                    .subtitle
                        font-size: 1.5em
                        //font-style: italic
                        color: $subtitle-color
                        font-family: $subtitle_font

                    .author
                        font-size: 1.1em

                .article-content
                    margin: 30px auto
                    line-height: 1.6
                    font-size: 1.3em
                    word-wrap: normal
                    //font-family: $article_font
                    font-family: $default_font
                    font-weight: 400 !important

                    p
                        margin-top: 2em

                    .article-image-box
                        position: relative
                        padding-bottom: 56.25%

                        img
                            position: absolute
                            top: 0
                            left: 0
                            width: 100%
                            height: 100%

            #cards-box-wrapper
                display: grid
                position: relative
                justify-content: center
                width: 100%
                grid-auto-rows: minmax(270px, 1fr)
                grid-template-columns: repeat(auto-fit, minmax(300px, 1fr))
                //grid-template-columns: repeat(3, minmax(300px, 1fr))
                grid-gap: 20px

        +media_screensize_mobile
            #content
                max-width: $content-width
                margin: 30px auto

                .article
                    font-size: 16px
            #top-card-wrapper
                display: block
    //чтобы был нормальный отступ на мобилке
</style>
