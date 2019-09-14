import api from "../api"

export const moduleAuthorization = {
    namespaced: true,// Локальное пространство имён
    state: {
        exp: window.localStorage.getItem('exp'),
        sub: window.localStorage.getItem('sub') || '',
        iat: window.localStorage.getItem('iat'),
        isLoading: false
    },
    mutations: {
        CheckAuthorize(state, data) {
            state.exp = data.exp
            state.iat = data.iat
            state.sub = data.sub
        },
        StartLoading(state){
            state.isLoading = true
        },
        EndLoading(state){
            state.isLoading = false
        }
    }
    ,
    actions: {
        CheckAuthorize({commit}) {
            return new Promise((resolve, reject) => {
                api.getSession()
                    .then(response => {
                        if (response.status === 200) {
                            window.localStorage.setItem('sub', response.data.sub)
                            commit('CheckAuthorize', response.data)
                            resolve(response.data)
                        }
                    })
                    .catch(error => {
                        if (error.response) {
                            window.localStorage.removeItem('sub')
                            commit('CheckAuthorize', {})
                            reject(error.response.status)
                        } else
                            console.error('Упс, сервер упал')
                    })
            })
        },
        login: function (undefined, payload) {
            return new Promise((resolve, reject) => {
                api.auth(payload)
                    .then(
                        response => {
                            resolve(response)
                        })
                    .catch(
                        error => {
                            reject(error)
                        })
            })

        },
        registration: function(undefined, payload){
            return new Promise((resolve, reject)=> {
                api.registration(payload)
                    .then(
                    response => {
                        if (response.status === 200) {
                            //TODO Изменить, когда на серве после регистрации будет создаваться сессия
                            resolve(response)
                        } else { //Сюда сложно попасть (невозможно)
                            reject(response)
                        }
                    })
                    .catch(error => {
                        reject(error)
                    })
            })
        },
        logout: function () {
            return new Promise((resolve, reject) => {
                api.logout()
                    .then(response => {
                            resolve(response)
                        },
                        error => { //Уже разлогинен
                            reject(error)
                        }
                    )
                    .catch(error => {
                        reject(error)
                    })
            })
        },
        ToggleLoading({commit}) {
            commit('ToggleLoading')
        },
        StartLoading({commit}){
            commit('StartLoading')
        },
        EndLoading({commit}){
            commit('EndLoading')
        }

    },
    getters: {
        isAuthorized: state => !!state.sub
    },
}
