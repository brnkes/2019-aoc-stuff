import R from 'ramda';
import fs from 'fs';

const splitInput = (windowSize) => R.compose(
    R.map(R.splitEvery(1)),
    R.splitEvery(windowSize)
);

function run_q1(dims, inputTxt) {
    const windowSize = dims.x * dims.y;

    const splat = splitInput(windowSize)(inputTxt);

    const leastZeroIdx = R.reduce(([acc,idx,result], next) => {
        const countOfZeroes = R.compose(
            R.prop(0),
            R.countBy(R.identity),
        )(next);

        if(countOfZeroes<acc) {
            return [countOfZeroes,idx+1,idx]
        } else {
            return [acc,idx+1,result]
        }
    }
    )([windowSize,0,-1], splat)[2];

    const mul1CountBy2Count = R.compose(
        ([a, b]) => a * b,
        counts => R.map(
            R.flip(R.apply)([counts])
        )([R.propOr(0,1), R.propOr(0,2)]),
        R.countBy(R.identity),
    )(splat[leastZeroIdx]);

    console.log(mul1CountBy2Count);
}

function test_q1() {
    run_q1({x:3,y:2},"123456789012");
}

const dims_question = {x: 25, y: 6};
const input_question = fs.readFileSync("./input.txt", 'utf8');
// run_q1(dims_question, input_question);
// test_q1();

function prettyprint(dims, arr) {
    R.compose(
        R.tap((x) => console.log(x)),
        R.map(R.join("")),
        R.splitEvery(dims.x),
        R.map((x) => {
            switch (x) {
                case 2 : return " ";
                case 0 : return " ";
                case 1 : return "X";
            }
        }),
    )(arr);
}

function run_q2(dims, inputTxt) {
    const windowSize = dims.x * dims.y;

    const res = R.compose(
        R.reduce((acc , nextLayer) =>
            R.zipWith((current,candidate) =>
                current !== 2 ? current : candidate
            , acc, nextLayer),
            R.repeat(2,windowSize)
        ),
        R.map(R.map(parseInt)),
        splitInput(windowSize)
    )(inputTxt);

    prettyprint(dims, res);
}

function test_q2() {
    run_q2({x:2,y:2}, "0222112222120000");
}

// test_q2();
run_q2(dims_question, input_question);