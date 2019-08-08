<template>
    <div class="modal-content">
        <div class="modal-header modal-element">
            <span class="header-text">Вход</span>
        </div>
        <div class="modal-body modal-element">
            <form class="authorization" name="authorization" @submit.prevent @keyup.enter="sendLoginData">
                <div class="form-item form-item--login">
                    <input class="input" v-model="user.username" type="text" id="blog-login" placeholder="Логин"
                           autocomplete="off"/>
                </div>
                <div class="form-item form-item--password">
                    <input class="input" v-model="user.password" type="password" id="password"
                           placeholder="Пароль" autocomplete="off"/>
                </div>
                <span v-if="error_message" class="error_message">{{ error_message }}</span>
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
                    username: undefined,
                    password: undefined
                },
                error_message: undefined,
            }
        },
        methods: {
            /* eslint-disable */
            sendLoginData: function () {
                if (!this.isDisabled) {
                    this.error_message = undefined //Очищаем поле ошибки
                    this.$store.dispatch('AuthorizationStore/ToggleLoading') //Ставим isLoading в true
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
                            if (response.status === 200) { //Успешный вход
                                self.$store.dispatch('ModalShownStore/ToggleModalShown', '') //Отключаем отображение modal
                                self.$store.dispatch('AuthorizationStore/CheckAuthorize').then(() =>
                                    self.$store.dispatch('AuthorizationStore/ToggleLoading') //Ставим isLoading в false
                                )
                            }
                            else { //Сюда сложно попасть (невозможно)
                                self.$store.dispatch('AuthorizationStore/ToggleLoading') //Ставим isLoading в false (крайний случай)
                            }
                        })
                        .catch(error => {
                            //Ошибка от серва (404, 401 и тд)
                            self.$store.dispatch('AuthorizationStore/ToggleLoading') //Ставим isLoading в false
                            if(error.response) {
                                if (error.response.status === 404) {
                                    self.error_message = 'Такого пользователя нет'
                                }
                                if (error.response.status === 401) {
                                    self.error_message = 'Неверный пароль'
                                }
                            }
                            else
                                self.error_message = 'Ошибка сервера'
                        })
                }
            },
        },
        watch: {
            user: {
                handler: function (newData, oldData) {
                    this.error_message = undefined
                },
                deep: true
            }
        }
    }

</script>

<style lang="sass" scoped>
    @import "../modal_auth_style"
</style>
