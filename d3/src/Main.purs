module Main where

import Prelude

import Solution(WireMovement, toWireMovement, solution)

import Data.Array (length, fromFoldable) as Arr
import Data.List (List(..), (:))
import Data.List (fromFoldable) as Ls 
import Data.Maybe (Maybe(..))
import Data.String (Pattern(..), split)
import Effect (Effect)
import Effect.Console (log)
import Node.Encoding (Encoding(..))
import Node.FS.Sync (readTextFile)

extract :: forall a. Array (Maybe a) -> Maybe (Array a)
extract arr = if sameLength then Just extracted else Nothing
  where
    ls = Ls.fromFoldable arr

    extractGo Nil = Nil
    extractGo ((Nothing):_) = Nil
    extractGo ((Just x):next) = x:(extractGo next)
    extracted = extractGo >>> Arr.fromFoldable $ ls

    sameLength = Arr.length arr == Arr.length extracted 

processInput :: String -> Maybe (Array (Array WireMovement))
processInput = 
  split (Pattern "\n") >>>
  map (
    split (Pattern ",") >>>
    map toWireMovement >>>
    extract
  ) >>>
  extract

main :: Effect Unit
main = do
  contents <- processInput <$> readTextFile UTF8 "./input-1.txt" 
  log (show contents)