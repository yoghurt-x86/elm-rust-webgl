port module RustCanvas exposing (..)


import Browser
import Color
import Date
import Dict exposing (Dict)
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Http
import Json.Decode as D
import Json.Decode.Pipeline as D
import Json.Encode as E
import List.Extra as List
import Material.Icons
import Material.Icons.Outlined
import Material.Icons.Types exposing (..)
import Tachyons exposing (TachyonsMedia)
import Time


type alias RustMessage = 
    { fps: Int }


view : Html D.Value 
view =
    node "rust-canvas" 
        [ on "rust_state" <|
            D.field "target"  <|
                D.value
        ] 
        []


stateDecoder : D.Decoder a -> D.Decoder a
stateDecoder decoder = 
    D.at [ "_canvas", "rust_state" ] <|
        decoder

port rustEvent : E.Value -> Cmd msg
