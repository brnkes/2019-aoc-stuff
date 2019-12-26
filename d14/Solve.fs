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
        let (producedQuantity, inputs) = Map.find (currentChemical) m
        
        let reactionMultiplier = (requiredQuantity |> float) / (producedQuantity |> float) |> ceil |> int
        
        // todo...
        let sumWithExistingBacklog acc (qNext,cNext) =
            Map.add cNext (reactionMultiplier * qNext) acc
        let mutable backlogNew = Seq.fold sumWithExistingBacklog Map.empty inputs
        
        let calculateLeftoversAndMutateBacklog leftoverKey =
            let leftoverQuantity = Map.findOrDefault leftoverKey 0 leftovers
            let mutable diff = 0
            
            let updateBacklog = (fun existing ->
                diff <- existing - leftoverQuantity
                if diff <= 0 then None else Some(diff) 
            )
            backlogNew <- Map.updateWith updateBacklog leftoverKey backlogNew
            
            (leftoverKey, if diff > 0 then 0 else -diff)
            
        let leftoversNew =
            Set.union (Map.keySet leftovers) (Map.keySet backlogNew)
            |> Set.map calculateLeftoversAndMutateBacklog
            |> Set.filter (fun (_,quantity) -> quantity > 0) 
            |> Set.toSeq
            |> Map.ofSeq
        
        (*let children = Map.filter (fun chemical _-> chemical.Equals("ORE") |> not) backlogNew
        
        (*Map.fold (fun (leftoverNext, backlogNext) chemical q ->
            let backlogWithoutNextChemical = Map.remove chemical backlogNext
            go leftoverNext backlogWithoutNextChemical (q, chemical) m
        ) (leftoversNew, backlogNew) children*)
        
        Map.fold (fun (leftoverC, backlogC) chemical q ->
            let merge a b f =
                Map.fold (fun s k v ->
                    match Map.tryFind k s with
                    | Some v' -> Map.add k (f k (v, v')) s
                    | None -> Map.add k v s
                ) a b
            let backlogWithoutNextChemical = Map.remove chemical backlogC
            let (leftoverNext, backlogNext) = go leftoverC Map.empty (q, chemical) m
            (leftoverNext, merge backlogNext backlogWithoutNextChemical (fun _ (v1,v2) -> v1+v2))
        ) (leftoversNew, backlogNew) children*)
        
        (backlogNew, leftoversNew)
        
    let rec loopBacklog leftovers backlog =
        let candidates = Map.filter (fun chemical _-> chemical.Equals("ORE") |> not) backlog
        
        let (leftoversNew, backlogNew) =
            Map.fold (fun (leftoverC,backlogC) chemical q ->
                let backlogWithoutNextChemical = Map.remove chemical backlogC            
                let (leftoverNext, backlogNext) = reifyDependency leftoverC (q, chemical)
                (leftoverNext, mergeMaps backlogNext backlogWithoutNextChemical (fun _ (v1,v2) -> v1+v2))
            ) (leftovers, backlog) candidates
        
        match Map.count backlogNew with
            | 0 -> loopBacklog leftoversNew backlogNew
            | _ -> (leftoversNew, backlogNew)
            
    loopBacklog Map.empty (Map.ofList [(("FUEL" : Chemical), 1)])

// 'Reaction list -> 'a'
let solve reactions =
    generateMap reactions Map.empty 
    |> traverseDependencies