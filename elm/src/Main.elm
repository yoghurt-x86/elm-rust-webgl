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
import Rust exposing (Msg(..), Skybox(..))
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
    , rust_global : Rust.Global
    , focused : Bool
    , input_fov : Float
    , input_skybox : Rust.Skybox
    }


flagsDecoder =
    D.succeed Model
        |> D.required "host" D.string
        |> D.required "tachyonsMedia" Tachyons.decoder
        |> D.hardcoded Nothing
        |> D.hardcoded RustCanvas.uninitialized
        |> D.hardcoded
            (Rust.Global
                90.0
                (Rust.Color (221.0 / 255.0) (218.0 / 255.0) (202.0 / 255.0))
                (Rust.Color (45.0 / 255.0) (107.0 / 255.0) (123 / 255.0))
                (Rust.Color (181.0 / 255.0) (131.0 / 255.0) (90.0 / 255.0))
                (Rust.Color (205.0 / 255.0) (171.0 / 255.0) (143.0 / 255.0))
            )
        |> D.hardcoded False
        |> D.hardcoded 90.0
        |> D.hardcoded Rust.Gradient


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
    | RustMsg RustCanvas.Msg
    | KeyDown Key.Key
    | KeyUp Key.Key
    | InputFov Float
    | InputAmbientColor ( Float, Float, Float )
    | InputEnvColor ( Float, Float, Float )
    | InputSkybox Rust.Skybox
    | InputGradient1Color ( Float, Float, Float )
    | InputGradient2Color ( Float, Float, Float )


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

        RustMsg rmsg ->
            case rmsg of
                RustCanvas.State value ->
                    ( { model | rust_ref = value }
                    , Cmd.none
                    )

                --The canvas is ready
                RustCanvas.Event Rust.Ready ->
                    ( model
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
            let
                color =
                    Rust.Color r g b

                global =
                    model.rust_global
            in
            ( { model | rust_global = { global | envLightColor = color } }
            , RustCanvas.sendRustMsg model.rust_ref <|
                ChangeEnvLight { color = color }
            )

        InputAmbientColor ( r, g, b ) ->
            let
                color =
                    Rust.Color r g b

                global =
                    model.rust_global
            in
            ( { model | rust_global = { global | ambientLightColor = color } }
            , RustCanvas.sendRustMsg model.rust_ref <|
                ChangeAmbientLight { color = color }
            )

        InputSkybox skybox ->
            ( { model | input_skybox = skybox }
            , RustCanvas.sendRustMsg model.rust_ref <|
                SetSkybox { sky = skybox }
            )

        InputGradient1Color ( r, g, b ) ->
            let
                color =
                    Rust.Color r g b

                global =
                    model.rust_global
            in
            ( { model | rust_global = { global | gradient1 = color } }
            , RustCanvas.sendRustMsg model.rust_ref <|
                SetGradient { color1 = color, color2 = model.rust_global.gradient2 }
            )

        InputGradient2Color ( r, g, b ) ->
            let
                color =
                    Rust.Color r g b

                global =
                    model.rust_global
            in
            ( { model | rust_global = { global | gradient2 = color } }
            , RustCanvas.sendRustMsg model.rust_ref <|
                SetGradient { color1 = model.rust_global.gradient1, color2 = color }
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

        field =
            String.padLeft 2 '0' << toHex

        env_colors =
            "#"
                ++ field model.rust_global.envLightColor.r
                ++ field model.rust_global.envLightColor.g
                ++ field model.rust_global.envLightColor.b

        ambient_colors =
            "#"
                ++ field model.rust_global.ambientLightColor.r
                ++ field model.rust_global.ambientLightColor.g
                ++ field model.rust_global.ambientLightColor.b

        gradient1 =
            "#"
                ++ field model.rust_global.gradient1.r
                ++ field model.rust_global.gradient1.g
                ++ field model.rust_global.gradient1.b

        gradient2 =
            "#"
                ++ field model.rust_global.gradient2.r
                ++ field model.rust_global.gradient2.g
                ++ field model.rust_global.gradient2.b
    in
    section
        [ attribute "style" "--pos:relative" ]
        [ div
            [ attribute "style"
                "--inset-top-left:10px; --pos:absolute;"
            ]
            [ h3 [] [ text (String.fromInt <| Basics.round time) ] ]
        , div
            [ attribute "style"
                "--inset-bottom-left:10px; --pos:absolute;"
            ]
            [ h3 [] [ text "Press 'Z' to control the camera."]]
        , div
            [ attribute "style"
                """
                --inset-top-right:10px;
                --pos:absolute;
                --bg:#00000066;
                --py:24px;
                --px:10px;
                """
            ]
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
            , label [ for "input-skyobx" ]
                [ text <| "Choose Skybox: "
                , select
                    [ name "type"
                    , stopPropagationOn
                        "input"
                        (D.map (\x -> ( x, True ))
                            (targetValue
                                |> D.andThen
                                    (\str ->
                                        case str of
                                            "Gradient" ->
                                                D.succeed Rust.Gradient

                                            "Bitmap" ->
                                                D.succeed Rust.Bitmap

                                            _ ->
                                                D.fail "Unknown value"
                                    )
                            )
                        )
                    ]
                    [ option [ value "Gradient" ] [ text "Gradient" ]
                    , option [ value "Bitmap" ] [ text "Bitmap" ]
                    ]
                    |> Html.map InputSkybox
                ]
            , case model.input_skybox of
                Gradient ->
                    div []
                        [ label [ for "input-gradient1-color" ]
                            [ text <| "Gradient1 color: "
                            , colorInput
                                [ id "input-gradient1-color"
                                , value gradient1
                                ]
                                []
                                |> Html.map InputGradient1Color
                            ]
                        , label [ for "input-gradient2-color" ]
                            [ text <| "Gradient2 color: "
                            , colorInput
                                [ id "input-gradient2-color"
                                , value gradient2
                                ]
                                []
                                |> Html.map InputGradient2Color
                            ]
                        ]

                _ ->
                    text ""
            ]
        , RustCanvas.view model.rust_global
            |> Html.map RustMsg
        ]
