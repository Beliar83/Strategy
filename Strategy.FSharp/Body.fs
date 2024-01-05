namespace Strategy.FSharp

open Godot

type Body() =
    inherit Node2D()

    override this._Ready() =
        this.Material <- this.Material.Duplicate() :?> Material

        let weapon =
            this.GetNode(new NodePath("Weapon")) :?> Node2D

        weapon.Material <- weapon.Material.Duplicate() :?> Material

    member this.SetBodyRotation(rotation: float32) = this.GlobalRotationDegrees <- rotation

    member this.SetWeaponRotation(rotation: float32) =
        let weapon =
            this.GetNode(new NodePath("Weapon")) :?> Node2D

        weapon.GlobalRotationDegrees <- rotation
