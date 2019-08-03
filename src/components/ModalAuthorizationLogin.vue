<template>
    <Modal key="modal">
        <template v-slot:header>
            <span class="header-text">Вход</span>
        </template>
        <template v-slot:body>
            <form class="authorization" name="authorization" @keypress.enter="sendLoginData" @submit="onSubmitForm">
                <div class="form-item form-item--login">
                    <input class="input" v-model="user.username" type="text" id="blog-login" placeholder="Логин"/>
                </div>
                <div class="form-item form-item--password">
                    <input class="input" v-model="user.password" type="password" id="password"
                           placeholder="Пароль"/>
                </div>
                <button class="button button-authorize" @click="sendLoginData">Войти</button>
            </form>
        </template>
        <template class="modal-footer" v-slot:footer>
            <span class="reg-link" @click="$emit('switch')">Создать аккаунт</span>
        </template>
    </Modal>
</template>

<script>
    import {HTTP} from '../server_defaults'
    import Modal from "@/components/Modal";

    export default {
        name: "ModalAuthorizationLogin",
        components: {Modal},
        data(){
            return{
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
            onSubmitForm: function (event) {
                event.preventDefault()
            },
        },
    }

</script>

<style lang="sass" scoped>

</style>
