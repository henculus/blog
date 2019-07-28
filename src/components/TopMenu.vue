<template>
    <div class="top-menu top-menu-show" v-scroll="handleScroll">
        <router-link tag="div" id="logo-box" to="/">
                <img id="logo" src="../assets/rnt_logo.png"/> <!--TODO Сделать логотип кликабельным-->
        </router-link>
        <nav id="menu-wrapper">
            <ul id="menu-list"
                :class="[{open: MenuActive}, {close: !MenuActive&&ButtonClicked&&window.width<=MaxWidth}]">
                <li class="menu-item"><a>Menu1</a></li>
                <li class="menu-item"><a>Menu2</a></li>
                <li class="menu-item"><a>Menu3</a></li>
            </ul>
            <a id="mobile-menu-link" @click="MenuActive=!MenuActive; ButtonClicked=true">
                <div id="mobile-menu" :class="{active: MenuActive}">
                    <span id="line1"></span>
                    <span id="line2"></span>
                    <span id="line3"></span>
                </div>
            </a>
        </nav>
    </div>
</template>

<script>
    export default {
        name: "TopMenu",
        data() {
            return {
                MaxWidth: 850,
                lastScrollTop: 0,
                lastScrollDown: 0,
                MenuActive: false,
                ButtonClicked: false,
                window: {
                    width: 0,
                    height: 0
                }
            }
        },
        created() {
            window.addEventListener('resize', this.handleResize)
            this.handleResize();
        },
        destroyed() {
            window.removeEventListener('resize', this.handleResize)
        },
        methods: {
            handleResize: function () {
                this.window.width = window.innerWidth;
                if (this.window.width >= this.MaxWidth && this.MenuActive) {
                    this.MenuActive = false
                    this.ButtonClicked = false
                }
                this.window.height = window.innerHeight;
            },
            handleScroll: function (evt, el) {
                let st = window.pageYOffset || document.documentElement.scrollTop;
                if (st > this.lastScrollTop && window.pageYOffset > 25) {
                    el.classList.remove("top-menu-show")
                    this.MenuActive = false
                    this.lastScrollDown = st
                }
                if (this.lastScrollDown - st > 50 || window.pageYOffset <= 25) {
                    el.classList.add("top-menu-show")
                }
                this.lastScrollTop = st <= 0 ? 0 : st;
            }

        },
        directives: {
            scroll: {
                inserted: function (el, binding) {
                    let f = function (evt) {
                        if (binding.value(evt, el)) {
                            window.removeEventListener('scroll', f)
                        }
                    }
                    window.addEventListener('scroll', f)
                }
            }
        }
    }
</script>

<style scoped lang="sass">
    @import "../variables.sass"

    .top-menu
        width: 100%
        display: flex
        flex-direction: row
        position: fixed
        z-index: 1
        top: 0
        justify-content: space-between
        height: $menu_height
        background-color: $menu_color
        align-items: center
        box-shadow: $bottom_box_shadow
        transform-origin: top
        transition: transform .3s
        transform: translateY(calc(-100% - 4px))

        &.top-menu-show
            transform: translateY(0)
            transition: transform .3s

        #logo-box
            height: 40px
            display: flex
            flex-direction: row
            align-items: center
            margin-left: 20px
            cursor: pointer

            #logo
                height: 100%

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

                    a
                        +deselect
                        font-size: $menu_font_size
                        cursor: pointer
                        line-height: $menu_height
                        height: 100%
                        color: $menu_text_color
                        //font-weight: bold
                        transition: color .5s
                        +mediacursorpointer
                            &:hover
                                color: white
                            &, &:hover, &:active, &:focus
                                text-decoration: none

                &:first-child
                    margin-left: 0

            #mobile-menu-link
                position: relative
                display: none

            #mobile-menu
                $menu_size: 20px
                position: relative
                width: $menu_size
                height: $menu_size
                $border_style: 1px solid white

                span
                    //Прописать это всё по нормальному с переменными
                                       position: absolute
                                       width: 100%
                                       transition: transform .5s ease, opacity .5s ease
                                       border-radius: 50px

                #line1
                    top: 0
                    transform: rotate(0)
                    transform-origin: -2px 50%
                    border: $border_style

                #line2
                    top: calc(50% - 1px)
                    opacity: 1
                    border: $border_style

                #line3
                    top: calc(100% - 2px)
                    transform: rotate(0)
                    transform-origin: -3px 50%
                    border: $border_style

                &.active
                    #line1
                        transform: rotate(45deg)

                    #line2
                        opacity: 0

                    #line3
                        transform: rotate(-45deg)

            +mediascreensize_mobile
                &
                    margin-right: 0

                    #mobile-menu-link
                        display: block
                        padding: 25px
                #menu-list
                    //display: none
                    transform: scaleY(0)
                    transform-origin: top
                    z-index: 2
                    position: absolute
                    left: 0
                    right: 0
                    top: 0
                    flex-direction: column
                    margin-top: $menu_height
                    width: 100%
                    background: $menu_color
                    height: auto
                    padding: 0

                    .menu-item
                        margin: 0
                        padding-left: 20px
                        border-top: 1px solid #322f2e

                    &.open
                        //display: block
                        transform: scaleY(1)
                        transition: transform .5s

                    &.close
                        transition: transform .5s
</style>
