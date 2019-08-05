<template>
    <div class="modal-content">
        <div class="modal-header modal-element">
            <span class="header-text">Вход</span>
        </div>
        <div class="modal-body modal-element">
            <form class="authorization" name="authorization" @submit.prevent @keyup.enter="sendLoginData">
                <div class="form-item form-item--login">
                    <input class="input" v-model="user.username" type="text" id="blog-login" placeholder="Логин"/>
                </div>
                <div class="form-item form-item--password">
                    <input class="input" v-model="user.password" type="password" id="password"
                           placeholder="Пароль"/>
                </div>
                <input class="button button-authorize" value="Войти" type="submit" @mousedown="sendLoginData">
            </form>
        </div>
        <div class="modal-footer modal-element">
            <span class="reg-link" @click="$emit('switch')">Создать аккаунт</span>
        </div>
    </div>
</template>

<script>
    import {HTTP} from '../server_defaults'

    export default {
        name: "ModalAuthorizationLogin",
        data() {
            return {
                user: {
                    username: '',
                    password: ''
                }
            }
        },
        methods: {
            sendLoginData: function () {
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

            },
        },
    }

</script>

<style lang="sass" scoped>
    @import "../modal_auth_style"
</style>
