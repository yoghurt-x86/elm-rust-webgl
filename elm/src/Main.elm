module Main exposing (Msg(..), init, main, subscriptions, update, view)

import Browser
import Browser.Events
import Dict exposing (Dict)
import Hex
import Html exposing (..)
import Html.Attributes exposing (..)
import Html.Events exposing (..)
import Json.Decode as D
import Json.Decode.Pipeline as D
import Json.Encode as E
import Key
import Material.Icons.Types exposing (..)
import Rust exposing (Msg(..))
import RustCanvas
import Tachyons exposing (TachyonsMedia)



-- MAIN


main =
    Browser.element
        { init = init
        , update =
            \msg model ->
                case model of
                    Ok m ->
                        update msg m
                            |> Tuple.mapFirst Ok

                    Err _ ->
                        ( model, Cmd.none )
        , subscriptions =
            \m ->
                case m of
                    Ok model ->
                        subscriptions model

                    Err _ ->
                        Sub.none
        , view =
            \m ->
                case m of
                    Ok model ->
                        view model

                    Err e ->
                        text (D.errorToString e)
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
            ( Err e, Cmd.none )



-- UPDATE


type Msg
    = GotTachyonsMedia TachyonsMedia
    | SetFlash (Maybe String)
    | GotRustRef RustCanvas.RustState
    | KeyDown Key.Key
    | KeyUp Key.Key
    | InputFov Float
    | InputAmbientColor ( Float, Float, Float )
    | InputEnvColor ( Float, Float, Float )


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
                    let
                        rmsg =
                            if model.focused then
                                Unfocus

                            else
                                Focus
                    in
                    ( { model | focused = not model.focused }
                    , RustCanvas.sendRustMsg model.rust_ref rmsg
                    )

                other ->
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
                ChangeFov { angle = float }
            )

        InputEnvColor ( r, g, b ) ->
            ( model
            , RustCanvas.sendRustMsg model.rust_ref <|
                ChangeEnvLight { r = r, g = g, b = b }
            )

        InputAmbientColor ( r, g, b ) ->
            ( model
            , RustCanvas.sendRustMsg model.rust_ref <|
                ChangeAmbientLight { r = r, g = g, b = b }
            )



-- SUBSCRIPTIONS


subscriptions : Model -> Sub Msg
subscriptions model =
    Sub.batch
        [ Tachyons.getMedia GotTachyonsMedia
        , Browser.Events.onKeyDown
            (D.map KeyDown Key.keyDecoder)
        , Browser.Events.onKeyUp
            (D.map KeyUp Key.keyDecoder)
        ]



-- VIEW


colorInput attr inner =
    input
        (stopPropagationOn
            "input"
            (D.map (\x -> ( x, True ))
                (targetValue
                    |> D.andThen
                        (\str ->
                            let
                                slice i j =
                                    String.slice i j (String.toLower str)
                                        |> Hex.fromString
                                        |> Result.map (\integer -> Basics.toFloat integer / 255.0)

                                r_s =
                                    slice 1 3

                                g_s =
                                    slice 3 5

                                b_s =
                                    slice 5 7

                                rgb =
                                    Result.map3 (\r g b -> ( r, g, b )) r_s g_s b_s
                            in
                            case rgb of
                                Ok parsed ->
                                    D.succeed parsed

                                Err e ->
                                    D.fail e
                        )
                )
            )
            :: type_ "color"
            :: attr
        )
        inner


view : Model -> Html Msg
view model =
    let
        time_res =
            RustCanvas.decodeValue
                model.rust_ref
                (D.field "fps" <|
                    D.float
                )

        time =
            Result.withDefault 0 time_res

        toHex f32 =
            Hex.toString (Basics.round (f32 * 255))

        field name =
            D.map ((String.padLeft 2 '0') << toHex) <| D.field name D.float

        env_colors =
            RustCanvas.decodeValue
                model.rust_ref
                (D.map3 (\r g b -> "#" ++ r ++ g ++ b)
                    (field "env_light_color_r")
                    (field "env_light_color_g")
                    (field "env_light_color_b")
                )
                |> Result.withDefault "#000000"

        ambient_colors =
            RustCanvas.decodeValue
                model.rust_ref
                (D.map3 (\r g b -> "#" ++ r ++ g ++ b)
                    (field "ambient_light_color_r")
                    (field "ambient_light_color_g")
                    (field "ambient_light_color_b")
                )
                |> Result.withDefault "#000000"
    in
    section
        [ attribute "style" "--pos:relative" ]
        [ div
            [ attribute "style" "--inset-top-left:10px; --pos:absolute;" ]
            [ h3 [] [ text (String.fromInt <| Basics.round time) ] ]
        , div
            [ attribute "style" "--inset-top-right:10px; --pos:absolute;" ]
            [ label [ for "input-fov" ]
                [ text <| "Fov: " ++ String.fromFloat model.input_fov
                , input
                    [ id "input-fov"
                    , type_ "range"
                    , Html.Attributes.min "30"
                    , Html.Attributes.max "175"
                    , value (String.fromFloat model.input_fov)
                    , stopPropagationOn
                        "input"
                        (D.map (\x -> ( x, True ))
                            (targetValue
                                |> D.andThen
                                    (\str ->
                                        case String.toFloat str of
                                            Just f ->
                                                D.succeed f

                                            Nothing ->
                                                D.fail "fail"
                                    )
                            )
                        )
                    ]
                    []
                    |> Html.map InputFov
                ]
            , label [ for "input-light-color" ]
                [ text <| "Sunlight color: "
                , colorInput
                    [ id "input-light-color"
                    , value env_colors
                    ]
                    []
                    |> Html.map InputEnvColor
                ]
            , label [ for "input-ambient-color" ]
                [ text <| "ambient light color: "
                , colorInput
                    [ id "input-light-color"
                    , value ambient_colors
                    ]
                    []
                    |> Html.map InputAmbientColor
                ]
            ]
        , RustCanvas.view
            |> Html.map GotRustRef
        ]
