;(() => {
    let gripZoneHeight = 0;
    window.onmousedown = (e) => {
        if (e.clientY <= gripZoneHeight && e.button === 0) {
            window.__FLURX__.emit("FLURX|grip::grab", {
                x: e.clientX,
                y: e.clientY
            });
        }
    };
    window.onmouseup = (e) => {
        if (e.button === 0) {
            window.__FLURX__.emit("FLURX|grip::release", {
                __FLURX__grip_release: 0
            });
        }
    };
    window.__FLURX__.listen("FLURX|grip::resize", (height) => {
        gripZoneHeight = height;
    });
})();