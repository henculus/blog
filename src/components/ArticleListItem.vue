<template>
    <router-link :to="{ name: 'article', params: { id: article.id } }" tag="div" class="list-item">
        <div class="top-image-wrapper">
            <div class="top-image-placeholder"></div>
            <img class="top-image" :src="article.img_src"/>
        </div>
        <div class="article-text-items">
            <div class="title">{{article.title}}</div>
            <div class="subtitle">{{article.subtitle}}</div>
            <!--<div class="article-content"></div>-->
        </div>
    </router-link>
</template>

<script>
    export default {
        name: "ArticleListItem",
        props: {
            article: Object
        },
    }
</script>

<style lang="sass" scoped>
    @import "../variables"
    .list-item
        +deselect
        min-width: 200px
        position: relative
        box-shadow: 0 0 4px rgba(0, 0, 0, 0.4)
        //border: 1px solid rgb(238, 238, 238)
        margin-bottom: 20px
        display: flex
        flex-direction: column
        cursor: pointer
        background: white
        color: black
        border-radius: $block_border_radius
        font-size: 16px
        font-style: normal
        font-weight: 400
        -webkit-font-smoothing: subpixel-antialiased
        backface-visibility: hidden
        transform: scale(1)
        +mediacursorpointer
            &:hover
                .top-image-wrapper .top-image-placeholder
                    padding-bottom: 25%

        .top-image-wrapper
            position: relative
            overflow: hidden
            border-top-right-radius: $block_border_radius
            border-top-left-radius: $block_border_radius
            max-height: 200px

            .top-image-placeholder
                display: block
                position: relative
                transition: all 0.2s ease-in-out
                padding-bottom: 14%

            .top-image
                transform: translateZ(0)
                width: 100%
                position: absolute
                top: 0
                left: 0

        .article-text-items
            padding: 30px

            .title
                font-size: 1.8em
                margin-bottom: 15px
                font-family: $title_font

            .subtitle
                font-size: 1.05em
                font-family: $subtitle_font
                color: $subtitle-color

            .article-content
                position: relative
                font-family: $default_font

    +mediascreensize_mobile
        .top-image-wrapper
            display: none
        .list-item
            font-size: 14px
            transition: all 0.15s ease-in-out

            &:hover
                //box-shadow: 0 0 10px rgba(98, 240, 193, 1)
                transform: translateX(10px)


</style>
