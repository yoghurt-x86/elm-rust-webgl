module Main exposing (Msg(..), init, main, subscriptions, update, view)

import Browser
import RustCanvas exposing (Msg(..))
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
        , update = 
            \ msg model ->
                case model of 
                    Ok m ->
                        update msg m
                            |> Tuple.mapFirst Ok 

                    Err _ ->
                        (model, Cmd.none)
        , subscriptions = 
            \ m ->
                case m of 
                    Ok model -> subscriptions model
                    Err _ -> Sub.none 
        , view = 
            \ m ->
                case m of 
                    Ok model -> view model
                    Err e -> text (D.errorToString e)
        }


type alias Model =
    { host : String
    , tachyonsMedia : TachyonsMedia
    , flashMsg : Maybe String
    , rust_ref : RustCanvas.RustState
    , focused : Bool
    , input_fov : Float
    }


flagsDecoder =
    D.succeed Model
        |> D.required "host" D.string
        |> D.required "tachyonsMedia" Tachyons.decoder
        |> D.hardcoded Nothing
        |> D.hardcoded RustCanvas.uninitialized 
        |> D.hardcoded False
        |> D.hardcoded 90.0



init : E.Value -> ( Result D.Error Model, Cmd Msg )
init value =
    let
        week =
            D.decodeValue flagsDecoder value
    in
    case week of
        Ok res ->
            ( Ok res
            , Cmd.none
            )

        Err e ->
            (Err e, Cmd.none) 


-- UPDATE


type Msg
    = GotTachyonsMedia TachyonsMedia
    | SetFlash (Maybe String)
    | GotRustRef (RustCanvas.RustState)
    | KeyDown Key.Key 
    | KeyUp Key.Key 
    | InputFov Float



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
                    let rmsg = if model.focused then Unfocus else Focus
                    in
                    ( { model | focused = not model.focused }
                    , RustCanvas.sendRustMsg model.rust_ref rmsg
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

        InputFov float ->
            ( { model | input_fov = float }
            , RustCanvas.sendRustMsg model.rust_ref <|
                ChangeFOV float
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
    let time_res = 
            RustCanvas.decodeValue 
                model.rust_ref
                (D.field "fps" <|
                    D.float 
                )
        time = 
            Result.withDefault 0 time_res
    in
    section 
        [ attribute "style" "--pos:relative" ]  
        [ div 
            [ attribute "style" "--inset-top-left:10px; --pos:absolute;"  ] 
            [ h3 [] [ text (String.fromInt <| Basics.round time)] ]
        , div 
            [ attribute "style" "--inset-top-right:10px; --pos:absolute;"  ] 
            [ label [ for "input-fov"] 
                [ text <| "Fov: " ++ (String.fromFloat model.input_fov)
                , input 
                    [ id "input-fov"
                    , type_ "range"
                    , Html.Attributes.min "30"
                    , Html.Attributes.max "175"
                    , value (String.fromFloat model.input_fov)
                    , stopPropagationOn 
                        "input" 
                            (D.map (\x -> (x, True))
                                (targetValue
                                |> D.andThen ( \ str ->
                                        case String.toFloat str of
                                            Just f -> D.succeed (InputFov f)
                                            Nothing -> D.fail "fail"
                                    )
                                )
                            ) 
                    ] 
                    [] 
                ]
            , label [ for "input-light-color"] 
                [ text <| "Sunlight color: "                 
                , input 
                    [ id "input-light-color"
                    , type_ "color"
                    ] 
                    [] 
                ]
            , label [ for "input-ambient-color"] 
                [ text <| "ambient light color: "                 
                , input 
                    [ id "input-ambient-color"
                    , type_ "color"
                    ] 
                    [] 
                ]
            ]
        , RustCanvas.view
            |> Html.map GotRustRef
        ]
