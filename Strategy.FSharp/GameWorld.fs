namespace Strategy.FSharp

open Godot
open Garnet.Composition

open HexMap

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
    let mutable draw = Unchecked.defaultof<_>
    override this._Ready() =        
        
        let cellSize = 40.0
        world.AddResource ("MapRadius", 1)
        world.AddResource ("Hexagon", HexagonPoints (float32 cellSize))
        world.AddResource ("UpdateMap", true)

        update <- world.On<Update>
            <| fun time ->
            let update_map = world.LoadResource<bool>("UpdateMap")
                
            if update_map then                
                let mapRadius = world.LoadResource<int>("MapRadius")            
                let fields =
                    CreateGrid mapRadius
                    |> Array.map (fun h -> { Position = h; Attackable = false; Moveable = false; Highlighted = false }) 
                world.AddResource ("Map", { CellSize = float32 cellSize; Cells = fields })
                world.AddResource ("UpdateMap", false)
                this.Update()
                
                                  

        
        let drawField cellSize (field : Field) =
            let position = Get2DPositionOfHexagon field.Position cellSize
            let hexagon = world.LoadResource<Vector2[]> "Hexagon"
            let adjusted_polygon =
                hexagon
                |> Array.map (fun h -> h + position)
            this.DrawPolyline(adjusted_polygon, Color.ColorN("Black"), float32 1.0)
        
        draw <- world.On<Draw>
            <| fun _ ->
                let map = world.LoadResource<HexMap> "Map"
                let drawField = drawField map.CellSize
                map.Cells
                |> Array.iter drawField
           
               
        
    override this._PhysicsProcess(delta) =
        world.Run <| {UpdateTime = delta}
        
    override this._Draw() =
        world.Run <| Draw()
        
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
        
        
    
        

