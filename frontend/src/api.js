import axios from 'axios'

export const HTTP = axios.create({
    // Empty when in production, which means that publicPath used
    baseURL: `${process.env.VUE_APP_API_LOCATION || ''}/api`
})

axios.defaults.baseURL = `${process.env.VUE_APP_API_LOCATION || ''}/api`
axios.defaults.withCredentials = true
axios.defaults.headers.patch['Content-Type'] = 'application/json'
axios.defaults.headers.post['Content-Type'] = 'application/json'
axios.defaults.headers.put['Content-Type'] = 'application/json'


export default {
    getPost(postID) {
        return axios.get('/posts/' + postID)
    },

    getPosts() {
        return axios.get('/posts')
    },

    getAuthorPosts(author) {
        return axios.get(`/posts?author=${author}&limit=500`)
    },

    getSession() {
        return axios.get('/session')
    },

    auth(data) {
        return axios.post('/session', data)
    },

    registration(data) {
        return axios.post('/users', data)
    },

    logout() {
        return axios.delete('/session')
    },

    sendPost(post) {
        return axios.post('/posts', post)
    },

    patchPost(post, id) {
        console.log(post)
        return axios.patch(`/posts/${id}`, post)
    },

    sendImage(image) {
        return axios({
            method: 'POST',
            url: `/image`,
            data: image,
            config: { headers: {'Content-Type': 'multipart/form-data' }}
        })
    }
}
