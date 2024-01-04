module Strategy.FSharp.Player

open System
open Godot

type Color = { R: int32; G : int32; B: int32; A : int32 }

let ColorFromGodotColor(color: Godot.Color) =
    { R = color.R8; G = color.G8; B = color.B8; A = color.A8 }
let GodotColorFromColor(color: Color) =
    Godot.Color((float32 <| color.R) / 255.0f, (float32 <| color.G) / 255.0f, (float32 <| color.B) / 255.0f, (float32 <| color.A) / 255.0f)
let Vector3FromColor(color: Color) =
    Vector3((float32 <| color.R) / 255.0f, (float32 <| color.G) / 255.0f, (float32 <| color.B) / 255.0f)

type PlayerData = { Color: Color }

type Player = { PlayerId: String }
