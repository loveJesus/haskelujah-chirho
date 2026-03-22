{-
Copyright (c) 2008, 2009, 2026
Russell O'Connor, Christoffer Stjernlöf

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.
-}
-- | Functions for converting 'Colour' values to and from the Oklab space, and
-- convenience functions for the derived Oklch space.
-- See <https://bottosson.github.io/posts/oklab/>.
module Data.Colour.Ok
 (okLab, okLabView
 ,okLCh, okLChView
 )
where

import Data.Complex (Complex((:+)), magnitude, mkPolar, phase)
import Data.Fixed (mod')

import Data.Colour.CIE
import Data.Colour.CIE.Illuminant
import Data.Colour.Matrix

-- |Returns the Oklab coordinates of a colour, which is a
-- perceptually uniform colour space inspired by CIELAB
-- but which handles blue hues more accurately.
--
-- A white point is not specified because Oklab assumes
-- a standard 'd65' daylight illuminant.
okLabView :: Floating a => Colour a -> (a,a,a)
okLabView colour = (l,a,b)
 where
  (x,y,z) = cieXYZView colour
  lms = mult okM1 [x,y,z]
  cbrt v = signum v * (abs v)**(1/3)
  lms' = map cbrt lms
  [l,a,b] = mult (inverse okM2inv) lms'

-- |Returns the colour for given Oklab coordinates, which
-- is a perceptually uniform colour space inspired by
-- CIELAB but which handles blue hues more accurately.
--
-- A white point is not specified because Oklab assumes
-- a standard 'd65' daylight illuminant.
okLab :: Floating a => a -- ^L* coordinate (lightness)
                    -> a -- ^a* coordinate
                    -> a -- ^b* coordinate
                    -> Colour a
okLab l a b = cieXYZ x y z
 where
  lms' = mult okM2inv [l, a, b]
  lms = map (**3) lms'
  [x,y,z] = mult (inverse okM1) lms

-- |Returns the Oklab LCh coordinates of a colour. The
-- lightness coordinate is the same as in Oklab, while
-- the chroma C and hue h are the (a,b)-coordinates
-- expressed in polar form.
okLChView :: RealFloat a => Colour a -> (a,a,a)
okLChView colour = (l,c, h `mod'` 360)
 where
  (l,a,b) = okLabView colour
  z = a :+ b
  c = magnitude z
  h = phase z * 180 / pi

-- |Constructs a colour from a lightness, chroma, and
-- hue given in LCh polar coordinates correspondong to
-- an Oklab coordinate.
okLCh :: Floating a => a -- ^L* coordinate (lightness)
                    -> a -- ^C* coordinate (chroma)
                    -> a -- ^h* coordinate (hue)
                    -> Colour a
okLCh l c h = okLab l a b
 where
  (a :+ b) = mkPolar c (h * pi / 180)

--------------------------------------------------------------------------
{- not for export -}

-- |Converts XYZ coordinates to approximate human cone responses.
-- See <https://github.com/color-js/color.js/blob/6b487db289f7c41b78d2933d9cb83bf8c06f3c5e/scripts/oklab_matrix_maker.py>
oklms ::  Fractional a => Chromaticity a -- ^White point
                       -> [[a]]
oklms white_ch = zipWith f m0 lmsWhite
 where
  white = chromaColour white_ch 1.0
  (xw,yw,zw) = cieXYZView white
  lmsWhite = mult m0 [xw, yw, zw]
  f v w = (/w) <$> v
  m0 = [[0.77849780, 0.34399940, -0.12249720]
       ,[0.03303601, 0.93076195, 0.03620204]
       ,[0.05092917, 0.27933344, 0.66973739]
       ]

-- |The M1 matrix from the Oklab specification. This converts XYZ coordinates
-- to approximate human cone responses.
okM1 :: Fractional a => [[a]]
okM1 = oklms d65

-- |The inverse M2 matrix from the Oklab specification. This converts
-- perceptually uniform LAB coordinates into
-- non-linearly-transformed (l', m', s') coordinates.
-- See <https://github.com/color-js/color.js/blob/6b487db289f7c41b78d2933d9cb83bf8c06f3c5e/scripts/oklab_matrix_maker.py>
okM2inv :: Fractional a => [[a]]
okM2inv =
 [[1.0, 0.3963377774, 0.2158037573]
 ,[1.0, -0.1055613458, -0.0638541728]
 ,[1.0, -0.0894841775, -1.2914855480]
 ]
