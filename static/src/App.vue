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
    body {
        margin: 0;
        background-color: #F0E6EF;
    }

    #app {
        font-family: 'Avenir', Helvetica, Arial, sans-serif;
        -webkit-font-smoothing: antialiased;
        -moz-osx-font-smoothing: grayscale;
        text-align: center;
    }
</style>
