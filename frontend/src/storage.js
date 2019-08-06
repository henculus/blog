import Vue from 'vue'
import Vuex from 'vuex'
import { moduleModalShown } from './storage_modules/ModalShown'

Vue.use(Vuex)

const store = new Vuex.Store({
    modules: {
        ModalShownStore: moduleModalShown
    }
})

export default store
