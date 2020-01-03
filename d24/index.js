import {compose, map, splitEvery, sum, tap, toString} from 'ramda';
import "./gpu-browser";
import Q1 from "./q1_input.txt";
// import Q1 from "./q1_test.txt";
import {runTests} from "./test";

function* loopOverspace(space) {
    for (let r = 1; r < space.length - 1; r++) {
        for (let c = 1; c < space[0].length - 1; c++) {
            yield { val:space[r][c], c, r };
        }
    }
}

export const convertToNum = (qq) => {
    const inputInner = qq.split("\n").map(x => [0, ...x.split("").map((x) => x === "#" ? 1 : 0), 0]);
    const empty = [...Array(inputInner[0].length).keys()].map(() => 0);
    const final = [empty, ...inputInner, empty].map(x => new Float32Array(x));

    return final;
};

const emptySpace = (dim) => [...Array(dim)].map(
    () => [...Array(dim)].map(() => 0)
);

const input = convertToNum(Q1);
console.log(input);

const debugFn = (space) => {
    console.log("DEBUG ====");
    console.log(strRepr(space));
};

const strRepr = (space) => {
    let buffer = "";
    for (const { val } of loopOverspace(space)) {
        buffer += val.toString()
    }

    const a = splitEvery(space[0].length - 2, buffer).join("\n");

    return a;
};

const spaceDims = [input.length, input[0].length];

const gpu = new GPU({
    mode: 'dev'
});

function range(begin, end) {
    return [...Array(end-begin+1).keys()].map(x => x+begin)
}

// todo : just use 7 instead of dims ?
function computationFn(space, upper, lower) {
    const {constants: {dims}, thread: {x, y}} = this;

    if ((x < 1 || x >= dims - 1) || (y < 1 || y >= dims - 1)) {
        return 0;
    }

    const middleIdx = Math.floor(dims / 2);
    if(x === middleIdx && y === middleIdx) {
        return 0;
    }

    let neighbourNorth;
    let neighbourSouth;
    let neighbourWest;
    let neighbourEast;

    if (y === 1) {
        neighbourNorth = upper[middleIdx - 1][middleIdx];
        neighbourSouth = space[y + 1][x];
    } else if (y === dims - 2) {
        neighbourNorth = space[y - 1][x];
        neighbourSouth = upper[middleIdx + 1][middleIdx];
    } else if (y === middleIdx - 1 && x === middleIdx) {
        neighbourNorth = space[y - 1][x];
        neighbourSouth = compose(
            sum,
            map((xL) => lower[1][xL]),
        )(range(1,dims-2));
    } else if (y === middleIdx + 1 && x === middleIdx) {
        neighbourNorth = compose(
            sum,
            map((xL) => lower[dims - 2][xL])
        )(range(1,dims-2));
        neighbourSouth = space[y + 1][x];
    } else {
        neighbourNorth = space[y - 1][x];
        neighbourSouth = space[y + 1][x];
    }

    if (x === 1) {
        neighbourWest = upper[middleIdx][middleIdx - 1];
        neighbourEast = space[y][x + 1];
    } else if (x === dims - 2) {
        neighbourWest = space[y][x - 1];
        neighbourEast = upper[middleIdx][middleIdx + 1];
    } else if (x === middleIdx - 1 && y === middleIdx) {
        neighbourWest = space[y][x - 1];
        neighbourEast = compose(
            sum,
            map((yL) => lower[yL][1])
        )(range(1,dims-2));
    } else if (x === middleIdx + 1 && y === middleIdx) {
        neighbourWest = compose(
            sum,
            map((yL) => lower[yL][dims - 2])
        )(range(1,dims-2));
        neighbourEast = space[y][x + 1];
    } else {
        neighbourWest = space[y][x - 1];
        neighbourEast = space[y][x + 1];
    }

    const sumOfNeighbours = neighbourNorth + neighbourSouth + neighbourWest + neighbourEast;

    if (space[y][x] > 0) {
        return sumOfNeighbours > 0 ? sumOfNeighbours > 1 ? 0 : 1 : 0;
    } else {
        return sumOfNeighbours > 0 ? sumOfNeighbours < 3 ? 1 : 0 : 0;
    }
};

const computation = gpu.createKernel(computationFn, {
    constants: {dims: spaceDims[0]},
    output: spaceDims
});

export function biodiversity(space) {
    let acc = 0;

    for (const { val, c, r } of loopOverspace(space)) {
        acc += val > 0
            ? Math.pow(2, ((c - 1) + (space.length - 2) * (r - 1)))
            : 0;
    }

    return acc;
}

export function isSpaceEmpty(space) {
    for (const { val } of loopOverspace(space)) {
        if(val === 1) {
            return false;
        }
    }

    return true;
}

export function totalBugs(space) {
    let acc = 0;

    for (const { val } of loopOverspace(space)) {
        acc += val
    }

    return acc;
}

function run() {
    let layersRoot = {
        spaceData: input,
        upper: null,
        lower: null,
        levelIdx: 0
    };

    const MINUTES = 200;

    for (let i = 1; i < MINUTES+1; i++) {
        console.log(">>", i);

        let depthProcessedTotal = 0;

        const process = (currentLevel, direction, freshLayer) => {
            const currentSpaceDataInitial = currentLevel.spaceData;
            const upper = (currentLevel.upper && currentLevel.upper.spaceData) || emptySpace(input.length);
            const lower = (currentLevel.lower && currentLevel.lower.spaceData) || emptySpace(input.length);

            // console.log("Pre @ level : ", currentLevel.levelIdx);
            // console.table(
            //     [[upper, currentLevel.spaceData, lower].map(strRepr)]
            // );

            const currentSpaceDataNext = computation(currentSpaceDataInitial, upper, lower);

            depthProcessedTotal+=1;

            // console.log("Post @ level : ", currentLevel.levelIdx);
            // console.table(
            //     [[upper, currentSpaceDataNext, lower].map(strRepr)]
            // );

            if(!freshLayer) {
                if (!currentLevel.upper) {
                    currentLevel.upper = {
                        spaceData: upper,
                        upper: null,
                        lower: currentLevel,
                        levelIdx: currentLevel.levelIdx + 1
                    };

                    process(currentLevel.upper, 1, true);

                    if (isSpaceEmpty(currentLevel.upper.spaceData)) {
                        currentLevel.upper = null;
                    }
                } else if (direction >= 0) {
                    process(currentLevel.upper, 1, false)
                }

                if (!currentLevel.lower) {
                    currentLevel.lower = {
                        spaceData: lower,
                        upper: currentLevel,
                        lower: null,
                        levelIdx: currentLevel.levelIdx - 1
                    };

                    process(currentLevel.lower, -1, true);

                    if (isSpaceEmpty(currentLevel.lower.spaceData)) {
                        currentLevel.lower = null;
                    }
                } else if (direction <= 0) {
                    process(currentLevel.lower, -1, false);
                }
            }

            currentLevel.spaceData = currentSpaceDataNext;
        };

        process(layersRoot, 0, false);

        console.log("Depth processed", depthProcessedTotal);

        if(i === MINUTES) {
            const sumLayers = (layer, direction) => {
                console.log("sl", layer);

                return totalBugs(layer.spaceData)
                    + ((direction >= 0 && layer.upper && sumLayers(layer.upper,1)) || 0)
                    + ((direction <= 0 && layer.lower && sumLayers(layer.lower,-1)) || 0);
            };

            return sumLayers(layersRoot, 0);
        }
    }

    throw "Max loop count reached, no solution.";
}

runTests();

const spaceResult = run();
console.log(spaceResult);
