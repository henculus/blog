<template>
    <div class="modal-wrapper" @click="modalClose">
        <div :class="{'disabled': $store.state.AuthorizationStore.isLoading}"
             class="modal-container"
             @click="$event.stopPropagation()">
            <modal-close-button class="close"
                                @click.native="modalClose"></modal-close-button>
            <component :is="$store.state.ModalStore.ModalComponent"></component>
        </div>
    </div>
</template>

<script>
    import ModalCloseButton from "@/components/ModalCloseButton"
    import ModalAuthorization from "@/components/ModalAuthorization"

    export default {
        name: "Modal",
        components: {
            ModalCloseButton,
            ModalAuthorization
        },
        data() {
            return {
                isDisabled: false
            }
        },
        methods: {
            modalClose: function () {
                if (!this.$store.state.AuthorizationStore.isLoading)
                    this.$store.dispatch('ModalStore/HideModal') //TODO Разделить на 2 экшна (открыть окно/закрыть окно)
            },
        }
    }
</script>

<style lang="sass" scoped>
    @import "../variables"
    .modal-wrapper
        font-size: 16px
        position: fixed
        display: flex
        flex-direction: column
        align-items: center
        justify-content: center
        z-index: 9998
        top: 0
        left: 0
        right: 0
        bottom: 0
        width: 100vw
        height: 100vh
        padding: 10px
        background-color: rgba(0, 0, 0, .8)
        overflow-y: auto

        .modal-container
            position: relative
            // margin: $content-padding-mobile
            border-radius: $block_border_radius
            pointer-events: all
            max-height: 100%
            display: flex
            flex-direction: column
            align-items: center
            justify-content: center
            transition: $ease_transition02
            background: white
            overflow-y: auto
            width: 100%
            margin: 0 20px
            min-width: 200px

            &:after
                content: ''
                transition: all .1s ease

            &.disabled
                pointer-events: none

                &:after
                    width: 100%
                    height: 100%
                    content: ''
                    position: absolute
                    background: rgba(0, 0, 0, 0.5)
                    z-index: 10000

        .close
            +deselect
            border-radius: 100%
            z-index: 9999
            position: absolute
            display: block
            top: 15px
            right: 15px
            padding: 10px

    .modal-content-enter, .modal-content-leave-to
        opacity: 0

    .modal-content-enter-to, .modal-content-leave
        opacity: 1

    .modal-content-leave-active, .modal-content-enter-active
        transition: $ease_transition02

    +media_screensize_mobile
        .modal-wrapper
            padding: 0
            .modal-container
                width: 400px

    +media_screensize_mobile_small
        .modal-wrapper
            .modal-container
                left: auto
                right: auto
                width: 400px
                margin: 0

</style>

<style lang="sass">
    @import "../variables"
    .close
        transition: $ease_transition02

        .close-button
            transition: $ease_transition02

    .no-touch
        .close:hover
            background: rgba(255, 0, 37, 0.06)

            .close-button
                fill: #ff0025

</style>
