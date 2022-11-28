// https://www.hackerearth.com/practice/algorithms/searching/linear-search/practice-problems/algorithm/maximum-inequality-b002b193/
process.stdin.resume();
process.stdin.setEncoding('utf-8');
const readline = require('readline');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
});

const input_array:string[] = []

rl.on('line', (line: string) => {
    // console.log(line.length);
    input_array.push(line);
});

rl.once('close', () => {

    let idx_ = 0;
    let T = parseInt(input_array[idx_++].trim(), 10);
    for (let t_i = 0; t_i < T; t_i++) {
        let N = parseInt(input_array[idx_++].trim(), 10);
        let S = input_array[idx_++].trim();

        let out_ = solve(N, S);
        process.stdout.write(out_.toString());
        process.stdout.write('\n');
    }

    process.exit();
});


//
function solve(N: number, S: string) {
    // console.log(N);
    // console.log(S);

    let s1: string = '';
    let s2: string = '';
    let changes = 0;

    // for (let v of S) {
    for (let i=0; i<N; i++) {
        // console.log(i, S.charAt(i));
        const v = S[i];
        if (!s1) {
            s1 = v;
        } else if (!s2 && v === s1) {
            s2 = v;
        } else if (s1 !== v) {
            s1 = v;
            changes++;
        } else if (s2 && s2 !== v) {
            s2 = v;
            changes++;
        }
        // console.log(v, s1, s2, changes);
    }

    // Write your code here
    return changes;
}
