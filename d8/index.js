import R from 'ramda';
import fs from 'fs';

function run(dims, inputTxt) {
    const windowSize = dims.x * dims.y;

    const splat = R.compose(
        R.map(R.splitEvery(1)),
        R.splitEvery(windowSize)
    )(inputTxt);

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

run({x: 25, y: 6}, fs.readFileSync("./input.txt", 'utf8'));
// test();

function test() {
    run({x:3,y:2},"123456789012");
}