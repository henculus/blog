<template>
    <div id="app">
        <top-bar class="top-bar"></top-bar>
        <transition name="fade" mode="out-in" appear>
            <router-view class="page-content"></router-view>
        </transition>
        <transition name="modal">
            <modal v-if="$store.state.ModalStore.ModalShown"></modal>
        </transition>
    </div>
</template>

<script>
    import TopBar from './components/TopBar'
    import Modal from "./components/Modal"

    export default {
        name: 'app',
        data() {
            return {
                authorized: false,
                sub: undefined
            }
        },
        components: {
            TopBar,
            Modal
        },
        beforeMount: function () {
            this.authorized = window.localStorage.getItem('authorized')
            this.sub = window.localStorage.getItem('sub')
            this.$store.dispatch('AuthorizationStore/CheckAuthorize').then(
                //eslint-disable-next-line
                result => {

                },
                error => {
                    console.error(error, 'Не авторизован')
                }
            )
        },
        computed: {
            ModalShown: function () {
                return this.$store.state.ModalStore.ModalShown
            },
        },
        watch: {
            ModalShown: function (newState) {
                if (newState)
                    document.body.style.overflow = 'hidden'
                else {
                    document.body.style.overflow = 'visible scroll'
                }
            },
        }
    }

</script>

<style lang="sass">
    @import "normalize/normalize.css"
    @import "variables"
    html, body, #app
        background: white
        position: relative
        display: block
        top: 0
        text-rendering: optimizeLegibility
        font-family: $default_font
        width: 100%
        height: 100%

    body
        overflow-y: scroll

    .top-bar
        z-index: 1

    .page-content
        z-index: 0
        display: flex
        flex-direction: row
        justify-content: center

    .modal-enter-active, .modal-leave-active
        transition: $ease_transition02

    .modal-enter, .modal-leave-to
        opacity: 0

    .modal-enter .modal-container,
    .modal-leave-to .modal-container
        transform: scale(0.6)

    .modal-enter-to, .modal-leave
        opacity: 1
</style>
