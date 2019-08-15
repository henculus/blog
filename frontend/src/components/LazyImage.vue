<template>
    <intersection-observer>
        <template v-slot = "{ displayed }">
            <template :displayed = "displayed">
                <div class="lazy-image-box">
                    <div class="padding-box" :style="{ paddingBottom: imgPadding + '%' }">
                        <div class="image-wrapper">
                            <div class="low-res-img-wrapper" :class="{hidden: imageLoaded}">
                                <img class="low-res-img" :src="lowResImgPath"/>
                            </div>
                            <img v-if="displayed" class="high-res-img" :src="highResImgPath"
                                 :class="{visible: imageLoaded}"
                                 @load="imageLoaded = true"/>
                        </div>
                    </div>
                </div>
            </template>
        </template>
    </intersection-observer>
</template>

<script>
    import IntersectionObserver from "./IntersectionObserver"

    export default {
        name: "LazyImage",
        components: {
            IntersectionObserver
        },
        props: {
            imgPadding: Number,
            highResImgPath: String,
            lowResImgPath: String,
            displayed: Boolean
        },
        data() {
            return {
                imageLoaded: false
            }
        },
        methods: {}
    }
</script>

<style lang="sass" scoped>
    @import "../variables"
    .lazy-image-box
        position: relative
        display: block
        z-index: 1

        .padding-box
            .image-wrapper
                position: absolute
                display: block
                overflow: hidden
                width: 100%
                height: 100%
                left: 0
                top: 0

                .low-res-img-wrapper
                    position: absolute
                    display: block
                    left: 0
                    top: 0
                    height: 100%
                    width: 100%
                    overflow: hidden
                    transform: translateZ(0)
                    transition: opacity .3s ease-in .4s
                    opacity: 1
                    z-index: 2

                    .low-res-img
                        filter: blur(20px)
                        transform: scale(1.1)
                        position: absolute
                        width: 100%
                        left: 0
                        bottom: 0
                        top: 0

                    &.hidden
                        opacity: 0

                .high-res-img
                    display: block
                    z-index: 1
                    width: 100%
                    position: absolute
                    top: 0
                    left: 0
                    opacity: 0
                    transition: opacity .3s ease-in 0s

                    &.visible
                        opacity: 1
</style>
