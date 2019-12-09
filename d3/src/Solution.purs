module Solution where

import Prelude

import Data.Foldable (foldl)
import Data.Generic.Rep (class Generic)
import Data.Generic.Rep.Show (genericShow)
import Data.Int (fromString) as I
import Data.Maybe (Maybe(..))
import Data.String (splitAt)

type Amount = Int

data Direction = U | D | R | L
derive instance genericDirection :: Generic Direction _
instance showDirection :: Show Direction where
  show = genericShow

toDirection :: String -> Maybe Direction
toDirection "D" = Just $ D
toDirection "U" = Just $ U
toDirection "R" = Just $ R
toDirection "L" = Just $ L
toDirection _ = Nothing

data WireMovement = WireMovement Direction Amount
derive instance genericWireMovement :: Generic WireMovement _
instance showWireMovement :: Show WireMovement where
  show = genericShow

toWireMovement :: String -> Maybe WireMovement
toWireMovement desc = 
  case (splitAt 1 $ desc) of
    { before , after } -> WireMovement <$> (toDirection before) <*> (I.fromString after)

type Space = { 
  xMin :: Int, xMax :: Int,
  yMin :: Int, yMax :: Int 
}
type Position = { x :: Int, y :: Int }

spaceEmpty = { xMin:0, xMax:0, yMin:0, yMax:0 } :: Space

allocateSpace :: Array WireMovement -> Space
allocateSpace arr = (foldl reduction accInitial arr).dim
  where
    positionInitial = { x:0, y:0 } :: Position
    accInitial = { dim : spaceEmpty, pos : positionInitial }

    reduction {dim: dimOld, pos: posOld} (WireMovement dir amt) = 
      let
        posNew = case dir of
          D -> posOld { y = posOld.y - amt }
          U -> posOld { y = posOld.y + amt }
          L -> posOld { x = posOld.x - amt }
          R -> posOld { x = posOld.x + amt } 
        dimNew = dimOld { 
          xMin = if posNew.x < dimOld.xMin then posNew.x else dimOld.xMin,
          xMax = if posNew.x > dimOld.xMax then posNew.x else dimOld.xMax,
          yMin = if posNew.y < dimOld.yMin then posNew.y else dimOld.yMin,
          yMax = if posNew.y > dimOld.yMax then posNew.y else dimOld.yMax
        }
      in {dim : dimNew, pos : posNew}

unionAllSpaces :: Array Space -> Space
unionAllSpaces = foldl unionTwoSpaces spaceEmpty
  where
    unionTwoSpaces :: Space -> Space -> Space
    unionTwoSpaces s1 s2 = { 
      xMin: min s1.xMin s2.xMin,
      yMin: min s1.yMin s2.yMin,
      xMax: max s1.xMax s2.xMax,
      yMax: max s1.yMax s2.yMax
    }

gatherSpaceInfo :: Array (Array WireMovement) -> Space
gatherSpaceInfo = 
  map allocateSpace >>> unionAllSpaces

solution :: Array (Array WireMovement) -> Int
solution input = solveGo
  where
    space = gatherSpaceInfo input
    solveGo
