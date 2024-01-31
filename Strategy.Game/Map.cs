using System.Collections.Immutable;
using Arch.Core;
using Arch.System;
using GeonBit.UI;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using Microsoft.Xna.Framework.Input;
using MonoGame.Extended.Screens;

namespace Strategy.Game;

public class Map : GameScreen
{
    private readonly World world = World.Create();
    private Group<GameTime> systems = null!;
    private readonly StrategyGame game;
    private readonly float cellSize;
    private Vector2 screenCenter;
    private Effect hexLayerEffect = null!;
    private Effect basicEffect = null!;
    private Matrix projectionMatrix;
    private readonly Dictionary<Hexagon, Field> cells;
    private Hexagon? mouseCell;

    public Map(Microsoft.Xna.Framework.Game game, Dictionary<string, PlayerData> players, float cellSize, int radius) : base(game)
    {
        this.game = (StrategyGame)game;
        Players = players;
        this.cellSize = cellSize;
        foreach (string playerId in players.Keys)
        {
            PlayerQueue.Enqueue(playerId);
        }

        cells = HexGrid
            .CreateGrid(radius)
            .ToDictionary(h => h, h => new Field(false, false, HexGrid.Get2DPositionOfHexagon(h, cellSize)));
    }

    /// <inheritdoc />
    public override void Initialize()
    {
        UserInterface.Initialize(Content, BuiltinThemes.hd);
        UserInterface.Active.UseRenderTarget = true;
        UserInterface.Active.IncludeCursorInRenderTarget = false;
        screenCenter = new Vector2(game.Graphics.PreferredBackBufferWidth / 2f,
            game.Graphics.PreferredBackBufferHeight / 2f);
        projectionMatrix = Matrix.CreateOrthographicOffCenter(0f, GraphicsDevice.Viewport.Width, GraphicsDevice.Viewport.Height, 0f, 0f,
            1f);
        systems = new Group<GameTime>(new MapUi(world, this, UserInterface.Active.Root));
        systems.Initialize();
        
        NewRound();
    }

    public Dictionary<string, PlayerData> Players { get; }
    public string? CurrentPlayer { get; set; }
    public Queue<string> PlayerQueue { get; } = new();

    /// <inheritdoc />
    public override void Update(GameTime gameTime)
    {
        MouseState mouseState = Mouse.GetState();

        Hexagon cell = Hexagon.At2DPosition(new Vector2(mouseState.X, mouseState.Y) - screenCenter, cellSize);

        if (cell != mouseCell)
        {
            if (cells.ContainsKey(cell))
            {
                mouseCell = cell;
            }
            else
            {
                mouseCell = null;
            }
        }

        systems.BeforeUpdate(gameTime);
        systems.Update(gameTime);
        systems.AfterUpdate(gameTime);
        UserInterface.Active.Update(gameTime);
    }

    /// <inheritdoc />
    public override void LoadContent()
    {
        hexLayerEffect = Content.Load<Effect>("HexLayers");
        hexLayerEffect.Parameters["MoveableColor"].SetValue(Color.Red.ToVector4());
        hexLayerEffect.Parameters["AttackableColor"].SetValue(Color.Blue.ToVector4());
        hexLayerEffect.Parameters["HighlightColor"].SetValue(Color.White.ToVector4());
        hexLayerEffect.Parameters["WorldViewProjection"].SetValue(projectionMatrix);

        basicEffect = Content.Load<Effect>("Basic");
        basicEffect.Parameters["Color"].SetValue(Color.Black.ToVector4());
        basicEffect.Parameters["WorldViewProjection"].SetValue(projectionMatrix);
    }

    /// <inheritdoc />
    public override void Draw(GameTime gameTime)
    {
        UserInterface.Active.Draw(game.SpriteBatch);
        GraphicsDevice.Clear(Color.CornflowerBlue);
        ImmutableList<Vector2> hexagonTriangles = HexGrid.GetHexagonTriangles(cellSize);
        VertexPosition[] triangleVertices = 
            hexagonTriangles
                .Select(p => new VertexPosition(new Vector3(p.X, p.Y, 0f))).ToArray();
        List<Vector2> hexagonPoints = HexGrid.GetHexagonPoints(cellSize);
        hexagonPoints.Insert(0, hexagonPoints[0]);
        VertexPosition[] hexagonVertices =
            hexagonPoints
                .Select(p => new VertexPosition(new Vector3(p.X, p.Y, 0f))).ToArray();
        
         game.SpriteBatch.Begin();

         foreach ((Hexagon cell, Field field) in cells)
         {
             Vector2 position = field.Position + screenCenter;

             bool highlighted = cell == mouseCell;

             if (field.Movable || field.Attackable || highlighted)
             {
                 SetHexlayerEffectOffset(new Vector3(position, -0.1f));
                 SetMoveableActive(field.Movable);
                 SetAttackableActive(field.Attackable);
                 SetHighlightActive(highlighted);
                 hexLayerEffect.CurrentTechnique.Passes[0].Apply();
                 GraphicsDevice.DrawUserPrimitives(
                     PrimitiveType.TriangleList, triangleVertices, 0, triangleVertices.Length / 3);
             }
             
             SetBasicEffectOffset(new Vector3(position, 0f));
             basicEffect.CurrentTechnique.Passes[0].Apply();
             
             game.Graphics.GraphicsDevice.DrawUserPrimitives(
                 PrimitiveType.LineStrip, 
                 hexagonVertices, 
                 0, 
                 hexagonVertices.Length - 1);
         }
         
         game.SpriteBatch.End();
         
         UserInterface.Active.DrawMainRenderTarget(game.SpriteBatch);
         
         return;

         void SetOffset(Effect effect, Vector3 offset)
         {
             effect.Parameters["Offset"].SetValue(offset);
         }

         void SetBasicEffectOffset(Vector3 offset)
         {
             SetOffset(basicEffect, offset);
         }
         
         void SetHexlayerEffectOffset(Vector3 offset)
         {
             SetOffset(hexLayerEffect, offset);
         }

         void SetMoveableActive(bool active)
         {
             hexLayerEffect.Parameters["MoveableActive"].SetValue(active);
         }
         
         void SetAttackableActive(bool active)
         {
             hexLayerEffect.Parameters["AttackableActive"].SetValue(active);
         }

         void SetHighlightActive(bool active)
         {
             hexLayerEffect.Parameters["HighlightActive"].SetValue(active);
         }         
    }
    
    public void NewRound()
    {
        if (!String.IsNullOrEmpty(CurrentPlayer))
        {
            PlayerQueue.Enqueue(CurrentPlayer);
        }
        CurrentPlayer = PlayerQueue.Dequeue();
    }
    
}
