<template>
    <form class="form" @submit.prevent="login">
        <h1>Sign in</h1>
        <input required v-model="username" type="text" placeholder="Username"/>
        <input required v-model="password" type="password" placeholder="Password">
        <button type="submit">Login</button>
    </form>
</template>

<script>
    export default {
        name: "Auth",
        methods: {
            login: async function () {
                const form_data = JSON.stringify({username: this.username, password: this.password});
                await this.$store.dispatch('login', form_data);
                this.$router.push('/')
            }
        }
    }
</script>

<style scoped>
    h1 {
        margin-bottom: .5rem;
    }

    .form {
        display: flex;
        flex-direction: column;
        margin: 10rem auto;
    }

    .form input {
        background-color: #F0E6EF;
        color: #9C89B8;
        padding: 10px 10px;
        justify-content: flex-start;
        border: 2px solid #9C89B8;
        border-radius: 3px;
        font-size: 16px;
        margin-bottom: .5rem;
    }
</style>