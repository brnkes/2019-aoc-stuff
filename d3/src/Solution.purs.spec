module Solution.Spec where

import Prelude

import Data.Array (zip)
import Data.Maybe (Maybe(..))
import Data.Traversable (traverse)
import Data.Tuple (Tuple(..))
import Data.UInt (fromInt)
import Effect.Class (liftEffect)
import Effect.Ref (new, read) as Ref
import Solution (Direction(..), WireMovement(..), applyWireMovement, blitForWireMovement, createMemoryForSpace, debugWireSpace, gatherSpaceInfo, genReport, solution, translateMemoryLocationToPos, translatePosToMemoryLocation)
import Test.Spec (Spec, describe, it, itOnly)
import Test.Spec.Assertions (shouldEqual)

spec :: Spec Unit
spec = do
  let 
    spaceSample = {
      xMin : -2,
      xMax : 3,
      yMin : -3,
      yMax : 4
    }

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

  describe "createMemoryForSpace" do
    it "works properly" do
      mem <- liftEffect $ createMemoryForSpace spaceSample

      mem.dims.x `shouldEqual` 6
      mem.dims.y `shouldEqual` 8
      mem.offset.x `shouldEqual` 2
      mem.offset.y `shouldEqual` 3
    
  describe "applyWireMovement" do
    it "works properly" $ liftEffect do
      mem <- liftEffect $ createMemoryForSpace spaceSample

      pos <- applyWireMovement mem ({ x:0, y:0 }) (WireMovement U 3)
      pos `shouldEqual` { x:0, y:3 }
      debugWireSpace mem

      pos2 <- applyWireMovement mem pos (WireMovement R 3)
      pos2 `shouldEqual` { x:3, y:3 }
      debugWireSpace mem

      pos3 <- applyWireMovement mem pos2 (WireMovement D 2)
      pos3 `shouldEqual` { x:3, y:1 }
      debugWireSpace mem

  describe "translatePosToMemoryLocation & translateMemoryLocationToPos" do
    it "works properly" $ liftEffect do
      mem <- (createMemoryForSpace spaceSample)

      let samples = [
        { x: (negate 2), y:(negate 3) },
        { x: 0, y:(negate 3) },
        { x: 3, y:(negate 3) },
        { x: (negate 2), y:(negate 2) },
        { x: 0, y:(negate 2) },
        { x: 2, y:3 },
        { x: 3, y:3 },
        { x: 3, y:4 }
      ]

      let memLocExpectations = [
        0,2,5,6,8,40,41,47
      ]

      _ <- traverse 
        (\(Tuple x y) -> shouldEqual x y) 
        (zip (map (translatePosToMemoryLocation mem) samples) memLocExpectations)

      let backAndForth = translatePosToMemoryLocation mem >>> translateMemoryLocationToPos mem

      _ <- traverse 
        (\(Tuple x y) -> shouldEqual x y) 
        (zip (map backAndForth samples) samples)

      pure unit

  describe "genReport" do
    it "works properly" $ liftEffect do
      ref <- (Ref.new Nothing) 

      let reportWithRef = genReport ref

      reportWithRef { x:10, y:10 }
      Ref.read ref >>= shouldEqual (Just { x:10, y:10 })

      reportWithRef { x:15, y:10 }
      Ref.read ref >>= shouldEqual (Just { x:10, y:10 })

      reportWithRef { x:15, y:15 }
      Ref.read ref >>= shouldEqual (Just { x:10, y:10 })

      reportWithRef { x:9, y:10 }
      Ref.read ref >>= shouldEqual (Just { x:9, y:10 })

      reportWithRef { x:9, y:8 }
      Ref.read ref >>= shouldEqual (Just { x:9, y:8 })

  describe "solution" do
    it "yields Nothing if #(lines) = 1" do
      let 
        input = [
          [(WireMovement D 3),(WireMovement L 3),(WireMovement U 6)]
        ]

      result <- liftEffect $ solution input

      result `shouldEqual` Nothing

    it "yields Nothing if #(lines) > 1 but wires don't cross" do
      let 
        input = [
          [(WireMovement D 3),(WireMovement L 3)],
          [(WireMovement L 3),(WireMovement U 5)]
        ]

      result <- liftEffect $ solution input

      result `shouldEqual` Nothing

    it "ignores self-crossing wires" do
      let 
        input = [
          [(WireMovement D 3),(WireMovement L 3),(WireMovement U 2),(WireMovement R 4)],
          [(WireMovement L 3),(WireMovement U 5)]
        ]

      result <- liftEffect $ solution input

      result `shouldEqual` Nothing    

    it "works properly (simple example)" do
      let 
        input = [
          [(WireMovement D 1),(WireMovement L 2),(WireMovement U 2)],
          [(WireMovement L 3),(WireMovement U 5)]
        ]

      result <- liftEffect $ solution input

      result `shouldEqual` (Just $ fromInt 2)      