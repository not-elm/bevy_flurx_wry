;(() => {
    document.onmousedown = async (e) => {
        if (e.clientY <= 20 && e.button === 0) {
            await window.__FLURX__.core.invoke("FLURX|toolbar::grab", {
                x: e.clientX,
                y: e.clientY
            });
        }
    };
    document.onmouseup = async (e) => {
        if (e.button === 0) {
            await window.__FLURX__.core.invoke("FLURX|toolbar::release");
        }
    };
})();