function watchForHover() {
    let hasHoverClass = false
    const container = document.body
    let lastTouchTime = 0

    function enableHover() { //Добавляем .no-touch класс к body, если это не тач устройство
        if (new Date() - lastTouchTime < 500) return //Если удерживать на таче долго, то hover не сработает
        if (hasHoverClass) return

        container.className += ' no-touch'
        hasHoverClass = true
    }

    function disableHover() {
        if (!hasHoverClass) return
        container.className = container.className.replace(' no-touch', '')
        hasHoverClass = false
    }

    function updateLastTouchTime() {
        lastTouchTime = new Date()
    }

    document.addEventListener('touchstart', updateLastTouchTime, true)
    document.addEventListener('touchend', disableHover, true)
    document.addEventListener('mousemove', enableHover, true)

    enableHover()
}
export {watchForHover}
