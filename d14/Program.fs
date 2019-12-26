// Learn more about F# at http://fsharp.org

open Parser

let main_q1 argv =
    parseInput "./input_q1.txt"
    |> processInput
    |> printfn "%A"

[<EntryPoint>]
let main_q2 argv =
    parseInput "./input_q1.txt"
    |> processInputForMaxOres 1000000000000L
    |> printfn "%A"

    0 // return an integer exit code