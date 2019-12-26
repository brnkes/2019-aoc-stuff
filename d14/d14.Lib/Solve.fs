module Solve

open FSharpx.Collections

type Chemical = string

type ChemicalAndQuantity = (int * Chemical)
type ReactionInputs = ChemicalAndQuantity list

type Reaction = {
    Result: ChemicalAndQuantity
    Inputs: ReactionInputs
}

type OutputToInputsMap = Map<Chemical,(int * ReactionInputs)>

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

let traverseDependencies m =    
    let rec reifyDependency leftovers (requiredQuantity, currentChemical) =
        // Deduct from leftovers & attempt to return early.
        let (requiredQuantityAfterLeftovers, leftoversAfterSupplyingRequiredQuantity) =
            match Map.tryFind currentChemical leftovers with
            | None -> (requiredQuantity,0)
            | Some(vLeftovers) ->
                let diff = (requiredQuantity - vLeftovers)
                (max 0 diff, max 0 -diff)
                
        let leftoversAfterSupplyingCurrentRequirement =
            Map.updateWith (fun _ -> if leftoversAfterSupplyingRequiredQuantity > 0 then Some(leftoversAfterSupplyingRequiredQuantity) else None) currentChemical leftovers
            
        // Otherwise, execute the reaction - currentChemical's leftovers will be zero remain at this point
        let executeReaction = lazy (
            let (producedQuantity, inputs) = Map.find (currentChemical) m
            
            let reactionMultiplier = (requiredQuantityAfterLeftovers |> float) / (producedQuantity |> float) |> ceil |> int
            let chemicalsQtyProducedWithThisReaction = reactionMultiplier * producedQuantity
            
            let leftoversPostReaction = 
                match (chemicalsQtyProducedWithThisReaction - requiredQuantityAfterLeftovers) with
                    | x when x > 0 -> Map.add currentChemical x leftoversAfterSupplyingCurrentRequirement
                    | _ -> leftoversAfterSupplyingCurrentRequirement
                        
            let populateBacklogWithNewInputs acc (qNext,cNext) =
                Map.add cNext (reactionMultiplier * qNext) acc
            let mutable backlogWithRecentlyDiscoveredInputs = Seq.fold populateBacklogWithNewInputs Map.empty inputs
            
            let calculateLeftoversAndMutateBacklog leftoverKey =
                let leftoversExisting = Map.findOrDefault leftoverKey 0 leftoversPostReaction
                
                let mutable excessInBacklog = -leftoversExisting
                let updateBacklog = (fun existingBacklog ->
                    excessInBacklog <- existingBacklog - leftoversExisting
                    if excessInBacklog <= 0 then None else Some(excessInBacklog) 
                )
                backlogWithRecentlyDiscoveredInputs <- Map.updateWith updateBacklog leftoverKey backlogWithRecentlyDiscoveredInputs
                
                (leftoverKey, if excessInBacklog >= 0 then 0 else -excessInBacklog)
                
            let leftoversNew =
                Set.union (Map.keySet leftoversPostReaction) (Map.keySet backlogWithRecentlyDiscoveredInputs)
                |> Set.map calculateLeftoversAndMutateBacklog
                |> Set.filter (fun (_,quantity) -> quantity > 0) 
                |> Set.toSeq
                |> Map.ofSeq
            
            (leftoversNew, backlogWithRecentlyDiscoveredInputs)
        )
            
        match requiredQuantityAfterLeftovers with
            | 0 -> (leftoversAfterSupplyingCurrentRequirement, Map.empty)
            | _ -> executeReaction.Force()
            
    let rec loopBacklog leftovers backlog =
        let filterOutOres = Map.filter (fun chemical _-> chemical.Equals("ORE") |> not)
        let candidates = filterOutOres backlog 
        
        let (leftoversNew, backlogNew) =
            Map.fold (fun (leftoverC,backlogC) chemical q ->
                let backlogWithoutNextChemical = Map.remove chemical backlogC            
                let (leftoverNext, backlogNext) = reifyDependency leftoverC (q, chemical)
                (leftoverNext, mergeMaps backlogNext backlogWithoutNextChemical (fun _ (v1,v2) -> v1+v2))
            ) (leftovers, backlog) candidates
        
        match backlogNew |> filterOutOres |> Map.count with
            | 0 -> (leftoversNew, backlogNew)
            | _ -> loopBacklog leftoversNew backlogNew
            
    loopBacklog Map.empty (Map.ofList [(("FUEL" : Chemical), 1)])

// 'Reaction list -> 'a'
let solve reactions =
    generateMap reactions Map.empty
    |> (fun x -> printfn "%A" x; x)
    |> traverseDependencies