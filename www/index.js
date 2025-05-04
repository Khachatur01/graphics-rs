import * as wasm from "whiteboard-rs";

const whiteboard = wasm.Whiteboard.new();

(function () {
    let canvas = document.querySelector('#canvas');
    const canvasRenderer = wasm.CanvasRenderer.new(canvas);

    let svg = document.querySelector('#svg');
    const svgRenderer = wasm.SVGRenderer.new(svg);

    canvas.addEventListener('mousedown', (event) => {
        whiteboard.mouse_down(event.offsetX, event.offsetY);
    });
    canvas.addEventListener('mousemove', (event) => {
        whiteboard.mouse_move(event.offsetX, event.offsetY);
    });
    canvas.addEventListener('mouseup', (event) => {
        whiteboard.mouse_up(event.offsetX, event.offsetY);
    });

    svg.addEventListener('mousedown', (event) => {
        whiteboard.mouse_down(event.offsetX, event.offsetY);
    });
    svg.addEventListener('mousemove', (event) => {
        whiteboard.mouse_move(event.offsetX, event.offsetY);
    });
    svg.addEventListener('mouseup', (event) => {
        whiteboard.mouse_up(event.offsetX, event.offsetY);
    });

    document.getElementById('rectangle').addEventListener('click', (event) => {
        whiteboard.activate_rectangle_tool();
    });

    document.getElementById('polygon').addEventListener('click', (event) => {
        whiteboard.activate_polygon_tool();
    });

    document.getElementById('select').addEventListener('click', (event) => {
        whiteboard.activate_select_tool();
    });

    function render() {
        whiteboard.render_canvas(canvasRenderer);
        whiteboard.render_svg(svgRenderer);
        window.requestAnimationFrame(render);
    }

    window.requestAnimationFrame(render);
})()
