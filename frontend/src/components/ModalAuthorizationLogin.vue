<template>
    <div class="modal-content">
        <div class="modal-header modal-element">
            <span class="header-text">Вход</span>
        </div>
        <div class="modal-body modal-element">
            <form class="authorization" name="authorization" @submit.prevent @keyup.enter="login">
                <div class="form-item form-item--login">
                    <label for="blog-login"></label>
                    <input class="input" v-model="user.username" type="text" id="blog-login" placeholder="Логин"
                                                           autocomplete="off"/>
                </div>
                <div class="form-item form-item--password">
                    <label for="password"></label>
                    <input class="input" v-model="user.password" type="password" id="password"
                                                         placeholder="Пароль" autocomplete="off"/>
                </div>
                <span v-if="$store.state.AuthorizationStore.errorMessage" class="error_message">
                    {{ $store.state.AuthorizationStore.errorMessage }}
                </span>
                <input :disabled="!inputFilled" class="button button-authorize" value="Войти" type="submit"
                       @mousedown="login">
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
                this.$store.dispatch('AuthorizationStore/login', this.user)
            },
        },
        computed: {
            inputFilled: function () {
                return !!this.user.username && this.user.password
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
