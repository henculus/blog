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
                <input class="button button-authorize" value="Войти" type="submit" @mousedown="sendLoginData"
                       :disabled="isDisabled">
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
                },
                isDisabled: false
            }
        },
        methods: {
            /* eslint-disable */
            sendLoginData: function () {
                if (!this.isDisabled) {
                    this.isDisabled = true
                    let self = this
                    HTTP({
                        method: 'post',
                        url: '/session',
                        headers: {'Content-type': 'application/json'},
                        dataType: 'application/json',
                        data: this.user,
                        withCredentials: true,
                        crossDomain: true
                    }).then(
                        response => {
                            if (response.status !== 200) {
                                //Проверка кодов и вывод ошибок (проблемы с сервером, POST ошибка)
                                self.isDisabled = false
                            }
                            console.log(response)
                            HTTP.get('/session', {withCredentials: true})
                                .then(response => {
                                        if (response.status !== 200) {
                                            //Так же обработка ошибок (неверный пароль, юзера нет и тд)
                                            self.isDisabled = false
                                        }
                                        if (response.status === 200) {
                                            self.$store.dispatch('ModalShownStore/ToggleModalShown', '')
                                        }
                                        console.log(response)
                                    })
                                .catch(error => {
                                    //Ошибка get (А так бывает? o_0)
                                })
                        })
                        .catch(error => {
                            //Серьезная ошибка серва (например Cors :) )
                            self.isDisabled = false
                        })
                }
            },
        },
    }

</script>

<style lang="sass" scoped>
    @import "../modal_auth_style"
</style>
