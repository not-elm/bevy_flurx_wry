;(() => {
    window.addEventListener('DOMContentLoaded', () => {
        const root = document.createElement('div');
        root.style.display = "flex";
        root.style.flexDirection = "column";

        document.body.parentNode.insertBefore(root, document.body);

        const toolbar = document.createElement("div");
        toolbar.onmousedown = async() => {
            await window.__FLURX__.core.invoke("FLURX|mouse::webview_move_start");
        };
        toolbar.style.height = "20px";
        toolbar.style.display = "flex";
        toolbar.style.background = "red";

        root.appendChild(toolbar);
        root.appendChild(document.body);
        document.onmousedown = async (e) => {
            await window.__FLURX__.core.invoke("FLURX|mouse::down")
        }
        document.onmouseup = async (e) => {
            await window.__FLURX__.core.invoke("FLURX|mouse::up")
        }
        document.onmousemove = (e) => {
            window.__FLURX__.core.invoke("FLURX|mouse::move", {
                x: e.movementX,
                y: e.movementY
            })
        };
    });
})();