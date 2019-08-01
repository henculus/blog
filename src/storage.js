import Vue from 'vue'
import Vuex from 'vuex'

Vue.use(Vuex)

const store = new Vuex.Store({
    state: {
        AuthorizationShown: false
    },
    mutations: {
        ToggleAuthorizationShown(state)
        {
            state.AuthorizationShown = !state.AuthorizationShown
        }
    },
    actions:{
        ToggleAuthorizationShown({ commit })
        {
            commit('ToggleAuthorizationShown')
        }
    }
})

export default store
