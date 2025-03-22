import * as wasm from "whiteboard-rs";

const whiteboard = wasm.Whiteboard.new();

(function () {
    let container = document.getElementById('container');
    const canvasRenderer = wasm.CanvasRenderer.new(container);

    container.addEventListener('mousedown', (event) => {
        whiteboard.mouse_down(event.offsetX, event.offsetY);
    })
    container.addEventListener('mousemove', (event) => {
        whiteboard.mouse_move(event.offsetX, event.offsetY);
    })
    container.addEventListener('mouseup', (event) => {
        whiteboard.mouse_up(event.offsetX, event.offsetY);
    })

    setInterval(() => {
        whiteboard.render(canvasRenderer);
    }, 10);
})()
