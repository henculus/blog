<template>
    <div id="app">
        <nav-bar/>
        <router-view/>
        <footer>
            <p>Created by LupusAnay</p>
            <p>Contact information: <a href="mailto:lupusanay@gmail.com">
                lupusanay@gmail.com</a>.</p>
        </footer>
    </div>
</template>

<script>
    import NavBar from "@/components/nav/NavBar";

    export default {
        components: {NavBar}
    }
</script>

<style>
    html, body {
        width: 100%;
        height: 100%;
    }
    body {
        margin: 0;
        background-color: #F0E6EF;
    }

    footer {
        color: #F0E6EF;
        background-color: #9C89B8;
    }

    #app {
        color: #67678c;
        min-height: 100%;
        display: grid;
        grid-template-rows: auto 1fr auto;
        grid-template-columns: 100%;
        font-family: 'Avenir', Helvetica, Arial, sans-serif;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-align: center;
    }

    button {
        background-color: #F0E6EF;
        color: #67678c;
        justify-content: flex-start;
        cursor: pointer;
        border: 2px solid #9C89B8;
        border-radius: 3px;
        padding: 10px 32px;
        text-align: center;
        text-decoration: none;
        font-size: 16px;
    }

    button:hover {
        background-color: #9C89B8;
        color: #F0A6CA;
    }
</style>
