<template>
    <div class="article-list-wrapper">
        <div class="article-block-name">Последние статьи</div>
        <div class="article-list">
            <article-list-item v-for="article in articles" :key="article.id" :article="article"></article-list-item>
        </div>
    </div>
</template>

<script>
    import ArticleListItem from "@/components/ArticleListItem";

    export default {
        name: "ArticleList",
        components: {
            ArticleListItem
        },
        data() {
            return {
                articles: [
                    {
                        'id': 1,
                        'img_src': 'https://images.wallpaperscraft.ru/image/uzory_ornament_dudly_137088_1920x1080.jpg',
                        'title': 'Заголовок первой статьи',
                        'subtitle': 'Подзаголовок первой статьи',
                        'content': "First article content"
                    },
                    {
                        'id': 2,
                        'img_src': 'https://images.wallpaperscraft.ru/image/uzor_simvoly_etnicheskij_130866_1920x1080.jpg',
                        'title': 'Заголовок второй статьи',
                        'subtitle': 'Подзаголовок второй статьи',
                        'content': 'Second article content'
                    },
                    {
                        'id': 3,
                        'img_src': 'https://images.wallpaperscraft.ru/image/uzory_ornament_vektor_136992_1920x1080.jpg',
                        'title': 'Заголовок третьей статьи',
                        'subtitle': 'Подзаголовок третьей статьи',
                        'content': 'Third article content'
                    },
                    {
                        'id': 4,
                        'img_src': 'https://images.wallpaperscraft.ru/image/uzory_tkan_fioletovyj_123846_1920x1080.jpg',
                        'title': 'Заголовок четвёртой статьи',
                        'subtitle': 'Подзаголовок четвёртой статьи',
                        'content': 'Fourth article content'
                    },
                    {
                        'id': 5,
                        'img_src': 'https://images.wallpaperscraft.ru/image/uzory_dudly_pattern_136687_1920x1080.jpg',
                        'title': 'Заголовок пятой статьи',
                        'subtitle': 'Подзаголовок пятой статьи',
                        'content': 'Fifth article content'
                    },
                    {
                        'id': 6,
                        'img_src': 'https://images.wallpaperscraft.ru/image/raznotsvetnyj_pestryj_psihodelicheskij_139324_1920x1080.jpg',
                        'title': 'Заголовок шестой статьи',
                        'subtitle': 'Подзаголовок шестой статьи',
                        'content': 'Sixth article content'
                    },
                    {
                        'id': 7,
                        'img_src': 'https://images.wallpaperscraft.ru/image/uzory_piatna_raznotsvetnyj_123044_1920x1080.jpg',
                        'title': 'Заголовок седьмой статьи',
                        'subtitle': 'Подзаголовок седьмой статьи',
                        'content': 'Seventh article content'
                    },
                    {
                        'id': 8,
                        'img_src': 'https://images.wallpaperscraft.ru/image/uzory_volnistyj_piatna_133960_1920x1080.jpg',
                        'title': 'Заголовок восьмой статьи',
                        'subtitle': 'Подзаголовок восьмой статьи',
                        'content': 'Eighth article content'
                    },
                    {
                        'id': 9,
                        'img_src': 'https://images.wallpaperscraft.ru/image/uzory_fon_pyatna_linii_66020_1920x1080.jpg',
                        'title': 'Заголовок девятой статьи',
                        'subtitle': 'Подзаголовок девятой статьи',
                        'content': 'Ninth article content'
                    },
                    {
                        'id': 10,
                        'img_src': 'https://images.wallpaperscraft.ru/image/uzory_fon_linii_pyatna_81511_1920x1080.jpg',
                        'title': 'Заголовок десятой статьи',
                        'subtitle': 'Подзаголовок десятой статьи',
                        'content': 'Tenth article content'
                    },
                ]
            }
        }
    }
</script>

<style lang="sass" scoped>
    @import "../variables"
    .article-list-wrapper
        display: flex
        flex-direction: column
        margin: 0 auto
        padding: 20px 10px
        width: 100%

        .article-block-name
            +deselect
            //align-self: center
            font-size: 1.8em

            margin-bottom: 20px
        .article-list
            //width: 50%
            position: relative
            display: block

    +media_screensize_mobile
        .article-list-wrapper
            position: relative
            width: $content-width
            padding: 20px 0
            .article-block-name
                font-size: 2em
</style>
