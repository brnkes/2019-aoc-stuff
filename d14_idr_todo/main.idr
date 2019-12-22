import Parse

Chemical : Type
Chemical = String

data Reaction = MkReaction (List Chemical) Chemical

-- Show Reaction where
--     show (MkReaction xs x) = "Reaction " ++ (show xs) ++ " -> " ++ (show x) 

processLine : String -> Reaction
processLine s = MkReaction ["yolo"] "hi" -- ?sss

main : JS_IO()
main = do
    s <- pure "10 A => 1 FUEL"
    let
        ll = ((map processLine) . lines) s
    -- printLn <$> ll
    pure ()