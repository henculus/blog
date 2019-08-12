<template>
    <div class="lazy-image-box">
        <div class="padding-box" :style="{ padding: imgPadding + '%' }">
            <div class="image-wrapper">
                <div class="low-res-img-wrapper" :class="{hidden: imageLoaded}">
                    <img class="low-res-img" src="../assets/8k-low_res.jpg"/>
                </div>
                <img class="high-res-img" src="../assets/8k.jpg" :class="{visible: imageLoaded}" @load="imageLoaded = true"/>
            </div>
        </div>
    </div>
</template>

<script>
    export default {
        name: "LazyImage",
        props: {
          imgPadding: Number
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

        .padding-box
            .image-wrapper
                position: absolute
                overflow: hidden
                width: 100%
                height: 100%
                left: 0
                top: 0
                .low-res-img-wrapper
                    position: relative
                    height: 100%
                    overflow: hidden
                    transition: opacity .3s ease-in .3s
                    z-index: 99
                    opacity: 1
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
                    z-index: 1
                    width: 100%
                    position: absolute
                    top: 0
                    left: 0
                    opacity: 0
                    transition: all .1s ease
                    &.visible
                        opacity: 1
</style>
