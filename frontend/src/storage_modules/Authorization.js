import api from "../api"

export const moduleAuthorization = {
    namespaced: true,// Локальное пространство имён
    state: {
        exp: window.localStorage.getItem('exp'),
        sub: window.localStorage.getItem('sub') || '',
        iat: window.localStorage.getItem('iat'),
        isLoading: false,
        errorMessage: ''
    },
    mutations: {
        SET_SESSION(state, data) {
            state.exp = data.exp
            state.iat = data.iat
            state.sub = data.sub
        },
        START_LOADING(state) {
            state.isLoading = true
        },
        END_LOADING(state) {
            state.isLoading = false
        },
        SET_ERROR_MESSAGE(state, payload) {
            state.errorMessage = payload
        },
        CLEAR_ERROR_MESSAGE(state) {
            state.errorMessage = ''
        }
    },
    actions: {
        CheckAuthorize({commit}) {
            return new Promise((resolve, reject) => {
                api.getSession()
                    .then(response => {
                        if (response.status === 200) {
                            window.localStorage.setItem('sub', response.data.sub)
                            commit('SET_SESSION', response.data)
                            resolve(response.data)
                        }
                    })
                    .catch(error => {
                        if (error.response) {
                            window.localStorage.removeItem('sub')
                            commit('SET_SESSION', {})
                            reject(error.response.status)
                        } else
                            console.error('Упс, сервер упал')
                    })
            })
        },
        login: function ({commit, dispatch, getters}, payload) {
            if (!getters.isLoading) {
                commit('START_LOADING')
                api.auth(payload).then(
                    response => {
                        if (response.status === 200) {
                            api.getSession().then(
                                response => {
                                    commit('SET_SESSION', response.data)
                                    dispatch('ModalStore/HideModal', '', {root: true})
                                },
                                error => {
                                    console.error('Error creating session: ', error)
                                })
                        }
                    },
                    error => {
                        commit('SET_ERROR_MESSAGE', 'Ошибка входа')
                        console.error('Ошибка входа: ', error)
                        if (error.response) {
                            if (error.response.status === 404) {
                                commit('SET_ERROR_MESSAGE', 'Такого пользователя нет')
                            }
                            if (error.response.status === 401) {
                                commit('SET_ERROR_MESSAGE', 'Неверный пароль')
                            }
                        } else
                            commit('SET_ERROR_MESSAGE', 'Ошибка сервера')
                    }
                ).catch(
                    error => {
                        commit('SET_ERROR_MESSAGE', 'Сервер не доступен')
                        console.error('Ошибка сервера: ', error)
                    }
                ).finally(() => {
                    commit('END_LOADING')
                    commit('CLEAR_ERROR_MESSAGE')
                })
            }
        },
        registration: function ({commit, dispatch, getters}, payload) {
            if (!getters.isLoading) {
                commit('START_LOADING')
                api.registration(payload)
                    .then(
                        response => {
                            if (response.status === 200) {
                                api.getSession().then(
                                    response => {
                                        commit('SET_SESSION', response.data)
                                        dispatch('ModalStore/HideModal', '', {root: true})
                                    },
                                    error => {
                                        console.error('Error creating session: ', error)
                                    })
                            }
                        },
                        error => {
                            if (error.response) {
                                if (error.response.status === 409) {
                                    commit('SET_ERROR_MESSAGE', 'Такой пользователь уже есть')
                                } else {
                                    commit('SET_ERROR_MESSAGE', 'Ошибка регистрации')
                                }
                                console.error('Ошибка при регистрации: ', error.response)
                            } else {
                                commit('SET_ERROR_MESSAGE', 'Ошибка сервера')
                            }
                        }
                    ).catch(error => {
                    commit('SET_ERROR_MESSAGE', 'Ошибка сервера')
                    console.error('Ошибка при регистрации: ', error)
                }).finally(() => {
                    commit('END_LOADING')
                    commit('CLEAR_ERROR_MESSAGE')
                })
            }
        },
        logout: function ({commit, getters}) {
            if (!getters.isLoading) {
                commit('START_LOADING')
                api.logout()
                    .then(response => {
                            if (response.status === 200) {
                                api.getSession().then(
                                    response => {
                                        console.error('Error delete session: ', response)
                                    },
                                    error => {
                                        commit('SET_SESSION', {})
                                        console.log('Успешный выход: ', error)
                                    }).catch(
                                    error => {
                                        console.log('Успешный выход', error)
                                    }
                                )
                            }
                        },
                        error => { //Уже разлогинен
                            console.error('Сессия уже удалена: ', error)
                        }
                    )
                    .catch(error => {
                        console.error('Ошибка сервера: ', error)
                    }).finally(() => {
                    commit('END_LOADING')
                })
            }
        },
        ToggleLoading({commit}) {
            commit('ToggleLoading')
        },
        StartLoading({commit}) {
            commit('StartLoading')
        },
        EndLoading({commit}) {
            commit('EndLoading')
        }

    },
    getters: {
        isAuthorized: state => !!state.sub,
        isLoading: state => !!state.isLoading
    },
}
