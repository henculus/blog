<template>
    <div class="top-menu-wrapper">
        <div class="top-menu">
            <router-link tag="div" id="logo-box" to="/">
                <img class="logo logo--colorful" src="../assets/rnt_logo.png" alt="Ranetka"/>
                <img class="logo logo--black" src="../assets/rnt_black.png" alt="Ranetka"/>
            </router-link>
            <nav id="menu-wrapper">
                <a v-cloak class="menu-item menu-item__link" @click="ShowAuth"
                   v-if="!$store.getters['AuthorizationStore/isAuthorized']">
                    <svg class="menu-item__logo" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                        <path d="M12 0c-6.627 0-12 5.373-12 12s5.373 12 12 12 12-5.373 12-12-5.373-12-12-12zm0 22c-3.123 0-5.914-1.441-7.749-3.69.259-.588.783-.995 1.867-1.246 2.244-.518 4.459-.981 3.393-2.945-3.155-5.82-.899-9.119 2.489-9.119 3.322 0 5.634 3.177 2.489 9.119-1.035 1.952 1.1 2.416 3.393 2.945 1.082.25 1.61.655 1.871 1.241-1.836 2.253-4.628 3.695-7.753 3.695z"></path>
                    </svg>
                    <span class="menu-item__text">Войти</span>
                </a>
                <div class="menu-item" v-if="$store.getters['AuthorizationStore/isAuthorized']&&!$store.state.AuthorizationStore.isLoading">
                    <a ref="openContextMenuButton" class="user-name menu-item__link"
                       @click="ContextMenuShown = !ContextMenuShown">{{ $store.state.AuthorizationStore.sub }}</a>
                </div>
                <transition name="context-menu">
                    <top-bar-context-menu v-if="ContextMenuShown" @closeContextMenu="ContextMenuShown = false"
                                          v-click-outside="{exclude: 'openContextMenuButton', isShown: 'ContextMenuShown'}"></top-bar-context-menu>
                </transition>
            </nav>
        </div>
    </div>
</template>

<script>
    import TopBarContextMenu from "./TopBarContextMenu";

    export default {
        name: "TopBar",
        components: {
            TopBarContextMenu
        },
        data() {
            return {
                ContextMenuShown: false
            }
        },
        methods: {
            ShowAuth: function () {
                this.$store.dispatch('ModalShownStore/ToggleModalShown', 'ModalAuthorization')
            },

        },

    }
</script>

<style scoped lang="sass">
    @import "../variables.sass"

    .top-menu-wrapper
        width: 100%
        display: flex
        flex-direction: row
        justify-content: center

    .top-menu
        position: relative
        width: $content-width
        padding: $content-padding-mobile
        max-width: $content-width
        display: flex
        flex-direction: row
        z-index: 1
        top: 0
        justify-content: space-between
        height: $menu_height
        align-items: center


        #logo-box
            height: 40px
            display: flex
            flex-direction: row
            align-items: center
            //margin-left: 20px
            cursor: pointer
            position: relative

            .logo
                +deselect
                height: 100%
                transition: all .3s ease
                pointer-events: none

            .logo--colorful
                top: 0
                left: 0
                position: absolute
                opacity: 0

        #menu-wrapper
            margin-right: 20px

            .menu-item__logo
                transition: $ease_transition02

            .menu-item__link
                transition: $ease_transition02

                &:hover
                    color: $rnt_green

                    .menu-item__logo
                        fill: $rnt_green

            .menu-item
                +deselect
                position: relative
                display: flex
                flex-direction: row
                align-items: center
                font-size: $menu_font_size
                cursor: pointer
                line-height: $menu_height
                height: 100%
                color: black


                .menu-item__logo
                    transition: all .3s ease
                    height: 40px

                .menu-item__text
                    margin-left: 10px

                &, &:hover, &:active, &:focus
                    text-decoration: none


            +media_screensize_mobile
                &
                    margin-right: 0

    .no-touch .top-menu #menu-wrapper #menu-list .menu-item:hover
        .menu-item
            color: $rnt_green

            .menu-item__logo
                fill: $rnt_green

    .no-touch #logo-box:hover
        .logo
            opacity: 0

        .logo--colorful
            opacity: 1

    +media_screensize_mobile
        .top-menu
            padding: 0

    [v-cloak]
        display: none
    .context-menu-enter, .context-menu-leave-to
        opacity: 0
    .context-menu-enter-to, .context-menu-leave
        opacity: 1
    .context-menu-enter-active, .context-menu-leave-active
        transition: all .1s ease-in-out
</style>
