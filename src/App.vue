<template>
    <div id="app">
        <top-menu></top-menu>
        <transition name="fade" mode="out-in">
            <router-view class="page-content"></router-view>
        </transition>
    </div>
</template>

<script>
    import TopMenu from './components/TopMenu'

    export default {
        name: 'app',
        components: {
            TopMenu,
        },
    }

    //Уровни необходимости 1 - простая необходимая задача 2 - посредственная, менее важная, более сложная 3 - при условии выполенения полного проекта
    //TODO 1. Ленивая загрузка картинок(1) 2. Редактор статей(3) 3. База данных (firebird) для работы со статьями, картинками и тд (2-3)
</script>

<style lang="sass">
    @import "normalize/normalize.css"
    @import "variables"
    html, body, #app
        position: relative
        top: 0
        //background: rgba(128, 128, 128, 0.02)
        text-rendering: optimizeLegibility
        font-family: $default_font
        //font-family: 'Montserrat Alternates', sans-serif;
        width: 100%
</style>
