import axios from 'axios'
export const HTTP = axios.create({
    // Empty when in production, which means that publicPath used
    baseURL: `${process.env.VUE_APP_API_LOCATION || ''}/api`
})

axios.defaults.baseURL = `${ process.env.VUE_APP_API_LOCATION || '' }/api`
axios.defaults.withCredentials = true

HTTP.defaults.headers.post['Content-Type'] = 'application/json'
HTTP.defaults.headers.put['Content-Type'] = 'application/json'

export default {
    getPost (postID) {
        return axios.get('/posts/' + postID)
    },

    getPosts () {
        return axios.get('/posts')
    },

    getAuthorPosts (author) {
        return axios.get(`/posts?author=${author}`)
    },

    getSession () {
        return axios.get('/session')
    },

    auth (data) {
        return axios.post('/session', data)
    },

    registration (data) {
        return axios.post('/users', data)
    },

    logout () {
        return axios.delete('/session')
    },

    sendPost () {
        return axios.post('/posts')
    }
}
