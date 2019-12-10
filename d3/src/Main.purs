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
extract arr = extracted
  where
    extractGo Nil = Nothing
    extractGo (x:next) = Cons <$> x <*> (extractGo next)
    extracted = extractGo >>> map Arr.fromFoldable $ Ls.fromFoldable arr

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