module Solution.Spec where

import Prelude

import Solution (Direction(..), WireMovement(..), gatherSpaceInfo)
import Test.Spec (Spec, describe, it)
import Test.Spec.Assertions (shouldEqual)

spec :: Spec Unit
spec = do
  describe "gatherSpaceInfo" do
    it "simple space" do
      let 
        input = [[(WireMovement D 100),(WireMovement L 100),(WireMovement U 200),(WireMovement R 200)]]
        expectation = { xMax: 100, xMin: -100, yMax: 100, yMin: -100 }
      (gatherSpaceInfo input) `shouldEqual` expectation

    it "multiple spaces" do
      let
        input = [
          [(WireMovement D 100),(WireMovement L 100),(WireMovement U 200),(WireMovement R 200)],
          [(WireMovement D 50),(WireMovement L 300),(WireMovement U 500),(WireMovement R 10)]
        ]
        expectation = { xMax: 100, xMin: -300, yMax: 450, yMin: -100 }
      (gatherSpaceInfo input) `shouldEqual` expectation  

  -- describe "solution" do
      