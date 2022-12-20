port module RustCanvas exposing (Msg(..), RustState, decodeValue, sendRustMsg, uninitialized, view)

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
import Rust exposing (Msg(..))
import Tachyons exposing (TachyonsMedia)
import Time


type RustState
    = RustState E.Value


type Msg
    = State RustState
    | Event Rust.Event


uninitialized : RustState
uninitialized =
    RustState E.null


sendRustMsg : RustState -> Rust.Msg -> Cmd msg
sendRustMsg (RustState value) msg =
    let
        obj =
            E.object <|
                [ ( "rust_canvas", value )
                , ( "msg"
                  , Rust.msgEncoder msg
                  )
                ]
    in
    rustEvent obj


view : Rust.Global -> Html Msg
view global =
    node "rust-canvas"
        [ on "rust_state" <|
            D.map (RustState >> State) <|
                D.field "target" <|
                    D.value
        , on "rust_event" <|
            (D.map Event <|
                (D.field "detail" <|
                    Rust.eventDecoder
                )
            )
        , property "global"
            (Rust.globalEncoder global)
        , style "display" "block"
        ]
        []


decodeValue : RustState -> D.Decoder a -> Result D.Error a
decodeValue (RustState value) decoder =
    D.decodeValue
        (D.at [ "_canvas", "rust_state" ] <|
            decoder
        )
        value


port rustEvent : E.Value -> Cmd msg
