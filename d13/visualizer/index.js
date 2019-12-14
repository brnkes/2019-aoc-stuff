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

const initializeGameRegion = (canvasSize) => {
    const region = document.querySelector("#main-region");
    region.style.width = `${canvasSize.x_max-canvasSize.x_min}rem`;
    region.style.height = `${canvasSize.y_max-canvasSize.y_min}rem`;

};

const runGame = () => {
    let prevTimestamp = 0;
    let gameObject = wasm.Game.initialize(input);
    let hasInitializedGameCanvas = false;

    const processGameData = (timestamp) => {
        let gameInterpreterNotDone = true;

        let watchdog = 5000;
        let oc = 0;
        while(gameInterpreterNotDone) {
            watchdog--;
            if(watchdog < 0) {
                throw "Infinite loop."
            }

            const loopResult = gameObject.loop_once();

            switch (loopResult) {
                case 2: {
                    console.log("Game Over/Done/Whatev");
                    return;
                }
                case 0 : {
                    oc++;
                }
                case 1 : {
                    if(!hasInitializedGameCanvas) {
                        initializeGameRegion(gameObject.get_arcade_size());
                        hasInitializedGameCanvas = true;
                    }

                    gameObject.pass_input(BigInt(recentlyPressedKey));
                    gameInterpreterNotDone = false;
                }
            }
        }

        console.log(`Output count : ${oc}`);

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