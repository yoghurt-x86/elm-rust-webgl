module Main exposing (Msg(..), init, main, subscriptions, update, view)

import Browser
import RustCanvas
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
import Browser
import Browser.Events
import Time
import Key


-- MAIN


main =
    Browser.element
        { init = init
        , update = update
        , subscriptions = subscriptions
        , view = view
        }


type alias Model =
    { host : String
    , tachyonsMedia : TachyonsMedia
    , flashMsg : Maybe String
    , rust_ref : E.Value
    , focused : Bool
    }


flagsDecoder =
    D.succeed Model
        |> D.required "host" D.string
        |> D.required "tachyonsMedia" Tachyons.decoder
        |> D.hardcoded Nothing
        |> D.hardcoded (E.string "uninitialized")
        |> D.hardcoded False



init : E.Value -> ( Model, Cmd Msg )
init value =
    let
        week =
            D.decodeValue flagsDecoder value
    in
    case week of
        Ok res ->
            ( res
            , Cmd.none
            )

        Err e ->
            let
                _ =
                    Debug.log "e" e
            in
            ( { host = "unknown" 
              , tachyonsMedia =
                    { ns = False
                    , m = False
                    , l = False
                    }
              , flashMsg = Nothing
              , rust_ref = E.string "unknown"
              , focused = False
              }
            , Cmd.none
            )


-- UPDATE


type Msg
    = GotTachyonsMedia TachyonsMedia
    | SetFlash (Maybe String)
    | GotRustRef (E.Value)
    | KeyDown Key.Key 
    | KeyUp Key.Key 



update : Msg -> Model -> ( Model, Cmd Msg )
update msg model =
    case msg of
        GotTachyonsMedia media -> 
            ( { model | tachyonsMedia = media }
            , Cmd.none
            )

        SetFlash flash ->
            ( { model | flashMsg = flash }
            , Cmd.none
            )

        GotRustRef value ->
            ( { model | rust_ref = value }
            , Cmd.none
            )

        KeyDown key ->
            case key of
                Key.Character 'z' ->
                    let rmsg = if model.focused then "unfocus" else "focus"
                    in
                    ( { model | focused = not model.focused }
                    , RustCanvas.rustEvent  <|
                        E.object 
                            [ ("msg", E.string rmsg) 
                            , ("rust_canvas", (model.rust_ref))
                            ]
                    )

                other -> 
                    let _ = Debug.log "key" other
                    in
                    ( model 
                    , Cmd.none
                    )

        KeyUp key ->
            ( model
            , Cmd.none
            )


-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch 
        [ Tachyons.getMedia GotTachyonsMedia
        , Browser.Events.onKeyDown 
            (D.map KeyDown Key.keyDecoder)
        , Browser.Events.onKeyUp 
            (D.map KeyUp Key.keyDecoder )
        ]

-- VIEW


view : Model -> Html Msg
view model =
    let decoder = 
            D.decodeValue  <|
                RustCanvas.stateDecoder <|
                    D.field "fps" <|
                        D.float 
        time = 
            (decoder model.rust_ref )
            |> Result.withDefault 0
    in
    section 
        [ class "relative" ]
        [ RustCanvas.view
            |> Html.map GotRustRef
        , div [ class "absolute top-0 left-0 f3 o-60 white bg-black-60" ] 
            [ text (String.fromInt <| Basics.round time)]
        ]
