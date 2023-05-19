export default defineEventHandler((event) => {
    return proxyRequest(
        event,
        `http://localhost:3001/${event.context.params!.path}`
    )
})