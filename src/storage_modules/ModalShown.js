export const moduleModalShown = {
    namespaced: true,// Локальное пространство имён
    state: {
        ModalShown: false,
        ModalComponent: ''
    }
    ,
    mutations: {
        ToggleModalShown(state, component) {
            state.ModalShown = !state.ModalShown
            state.ModalComponent = component
        },
    }
    ,
    actions: {
        ToggleModalShown({commit}, component) {
            commit('ToggleModalShown', component)
        }
    }
}
