<template>
    <div class="modal-wrapper" @click="$store.dispatch('ModalShownStore/ToggleModalShown')">
        <div class="modal-container" @click="modalClick">
            <modal-close-button class="close"
                                @click.native="$store.dispatch('ModalShownStore/ToggleModalShown')"></modal-close-button>
            <component :is="$store.state.ModalShownStore.ModalComponent"></component>
        </div>
    </div>
</template>

<script>
    import ModalCloseButton from "@/components/ModalCloseButton";
    import ModalAuthorization from "@/components/ModalAuthorization";

    export default {
        name: "Modal",
        components: {
            ModalCloseButton,
            ModalAuthorization
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
            transition: $ease_transition02
            background: white
            max-width: 560px
            overflow-y: auto
            width: 100%
            border-radius: 0

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
            .modal-container
                border-radius: $block_border_radius
                width: auto

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
