using System.Linq;
using Garnet.Composition;
using Godot;
using Godot.Collections;
using Microsoft.FSharp.Collections;

namespace Strategy;

[GlobalClass]
[Tool]
public partial class Entity : Resource
{
    public Eid Id { get; set; }

    private GameWorld? gameWorld;

    public GameWorld? GameWorld
    {
        get => gameWorld;
        set
        {
            if (Id.IsDefined)
            {
                gameWorld?.RemoveEntity(Id);
            }

            gameWorld = value;
            Id = gameWorld?.AddEntity() ?? Eid.Undefined;

            SyncComponents();
        }
    }

    private Array<Component> components = new();

    public void UpdateComponent(object component)
    {
        GameWorld?.SetComponent(Id, component);
    }

    [Export(PropertyHint.ResourceType, "Components/Component")]
    public Array<Component> Components
    {
        get => components;
        set
        {
            components = value;
            foreach (Component component in value.Where(c => c is not null))
            {
                if (component.Entity != this)
                {
                    component.Entity = this;
                }
            }

            SyncComponents();
        }
    }

    private void SyncComponents()
    {
        GameWorld?.SetComponents(Id,
            ListModule.OfSeq(components.Select(c => c?.GetValue()).Where(v => v is not null)));
    }
}
