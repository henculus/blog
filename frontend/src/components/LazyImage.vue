<template>
    <intersection-observer>
        <template v-slot="{ displayed }">
            <template :displayed="displayed">
                <div class="lazy-image-box">
                    <div class="padding-box" :style="{ paddingBottom: imgPadding + '%' }">
                        <div class="image-wrapper">
                            <div class="low-res-img-wrapper" :class="{hidden: imageLoaded}">
                                <img class="low-res-img" :src="lowResImgPath"/>
                            </div>
                            <img v-if="displayed || cached" class="high-res-img" :src="highResImgPath"
                                 :class="{visible: imageLoaded, cached: cached, 'ultra-high-res': ultraHighRes}"
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
            displayed: Boolean,
        },
        data() {
            return {
                imageLoaded: false,
                cached: false,
                ultraHighRes: false
            }
        },
        beforeMount: function () {
            let image = new Image()
            image.src = this.highResImgPath
            this.cached = image.complete || (image.width + image.height) > 0
            if (this.cached && (image.width+image.height > 7000)) {
                this.ultraHighRes = true
            }
        },
        methods: {}
    }
</script>

<style lang="sass" scoped>
    @import "../variables"
    .lazy-image-box
        +deselect
        position: relative
        display: block
        z-index: 1

        .padding-box
            .image-wrapper
                background: #ebebeb
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
                    transition: opacity .2s ease-in .4s
                    opacity: 1
                    //z-index: 2

                    .low-res-img
                        filter: blur(20px)
                        transform: scale(1.1)
                        position: absolute
                        width: 100%
                        left: 0
                        top: 0
                        transition: visibility 0s ease-in .4s
                        visibility: visible

                    &.hidden
                        opacity: 0
                        .low-res-img
                            visibility: hidden


                .high-res-img
                    display: block
                    //z-index: 1
                    width: 100%
                    position: absolute
                    top: 0
                    left: 0
                    opacity: 0
                    transition: opacity .2s ease-in 0s
                    &.ultra-high-res
                        transition: opacity .2s ease-in .2s

                    &.visible
                        opacity: 1
</style>
