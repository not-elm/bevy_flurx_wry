;(() => {
    let gripZoneHeight = 0;
    document.onmousedown = async (e) => {
        if (e.clientY <= gripZoneHeight && e.button === 0) {
            await window.__FLURX__.core.invoke("FLURX|grip::grab", {
                x: e.clientX,
                y: e.clientY
            });
        }
    };
    document.onmouseup = async (e) => {
        if (e.button === 0) {
            await window.__FLURX__.core.invoke("FLURX|grip::release");
        }
    };
    window.__FLURX__.core.listen("FLURX|grip::resize", (height) => {
        gripZoneHeight = height;
    });
})();