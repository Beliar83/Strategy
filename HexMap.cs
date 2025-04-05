using System.Collections.Generic;
using System.Linq;
using Godot;

namespace Strategy;

[Tool]
[GlobalClass]
public partial class HexMap : TileMapLayer
{
    private TileMapLayer? hoverLayer;
    private TileMapLayer? selectionLayer;
    private TileMapLayer? attackableLayer;
    private TileMapLayer? remainingMovementLayer;
    private TileMapLayer? movementRangeLayer;
    private TileMapLayer? attackRangeLayer;

    [Export(PropertyHint.NodeType, nameof(TileMapLayer))]
    public TileMapLayer? HoverLayer
    {
        get => hoverLayer;
        set
        {
            hoverLayer = value;
            UpdateConfigurationWarnings();
        }
    }

    [Export(PropertyHint.NodeType, nameof(TileMapLayer))]
    public TileMapLayer? SelectionLayer
    {
        get => selectionLayer;
        set
        {
            selectionLayer = value;
            UpdateConfigurationWarnings();
        }
    }

    [Export(PropertyHint.NodeType, nameof(TileMapLayer))]
    public TileMapLayer? AttackableLayer
    {
        get => attackableLayer;
        set
        {
            attackableLayer = value;
            UpdateConfigurationWarnings();
        }
    }

    [Export(PropertyHint.NodeType, nameof(TileMapLayer))]
    public TileMapLayer? AttackRangeLayer
    {
        get => attackRangeLayer;
        set
        {
            attackRangeLayer = value;
            UpdateConfigurationWarnings();
        }
    }

    [Export(PropertyHint.NodeType, nameof(TileMapLayer))]
    public TileMapLayer? RemainingMovementLayer
    {
        get => remainingMovementLayer;
        set
        {
            remainingMovementLayer = value;
            UpdateConfigurationWarnings();
        }
    }

    [Export(PropertyHint.NodeType, nameof(TileMapLayer))]
    public TileMapLayer? MovementRangeLayer
    {
        get => movementRangeLayer;
        set
        {
            movementRangeLayer = value;
            UpdateConfigurationWarnings();
        }
    }    
    
    public List<HexagonNode2D> GetNodesAtCell(Hexagon axialPosition)
    {
        return GetChildren().OfType<HexagonNode2D>().Where(n => !n.IsQueuedForDeletion() && n.Hexagon.Equals(axialPosition)).ToList();
    }

    public List<HexagonNode2D> GetNodesAtCell(Vector2I offsetPosition)
    {
        return GetNodesAtCell(Hexagon.AtOffsetPosition(offsetPosition));
    }
    
    public override string[] _GetConfigurationWarnings()
    {
        List<string> warnings = [];

        if (HoverLayer is null)
        {
            warnings.Add("Hover layer is unset");
        }

        if (SelectionLayer is null)
        {
            warnings.Add("Selection layer is unset");
        }

        if (MovementRangeLayer is null)
        {
            warnings.Add("Movement Range layer is unset");
        }
        
        if (RemainingMovementLayer is null)
        {
            warnings.Add("Remaining Movement layer is unset");
        }

        if (AttackableLayer is null)
        {
            warnings.Add("Attackable layer is unset");
        }
        
        if (AttackRangeLayer is null)
        {
            warnings.Add("Attack Range layer is unset");
        }

        return warnings.ToArray();
    }    
}