import {HTTP} from '../server_defaults'

export const moduleAuthorization = {
    namespaced: true,// Локальное пространство имён
    state: {
        exp: window.localStorage.getItem('exp'),
        sub: window.localStorage.getItem('sub') || '',
        iat: window.localStorage.getItem('iat'),
        isLoading: false
    }
    ,
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
        sendLoginData: function ({dispatch }, payload) {
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
                            if (response.status === 200) { //Успешный вход
                                dispatch('ModalShownStore/ToggleModalShown', null, {root: true})
                                dispatch('CheckAuthorize').then(() =>
                                    dispatch('ToggleLoading')
                                )
                            } else { //Крайний случай
                                dispatch('ToggleLoading')
                            }
                        })
                    .catch(
                        error => {
                            dispatch('ToggleLoading')
                            if (error.response) {
                                if (error.response.status === 404) {
                                    reject('Такого пользователя нет')
                                }
                                if (error.response.status === 401) {
                                    reject('Неверный пароль')
                                }
                            } else
                                reject('Ошибка сервера')
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
