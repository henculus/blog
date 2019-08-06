<template>
    <div class="modal-content">
        <div class="modal-header modal-element">
            <span class="header-text">Регистрация</span>
        </div>
        <div class="modal-body modal-element">
            <form class="authorization" name="authorization" @submit.prevent @keyup.enter="sendRegData">
                <div class="form-item form-item--login">
                    <input class="input" v-model="user.username" type="text" id="blog-login" placeholder="Логин"/>
                </div>
                <div class="form-item form-item--password">
                    <input class="input" v-model="user.password" type="password" id="password"
                           placeholder="Пароль"/>
                </div>
                <input class="button button-authorize" value="Зарегестрироваться" type="submit" @mousedown="sendRegData">
            </form>
        </div>
        <div class="modal-footer modal-element">
            <span class="reg-link" @click="$emit('switch')">Уже есть аккаунт</span>
        </div>
    </div>
</template>

<script>
    import {HTTP} from '../server_defaults'

    export default {
        name: "ModalAuthorizationRegistration",

        data() {
            return {
                user: {
                    username: '',
                    password: ''
                }
            }
        },
        methods: {
            sendRegData: function () {
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
            onSubmitForm: function (event) {
                event.preventDefault()
            },
        },
    }
</script>

<style lang="sass" scoped>
    @import "../modal_auth_style"
</style>
