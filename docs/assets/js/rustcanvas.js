import init, {Client, Movement} from "../../pkg/info_player_start.js";

const m = await init();

export class RustCanvas extends HTMLElement{
    constructor() {
        super();
        const canvas = document.createElement("canvas", { id: "canvas" });
        canvas.setAttribute("id", "canvas");
        this._canvas = canvas;

        canvas.held_keys = new Set();
        canvas.movement2 = new Movement();
        canvas.messages = [];
        document.addEventListener("keydown", (e) => this.handle_keydown(canvas, e), false)
        document.addEventListener("keyup", (e) => this.handle_keyup(canvas, e), false)
        canvas.addEventListener("mousemove", (e) => this.update_position(canvas, e), false);
    }

    handle_keydown(canvas, e) {
        canvas.held_keys.add(e.key);
    }

    handle_keyup(canvas, e) {
        canvas.held_keys.delete(e.key);
    }

    update_position(canvas, e) {
        const x = e.movementX;
        const y = e.movementY;
        if (document.pointerLockElement === canvas) {
            canvas.movement2.x = e.movementX + canvas.movement2.x;
            canvas.movement2.y = e.movementY + canvas.movement2.y;
        }
    }

    handle_msg(e) {
        this._canvas.messages.push(e.msg);
    }

    connectedCallback() {
        const canvas = this._canvas;
        this.appendChild(canvas);

        const gl = canvas.getContext("webgl", { antialias: true });
        if (!gl) {
            alert('Failed to initialize WebGL');
            return;
        }

        const client_promise = (new Client(canvas));

        client_promise.then( (client) => {
            canvas.client = client;

            const FPS_THROTTLE = 1000.0 / 300.0; // milliseconds / frames
            const JS_THROTTLE = 1000.0 / 10.0; // milliseconds / frames
            const js_event = new CustomEvent('rust_state', {});

            const initialTime = performance.now();
            let lastDrawTime = -1.0;// In milliseconds
            let lastJsTime = -1.0;// In milliseconds

            function render(time) {
                window.requestAnimationFrame(render);
                const currTime = time;

                if (currTime >= lastDrawTime + FPS_THROTTLE) {
                    lastDrawTime = currTime;

                    if (window.innerHeight !== canvas.height || window.innerWidth !== canvas.width) {
                        canvas.height = window.innerHeight;
                        canvas.style.height = window.innerHeight;

                        canvas.width = window.innerWidth;
                        canvas.style.width = window.innerWidth;

                        gl.viewport(0, 0, window.innerWidth, window.innerHeight);
                    }

                    let elapsedTime = currTime - initialTime;

                    canvas.rust_state = 
                        canvas.client.update(
                            elapsedTime, 
                            window.innerHeight, 
                            window.innerWidth, 
                            canvas.held_keys, 
                            canvas.movement2, 
                            document.pointerLockElement === canvas,
                            canvas.messages,
                            );


                    canvas.client.render();

                    //Clear things 
                    canvas.movement2.x = 0;
                    canvas.movement2.y = 0;
                    canvas.messages = [];

                    if (currTime >= lastJsTime + JS_THROTTLE) {
                        lastJsTime = currTime;
                        setTimeout(() => canvas.parentNode.dispatchEvent(js_event));
                    }
                }
            }
            render(initialTime);
        });
    }
}

