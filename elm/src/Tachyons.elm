port module Tachyons exposing (..)

import Json.Decode as D

type alias TachyonsMedia =
    { ns : Bool
    , m : Bool
    , l : Bool
    }

decoder =
    D.map3 TachyonsMedia
        (D.field "ns" D.bool)
        (D.field "m" D.bool)
        (D.field "l" D.bool)


port getMedia : (TachyonsMedia -> msg) -> Sub msg
