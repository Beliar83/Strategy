namespace Strategy.FSharp

open Godot
open Godot.Collections

type Body() =
    inherit Node2D()
    
    let mutable nodesWithColor : Array<NodePath> = Array<NodePath>() 

    member this.NodesWithColor
        with get () = nodesWithColor
        and set value = nodesWithColor <- value
    
    override this._Ready() =
        for node in nodesWithColor do
            let node = this.GetNode(node) :?> Node2D
            node.Material <- node.Material.Duplicate() :?> Material

    member this.SetBodyRotation(rotation: float32) = this.GlobalRotationDegrees <- rotation

    member this.SetWeaponRotation(rotation: float32) =
        let weapon =
            this.GetNode(new NodePath("Weapon")) :?> Node2D

        weapon.GlobalRotationDegrees <- rotation
