namespace Strategy

open System
open FSharp.Linq.NullableOperators
open Garnet.Composition
open Garnet.Composition.Join
open Microsoft.Xna.Framework
open Microsoft.Xna.Framework.Graphics
open Microsoft.Xna.Framework.Input
// open MonoGame.Extended.Shapes
open Strategy.Components
open Strategy.Components.Field
open Strategy.Components.Hexagon
open Strategy.Systems.HexGrid
// open MonoGame.Extended

type Highlighted =
    struct

    end

[<Struct>]
type Draw = { DrawTime: GameTime }

[<Struct>]
type Update = { UpdateTime: GameTime }

[<Struct>]
type Position = { X: float32; Y: float32 }

[<Struct>]
type HexGrid = { FieldSize: int32; Radius: int32 }

type StrategyGame() as this =
    inherit Game()
    let world = Container()
    let mutable updateLOS = true
    let mutable update = Unchecked.defaultof<_>
    let mutable draw = Unchecked.defaultof<_>
    let graphics : GraphicsDeviceManager = new GraphicsDeviceManager(this)
    let mutable losPoint = Vector2.Zero

    do
        this.Content.RootDirectory <- "Content"
        this.IsMouseVisible <- true

    override this.Initialize() =
        // TODO: Add your initialization logic here

        let distanceFromCenter vector = Vector2.Distance(losPoint, vector)

        let intersects_distance (position: Vector2) radius (ray: Ray) distance =
            let intersects =
                ray.Intersects(BoundingSphere(Vector3(position.X, position.Y, 0f), radius))

            intersects ?<= distance

        let intersects (position: Vector2) =
            let direction = position - losPoint
            direction.Normalize()
            let distance = distanceFromCenter position

            let ray =
                Ray(Vector3(losPoint.X, losPoint.Y, 0f), Vector3(direction.X, direction.Y, 0f))

            let filter (otherPosition, radius) =
                (not <| position.Equals(otherPosition))
                && intersects_distance otherPosition radius ray distance

            false

        let hexagonEquals (first: Hexagon) (second: Hexagon) =
            first.Q = second.Q
            && first.R = second.R
            && first.S = second.S


        update <-
            world.On<Update>
            <| fun time ->

                let mouseState =
                    world.LoadResource<MouseState> "MouseState"

                let mousePos =
                    Point(mouseState.X, mouseState.Y)

                world.AddResource("MousePos", mousePos)

                let leftPressed =
                    match mouseState.LeftButton with
                    | ButtonState.Pressed -> true
                    | ButtonState.Released -> false
                    | _ -> false

                world.AddResource("LeftButtonHeld", leftPressed)


                let recreateGrid = world.LoadResource<bool> "RecreateGrid"

                let hexfieldSize = world.LoadResource<float32> "FieldSize"

                let screenCenter =
                    world.LoadResource<Vector2> "ScreenCenter"

                let highlightedQuery = world.Query<Eid, Field, Highlighted>()

                let highlightedField =
                    if highlightedQuery.GetCount() > 0 then
                        highlightedQuery
                        |> Seq.map (fun q -> q.Value1)
                        |> Seq.head
                        |> Some
                    else
                        None

                let points = PolygonPoints hexfieldSize

                let mouseHexagons =
                    world.Query<Eid, Field>()
                    // |> Seq.filter
                    //     (fun item ->
                    //         let field = item.Value2
                    //
                    //         let center =
                    //             Get2DPositionOfHexagon field.Location hexfieldSize
                    //             + screenCenter
                    //
                    //         let points =
                    //             points |> Array.map (fun p -> p + center)
                    //
                    //         let polygon = Polygon(points)
                    //         polygon.Contains(Vector2(mousePos.X, mousePos.Y)))
                    |> Seq.map (fun item -> item.Value1)

                let changeHighlight =
                    match highlightedField with
                    | Some field ->
                        not
                        <| Seq.exists (fun id -> id = field) mouseHexagons
                    | None -> true

                if changeHighlight then
                    highlightedQuery
                    |> Seq.iter
                        (fun item ->
                            let entity = world.Get item.Value1
                            entity.Remove<Highlighted>())

                    if Seq.length mouseHexagons > 0 then
                        let entity = world.Get <| Seq.head mouseHexagons
                        let highlighted = Highlighted()
                        entity.Add<Highlighted> highlighted



                if recreateGrid then
                    for field in world.Query<Eid, Field>() do
                        world.Destroy(field.Value1)

                    let radius = world.LoadResource<Int32> "Radius"
                    let grid = CreateGrid radius

                    grid
                    |> Array.map Field.FromHexagon
                    |> Array.iter (fun hex -> world.Create().With hex |> ignore)

                    world.AddResource("RecreateGrid", false)

                if updateLOS then
                    updateLOS <- false

                    for field in world.Query<Eid, Field>() do
                        let entity = world.Get field.Current.Value1

                        let newField =
                            Field(field.Value2.Location, false, false)

                        entity.Add<Field> newField

        draw <-
            world.On<Draw>
            <| fun time ->
                let spriteBatch =
                    world.LoadResource<SpriteBatch> "spriteBatch"

                let hexfieldSize = world.LoadResource<float32> "FieldSize"

                let center =
                    world.LoadResource<Vector2> "ScreenCenter"

                let basicEffect = world.LoadResource<Effect> "BasicEffect"

                let hexlayerEffect =
                    world.LoadResource<Effect> "HexLayerEffect"

                let matrix =
                    world.LoadResource<Matrix> "ProjectionMatrix"

                let polygonTriangles = PolygonTriangles hexfieldSize
                let polygonPoints = PolygonPoints hexfieldSize

                let triangleVertices =
                    polygonTriangles
                    |> Array.map (fun point -> VertexPosition(Vector3(point.X, point.Y, 0f)))

                let polygonVertices =
                    polygonPoints
                    |> Array.take 1
                    |> Array.append polygonPoints
                    |> Array.map (fun point -> VertexPosition(Vector3(point.X, point.Y, 0f)))

                let highlightQuery = world.Query<Field, Highlighted>()

                let highlightedField =
                    if highlightQuery.GetCount() > 0 then
                        highlightQuery
                        |> Seq.map (fun q -> q.Value1)
                        |> Seq.head
                        |> Some
                    else
                        None

                let worldViewProjection =
                    hexlayerEffect.Parameters.Item "WorldViewProjection"

                worldViewProjection.SetValue(matrix)

                let worldViewProjection =
                    basicEffect.Parameters.Item "WorldViewProjection"

                worldViewProjection.SetValue(matrix)
                
                let SetOffset (effect: Effect) (offset: Vector3) =
                    let item = effect.Parameters.Item "Offset"
                    item.SetValue(offset)

                let SetBasicEffectOffset = SetOffset basicEffect

                let SetBasicEffectColor (color: Color) =
                    let item = basicEffect.Parameters.Item "Color"
                    item.SetValue(color.ToVector4())

                SetBasicEffectColor Color.Black


                let SetHexlayerEffectOffset = SetOffset hexlayerEffect

                let SetMoveableActive (active: bool) =
                    let item =
                        hexlayerEffect.Parameters.Item "MoveableActive"

                    item.SetValue(active)

                let SetAttackableActive (active: bool) =
                    let item =
                        hexlayerEffect.Parameters.Item "AttackableActive"

                    item.SetValue(active)

                let SetHighlightActive (active: bool) =
                    let item =
                        hexlayerEffect.Parameters.Item "HighlightActive"
                    
                    item.SetValue(active)

                //TODO: Use Index
                spriteBatch.Begin()
                let query = world.Query<Field>()

                for field in query do
                    let position =
                        Get2DPositionOfHexagon field.Value.Location hexfieldSize
                        + center

                    let highlighted =
                        match highlightedField with
                        | Some highlightedField -> hexagonEquals field.Value.Location highlightedField.Location
                        | None -> false

                    if field.Value.Movable
                       || field.Value.Attackable
                       || highlighted then
                        SetHexlayerEffectOffset(Vector3(position, -0.1f))
                        SetMoveableActive field.Value.Movable
                        SetAttackableActive field.Value.Attackable
                        SetHighlightActive highlighted

                        hexlayerEffect
                            .CurrentTechnique
                            .Passes
                            .Item(0)
                            .Apply()

                        graphics.GraphicsDevice.DrawUserPrimitives(
                            PrimitiveType.TriangleList,
                            triangleVertices,
                            0,
                            triangleVertices.Length / 3
                        )

                    SetBasicEffectOffset(Vector3(position, 0.0f))

                    basicEffect
                        .CurrentTechnique
                        .Passes
                        .Item(0)
                        .Apply()

                    graphics.GraphicsDevice.DrawUserPrimitives(
                        PrimitiveType.LineStrip,
                        polygonVertices,
                        0,
                        polygonVertices.Length - 1
                    )

                spriteBatch.End()

        base.Initialize() // Load content has been called after

        losPoint <-
            Vector2(float32 graphics.PreferredBackBufferWidth / 2f, float32 graphics.PreferredBackBufferHeight / 2f)


    override this.LoadContent() =
        world.AddResource("FieldSize", 40f)
        world.AddResource("Radius", 3)
        world.AddResource("RecreateGrid", true)
        world.AddResource("SpriteBatch", new SpriteBatch(this.GraphicsDevice))
        world.AddResource("MousePos", Vector2.Zero)
        world.AddResource("LeftButtonHeld", false)
        world.AddResource("MouseState", Mouse.GetState())

        world.AddResource(
            "ScreenCenter",
            Vector2(float32 graphics.PreferredBackBufferWidth / 2f, float32 graphics.PreferredBackBufferHeight / 2f)
        )

        let hexlayerEffect = this.Content.Load<Effect>("HexLayers")

        let item =
            hexlayerEffect.Parameters.Item "MoveableColor"

        item.SetValue(Color.Red.ToVector4())

        let item =
            hexlayerEffect.Parameters.Item "AttackableColor"

        item.SetValue(Color.Blue.ToVector4())

        let item =
            hexlayerEffect.Parameters.Item "HighlightColor"

        item.SetValue(Color.White.ToVector4())

        world.AddResource("HexLayerEffect", hexlayerEffect)
        world.AddResource("BasicEffect", this.Content.Load<Effect>("Basic"))

        world.AddResource(
            "ProjectionMatrix",
            Matrix.CreateOrthographicOffCenter(
                0f,
                float32 graphics.GraphicsDevice.Viewport.Width,
                float32 graphics.GraphicsDevice.Viewport.Height,
                0f,
                0f,
                1f
            )
        )

    // TODO: use this.Content to load your game content here

    override this.Update gameTime =
        // if (GamePad.GetState(PlayerIndex.One).Buttons.Back = ButtonState.Pressed
        //     || Keyboard.GetState().IsKeyDown(Keys.Escape)) then
        //     this.Exit()

        world.AddResource("MouseState", Mouse.GetState())
        world.Run <| { UpdateTime = gameTime }
        base.Update(gameTime)

    override this.Draw gameTime =
        this.GraphicsDevice.Clear Color.CornflowerBlue

        world.Run <| { DrawTime = gameTime }


        // TODO: Add your drawing code here

        base.Draw(gameTime)
