namespace Strategy.FSharp

open System.Collections.Generic
open Godot
open Garnet.Composition

open HexMap
open Strategy.FSharp.Hexagon

[<Struct>]
type Position = { X: float32; Y: float32 }

[<Struct>]
type Node = {id: uint64} 

[<Struct>]
type Update = { UpdateTime: float32 }

[<Struct>]
type Draw = struct end

type GameWorld() = 
    inherit Node2D()    
    let world = Container()
    let mutable update = Unchecked.defaultof<_>
    override this._Ready() =        
                
        let cellSize = 4.0f
        world.AddResource ("MapRadius", 1)
        world.AddResource ("UpdateMap", true)

        update <- world.On<Update>
            <| fun time ->
            let update_map = world.LoadResource<bool>("UpdateMap")
                
            if update_map then                
                let mapRadius = world.LoadResource<int>("MapRadius")            
                let cells = CreateGrid mapRadius
                                
                let cellsNode = new NodePath("Cells") |> this.GetNode
                
                while cellsNode.GetChildCount() > 0 do
                    let node = cellsNode.GetChildOrNull<Godot.Node> 0
                    if not <| isNull node then
                        node.QueueFree()
                        cellsNode.RemoveChild node
                
                        
                        
                let hexagon = GD.Load("res://Hexagon.tscn") :?> PackedScene
                
                cells
                |> Array.map (fun c ->
                              let node = hexagon.Instance() :?> Node2D
                              node.Position <- Get2DPositionOfHexagon c (float32 cellSize)
                              node.Scale <- Vector2(cellSize, cellSize)
                              node
                              )
                |> Array.iter cellsNode.AddChild
                
                world.AddResource ("UpdateMap", false)
                this.Update()           

    override this._PhysicsProcess(delta) =
        world.Run <| {UpdateTime = delta}
        
    override this._Input(event) =
        match event with
            | :? InputEventKey as event ->
                let scancode = enum<KeyList>(int32 event.Scancode)
                if not event.Pressed then
                    match scancode with
                        | KeyList.Plus | KeyList.KpAdd ->
                            let radius = world.LoadResource<int> "MapRadius"
                            world.AddResource ("MapRadius", radius + 1)
                            world.AddResource ("UpdateMap", true)
                        | KeyList.Minus | KeyList.KpSubtract ->
                            let radius = world.LoadResource<int> "MapRadius"
                            world.AddResource ("MapRadius", max 1 (radius - 1))
                            world.AddResource ("UpdateMap", true)
                        | _ -> ()
            | _ -> ()
        
        
    
        

