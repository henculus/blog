<template>
    <div id="app">
        <top-menu></top-menu>
        <transition name="fade" mode="out-in" appear>
            <router-view class="page-content"></router-view>
        </transition>
        <transition name="modal">
            <modal v-if="$store.state.ModalShownStore.ModalShown"></modal>
        </transition>
    </div>
</template>

<script>
    import TopMenu from './components/TopMenu'
    import Modal from "@/components/Modal"

    export default {
        name: 'app',
        data() {
            return {}
        },
        components: {
            TopMenu,
            Modal
        },
        computed: {
            ModalShown: function () {
                return this.$store.state.ModalShownStore.ModalShown
            }
        },
        watch: {
            ModalShown: function (newState) {
                if (newState)
                    document.body.style.overflow = 'hidden';
                else
                    document.body.style.overflow = 'visible';
            }
        }
    }

</script>

<style lang="sass">
    @import "normalize/normalize.css"
    @import "variables"
    html, body, #app
        position: relative
        display: block
        top: 0
        text-rendering: optimizeLegibility
        font-family: $default_font
        width: 100%

    .modal-enter-active, .modal-leave-active
        transition: all .2s

    .modal-enter, .modal-leave-to
        opacity: 0

    .modal-enter .modal-container,
    .modal-leave-to .modal-container
        transform: scale(0.6)

    .modal-enter-to, .modal-leave
        opacity: 1
</style>
