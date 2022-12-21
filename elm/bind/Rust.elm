
-- generated by elm_rs


module Rust exposing (..)

import Dict exposing (Dict)
import Http
import Json.Decode
import Json.Encode
import Url.Builder


resultEncoder : (e -> Json.Encode.Value) -> (t -> Json.Encode.Value) -> (Result e t -> Json.Encode.Value)
resultEncoder errEncoder okEncoder enum =
    case enum of
        Ok inner ->
            Json.Encode.object [ ( "Ok", okEncoder inner ) ]
        Err inner ->
            Json.Encode.object [ ( "Err", errEncoder inner ) ]


resultDecoder : Json.Decode.Decoder e -> Json.Decode.Decoder t -> Json.Decode.Decoder (Result e t)
resultDecoder errDecoder okDecoder =
    Json.Decode.oneOf
        [ Json.Decode.map Ok (Json.Decode.field "Ok" okDecoder)
        , Json.Decode.map Err (Json.Decode.field "Err" errDecoder)
        ]


type Msg
    = Focus
    | Unfocus
    | ChangeFov { angle : Float }
    | ChangeEnvLight { color : Color }
    | ChangeAmbientLight { color : Color }
    | SetSkybox { sky : Skybox }
    | SetGradient { color1 : Color, color2 : Color }


msgEncoder : Msg -> Json.Encode.Value
msgEncoder enum =
    case enum of
        Focus ->
            Json.Encode.string "Focus"
        Unfocus ->
            Json.Encode.string "Unfocus"
        ChangeFov { angle } ->
            Json.Encode.object [ ( "ChangeFOV", Json.Encode.object [ ( "angle", Json.Encode.float angle ) ] ) ]
        ChangeEnvLight { color } ->
            Json.Encode.object [ ( "ChangeEnvLight", Json.Encode.object [ ( "color", colorEncoder color ) ] ) ]
        ChangeAmbientLight { color } ->
            Json.Encode.object [ ( "ChangeAmbientLight", Json.Encode.object [ ( "color", colorEncoder color ) ] ) ]
        SetSkybox { sky } ->
            Json.Encode.object [ ( "SetSkybox", Json.Encode.object [ ( "sky", skyboxEncoder sky ) ] ) ]
        SetGradient { color1, color2 } ->
            Json.Encode.object [ ( "SetGradient", Json.Encode.object [ ( "color1", colorEncoder color1 ), ( "color2", colorEncoder color2 ) ] ) ]

type alias Global =
    { fov : Float
    , envLightColor : Color
    , ambientLightColor : Color
    , gradient1 : Color
    , gradient2 : Color
    }


globalEncoder : Global -> Json.Encode.Value
globalEncoder struct =
    Json.Encode.object
        [ ( "fov", (Json.Encode.float) struct.fov )
        , ( "env_light_color", (colorEncoder) struct.envLightColor )
        , ( "ambient_light_color", (colorEncoder) struct.ambientLightColor )
        , ( "gradient1", (colorEncoder) struct.gradient1 )
        , ( "gradient2", (colorEncoder) struct.gradient2 )
        ]


type alias Color =
    { r : Float
    , g : Float
    , b : Float
    }


colorEncoder : Color -> Json.Encode.Value
colorEncoder struct =
    Json.Encode.object
        [ ( "r", (Json.Encode.float) struct.r )
        , ( "g", (Json.Encode.float) struct.g )
        , ( "b", (Json.Encode.float) struct.b )
        ]


type Skybox
    = Gradient
    | Bitmap


skyboxEncoder : Skybox -> Json.Encode.Value
skyboxEncoder enum =
    case enum of
        Gradient ->
            Json.Encode.string "Gradient"
        Bitmap ->
            Json.Encode.string "Bitmap"

msgDecoder : Json.Decode.Decoder Msg
msgDecoder = 
        let
            elmRsConstructChangeFov angle =
                        ChangeFov { angle = angle }
            elmRsConstructChangeEnvLight color =
                        ChangeEnvLight { color = color }
            elmRsConstructChangeAmbientLight color =
                        ChangeAmbientLight { color = color }
            elmRsConstructSetSkybox sky =
                        SetSkybox { sky = sky }
            elmRsConstructSetGradient color1 color2 =
                        SetGradient { color1 = color1, color2 = color2 }
        in
    Json.Decode.oneOf
        [ Json.Decode.string
            |> Json.Decode.andThen
                (\x ->
                    case x of
                        "Focus" ->
                            Json.Decode.succeed Focus
                        unexpected ->
                            Json.Decode.fail <| "Unexpected variant " ++ unexpected
                )
        , Json.Decode.string
            |> Json.Decode.andThen
                (\x ->
                    case x of
                        "Unfocus" ->
                            Json.Decode.succeed Unfocus
                        unexpected ->
                            Json.Decode.fail <| "Unexpected variant " ++ unexpected
                )
        , Json.Decode.field "ChangeFOV" (Json.Decode.succeed elmRsConstructChangeFov |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "angle" (Json.Decode.float))))
        , Json.Decode.field "ChangeEnvLight" (Json.Decode.succeed elmRsConstructChangeEnvLight |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "color" (colorDecoder))))
        , Json.Decode.field "ChangeAmbientLight" (Json.Decode.succeed elmRsConstructChangeAmbientLight |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "color" (colorDecoder))))
        , Json.Decode.field "SetSkybox" (Json.Decode.succeed elmRsConstructSetSkybox |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "sky" (skyboxDecoder))))
        , Json.Decode.field "SetGradient" (Json.Decode.succeed elmRsConstructSetGradient |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "color1" (colorDecoder))) |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "color2" (colorDecoder))))
        ]

globalDecoder : Json.Decode.Decoder Global
globalDecoder =
    Json.Decode.succeed Global
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "fov" (Json.Decode.float)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "env_light_color" (colorDecoder)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "ambient_light_color" (colorDecoder)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "gradient1" (colorDecoder)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "gradient2" (colorDecoder)))


colorDecoder : Json.Decode.Decoder Color
colorDecoder =
    Json.Decode.succeed Color
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "r" (Json.Decode.float)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "g" (Json.Decode.float)))
        |> Json.Decode.andThen (\x -> Json.Decode.map x (Json.Decode.field "b" (Json.Decode.float)))


type Event
    = Ready


eventDecoder : Json.Decode.Decoder Event
eventDecoder = 
    Json.Decode.oneOf
        [ Json.Decode.string
            |> Json.Decode.andThen
                (\x ->
                    case x of
                        "Ready" ->
                            Json.Decode.succeed Ready
                        unexpected ->
                            Json.Decode.fail <| "Unexpected variant " ++ unexpected
                )
        ]

skyboxDecoder : Json.Decode.Decoder Skybox
skyboxDecoder = 
    Json.Decode.oneOf
        [ Json.Decode.string
            |> Json.Decode.andThen
                (\x ->
                    case x of
                        "Gradient" ->
                            Json.Decode.succeed Gradient
                        unexpected ->
                            Json.Decode.fail <| "Unexpected variant " ++ unexpected
                )
        , Json.Decode.string
            |> Json.Decode.andThen
                (\x ->
                    case x of
                        "Bitmap" ->
                            Json.Decode.succeed Bitmap
                        unexpected ->
                            Json.Decode.fail <| "Unexpected variant " ++ unexpected
                )
        ]

