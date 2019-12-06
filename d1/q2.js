import R from 'ramda';
import fs from 'fs';

const inputTxt = fs.readFileSync("./q2-input.txt", 'utf8');
const masses = inputTxt.trim().split("\n").map(x => Number(x));
const fuelRequired = x => Math.floor(x / 3) - 2;

const result = R.pipe(
    R.map(nextModule =>
        R.until(
            ({next}) => fuelRequired(next) <= 0,
            ({total, next}) => {
                const required = fuelRequired(next);
                return {
                    total : total + required,
                    next: required
                }
            },
        )({total : 0, next: nextModule})
    ),
    R.reduce((acc,{total:next}) => acc + next,0),
)(masses);

console.log(result);