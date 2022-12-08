module Key exposing (..)

import Json.Decode as Decode

--- https://github.com/elm/browser/blob/1.0.2/notes/keyboard.md 

type Key
  = Character Char
  | Control String

keyDecoder : Decode.Decoder Key
keyDecoder =
  Decode.map toKey (Decode.field "key" Decode.string)

toKey : String -> Key
toKey string =
  case String.uncons string of
    Just (char, "") ->
      Character char

    _ ->
      Control string
