module Main.Spec where

import Prelude

import Data.Maybe (Maybe(..))
import Main (extract, processInput)
import Test.Spec (Spec, describe, it)
import Test.Spec.Assertions (shouldEqual)

spec :: Spec Unit
spec = do
  describe "extract" do
    it "treates non-empty arrays properly" do
      let result = extract [(Just 1),(Just 2)]
      result `shouldEqual` (Just [1,2])
    it "will yield Nothing if at least one element is erroneous" do
      let result = extract [(Just 1),(Just 2),Nothing,(Just 5)]
      result `shouldEqual` Nothing

  describe "processInput" do
    it "works properly" do
      let
        result = processInput "R990,U475,L435\nL974,D745,R504"
        expected = "(Just [[(WireMovement R 990),(WireMovement U 475),(WireMovement L 435)],[(WireMovement L 974),(WireMovement D 745),(WireMovement R 504)]])"
      (show result) `shouldEqual` expected