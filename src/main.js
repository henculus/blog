import Vue from 'vue'
import App from './App.vue'
import VueRouter from "vue-router"
import ArticleList from "@/components/ArticleList";
import ArticleComponent from "@/components/ArticleComponent";

Vue.config.productionTip = false

Vue.use(VueRouter)

if ('scrollRestoration' in history) {
    history.scrollRestoration = 'manual';
}


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
            }, 150)
        })
    }
})

new Vue({
    render: h => h(App),
    router
}).$mount('#app')
