<template>
    <div class="top-menu-wrapper">
        <div class="top-menu">
            <router-link tag="div" id="logo-box" to="/">
                <img class="logo logo--colorful" src="../assets/rnt_logo.png" alt="Ranetka"/>
                <img class="logo logo--black" src="../assets/rnt_black.png" alt="Ranetka"/>
            </router-link>
            <nav id="menu-wrapper">
                <ul id="menu-list">
                    <li class="menu-item">
                        <a class="menu-item__link" @click="$store.dispatch('AuthShown/ToggleAuthorizationShown')"> <!-- TODO Сделать страницу логина -->
                            <svg class="menu-item__link__logo" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                                <path d="M12 0c-6.627 0-12 5.373-12 12s5.373 12 12 12 12-5.373 12-12-5.373-12-12-12zm0 22c-3.123 0-5.914-1.441-7.749-3.69.259-.588.783-.995 1.867-1.246 2.244-.518 4.459-.981 3.393-2.945-3.155-5.82-.899-9.119 2.489-9.119 3.322 0 5.634 3.177 2.489 9.119-1.035 1.952 1.1 2.416 3.393 2.945 1.082.25 1.61.655 1.871 1.241-1.836 2.253-4.628 3.695-7.753 3.695z"/>
                            </svg>
                            <span class="menu-item__link__text">Войти</span>
                        </a>
                    </li>
                </ul>
            </nav>
        </div>
    </div>
</template>

<script>
    export default {
        name: "TopMenu",
        data() {
            return {}
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
        //border-bottom: 1px $menu_border_color solid
    .top-menu
        width: $content-width
        max-width: $content-width
        display: flex
        flex-direction: row
        position: relative
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
                position: absolute
                opacity: 0

        #menu-wrapper
            margin-right: 20px

            #menu-list
                height: 100%
                display: flex
                flex-direction: row
                padding: 0

                .menu-item
                    display: flex
                    flex-direction: row
                    justify-content: flex-start
                    list-style: none
                    margin-left: 20px

                    &:first-child
                        margin-left: 0

                    .menu-item__link
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
                        transition: color .3s ease

                        .menu-item__link__logo
                            transition: all .3s ease
                            height: 40px

                        .menu-item__link__text
                            margin-left: 10px

                        &, &:hover, &:active, &:focus
                            text-decoration: none

                &:first-child
                    margin-left: 0


            +mediascreensize_mobile
                &
                    margin-right: 0

    .no-touch .top-menu #menu-wrapper #menu-list .menu-item:hover
        .menu-item__link
            color: $rnt_green

            .menu-item__link__logo
                fill: $rnt_green

    .no-touch #logo-box:hover
        .logo
            opacity: 0

        .logo--colorful
            opacity: 1
    +mediascreensize_mobile
        .top-menu
            padding: $content-padding-mobile
</style>
