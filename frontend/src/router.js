import Vue from 'vue'
import VueRouter from "vue-router"
import ArticleComponent from "./components/ArticleComponent"
import store from "./storage"
import UserPublications from "./components/UserPublications"
import ArticleFeed from "./components/ArticleFeed"
import ArticlesEditor from "./components/editor/ArticlesEditor"

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
        component: ArticleFeed,
    },
    {
        path: '/articles/:id',
        name: 'article',
        component: ArticleComponent
    },
    {
        path: '/my-publications',
        name: 'my-publications',
        component: UserPublications
    },
    {
        path: '/editor',
        name: "articles-editor",
        component: ArticlesEditor,
        beforeEnter: function (to, from, next) {
            store.dispatch('AuthorizationStore/CheckAuthorize').then(
                result => {
                    console.log(result)
                    next()
                },
                error => {
                    console.error(error)
                    next(from)
                }
            )
        }
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
// router.beforeEach((to, from, next) => {
//     if (store.state.ModalShownStore.ModalShown) {
//         store.dispatch('ModalShown/ToggleModalShown')
//         next(false)
//     } else
//         next()
// })

export default router
