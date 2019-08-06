<template>
    <div class="card-wrapper">
        <div class="image-box"
             :style="{'background-image': 'url('+image_url+')'}"></div>
        <div class="card-text"><span>{{image_text}}</span></div>
    </div>
</template>

<script>
    export default {
        name: "ContentBoxCard",
        props: {
            image_url: String,
            image_text: String
        },
    }
</script>

<style scoped lang="sass">
    @import "../variables.sass"
    .card-wrapper
        position: relative
        overflow: hidden
        box-shadow: 0 0 4px rgba(0, 0, 0, 0.5)
        //Тень карточек, нужно или нет - подумать (Если использовать, то перенести в отдельный файл, сделать переменную sass и импортить)
        .image-box
            display: flex
            flex-direction: row
            align-items: flex-end
            width: 100%
            height: 100%
            background-size: cover
            background-color: lightgray
            background-position: center
            transition: all 0.3s ease

        .card-text
            position: absolute
            bottom: 0
            display: flex
            flex-direction: row
            justify-content: center
            align-items: center
            color: white
            background-color: rgba(0, 0, 0, 0.5)
            width: 100%
            height: 40px

        +mediacursorpointer
            &:hover .image-box
                transform: scale(1.1)
</style>