;(() => {
    window.onmousemove = (e) => {
        window.__FLURX__.emit("FLURX|grip::drag", {
            x: e.movementX,
            y: e.movementY,
        });
    };
})();