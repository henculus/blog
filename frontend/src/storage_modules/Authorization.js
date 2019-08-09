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
        ToggleLoading({commit}) {
            commit('ToggleLoading')
        }

    },
    getters: {
        isAuthorized: state => !!state.sub
    },
    modules: {
        loginModule: {
            namespaced: true,
            state: {},
            mutations: {},
            actions: {
                sendLoginData: function ({dispatch }, payload) {
                    dispatch('AuthorizationStore/ToggleLoading', null, {root: true})
                    function rootDispatch(target) {
                        dispatch(target, null, {root: true})
                    }
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
                                        rootDispatch('ModalShownStore/ToggleModalShown')
                                        rootDispatch('AuthorizationStore/CheckAuthorize').then(() =>
                                            rootDispatch('AuthorizationStore/ToggleLoading')
                                        )
                                    } else { //Крайний случай
                                        rootDispatch('AuthorizationStore/ToggleLoading')
                                    }
                                })
                            .catch(
                                error => {
                                    dispatch('AuthorizationStore/ToggleLoading', null, {root: true})
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

                }
            },
            getters: {}
        }
    }
}
