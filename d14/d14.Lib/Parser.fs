module Parser

open System
open System.Text.RegularExpressions

open Solve

let parseLine line =
    let matchResult = Regex.Matches(line, @"((?:\s)*(\d+) (\w+))")
    match matchResult.Count with
        | c when c >= 2 ->
            let groups =
                matchResult
                |> Seq.map (fun x ->
                    (x.Groups.[2].Value.Trim() |> int64, x.Groups.[3].Value.Trim())
                )
                |> Seq.toList
                |> List.rev
            Some { Result = List.head groups ; Inputs = List.tail groups |> List.rev }
        | _ -> None

let parseInput inputFileLoc = IO.File.ReadLines(inputFileLoc)

let processShared input =
    input
    |> Seq.map parseLine
    |> Seq.fold (fun acc next ->
        match (acc,next) with
            | (None,_) -> None
            | (Some(ls), Some(v)) -> Some(v :: ls)
            | _ -> None
    ) (Some([]))

let processInput input =
    input |> processShared |> Option.map (Solve.solve 1L)
    
let processInputForMaxOres oreLimit input =
    input
    |> processShared
    |> (fun reactionList ->
            Option.map (fun ls ->
                let rec go fuelCount firstCall previousTooLarge =
                    let res = Solve.solve fuelCount ls
                    let oresNeeded = res |> snd |> Map.find "ORE"
                    
                    let oresPerFuel = oresNeeded / fuelCount
                    let excessOres = oreLimit - oresNeeded
                    let fuelDeltaToApproximatelySatisfyExcessOres = (excessOres / oresPerFuel)
                    
//                    let approximateMultiplier = (oreLimit / (oresNeeded |> int64))
                    
                    printfn "Ores : %A, Fuel : %A, OPF : %A, DEL : %A" oresNeeded fuelCount oresPerFuel fuelDeltaToApproximatelySatisfyExcessOres
                    
                    if firstCall then
                        // Calling it directly as 1 Trillion >>>>>> Whatever input supplies.
                        go (fuelDeltaToApproximatelySatisfyExcessOres + fuelCount) false false
                    else
                        match (previousTooLarge, (oresNeeded |> int64) < oreLimit) with
                            | (true, true) -> printfn "Max fuel : %i" fuelCount
                            | (false, true) -> go (fuelCount + fuelDeltaToApproximatelySatisfyExcessOres) false false
                            | (_, false) -> go (fuelCount - 1L) false true
                    
                go 1L true false
            ) reactionList
        )