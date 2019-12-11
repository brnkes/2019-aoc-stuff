module Main where

import Prelude

import Control.Monad.Trans.Class (lift)
import Data.Array (length, fromFoldable) as Arr
import Data.List (List(..), (:))
import Data.List (fromFoldable) as Ls
import Data.Maybe (Maybe(..))
import Data.String (Pattern(..), split)
import Data.Traversable (sequence)
import Effect (Effect)
import Effect.Console (log)
import Effect.Unsafe (unsafePerformEffect)
import Node.Encoding (Encoding(..))
import Node.FS.Sync (readTextFile)
import Solution (WireMovement, toWireMovement, solution)

extract :: forall a. Array (Maybe a) -> Maybe (Array a)
extract arr = extracted
  where
    extractGo Nil = Just Nil
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
  result <- sequence $ solution <$> contents
  log (show result)

  -- (\contentJust -> do
  --   result <- solution contentJust
  --   log (show result)
  -- ) <$> contents