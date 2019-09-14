export const moduleModal = {
    namespaced: true,// Локальное пространство имён
    state: {
        ModalShown: false,
        ModalComponent: ''
    }
    ,
    mutations: {
        HideModal(state){
            state.ModalShown = false
            state.ModalComponent = undefined
        },
        ShowModal(state, component) {
            state.ModalShown = true
            state.ModalComponent = component
        },
    }
    ,
    actions: {
        HideModal({commit}) {
            commit('HideModal')
        },
        ShowModal({commit}, component) {
            commit('ShowModal', component)
        },
    }
}
