using Arch.Core;
using Arch.System;
using GeonBit.UI.Entities;
using Microsoft.Xna.Framework;
using Entity = GeonBit.UI.Entities.Entity;

namespace Strategy.Game;

public class MapUi : BaseSystem<World, GameTime>
{
    private readonly Map map;
    private readonly Label playerLabel = new();

    private readonly Button endRoundButton = new("End Round", size: new Vector2(200f, 50f), anchor: Anchor.BottomRight,
        offset: new Vector2(10f, 10f));

    public MapUi(World world, Map map, Entity parent) : base(world)
    {
        this.map = map;
        parent.AddChild(playerLabel);
        parent.AddChild(endRoundButton);
        playerLabel.OutlineColor = Color.Transparent;
        playerLabel.Offset = new Vector2(10f, 10f);
        endRoundButton.OnClick += _ => map.NewRound();
    }

    /// <inheritdoc />
    public override void Update(in GameTime t)
    {
        if (String.IsNullOrWhiteSpace(map.CurrentPlayer))
        {
            playerLabel.Text = "";
        }
        else
        {
            playerLabel.Text = map.CurrentPlayer;
            playerLabel.FillColor = map.Players[map.CurrentPlayer].Color;
        }
    }
}
