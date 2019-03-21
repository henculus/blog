<template>
    <div id="app">
        <MenuBar/>
        <router-view/>
        <footer>
            <p>Created by LupusAnay</p>
            <p>Contact information: <a href="mailto:lupusanay@gmail.com">
                lupusanay@gmail.com</a>.</p>
        </footer>
    </div>
</template>

<script>
    import MenuBar from "@/components/MenuBar";

    export default {
        components: {MenuBar}
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

    #app {
        min-height: 100%;
        display: grid;
        grid-template-rows: auto 1fr auto;
        grid-template-columns: 100%;
        font-family: 'Avenir', Helvetica, Arial, sans-serif;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-align: center;
    }
</style>
