import Vue from 'vue'
import Vuex from 'vuex'
import { moduleModalShown } from './storage_modules/ModalShown'
import { moduleAuthorization } from "./storage_modules/Authorization"

Vue.use(Vuex)

const store = new Vuex.Store({
    modules: {
        ModalShownStore: moduleModalShown,
        AuthorizationStore: moduleAuthorization
    }
})

export default store
