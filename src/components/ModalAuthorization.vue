<template>
    <keep-alive>
        <component
                @switch="currentComponent==='login-component' ? currentComponent='reg-component' : currentComponent='login-component'"
                :is="currentComponent"></component> <!--Придумать как сделать, чтобы модальное окно не перезагружалось-->
    </keep-alive>
</template>

<script>
    import ModalAuthorizationLogin from "@/components/ModalAuthorizationLogin";
    import ModalAuthorizationRegistration from "@/components/ModalAuthorizationRegistration";

    export default {
        name: "ModalAuthorization",
        components: {
            'login-component': ModalAuthorizationLogin,
            'reg-component': ModalAuthorizationRegistration
        },
        data() {
            return {
                currentComponent: 'login-component'
            }
        },

    }
</script>


<style lang="sass">
    @import "../variables"
    .authorization
        width: 100%

        .form-item
            margin-bottom: 10px
            width: 100%


            &:last-child
                margin-bottom: 0

            .input
                display: block
                position: relative
                height: 50px
                padding: 0 10px
                border-radius: $block_border_radius
                border: 1px solid $menu_border_color
                transition: all .3s ease
                width: 100%

                &:focus
                    border-color: $rnt_green

        .button-authorize
            border: none
            margin-top: 20px
            font-weight: bold
            background: $rnt_green
            color: white
            border-radius: $block_border_radius
            height: 50px
            width: 100%
            transition: all .2s ease

    .header-text
        font-size: 1.8em
        font-weight: bold
        color: $rnt_green

    .reg-link
        transition: $ease_transition02
        cursor: pointer
        font-size: 1em
        color: #007de6

    .no-touch
        .button-authorize:hover
            background: #3dd779
            color: white

        .reg-link:hover
            color: #00abf5
</style>
