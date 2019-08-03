<template>
    <transition name="modal" appear>
        <div class="modal-wrapper" @click="$store.dispatch('AuthShown/ToggleAuthorizationShown')">
            <div class="modal-container" @click="modalClick">
                <modal-close-button class="close"
                                    @click.native="$store.dispatch('AuthShown/ToggleAuthorizationShown')"></modal-close-button>
                <div class="modal-content">

                    <div class="modal-header modal-element">
                        <slot name="header">
                            default header
                        </slot>
                    </div>

                    <div class="modal-body modal-element">
                        <slot name="body">
                            default body
                        </slot>
                    </div>

                    <div class="modal-footer modal-element">
                        <slot name="footer">
                        </slot>
                    </div>
                </div>
            </div>
        </div>
    </transition>
</template>

<script>
    import ModalCloseButton from "@/components/ModalCloseButton";

    export default {
        name: "Modal",
        components: {
            ModalCloseButton
        },
        methods: {
            modalClick: function (event) {
                event.stopPropagation()
            }
        }
    }
</script>

<style lang="sass" scoped>
    @import "../variables"
    .modal-wrapper
        font-size: 16px
        position: fixed
        display: flex
        flex-direction: row
        align-items: center
        justify-content: center
        z-index: 9998
        top: 0
        left: 0
        width: 100%
        height: 100%
        background-color: rgba(0, 0, 0, .8)
        overflow-y: auto

        .modal-container
            position: absolute
            max-height: 100%
            display: flex
            flex-direction: column
            align-items: center
            transition: all .3s ease
            background: white
            border-radius: $block_border_radius
            //padding: 50px
            max-width: 560px
            overflow-y: auto
            .modal-content
                position: relative
                display: block
                padding: 50px 50px 30px 50px
                height: auto
                width: 100%
            .modal-element
                position: relative
                display: block

            .modal-header
                +deselect
                margin-bottom: 30px

            .modal-body
                width: 100%

            .modal-footer
                +deselect
                margin-top: 20px
                width: 100%

        .close
            +deselect
            border-radius: 100%
            z-index: 9999
            position: absolute
            display: block
            top: 15px
            right: 15px
            padding: 10px

    .modal-enter-active, .modal-leave-active
        transition: all .2s

    .modal-enter, .modal-leave-to
        opacity: 0

    .modal-enter .modal-container,
    .modal-leave-to .modal-container
        transform: scale(0.6)

    .modal-enter-to, .modal-leave
        opacity: 1

    +mediascreensize_mobile
        .modal-wrapper
            //background: white Полностью белый экран - скорее нет, чем да
            .modal-container
                width: 100%
                border-radius: 0

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
