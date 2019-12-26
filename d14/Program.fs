// Learn more about F# at http://fsharp.org

open Parser

[<EntryPoint>]
let main argv =
    parseInput "./input_q1.txt"
    |> processInput
    |> printfn "%A"
    
    0 // return an integer exit code