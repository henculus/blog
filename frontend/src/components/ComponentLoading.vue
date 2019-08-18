<template>
    <div class="load-wrapper">
        <div class="load-icon-wrapper">
            <div class="circle load-circle"></div>
            <div class="circle load-circle-part"></div>
        </div>
    </div>
</template>

<script>
    export default {
        name: "ComponentLoading"
    }
</script>

<style lang="sass" scoped>

    $load-icon-size: 26px
    @import "../variables"
    @keyframes spin
        0%
            transform: rotate(0deg)
        100%
            transform: rotate(360deg)
    .load-wrapper
        width: 100%
        display: flex
        flex-direction: row
        align-items: center
        justify-content: center

        .load-icon-wrapper
            width: $load-icon-size
            height: $load-icon-size
            position: relative

            .circle
                width: 100%
                height: 100%
                border-radius: $circle-border-radius
            .load-circle
                border: 3px solid $rnt_green_light
            .load-circle-part
                position: absolute
                animation: spin .8s linear infinite
                left: 0
                top: 0
                border-width: 3px
                border-style: solid
                border-right-color: $rnt_green
                border-left-color: transparent
                border-bottom-color: transparent
                border-top-color: transparent


</style>
