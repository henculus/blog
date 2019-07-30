import Vue from 'vue'
import App from './App.vue'
import VueRouter from "vue-router"
import ArticleList from "@/components/ArticleList";
import ArticleComponent from "@/components/ArticleComponent";

Vue.config.productionTip = false

Vue.use(VueRouter)

if ('scrollRestoration' in history) { //Отключение дефолтной истории скролла браузера
    history.scrollRestoration = 'manual';
}

function watchForHover() {
    var hasHoverClass = false;
    var container = document.body;
    var lastTouchTime = 0;

    function enableHover() { //Добавляем .no-touch класс к body, если это не тач устройство
        if (new Date() - lastTouchTime < 500) return; //Если удерживать на таче долго, то hover не сработает
        if (hasHoverClass) return;

        container.className += ' no-touch';
        hasHoverClass = true;
    }

    function disableHover() {
        if (!hasHoverClass) return;
        container.className = container.className.replace(' no-touch', '');
        hasHoverClass = false;
    }

    function updateLastTouchTime() {
        lastTouchTime = new Date();
    }

    document.addEventListener('touchstart', updateLastTouchTime, true);
    document.addEventListener('touchend', disableHover, true);
    document.addEventListener('mousemove', enableHover, true);

    enableHover();
}

watchForHover();

Vue.directive('scroll', {
    inserted: function (el, binding) {
        let f = function (evt) {
            if (binding.value(evt, el)) {
                window.removeEventListener('scroll', f)
            }
        }
        window.addEventListener('scroll', f)
    }
})

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
    router
}).$mount('#app')
