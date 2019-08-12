<template>
    <div class="context-menu-wrapper">
        <div class="arrow"></div>
        <div class="arrow arrow--before"></div>
        <nav class="context-menu">
            <ul class="menu-list">
                <li class="menu-list__item">Пункт 1</li>
                <li class="menu-list__item">Пункт 2</li>
                <li class="menu-list__item">Пункт 3</li>
                <li class="menu-list__item" @click.once="logout">Выход</li>
            </ul>
        </nav>
    </div>
</template>

<script>
    //TODO Возможно начать сохранять состояние loading при выполнении запросов

    export default {
        name: "TopBarContextMenu",
        data() {
            return {}
        },
        methods: {
            logout: function () {
                this.$emit('closeContextMenu')
                this.$store.dispatch('AuthorizationStore/logout').then(
                    response => {
                        console.log(response, 'Результат удаления')
                        this.$store.dispatch('AuthorizationStore/CheckAuthorize').then(
                            response => console.log(response),
                            error => console.log(error, 'Успешный выход')
                        )
                    },
                    error => {
                        console.log(error, 'Ошибка удаления')
                        this.$store.dispatch('AuthorizationStore/CheckAuthorize').then(
                            response => console.log(response),
                            error => console.error(error)
                        )
                    }
                )
            }
        }
    }
</script>

<style lang="sass" scoped>
    @import "../variables"
    .context-menu-wrapper
        box-shadow: 0 0 2px rgb(235, 235, 235)
        border: 1px solid $menu_border_color
        border-radius: $block_border_radius
        right: 0
        left: 0
        top: $menu_height + 10px
        background: white
        display: block
        position: absolute
        margin: $content-padding-mobile
        transition: $ease_transition02

        .arrow
            z-index: 1
            top: -10px
            height: 10px
            right: 35px
            width: 0
            border-color: transparent
            border-style: dashed dashed solid
            border-width: 0 8.5px 8.5px
            border-bottom-color: #fff
            position: absolute

        .arrow--before
            z-index: 0
            top: -11px
            border-bottom-color: $menu_border_color

        .menu-list
            margin: 0
            padding: 0
            text-decoration: none
            list-style: none

            .menu-list__item
                cursor: pointer
                white-space: nowrap
                padding: 20px 10px
                border-bottom: 1px solid $menu_border_color
                transition: $ease_transition02

                &:hover
                    color: $rnt_green

                &:last-child
                    border-bottom: none

    +media_screensize_mobile_small
        .context-menu-wrapper
            left: auto
            min-width: 300px
            margin: $content-padding-mobile

    +media_screensize_mobile
        .context-menu-wrapper
            margin: 0
            min-width: 300px

            .arrow
                right: 15px
</style>
