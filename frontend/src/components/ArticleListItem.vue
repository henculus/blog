<template>
    <div @click="openArticle" class="list-item">
        <div class="top-image-wrapper">
            <lazy-image class="top-image" :img-padding="30.5623" :low-res-img-path="`https://i.imgur.com/xK5T9H0.jpg`"
                        :high-res-img-path="`https://i.imgur.com/MOhedrY.jpg`"></lazy-image>
        </div>
        <div class="article-text-items">
            <div class="title">{{article.title}}</div>
            <div class="subtitle">{{article.subtitle}}</div>
            <div class="author">{{article.author}}</div> <!--TODO сделать ссылку на страницу автора-->
            <!--<div class="article-content"></div>-->
        </div>
    </div>
</template>

<script>
    import LazyImage from "./LazyImage"
    export default {
        name: "ArticleListItem",
        components: {LazyImage},
        props: {
            article: Object
        },
        methods: {
            openArticle: function () {
                if (this.article.published) {
                    this.$router.push(`/articles/${this.article.id}`)
                }
                else {
                    this.$router.push(`/editor/${this.article.id}`)
                }
            }
        }
    }
</script>

<style lang="sass" scoped>
    @import "../variables"
    .list-item
        +deselect
        min-width: 200px
        position: relative
        border: 1px solid $menu_border_color
        margin-bottom: 20px
        display: flex
        flex-direction: column
        cursor: pointer
        background: white
        color: black
        border-radius: $block_border_radius
        font-style: normal
        font-weight: 400
        -webkit-font-smoothing: subpixel-antialiased
        backface-visibility: hidden
        transform: scale(1)
        font-size: 14px
        transition: all 0.15s ease-in-out

        .top-image-wrapper
            position: relative
            overflow: hidden
            border-top-right-radius: $block_border_radius
            border-top-left-radius: $block_border_radius
            height: 100%
            .top-image
                transform: translateZ(0)
                transition: all .2s ease-out
                width: 100%
                height: 100%
                position: relative
                pointer-events: none
                top: 0
                bottom: 0
                left: 0

        .article-text-items
            padding: 20px
            display: flex
            flex-direction: column
            .title
                font-size: 2em
                margin-bottom: 15px
                font-weight: normal
                font-family: $title_font

            .subtitle
                font-size: 1.05em
                font-family: $subtitle_font
                color: $subtitle-color

            .article-content
                position: relative
                font-family: $default_font
            .author
                justify-self: flex-end
                font-size: 0.9em
    .no-touch .list-item:hover
        border: 1px solid $rnt_green
        box-shadow: 0 0 4px $rnt_green

    +media_screensize_mobile
        .list-item
            font-size: 16px
            .top-image-wrapper
                display: block
                max-height: 250px
        .no-touch .list-item:hover
            border: 1px solid $menu_border_color
            box-shadow: none

    .no-touch
        .list-item:hover
            .top-image
                transform: scale(1.05)


</style>
