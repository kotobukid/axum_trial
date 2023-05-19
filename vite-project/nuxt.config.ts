// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
    // @ts-ignore
    devServer: {
        host: "localhost",
        https: false,
        port: 3001,
        url: "http://localhost:3001"
    },
})
