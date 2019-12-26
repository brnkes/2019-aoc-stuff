module d14.Tests

open System.IO
open NUnit.Framework
open Parser
open FSharpx.Collections

[<SetUp>]
let Setup () =
    ()
    
let boilerplate inputSrc testfn =
    let result = Path.Combine(__SOURCE_DIRECTORY__, inputSrc) |> parseInput |> processInput
    match result with
        | None -> Assert.Fail("Empty result.")
        | Some(res) -> testfn res

[<TestFixture>]
type TestProgram () =
    
    [<Test>]
    member this.Test1() =
        boilerplate "./test1.txt" (fun res ->
             printfn "%A" res
             Assert.That(snd res |> Map.find "ORE", Is.EqualTo(31))
        )
        
    [<Test>]
    member this.Test2() =
        boilerplate "./test2.txt" (fun res ->
             printfn "%A" res
             Assert.That(snd res |> Map.find "ORE", Is.EqualTo(165))
        )
        
    [<Test>]
    member this.Test3() =
        boilerplate "./test3.txt" (fun res ->
             printfn "%A" res
             Assert.That(snd res |> Map.find "ORE", Is.EqualTo(13312))
        )
        
    [<Test>]
    member this.Test4() =
        boilerplate "./test4.txt" (fun res ->
             printfn "%A" res
             Assert.That(snd res |> Map.find "ORE", Is.EqualTo(180697))
        )    


    [<Test>]
    member this.Test5() =
        boilerplate "./test5.txt" (fun res ->
             printfn "%A" res
             Assert.That(snd res |> Map.find "ORE", Is.EqualTo(2210736))
        )    

