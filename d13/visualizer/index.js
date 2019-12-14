const repr_object = {
    0: " ",
    1: "🕳",
    2: "🔲",
    3: "📀",
    4: "⚽️"
};

const FRAMERATE = 15;

const processGameData = () => {

};

const updateGame = () => {
    let totalElapsed = 0;

    const loop = (timeElapsed) => {
        totalElapsed += timeElapsed;
        if(totalElapsed < 1000 * FRAMERATE / 60) {
            requestAnimationFrame(loop);
        }

        processGameData();

        totalElapsed = 0;
        requestAnimationFrame(loop);
    };

    return loop;
};

requestAnimationFrame(updateGame());