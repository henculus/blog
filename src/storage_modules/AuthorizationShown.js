export const moduleAuthorizationShown = {
    namespaced: true,// Локальное пространство имён
    state: {
        AuthorizationShown: false
    }
    ,
    mutations: {
        ToggleAuthorizationShown(state) {
            state.AuthorizationShown = !state.AuthorizationShown
        }
    }
    ,
    actions: {
        ToggleAuthorizationShown({commit}) {
            commit('ToggleAuthorizationShown')
        }
    }
}
