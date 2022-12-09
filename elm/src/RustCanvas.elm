port module RustCanvas exposing (Msg(..), sendRustMsg, view, decodeValue, RustState, uninitialized)


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

type RustState 
    = RustState E.Value

uninitialized : RustState  
uninitialized = 
    RustState E.null


type Msg 
    = Focus
    | Unfocus
    | ChangeFOV Float


msgString : Msg -> String 
msgString msg =
    case msg of 
        Focus -> "Focus"
        Unfocus -> "Unfocus"
        ChangeFOV _ -> "ChangeFOV"


msgData : Msg -> List (String, E.Value)
msgData msg =
    case msg of 
        Focus -> []
        Unfocus -> []
        ChangeFOV degrees -> 
            [( "angle", E.float degrees)]


sendRustMsg : RustState -> Msg -> Cmd msg
sendRustMsg (RustState value) msg =
    let obj = 
            E.object <|
                [ ("rust_canvas", value)
                , ("msg"
                  , E.object <| 
                      ("type" , E.string (msgString msg))
                         :: (msgData msg)
                  )
                ]
    in 
    rustEvent obj


type alias RustMessage = 
    { fps: Int }


view : Html RustState
view =
    node "rust-canvas" 
        [ on "rust_state" <|
            D.map  RustState <|
                D.field "target"  <|
                    D.value
        , style "display" "block"
        ] 
        []


decodeValue : RustState -> D.Decoder a -> Result D.Error a 
decodeValue (RustState value) decoder = 
    D.decodeValue 
        ( D.at [ "_canvas", "rust_state" ] <|
            decoder
        )
        value



port rustEvent : E.Value -> Cmd msg
