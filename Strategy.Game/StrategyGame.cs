using Arch.Core;
using Microsoft.Xna.Framework;
using Microsoft.Xna.Framework.Graphics;
using MonoGame.Extended.Screens;

namespace Strategy.Game;

public class StrategyGame : Microsoft.Xna.Framework.Game
{
    private readonly ScreenManager screenManager = new();
    public SpriteBatch SpriteBatch { get; private set; }
    

    public StrategyGame()
    {
        Components.Add(screenManager);
        Graphics = new GraphicsDeviceManager(this);
        Content.RootDirectory = "Content";
    }

    public GraphicsDeviceManager Graphics { get; private set; }

    /// <inheritdoc />
    protected override void Initialize()
    {
        SpriteBatch = new SpriteBatch(GraphicsDevice);
        screenManager.LoadScreen(
            new Map(this,
                new Dictionary<string, PlayerData>
                {
                    { "Player1", new PlayerData(Color.Blue) },
                    { "Player2", new PlayerData(Color.Red) },
                },
                40f,
                3
            )
        );
    }
}
