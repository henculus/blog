import Vue from 'vue'
import VueRouter from "vue-router";
import ArticleList from "./components/ArticleList";
import ArticleComponent from "./components/ArticleComponent";
import store from "./storage";

Vue.use(VueRouter)

const routes = [
    {
        path: '/',
        redirect: '/articles',
        beforeEnter: (to, from, next) => {
            store.dispatch('AuthorizationStore/CheckAuthorize').then(
                result => {
                    console.log(result)
                    next()
                },
                error => {
                    console.error(error)
                    next()
                }
            )
        }
    },
    {
        path: '/articles',
        name: 'articles',
        component: ArticleList,
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
});
router.beforeEach((to, from, next) => {
    if (store.state.ModalShownStore.ModalShown) {
        store.dispatch('ModalShown/ToggleModalShown')
        next(false)
    } else
        next()
});

export default router
