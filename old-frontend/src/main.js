import Vue from 'vue'
import VueRouter from 'vue-router'
import VueCookie from 'vue-cookie'
import App from '@/App.vue'
import Post from "@/components/posts/Post";
import Posts from "@/components/posts/Posts";
import Auth from "@/components/auth/Auth";
import store from "@/store";

Vue.config.productionTip = false;

const routes = [
    {path: '/', component: Posts},
    {path: '/post/:id', component: Post},
    {path: '/auth', component: Auth}
];

Vue.use(VueCookie);
VueCookie.install(Vue);
Vue.use(VueRouter);

const router = new VueRouter({
    mode: 'history',
    routes: routes,
});

new Vue({
    router,
    store,
    render: h => h(App)
}).$mount('#app');