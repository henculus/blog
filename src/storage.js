import Vue from 'vue'
import Vuex from 'vuex'
import { moduleAuthorizationShown } from './storage_modules/AuthorizationShown'

Vue.use(Vuex)

const store = new Vuex.Store({
    modules: {
        AuthShown: moduleAuthorizationShown
    }
})

export default store
