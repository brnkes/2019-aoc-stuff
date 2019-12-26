// Learn more about F# at http://fsharp.org

open System
open System.Text.RegularExpressions

open Solve

let parseLine line =
    let matchResult = Regex.Matches(line, @"((?:\s)*(\d+) (\w+))")
    match matchResult.Count with
        | c when c >= 2 ->
            let g =
                matchResult
                |> Seq.map (fun x ->
                    (x.Groups.[2].Value.Trim() |> int, x.Groups.[3].Value.Trim())
                )
                |> Seq.toList
                |> List.rev
            Some { Result = List.head g ; Inputs = List.tail g |> List.rev }
        | _ -> None

let parsedInput = IO.File.ReadLines("./input.txt");

[<EntryPoint>]
let main argv =
    parsedInput
    |> Seq.map parseLine
    |> Seq.fold (fun acc next ->
        match (acc,next) with
            | (None,_) -> None
            | (Some(ls), Some(v)) -> Some(v :: ls)
            | _ -> None
    ) (Some([]))
    |> Option.map Solve.solve
    |> printfn "%A"
    
    0 // return an integer exit code
