;(() => {
    let isDown = false;
    document.onmousedown = async (e) => {
        isDown = true;
        await window.__FLURX__.core.invoke("FLURX|mouse::down")
    }
    document.onmouseup = async (e) => {
        isDown = true;
        await window.__FLURX__.core.invoke("FLURX|mouse::up")
    }
    document.onmouseleave = async (e) => {
        if (isDown) {
            isDown = false;
            await window.__FLURX__.core.invoke("FLURX|mouse::up");
        }
    }
})();