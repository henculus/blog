import Vue from 'vue'
import App from './App'
import {watchForHover} from './hover_watcher'
import store from "@/storage"
import router from "@/router"
import '@/directives'

Vue.config.productionTip = false

if ('scrollRestoration' in history) { //Отключение дефолтной истории скролла браузера
    history.scrollRestoration = 'manual'
}

watchForHover() //Коллим проверку на ховер (добавление к body класса no-touch в случае не тач устройств)

new Vue({
    render: h => h(App),
    router,
    store
}).$mount('#app')
