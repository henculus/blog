<template>
    <transition name="modal" appear>
        <div class="modal-wrapper">
            <div class="modal-container" v-click-outside>
                <modal-close-button class="close" @click.native="$store.dispatch('AuthShown/ToggleAuthorizationShown')"></modal-close-button>
                <div class="modal-header">
                    <slot name="header">
                        default header
                    </slot>
                </div>

                <div class="modal-body">
                    <slot name="body">
                        default body
                    </slot>
                </div>

                <div class="modal-footer">
                    <slot name="footer">
                        default footer
                    </slot>
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
        justify-content: center
        align-items: center
        z-index: 9998
        top: 0
        left: 0
        width: 100%
        height: 100%
        background-color: rgba(0, 0, 0, .8)
        .modal-container
            position: relative
            display: flex
            transition: all .3s ease
            flex-direction: column
            justify-content: center
            align-items: center
            background: white
            border-radius: $block_border_radius
            padding: 50px
            .close
                +deselect
                position: absolute
                display: block
                padding: 5px
                top: 20px
                right: 20px
            .modal-header
                +deselect
                margin-bottom: 30px
                font-size: 1.6em
            .modal-body
                width: 100%
            .modal-footer
                +deselect
                width: 100%
                font-size: 1.4em
                margin-top: 30px

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
            .modal-container
                width: 100%
                height: 100%
                border-radius: 0
</style>

<style lang="sass">
    .no-touch .close:hover .close-button
        fill: red
</style>
