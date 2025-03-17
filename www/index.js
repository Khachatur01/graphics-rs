import * as wasm from "graphics-rs";

const whiteboard = wasm.Whiteboard.new();

(function () {
    let element = document.getElementById('container');

    element.addEventListener('mousedown', (event) => {
        whiteboard.mouse_down(event.offsetX, event.offsetY);
    })
    element.addEventListener('mousemove', (event) => {
        whiteboard.mouse_move(event.offsetX, event.offsetY);
    })
    element.addEventListener('mouseup', (event) => {
        whiteboard.mouse_up(event.offsetX, event.offsetY);
    })

    setInterval(() => {
        console.log('rendering');
    }, 2000);
})()
