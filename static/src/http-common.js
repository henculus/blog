import axios from 'axios';

export const HTTP = axios.create({
    // Empty when in production, which means that publicPath used
    baseURL: process.env.VUE_APP_API_LOCATION,
    headers: {
        Authorization: 'Bearer {token}'
    }
});