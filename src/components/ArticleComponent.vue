<template>
    <div id="content-wrapper">
        <div></div>
        <div id="top-card-wrapper">
            <div id="top-card">
            </div>
        </div>
        <div id="content">
            <article class="article">
                <section class="headline">
                    <div class="title">Твоё имя</div>
                    <div class="subtitle">Я ищу тебя, хотя не знаю кто ты</div>
                </section>
                <section class="article-content">
                    <!--<div class="article-image-box">-->
                    <!--<img src="../assets/8k-low_res.jpg"/>-->
                    <!--<img src="../assets/8k.jpg" @load="loaded"/>-->
                    <!--</div>-->
                    <p>Мицуха Миямизу живет в горном поселении в японской глубинке и больше
                        всего мечтает переехать в шумный мегаполис.
                        Девушка убеждена, что в родных краях нет будущего, она лишена возможности развиваться и не
                        находит
                        простора для самореализации.
                        Она рассказывает о сокровенных помыслах отцу, но он категорически запрещает дочери помышлять о
                        переезде.
                        Героиня огорчена, обескуражена и неожиданно приходит к мысли, что хотела бы родиться мальчиком –
                        тогда к ее стремлениям относились бы с большим уважением.</p>
                    <p>Жизнь Таки Татибана прямо противоположна – он обитает в Токио, зарабатывая подработкой в
                        небольшом ресторанчике.
                        Парень устал от шума, огней и суеты огромного города, и уверен, что многое отдал бы за домик в
                        тихой местности, окруженной девственной природой.
                        Не будучи знакомыми и находясь за сотни километров, герои загадывают желания, чтобы их судьбы
                        изменились.</p>
                    <p>Мироздание вмешивается в происходящее, и молодые люди видят сны, в которых все надежды
                        оправдались, но обоих удивляет странная особенность сновидений – каждый из них обнаружил себя в
                        чужом теле.
                        Заветные мечты исполнены, но таким ли образом, как представляли ребята?
                        Как долго это будет продолжаться, что означает их мистическая связь, и суждено ли им
                        встретиться?</p>
                </section>
            </article>
        </div>
    </div>
</template>

<script>

    export default {
        name: "ArticleComponent",
        components: {
        },
        data() {
            return {
                card_info: {
                    0: {'image_url': 'https://i.imgur.com/eaPk4sP.png', 'image_text': 'Image1'},
                    1: {'image_url': 'https://i.imgur.com/fn4UddZ.png', 'image_text': 'Image2'},
                    2: {'image_url': 'https://i.imgur.com/M5jF6Pz.jpg', 'image_text': 'Image3'},
                    3: {'image_url': 'https://i.imgur.com/h5e29Fw.png', 'image_text': 'Image4'},
                    4: {'image_url': 'https://i.imgur.com/lrWuKmt.jpg', 'image_text': 'Image5'},
                    5: {'image_url': 'https://i.imgur.com/kiN9qso.png', 'image_text': 'Image6'}
                }
            }
        },
    }
</script>

<style scoped lang="sass">
    @import "../variables.sass"

    #content-wrapper
        display: flex
        flex-direction: column
        align-items: center
        width: 100%
        //top: 70px //Выключено чтобы под меню не было лага
        //Если буду делать fixed
        position: relative

        #top-card-wrapper
            display: block
            position: relative
            box-shadow: $bottom_box_shadow
            background: lightgray
            width: 100%
            //max-height: 50vh
            height: $banner_height
        //min-height: 45vh
        #top-card
            display: block
            overflow: hidden
            width: 100%
            height: 100%
            background: url("../assets/banner/banner.jpg") no-repeat bottom
            background-size: cover

            &:before
                left: 0
                right: 0
                bottom: 0
                top: 0
                content: ''
                position: absolute

        #content
            max-width: 700px
            margin: 60px auto

            .article
                word-wrap: break-word
                font-size: 16px

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

        +mediascreensize_mobile
            #content
                width: 100%
                margin: 20px auto
                padding: $content-padding-mobile

                .article
                    font-size: 13px
            #top-card-wrapper
                display: none
    //чтобы был нормальный отступ на мобилке
</style>
