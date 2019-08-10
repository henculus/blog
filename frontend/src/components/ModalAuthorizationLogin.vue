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
            sendLoginData: function () {
                if (!this.$store.state.AuthorizationStore.isLoading) {
                    this.$store.dispatch('AuthorizationStore/sendLoginData', this.user).then(
                        response => {
                            console.log('Auth done', response)
                        },
                        error => {
                            this.error_message = error
                        }
                    )
                }
            },
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
