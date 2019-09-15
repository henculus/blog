import Vue from 'vue'
import Vuex from 'vuex'
import { moduleModal } from './storage_modules/ModalShown'
import { moduleAuthorization } from "./storage_modules/Authorization"

Vue.use(Vuex)

const store = new Vuex.Store({

    strict: true, // not for production

    modules: {
        ModalStore: moduleModal,
        AuthorizationStore: moduleAuthorization
    }
})

export default store
