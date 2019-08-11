import {HTTP} from '../server_defaults'

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
        ToggleLoading(state) {
            state.isLoading = !state.isLoading
        }
    }
    ,
    actions: {
        CheckAuthorize({commit}) {
            return new Promise((resolve, reject) => {
                HTTP.get('/session', {withCredentials: true})
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
                HTTP({
                    method: 'post',
                    url: '/session',
                    headers: {'Content-type': 'application/json'},
                    dataType: 'application/json',
                    data: payload,
                    withCredentials: true,
                    crossDomain: true
                })
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
        logout: function () {
            return new Promise((resolve, reject) => {
                HTTP.delete('session', {withCredentials: true})
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

    },
    getters: {
        isAuthorized: state => !!state.sub
    },
}
