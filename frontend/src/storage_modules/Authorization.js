import {HTTP} from '../server_defaults'

export const moduleAuthorization = {
    namespaced: true,// Локальное пространство имён
    state: {
        exp: window.localStorage.getItem('exp'),
        sub: window.localStorage.getItem('sub') || '',
        iat: window.localStorage.getItem('iat')
    }
    ,
    mutations: {
        CheckAuthorize(state, data) {
            state.exp = data.exp
            state.iat = data.iat
            state.sub = data.sub

        },
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
                        // eslint-disable-next-line
                        window.localStorage.removeItem('sub')
                        commit('CheckAuthorize', {})
                        reject(error.response.status)
                    })
            })

        }
    },
    getters: {
        isAuthorized: state => !!state.sub
    }
}
