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
                           placeholder="Пароль" @blur="passwordState"/>
                </div>
                <div class="form-item form-item--password-repeat">
                    <input class="input" v-model="user.password_repeat" type="password" id="password-repeat"
                           placeholder="Повторите пароль" @blur="passwordState"/>
                </div>
                <span v-if="error_message" class="error_message">{{ error_message }}</span>
                <input class="button button-authorize" :disabled="!all_right" value="Зарегестрироваться" type="submit"
                       @mousedown="sendRegData">
            </form>
        </div>
        <div class="modal-footer modal-element">
            <span class="reg-link" @click="$emit('switch')">Уже есть аккаунт</span>
        </div>
    </div>
</template>

<script>

    export default {
        name: "ModalAuthorizationRegistration",

        data() {
            return {
                user: {
                    username: undefined,
                    password: undefined,
                    password_repeat: undefined
                },
                error_message: undefined,
            }
        },
        methods: {
            sendRegData: function () {
                let self = this
                self.$store.dispatch('AuthorizationStore/ToggleLoading')
                if (this.all_right) {
                    this.$store.dispatch('AuthorizationStore/registration', {username: this.user.username, password: this.user.password}).then(
                        response => {
                            console.log(response)
                            self.$store.dispatch('ModalShownStore/ToggleModalShown', '')
                            self.$store.dispatch('AuthorizationStore/ToggleLoading')
                        },
                        error => {
                            console.error(error)
                            self.$store.dispatch('AuthorizationStore/ToggleLoading')
                            if (error.response) {
                                if (error.response.status === 409)
                                    self.error_message = 'Такой пользователь уже есть'
                                else
                                    self.error_message = 'Некая ошибка' //TODO Сделать полную обработку ошибок
                                console.error(error.response)
                            } else
                                self.error_message = 'Ошибка сервера'
                        }
                    )
                }

            },
            passwordState: function () {
                if (!(this.passwords_match) && this.user.password_repeat)
                    this.error_message = 'Пароли не совпадают'
                else
                    this.error_message = undefined
            }

        },
        computed: {
            passwords_match: function () {
                return this.user.password === this.user.password_repeat
            },
            all_right: function () {
                return !!(this.passwords_match && this.user.username && this.user.password)
            }
        },
        watch: {
            user: {
                handler: function () {
                    this.error_message = undefined //Очищаем ошибку при вводе в поля
                },
                deep: true
            },
        }
    }
</script>

<style lang="sass" scoped>
    @import "../modal_auth_style"
</style>
