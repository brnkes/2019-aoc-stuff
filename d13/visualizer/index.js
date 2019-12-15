import "./index.css";
import * as wasm from "wasm-main-app";
import input from "../input.txt";

const repr_object = {
    0: " ",
    1: "🕳",
    2: "🔲",
    3: "📀",
    4: "⚽️"
};

const FRAMERATE = 15;

let recentlyPressedKey = 0;

const initializeGameRegion = (canvasSize, objects) => {
    console.log(`Init region`);

    const region = document.querySelector("#main-region");
    region.style.width = `${canvasSize.x_max-canvasSize.x_min}rem`;
    region.style.height = `${canvasSize.y_max-canvasSize.y_min}rem`;

    // objects.forEach(obj => {
    //     const el = document.createElement("div");
    //     el.textContent = repr_object[obj];
    //     region.appendChild(el);
    // });
};

const runGame = () => {
    let prevTimestamp = 0;
    let gameObject = wasm.Game.initialize(input);
    let hasInitializedGameCanvas = false;

    let object_queue_pre_canvas_prep = [];
    const put_to_object_queue = (ob) => {
        object_queue_pre_canvas_prep = [...object_queue_pre_canvas_prep, ...ob];
    };

    const processGameData = (timestamp) => {
        let notWaitingForInput = true;

        let watchdog = 5000;
        let oc = 0;
        while(notWaitingForInput) {
            watchdog--;
            if(watchdog < 0) {
                throw "Infinite loop."
            }

            const loopResult = gameObject.loop_once();

            if (loopResult !== 0) {
                if(!hasInitializedGameCanvas) {
                    initializeGameRegion(gameObject.get_arcade_size(), object_queue_pre_canvas_prep);
                    hasInitializedGameCanvas = true;
                }
            }

            switch (loopResult) {
                case 0 : {
                    if(!hasInitializedGameCanvas) {
                        put_to_object_queue(gameObject.get_output())
                    }
                    oc++;
                    break;
                }
                case 1 : {
                    gameObject.pass_input(BigInt(recentlyPressedKey));
                    notWaitingForInput = false;
                    console.log(`Outputs so far : ${oc}`);
                    break;
                }
                case 2: {
                    console.log(`Outputs so far : ${oc}`);
                    console.log("Game Over/Done/Whatev");
                    return;
                }
            }
        }

        prevTimestamp = timestamp;
        requestAnimationFrame(loop);
    };

    const loop = (timestamp) => {
        const sinceLastFrame = timestamp - prevTimestamp;
        if(sinceLastFrame < 1000 * FRAMERATE / 60) {
           return requestAnimationFrame(loop);
        }

        processGameData(timestamp);
    };

    return loop;
};

requestAnimationFrame(runGame());

document.addEventListener("keydown", event => {
    let paddleDir;
    switch (event.key) {
        case "ArrowLeft": { recentlyPressedKey = -1; break; }
        case "ArrowRight": { recentlyPressedKey = 1; break; }
        default:
            break;
    }
});

document.addEventListener("keyup", event => {
    recentlyPressedKey = 0;
});