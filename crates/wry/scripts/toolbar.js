;(() => {
    window.addEventListener('DOMContentLoaded', () => {
        const root = document.createElement('div');
        root.style.position = "absolute";

        document.body.parentElement.insertBefore(root, document.body);
        document.body.style.position = "relative";
        document.body.style.top = "<TOOLBAR_HEIGHT>";
        const toolbar = document.createElement("header");
        toolbar.onmousedown = async (e) => {
            await window.__FLURX__.core.invoke("FLURX|mouse::webview_move_start", {
                x: e.clientX,
                y: e.clientY
            });
        };
        toolbar.style.height = "<TOOLBAR_HEIGHT>";
        toolbar.style.color = "<TOOLBAR_COLOR>";
        toolbar.style.width = "100%";
        toolbar.style.position = "fixed";
        toolbar.style.top = "0";
        toolbar.style.zIndex = "calc(infinity)";

        root.appendChild(toolbar);
        root.appendChild(document.body);

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
    });
})();