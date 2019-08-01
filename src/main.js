import Vue from 'vue'
import App from './App.vue'
import VueRouter from "vue-router"
import ArticleList from "@/components/ArticleList";
import ArticleComponent from "@/components/ArticleComponent";
import {watchForHover} from './hover_watcher'
import store from "@/storage";



Vue.config.productionTip = false

Vue.use(VueRouter)

if ('scrollRestoration' in history) { //Отключение дефолтной истории скролла браузера
    history.scrollRestoration = 'manual';
}

Vue.directive('click-outside', {
    bind(el, binding) {
        el.clickOutsideEvent = function (event) {
            if (!(el === event.target || el.contains(event.target))) {
                store.dispatch(binding.arg)
            }
        };
        document.body.addEventListener('mousedown', el.clickOutsideEvent);
    },
    unbind(el) {
        document.body.removeEventListener('mousedown', el.clickOutsideEvent);
    }
});


watchForHover(); //Коллим проверку на ховер

const routes = [
    {
        path: '/',
        redirect: '/articles',
    },
    {
        path: '/articles',
        name: 'articles',
        component: ArticleList
    },
    {
        path: '/articles/:id',
        name: 'article',
        component: ArticleComponent
    }
]

const router = new VueRouter({
    routes,
    mode: 'history',
    scrollBehavior(to, from, savedPosition) {
        return new Promise((resolve) => {
            setTimeout(() => {
                if (savedPosition) {
                    resolve(savedPosition)
                } else {
                    resolve({x: 0, y: 0})
                }
            }, 200)
        })
    }
})

new Vue({
    render: h => h(App),
    router,
    store
}).$mount('#app')
