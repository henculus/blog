<template>
    <div class="modal-content">
        <div class="modal-header modal-element">
            <span class="header-text">Вход</span>
        </div>
        <div class="modal-body modal-element">
            <form class="authorization" name="authorization" @submit.prevent @keyup.enter="login">
                <div class="form-item form-item--login">
                    <input class="input" v-model="user.username" type="text" id="blog-login" placeholder="Логин"
                           autocomplete="off"/>
                </div>
                <div class="form-item form-item--password">
                    <input class="input" v-model="user.password" type="password" id="password"
                           placeholder="Пароль" autocomplete="off"/>
                </div>
                <span v-if="error_message" class="error_message">{{ error_message }}</span>
                <input :disabled="!inputFilled" class="button button-authorize" value="Войти" type="submit" @mousedown="login">
            </form>
        </div>
        <div class="modal-footer modal-element">
            <span class="reg-link" @click="$emit('switch')">Создать аккаунт</span>
        </div>
    </div>
</template>

<script>
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
            login: function () {
                if (!this.$store.state.AuthorizationStore.isLoading&&
                    !this.$store.getters['AuthorizationStore/isAuthorized']) {
                    this.$store.dispatch('AuthorizationStore/ToggleLoading')
                    this.$store.dispatch('AuthorizationStore/login', this.user).then(
                        response => {
                            console.log(response)
                            this.$store.dispatch('AuthorizationStore/CheckAuthorize')
                                .then(
                                    response => {
                                        console.log(response)
                                        this.$store.dispatch('AuthorizationStore/ToggleLoading')
                                        this.$store.dispatch('ModalStore/HideModal')
                                    },
                                    error => {
                                        console.error(error)
                                        this.error_message = 'Ошибка создания сессии'
                                        this.$store.dispatch('AuthorizationStore/ToggleLoading')
                                    }
                                ).catch(
                                error => {
                                    console.error(error)
                                    this.error_message = 'Сервер не доступен'
                                    this.$store.dispatch('AuthorizationStore/ToggleLoading')
                                }
                            )
                        },
                        error => {
                            this.$store.dispatch('AuthorizationStore/ToggleLoading')
                            if (error.response) {
                                if (error.response.status === 404) {
                                    this.error_message = 'Такого пользователя нет'
                                }
                                if (error.response.status === 401) {
                                    this.error_message = 'Неверный пароль'
                                }
                            } else
                                this.error_message = 'Ошибка сервера'
                        }
                    )
                }
            },
        },
        computed: {
            inputFilled: function () {
                return !!this.user.username&&this.user.password
            }
        },
        watch: {
            user: {
                handler: function () {
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
