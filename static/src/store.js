import Vue from 'vue'
import Vuex from 'vuex'
import {HTTP} from '@/http-common'

Vue.use(Vuex);

// noinspection JSValidateTypes
const store = new Vuex.Store({
    state: {
        status: '',
        posts: [],
        user: {},
    },
    mutations: {
        auth_request(state) {
            state.status = 'loading';
        },
        auth_success(state, user) {
            state.status = 'success';
            state.user = user;
        },
        auth_error(state) {
            state.status = 'error';
        },
        logout(state) {
            state.status = '';
            state.user = {};
        },
    },
    actions: {
        async login({commit}, user) {
            commit('auth_request');
            try {
                await HTTP.post('/session', user);
                let response = await HTTP.get('/session');
                let payload_data = JSON.parse(response.data);
                commit('auth_success', payload_data)
            } catch (err) {
                commit('auth_error', err);
                throw err
            }
        }
    },
    getters: {
        isLoggedIn: state => !!state.user,
        authStatus: state => !!state.status,
        user: state => state.user
    }
});

export default store;