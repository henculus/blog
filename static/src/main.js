import Vue from 'vue'
import VueRouter from 'vue-router'
import App from '@/App.vue'
import Post from "@/components/Post";
import Posts from "@/components/Posts";

Vue.config.productionTip = false;

const routes = [
    {path: '/', component: Posts},
    {path: '/post/:id', component: Post}
];

Vue.use(VueRouter);

const router = new VueRouter({
    mode: 'history',
    routes: routes,
});

new Vue({
    router,
    render: h => h(App)
}).$mount('#app');