using System;
using System.Collections.Generic;
using System.Linq;
using Godot;
using Array = Godot.Collections.Array;

namespace Strategy;

[Tool]
[GlobalClass]
public partial class MapUI : CanvasLayer
{
    private const string RadialMenuScene = "uid://iopwhmejcr8b";
    private GameWorld? gameWorld;
        
    [Export(PropertyHint.NodeType, "Label")] public Label? PlayerLabel { get; set; }

    [Export(PropertyHint.NodeType, nameof(GameWorld))]
    public GameWorld? GameWorld
    {
        get => gameWorld;
        set
        {
            if (gameWorld != null)
            {
                gameWorld.OnPlayerChanged -= PlayerChanged;
            }
            gameWorld = value;
            if (gameWorld != null)
            {
                gameWorld.OnPlayerChanged += PlayerChanged;
            }
        }
    }

    [Export]
    public Camera2D? Camera { get; set; }
        
    private void PlayerChanged(StringName playerId)
    {
        if (PlayerLabel != null)
        {
            PlayerLabel.Text = playerId.ToString();
            if (GameWorld != null)
            {
                PlayerLabel.AddThemeColorOverride("font_color", GameWorld.PlayersByName[playerId].Color);
            }
            else
            {
                PlayerLabel.RemoveThemeColorOverride("font_color");
            }
                
        }
    }
        
    public void ShowContextMenu(List<MenuItem> menuItems, Vector2 position, Action closedHandler)
    {
        PackedScene? scene = GD.Load<PackedScene>(RadialMenuScene);
        if (scene is null)
        {
            GD.PrintErr("Could load radial menu scene");
            return;
        }
            
        Control radialMenu = scene.Instantiate<Control>();
        AddChild(radialMenu);
        radialMenu.Call("set_items", new Array());

        foreach ((MenuItem menuItem, int index) in menuItems.Select((x, i) => (x, i)))
        {
            switch (menuItem.ItemType)
            {
                case ItemType.IconItem itemData:
                {
                    Texture2D icon = ResourceLoader.Load<Texture2D>(itemData.IconPath);
                    radialMenu.Call("add_icon_item", [icon, menuItem.Label, index]);
                    break;
                }
                case ItemType.Item:
                {
                    MenuNode subViewport = new (menuItem.Label);
                    radialMenu.AddChild(subViewport);
                        
                    radialMenu.Call("add_icon_item", [subViewport.GetTexture(), menuItem.Label, index]);
                    break;
                }
            }
        }

        SignalAwaiter itemSelected = radialMenu.ToSignal(radialMenu, new StringName("item_selected"));
        SignalAwaiter canceled = radialMenu.ToSignal(radialMenu, new StringName("canceled"));
            
        itemSelected.OnCompleted(() =>
        {
            Variant[] result = itemSelected.GetResult();
            int index = result[0].AsInt32();
            if (index >= 0)
            {
                menuItems[index].Command();
            }
            radialMenu.QueueFree();
        });

        canceled.OnCompleted(() =>
        {
            closedHandler.Invoke();
            radialMenu.QueueFree();
        });

        Vector2 scale = Camera?.Zoom ?? Vector2.One; 
            
        scale.X = MathF.Max(scale.X, 0.5f);
        scale.Y = MathF.Max(scale.Y, 0.5f);
            
        radialMenu.Scale = scale;


        Rect2 menuRect = radialMenu.GetRect();
        Vector2 halfMenuSize = menuRect.Size / 2;
        halfMenuSize.Y *= 1.5f;
        menuRect.Position = position - halfMenuSize;
            
        Popup popup = new();
        AddChild(popup);
        popup.Popup((Rect2I?)menuRect);
        position = popup.Position + halfMenuSize;
        popup.QueueFree();

        radialMenu.Call("open_menu", position);
    }
}