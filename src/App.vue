<template>
    <div id="app">
        <top-menu></top-menu>
        <transition name="fade" mode="out-in">
            <router-view class="page-content"></router-view>
        </transition>
        <modal-authorization v-if="$store.state.AuthShown.AuthorizationShown"></modal-authorization>
    </div>
</template>

<script>
    import TopMenu from './components/TopMenu'
    import ModalAuthorization from "@/components/ModalAuthorization";

    export default {
        name: 'app',
        data() {
            return {}
        },
        components: {
            TopMenu,
            ModalAuthorization
        },
        computed: {
            AuthorizationShown: function () {
                return this.$store.state.AuthShown.AuthorizationShown
            }
        },
        watch: {
            AuthorizationShown: function (newState) {
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
        //background: rgba(128, 128, 128, 0.02)
        text-rendering: optimizeLegibility
        font-family: $default_font
        //font-family: 'Montserrat Alternates', sans-serif;
        width: 100%
</style>
