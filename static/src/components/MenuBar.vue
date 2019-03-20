<template>
    <div id="nav-wrapper">
        <nav id="nav">
            <router-link class='nav-item'
                         v-for="routes in links"
                         v-bind:key="routes.id"
                         :to="`${routes.page}`">
                <p class="link-text">{{routes.text}}</p>
            </router-link>
        </nav>
    </div>
</template>

<script>
    export default {
        name: "MenuBar",
        data() {
            return {
                links: [
                    {
                        id: 0,
                        text: 'Rusty Blog',
                        page: '/'
                    },
                    {
                        id: 1,
                        text: 'About',
                        page: '/about'
                    },
                    {
                        id: 2,
                        text: 'Search',
                        page: '/search'
                    }
                ]
            }
        }
    }
</script>

<style scoped>
    #nav-wrapper {
        margin-bottom: 4rem;
        background-color: #9C89B8;
        display: flex;
        align-items: stretch;
        justify-content: center;
        min-height: 4rem;
    }

    #nav {
        display: flex;
        flex-direction: row;
        justify-content: flex-start;
        align-items: stretch;
        width: 60rem;
    }

    .nav-item {
        font-size: 24px;
        text-decoration: none;
        color: #F0E6EF;
    }

    .nav-item:hover {
        background-color: #EFC3E6;
        color: #B8BEDD;
    }

    .link-text {
        margin: 1rem
    }
</style>