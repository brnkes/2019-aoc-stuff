import "./index.css";
import * as wasm from "wasm-main-app";
import input from "../input-q2.txt";

const repr_object = {
    0: " ",
    1: "🔲",
    2: "🕳",
    3: "📀",
    4: "⚽️"
};

const FRAMERATE = 60;
const region = document.querySelector("#main-region");
const tiles = {

};

let recentlyPressedKey = 0;

const getIfScore = (threetuple) => {
    return threetuple[0] === -1n && threetuple[1] === 0n ? threetuple[2] : null;
}

const initializeGameRegion = (canvasSize, objects) => {
    console.log(`Init region`);

    region.style.width = `${canvasSize.x_max-canvasSize.x_min}rem`;
    region.style.height = `${canvasSize.y_max-canvasSize.y_min}rem`;

    objects.forEach(threetuple => {
        const el = document.createElement("div");
        el.classList.add("game-block");
        el.style.transform = `translate(${threetuple[0]}rem,${threetuple[1]}rem)`;
        region.appendChild(el);

        tiles[`${threetuple[0]},${threetuple[1]}`] = el;

        updateGame(threetuple)
    });
};

const updateScore = (score) => {
    const scoreEl = document.querySelector("#score");
    scoreEl.textContent = `${score}`;
};

const updateGame = (threetuple) => {
    const score = getIfScore(threetuple);
    if(score) {
        updateScore(score);
    } else {
        const el = tiles[`${threetuple[0]},${threetuple[1]}`];
        el.textContent = repr_object[threetuple[2]];
    }
};

const runGame = () => {
    let prevTimestamp = 0;
    let gameObject = wasm.Game.initialize(input);
    let hasInitializedGameCanvas = false;
    let notWaitingForInput;

    let object_queue_pre_canvas_prep = [];
    const put_to_object_queue = (ob) => {
        object_queue_pre_canvas_prep = [...object_queue_pre_canvas_prep, ob];
    };

    const processGameData = () => {
        // console.log("Process start");

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
                    const out = gameObject.get_output();
                    if(!hasInitializedGameCanvas) {
                        put_to_object_queue(out)
                    } else {
                        updateGame(out);
                    }
                    oc++;
                    break;
                }
                case 1 : {
                    // console.log(`Outputs so far A : ${oc}`);
                    notWaitingForInput = false;
                    break;
                }
                case 2: {
                    // console.log(`Outputs so far B : ${oc}`);
                    // console.log("Game Over/Done/Whatev");
                    return;
                }
            }

            // console.log("Terminate loop", loopResult);
        }

        gameObject.pass_input(BigInt(recentlyPressedKey));
        notWaitingForInput = true;

        requestAnimationFrame(loop);
    };

    const loop = (timestamp) => {
        const sinceLastFrame = timestamp - prevTimestamp;
        if(sinceLastFrame < 1000 / FRAMERATE) {
           return requestAnimationFrame(loop);
        }

        prevTimestamp = timestamp;
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