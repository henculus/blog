<template>
    <Modal>
        <template v-slot:header>
            <span>Вход</span>
        </template>
        <template v-slot:body>
            <form name="authorization" @keypress.enter="authorization">
                <div class="form-item form-item--login">
                    <input class="input" v-model="user.username" type="text" id="blog-login" placeholder="Логин"/>
                </div>
                <div class="form-item form-item--password">
                    <input class="input" v-model="user.password" type="password" id="password"
                           placeholder="Пароль"/>
                </div>
            </form>
        </template>
        <template v-slot:footer>
            <button class="button button-authorize" @click="authorization">Войти</button>
        </template>
    </Modal>
</template>

<script>
    import Modal from "@/components/Modal";
    import {HTTP} from '../server_defaults'

    export default {
        name: "ModalAuthorization",
        components: {
            Modal
        },
        data() {
            return {
                user: {
                    username: '',
                    password: ''
                }
            }
        },
        methods: {
            authorization: function () {
                HTTP({
                    method: 'post',
                    url: '/session',
                    headers: {'Content-type': 'application/json'},
                    dataType: 'application/json',
                    data: this.user,
                    withCredentials: true,
                    crossDomain: true
                }).then(response => {
                    /* eslint-disable */
                        console.log(response)
                        HTTP.get('/session', {withCredentials: true})
                            .then(response => console.log(response))
                    }
                )

            }
        },
        mounted() {

        }
    }
</script>


<style lang="sass" scoped>
    @import "../variables"
    .form-item
        margin-bottom: 10px

        &:last-child
            margin-bottom: 0

        .input
            height: 50px
            padding: 0 10px
            border-radius: $block_border_radius
            border: 1px solid $menu_border_color
            transition: all .2s ease

            &:focus
                border: 1px solid $rnt_green

    .button-authorize
        background: white
        border-radius: $block_border_radius
        border: 1px solid $menu_border_color
        height: 50px
        width: 100%
        transition: all .2s ease

    .no-touch .button-authorize:hover
        background: $rnt_green
        border: 1px solid $rnt_green
        color: white
</style>
