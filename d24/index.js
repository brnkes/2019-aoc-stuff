import "./gpu-browser";
import Q1 from "./q1_input.txt";

const convertToNum = (qq) => {
    const inputInner = qq.split("\n").map(x => [0, ...x.split("").map((x) => x === "#" ? 1 : 0), 0]);
    const empty = [...Array(inputInner[0].length).keys()].map(() => 0);
    const final = [empty, ...inputInner, empty].map(x => new Float32Array(x));

    return final;
};

const input = convertToNum(Q1);
console.log(input);

const debugFn = (space) => {
    const d = space.map(x =>
        Array.from(x).map(
            y => y // === 0 ? "." : "#"
        ).join("")
    ).join("\n");

    console.log("DEBUG ====");
    console.log(d);
};


const spaceDims = [input.length,input[0].length];

const gpu = new GPU({
    mode: 'webgl2'
});

function computationFn (space) {
    const { thread: {x, y} } = this;

    if((x < 1 || x >= this.constants.dims - 1) || (y < 1 || y >= this.constants.dims - 1)) {
        return 0;
    }

    const neighbourA = space[y - 1][x];
    const neighbourB = space[y + 1][x];
    const neighbourC = space[y][x - 1];
    const neighbourD = space[y][x + 1];

    const sumOfNeighbours = neighbourA + neighbourB + neighbourC + neighbourD;

    if(space[y][x] > 0) {
        return sumOfNeighbours > 0 ? sumOfNeighbours > 1 ? 0 : 1 : 0;
    } else {
        return sumOfNeighbours > 0 ? sumOfNeighbours < 3 ? 1 : 0 : 0;
    }
};

const computation = gpu.createKernel(computationFn, {
    constants: { dims: spaceDims[0] },
    output: spaceDims
});

function isSameFn (bufCurrent, target) {
    for(let x = 0 ; x < this.constants.dims ; x ++) {
        for(let y = 0 ; y < this.constants.dims ; x ++) {
            if(bufCurrent[y][x] !== target[y][x]) {
                return 0;
            }
        }
    }

    return 1;
};

const isSame = gpu.createKernel(isSameFn, {
    output: [1],
    constants: {
        dims: spaceDims[0]
    },
});

function biodiversity(space) {
    let acc = 0;
    for(let r = 1 ; r < space.length-1; r++) {
        for(let c = 1 ; c < space[0].length-1 ; c++) {
            acc += space[r][c] > 0
                ? Math.pow(2, ((c-1) + (space.length-2)*(r-1)))
                : 0;
        }
    }

    return acc;
}

const divTest = `.....
.....
.....
#....
.#...`;

const divTestN = convertToNum(divTest);
const divTestR = biodiversity(divTestN);
if(divTestR !== 2129920) {
    throw `Incorrect biodiversity fn : ${divTestR} =/= 2129920`;
}

function run() {
    let currentTarget = input;
    debugFn(currentTarget);

    let spaces = [input];
    let spaceBiodiversities = [biodiversity(input)];

    const MAX_LOOPS = 64;

    for (let i = 1; i < MAX_LOOPS; i++) {
        console.log(">>", i);
        const result = computation(currentTarget);
        const resultBiodiversity = biodiversity(result);

        debugFn(result);

        for (let a = 0; a < spaceBiodiversities.length; a ++) {
            if(resultBiodiversity === spaceBiodiversities[a]) {
                console.log("Found : ", i, "=", a);
                debugFn(result);
                return resultBiodiversity;
            }
        }

        currentTarget = result;
        spaces = [...spaces, result];
        spaceBiodiversities = [...spaceBiodiversities, resultBiodiversity];
    }

    throw "Max loop count reached, no solution.";
}

const spaceResult = run();
console.log(spaceResult);
