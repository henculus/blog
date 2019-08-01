import axios from 'axios'
export const HTTP = axios.create({
    // Empty when in production, which means that publicPath used
    baseURL: 'http://lupusanay.me/api'
});
