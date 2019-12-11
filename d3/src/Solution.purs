module Solution where

import Prelude
import Prelude

import Control.Alt ((<|>))
import Control.Apply (lift2)
import Data.Array ((!!))
import Data.ArrayBuffer.DataView (setUint8)
import Data.ArrayBuffer.Typed (Index, at, empty, fill, length, reduce, traverseWithIndex, traverseWithIndex_, unsafeAt, buffer)
import Data.ArrayBuffer.Types (ArrayView, Uint32)
import Data.Enum (enumFromTo)
import Data.Foldable (foldM, foldl, traverse_)
import Data.Generic.Rep (class Generic)
import Data.Generic.Rep.Show (genericShow)
import Data.Int (fromString) as I
import Data.Int (rem)
import Data.List.Lazy (List(..))
import Data.Maybe (Maybe(..))
import Data.Ord (abs)
import Data.Sequence (Seq, empty, snoc, toUnfoldable) as Seq
import Data.String (splitAt)
import Data.Traversable (sequence, traverse)
import Data.Tuple (Tuple(..), fst, snd)
import Data.Typelevel.Bool (and)
import Data.Typelevel.Num (lt)
import Data.UInt (UInt, fromInt, toInt)
import Effect (Effect)
import Effect.Class.Console (log)
import Effect.Ref (Ref)
import Effect.Ref (modify, new, read) as Ref
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
type WireSpace = { repr :: (ArrayView Uint32), offset :: Offset, dims :: Dimension }
type BlitAccumulator = {space :: WireSpace, pos :: Position }

debugWireSpace :: WireSpace -> Effect Unit
debugWireSpace {repr, dims} = do
  log "===========>"
  let blank = Seq.empty :: Seq.Seq UInt

  _ <- reduce (\acc val idx -> do
    let accNew = Seq.snoc acc val
    if ((idx+1) `mod` dims.x) == 0 
      then do
        log (foldl (\buf next -> buf <> " " <> (show <<< toInt $ next)) "" accNew)
        pure blank
      else pure $ accNew
  ) blank repr

  log "<==========="
  pure unit

translatePosToMemoryLocation :: WireSpace -> Position -> Int
translatePosToMemoryLocation {offset:{x:xOffset, y:yOffset}, dims:{x:xDim}} {x:xPos, y:yPos} = 
  (xPos + xOffset) + ((yPos + yOffset) * xDim)

translateMemoryLocationToPos :: WireSpace -> Int -> Position
translateMemoryLocationToPos {offset:{x:xOffset, y:yOffset}, dims:{x:xDim}} memoryPos = 
  let
    xPos = (memoryPos `rem` xDim) - xOffset
    yPos = ((memoryPos - xPos) `div` xDim) - yOffset
  in
    { x:xPos, y:yPos }

type ReportMinDistance = Position -> UInt -> Effect Unit 
type WireMovementTracker = {pos::Position, steps::UInt}

applyWireMovement :: WireSpace -> WireMovementTracker -> WireMovement -> Effect WireMovementTracker
applyWireMovement space {pos, steps} (WireMovement direction amount) = do
  let
    walk :: WireMovementTracker -> Int -> Effect WireMovementTracker
    walk {pos:locCurrent, steps:stepCurrent} _ = do
      let
        {x:pX,y:pY} = locCurrent
        locNewUntranslated = case direction of
          D -> pos{ y = pY - 1 }
          U -> pos{ y = pY + 1 }
          L -> pos{ x = pX - 1 }
          R -> pos{ x = pX + 1 }
        locInMemory = translatePosToMemoryLocation space locNewUntranslated
        stepPlus1 = stepCurrent + (fromInt 1)

      -- log $ "Writing to " <> (show locNewUntranslated)
      currentMemVal <- unsafePartial (unsafeAt space.repr (locInMemory))
      fill (if currentMemVal > (fromInt 0) then (min currentMemVal stepPlus1) else stepPlus1) (locInMemory) (locInMemory+1) space.repr
      -- debugWireSpace space
      pure {pos:locNewUntranslated, steps:stepPlus1}

  foldM walk {pos,steps} (enumFromTo 1 amount :: List Int)

blitForWireMovement :: Space -> Array WireMovement -> Effect WireSpace
blitForWireMovement spaceInfo movements = do
  spaceMem <- createMemoryForSpace spaceInfo

  log ("Space create")

  --log $ "off" <> (show spaceMem.offset)
  --log $ "dim" <> (show spaceMem.dims)

  _ <- foldM (applyWireMovement spaceMem) {pos:{ x:0, y:0 }, steps:(fromInt 0)} movements
  pure spaceMem

aggregateSpaces :: ReportMinDistance -> Space -> WireSpace -> WireSpace -> Effect Unit
aggregateSpaces report spaceInfo spaceA spaceB = do
  let
    applyToTarget :: Int → UInt → Effect Unit
    applyToTarget idx stepsB = do
      stepsA <- unsafePartial (unsafeAt spaceA.repr (idx))

      let
        totalSteps = (stepsA + stepsB)
        locUntranslated = translateMemoryLocationToPos spaceA idx

        shouldReport = 
          (stepsA > (fromInt 0)) &&
          (stepsB > (fromInt 0)) &&
          (locUntranslated /= {x:0, y:0})

      -- log ("!!!")
      -- log (show locUntranslated)
      -- log (show stepsA <> " + " <> show stepsB <> " = " <> show totalSteps <> " ±±± " <> (show $ shouldReport))
      if shouldReport then log ("Reporting >>>" <> (show locUntranslated)) else pure unit
      if shouldReport then log (show totalSteps) else pure unit
      if shouldReport then report locUntranslated  totalSteps else pure unit

  traverseWithIndex_ applyToTarget spaceB.repr

createMemoryForSpace :: Space -> Effect WireSpace
createMemoryForSpace space = do
  let 
    dimX = (space.xMax - space.xMin) + 1
    dimY = (space.yMax - space.yMin) + 1
  mem <- empty $ dimX * dimY 
  fill (fromInt 0) 0 (length mem) mem
  pure $ { repr:mem, offset:{ x:(negate space.xMin), y:(negate space.yMin) }, dims: {x:dimX, y:dimY} }

distanceManhattan :: Position -> Position -> UInt
distanceManhattan a b = fromInt $ abs (a.x - b.x) + abs (a.y - b.y)

distanceManhattanToOrigin :: Position -> UInt
distanceManhattanToOrigin = (flip distanceManhattan) { x:0, y:0 }

type ReportRefType = (Maybe (Tuple Position UInt))

genReport :: Ref ReportRefType -> ReportMinDistance
genReport leastStepsPositionMutRef posCandidate stepsCandidate = do
  Ref.modify mutateClosestPosReference leastStepsPositionMutRef
  $> unit

  where
    mutateClosestPosReference =
      (\bestSoFar -> case bestSoFar of
        Just best@(Tuple posBest stepsBest) -> Just $ pickNextBest
          where
            pickNextBest = 
              if stepsCandidate < stepsBest 
                then (Tuple posCandidate stepsCandidate)
                else best
        Nothing -> Just (Tuple posCandidate stepsCandidate)
      )

solution :: Array (Array WireMovement) -> Effect (Maybe UInt)
solution input = do
  let spaceInfo = gatherSpaceInfo input

  leastStepsPositionMutRef <- (Ref.new Nothing) :: Effect (Ref ReportRefType)
  wireSpaces <- traverse (blitForWireMovement spaceInfo) input

  log ("Blitted...")

  -- traverse_ debugWireSpace wireSpaces

  let aggregation = (aggregateSpaces (genReport leastStepsPositionMutRef) spaceInfo)
  _ <- sequence $ (lift2 aggregation) (wireSpaces !! 0) (wireSpaces !! 1)

  liftA1 snd <$> Ref.read leastStepsPositionMutRef

  --log (show latestPos)

  -- pure $ latestPos  
