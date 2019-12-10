module Solution where

import Prelude

import Control.Alt ((<|>))
import Data.ArrayBuffer.Typed (Index, at, empty, fill, length, unsafeAt)
import Data.ArrayBuffer.Types (ArrayView, Uint8)
import Data.Enum (enumFromTo)
import Data.Foldable (foldM, foldl, traverse_)
import Data.Generic.Rep (class Generic)
import Data.Generic.Rep.Show (genericShow)
import Data.Int (fromString) as I
import Data.List.Lazy (List(..))
import Data.Maybe (Maybe(..))
import Data.String (splitAt)
import Data.Traversable (traverse)
import Data.Typelevel.Num (lt)
import Data.UInt (UInt, fromInt)
import Effect (Effect)
import Effect.Ref (Ref)
import Effect.Ref (new, modify) as Ref
import Partial.Unsafe (unsafePartial)

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

type Offset = Position
type Dimension = Position
type WireSpace = { repr :: (ArrayView Uint8), offset :: Offset, dims :: Dimension }
type BlitAccumulator = {space :: WireSpace, pos :: Position }

translatePos :: WireSpace -> Position -> Int
translatePos {offset:{x:xOffset, y:yOffset}, dims:{x:xDim}} {x:xPos, y:yPos} = 
  (xPos + xOffset) + ((yPos + yOffset) * xDim)

type ReportMinDistance = Position -> Effect Unit 

applyWireMovement :: WireSpace -> ReportMinDistance -> Position -> WireMovement -> Effect Position
applyWireMovement space report pos (WireMovement direction amount) = do
  let
    walk acc _ = do
      let
        {x:pX,y:pY} = acc
        locUntranslated = case direction of
          D -> pos{ y = pY - 1 }
          U -> pos{ y = pY + 1 }
          L -> pos{ x = pX - 1 }
          R -> pos{ x = pX + 1 }
        locInMemory = translatePos space locUntranslated

      currentCount <- unsafePartial (unsafeAt space.repr (locInMemory))

      let newCountValue = (currentCount + (fromInt 1))
      if newCountValue > (fromInt 1) then report(locUntranslated) else pure unit

      fill newCountValue (locInMemory) (locInMemory+1) space.repr
      pure locUntranslated

  foldM walk pos (enumFromTo 0 amount :: List Int)

blitForWireMovement :: WireSpace -> ReportMinDistance -> Array WireMovement -> Effect Unit
blitForWireMovement wireSpace report movements = do
  foldM (applyWireMovement wireSpace report) { x:0, y:0 } movements
  $> unit

createMemoryForSpace :: Space -> Effect WireSpace
createMemoryForSpace space = do
  let 
    dimX = (space.xMax - space.xMin)
    dimY = (space.yMax - space.yMin)
  mem <- empty $ dimX * dimY 
  fill (fromInt 0) 0 (length mem) mem
  pure $ { repr:mem, offset:{ x:(negate space.xMin), y:(negate space.yMin) }, dims: {x:dimX, y:dimY} }

solution :: Array (Array WireMovement) -> Effect Int
solution input = do
  let spaceInfo = gatherSpaceInfo input  
  spaceMem <- createMemoryForSpace spaceInfo

  closestPositionMutRef <- (Ref.new Nothing) 
  let 
    report :: ReportMinDistance
    report pos = do
      Ref.modify mutateClosestPosReference closestPositionMutRef
      $> unit

      where
        mutateClosestPosReference =
          (\closestCurrent -> 
            returnClosest <$> pos <*> closestCurrent
            -- <|> Just pos
          )

        returnClosest :: Position -> Position -> Position
        returnClosest = (\new old -> 
          if distanceManhattanToOrigin new < distanceManhattanToOrigin old then new else old
        )

        distanceManhattan :: Position -> Position -> UInt
        distanceManhattan a b = fromInt $ a.x - b.x + a.y - b.y

        distanceManhattanToOrigin = (flip distanceManhattan) { x:0, y:0 }

  traverse (blitForWireMovement spaceMem report) input
  $> 1
  
