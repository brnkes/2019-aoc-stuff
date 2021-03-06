module Solve

open FSharpx.Collections

type Chemical = string

type ChemicalAndQuantity = (int64 * Chemical)
type ReactionInputs = ChemicalAndQuantity list

type Reaction = {
    Result: ChemicalAndQuantity
    Inputs: ReactionInputs
}

type OutputToInputsMap = Map<Chemical,(int64 * ReactionInputs)>

let mergeMaps a b f =
    Map.fold (fun s k v ->
        match Map.tryFind k s with
        | Some v' -> Map.add k (f k (v, v')) s
        | None -> Map.add k v s
    ) a b

let rec generateMap ls m =
    match ls with
        | [] -> m
        | ({Result = (quantity, res) ; Inputs = ins}) :: xs -> generateMap (xs) (Map.add res (quantity,ins) m)

let traverseDependencies fuelCount m =    
    let rec reifyDependency leftovers (requiredQuantity, currentChemical) =
//        printfn ">>>>>> Processing : %A, required : %d" currentChemical requiredQuantity

        // Deduct from leftovers & attempt to return early.
        let (requiredQuantityAfterLeftovers, leftoversAfterSupplyingRequiredQuantity) =
            match Map.tryFind currentChemical leftovers with
            | None -> (requiredQuantity, 0L)
            | Some(vLeftovers) ->
                let diff = (requiredQuantity - vLeftovers)
                (max 0L diff, max 0L -diff)
                
        let leftoversAfterSupplyingCurrentRequirement =
            Map.updateWith (fun _ -> if leftoversAfterSupplyingRequiredQuantity > 0L then Some(leftoversAfterSupplyingRequiredQuantity |> int64) else None) currentChemical leftovers
            
        // Otherwise, execute the reaction - currentChemical's leftovers will be zero remain at this point
        let executeReaction = lazy (
            let (producedQuantity, inputs) = Map.find (currentChemical) m
            
            let reactionMultiplier = (requiredQuantityAfterLeftovers |> double) / (producedQuantity |> double) |> ceil |> int64
            let chemicalsQtyProducedWithThisReaction = reactionMultiplier * producedQuantity
                        
            let generateNewBacklogWithInputs acc (qNext,cNext) =
                Map.add cNext (reactionMultiplier * qNext) acc
            let mutable backlogWithRecentlyDiscoveredInputs = Seq.fold generateNewBacklogWithInputs Map.empty inputs
            
            let calculateLeftoversAndMutateBacklog leftoverKey =
                let leftoversExisting = Map.findOrDefault leftoverKey 0L leftoversAfterSupplyingCurrentRequirement
                
                let mutable excessInBacklog = -leftoversExisting
                let updateBacklog = (fun existingBacklog ->
                    excessInBacklog <- existingBacklog - leftoversExisting
                    if excessInBacklog <= 0L then None else Some(excessInBacklog) 
                )
                backlogWithRecentlyDiscoveredInputs <- Map.updateWith updateBacklog leftoverKey backlogWithRecentlyDiscoveredInputs
                
                (leftoverKey, if excessInBacklog >= 0L then 0L else -excessInBacklog)
                
            let leftoversNew =
                Set.union (Map.keySet leftoversAfterSupplyingCurrentRequirement) (Map.keySet backlogWithRecentlyDiscoveredInputs)
                |> Set.map calculateLeftoversAndMutateBacklog
                |> Set.filter (fun (_,quantity) -> quantity > 0L) 
                |> Set.toSeq
                |> Map.ofSeq
            
            let leftoversPostReaction = 
                match (Map.count backlogWithRecentlyDiscoveredInputs, (chemicalsQtyProducedWithThisReaction - requiredQuantityAfterLeftovers)) with
                    | (b,x) when b > 0 && x > 0L -> Map.add currentChemical x leftoversNew
                    | _ -> leftoversNew // All supplied from leftovers & reaction didn't occur OR chemicalsQty = requiredQty.
            
            (leftoversPostReaction, backlogWithRecentlyDiscoveredInputs)
        )
            
        match requiredQuantityAfterLeftovers with
            | 0L -> (leftoversAfterSupplyingCurrentRequirement, Map.empty)
            | _ -> executeReaction.Force()
            
    let rec loopBacklog leftovers backlog =
        let filterOutOres = Map.filter (fun chemical _-> chemical.Equals("ORE") |> not)
        let candidates = filterOutOres backlog 
        
//        printfn "==========="
//        printfn "Backlog Pre : %A" backlog
//        printfn "Leftovers Pre : %A" leftovers
        
        let (leftoversNew, backlogNew) =
            Map.fold (fun (leftoverC, backlogC) chemical _ ->
                let q = Map.find chemical backlogC 
                let backlogWithoutNextChemical = Map.remove chemical backlogC            
                let (leftoverNext, backlogNext) = reifyDependency leftoverC (q, chemical)
                let backlogF =  mergeMaps backlogNext backlogWithoutNextChemical (fun _ (v1,v2) -> v1+v2)
                
//                printfn "Backlog After %s : %A" chemical backlogF
//                printfn "Leftovers After %s : %A" chemical leftoverNext
//                printfn ""
                
                (leftoverNext, backlogF)

            ) (leftovers, backlog) candidates
       
        match backlogNew |> filterOutOres |> Map.count with
            | 0 -> (leftoversNew, backlogNew)
            | _ -> loopBacklog leftoversNew backlogNew
            
    loopBacklog Map.empty (Map.ofList [(("FUEL" : Chemical), fuelCount)])

// 'Reaction list -> 'a'
let solve fuelCount reactions =
    generateMap reactions Map.empty
//    |> (fun x -> printfn "%A" x; x)
    |> traverseDependencies fuelCount