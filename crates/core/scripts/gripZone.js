;(() => {
    Object.defineProperty(window.__FLURX__, "gripZoneHeight", {
        value: 0,
        writable: true,
        configurable: false,
    });

    window.onmousedown = (e) => {
        document.getElementById("count").innerText = e.button.toString()
        if (e.clientY <= window.__FLURX__.gripZoneHeight && e.button === 0) {
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
})();