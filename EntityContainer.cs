using System.Collections.Generic;
using System.Linq;
using Garnet.Composition;
using Godot;
using Godot.Collections;
using Microsoft.FSharp.Collections;

namespace Strategy;

[GlobalClass]
[Tool]
public partial class EntityContainer : Resource
{
    private static readonly StringName EntitiesName = new("Entities");

    private GameWorld? gameWorld;

    private Array<Entity?> entities = new();

    public GameWorld? GameWorld
    {
        get => gameWorld;
        set
        {
            if (gameWorld != null)
            {
                foreach (Entity entity in entities.Where(e => e is not null).Cast<Entity>())
                {
                    gameWorld.RemoveEntity(entity.Id);
                    entity.Id = Eid.Undefined;
                }
            }

            gameWorld = value;
            // Needed as godot creates items as null
            Array<Entity?> actualArray = SyncEntities(entities);

            entities = actualArray;
        }
    }

    /// <inheritdoc />
    public override Variant _Get(StringName property)
    {
        return property == EntitiesName ? entities : base._Get(property);
    }

    /// <inheritdoc />
    public override bool _Set(StringName property, Variant value)
    {
        if (property == EntitiesName)
        {
            // Needed as godot creates items as null
            Array<Entity?> actualArray = SyncEntities(value);

            entities = actualArray;

            return true;
        }

        return base._Set(property, value);
    }

    private Array<Entity?> SyncEntities(Variant value)
    {
        Array<Entity?> entitiesValue = value.AsGodotArray<Entity?>();
        if (GameWorld is null)
        {
            return entitiesValue;
        }

        var actualArray = new Array<Entity?>();
        FSharpMap<Eid, FSharpList<object>> internalEntities = GameWorld.GetEntities();

        if (internalEntities.Count == 0)
        {
            foreach (Entity entity in entitiesValue.Where(e => e is not null).Cast<Entity>())
            {
                entity.Id = Eid.Undefined;
            }
        }

        var entitiesToDelete = new HashSet<Eid>(internalEntities.Keys);

        foreach (Entity? entity in entitiesValue)
        {
            Entity actualEntity;
            if (entity?.Id.IsDefined ?? false)
            {
                entitiesToDelete.Remove(entity.Id);
                actualEntity = entity;
            }
            else
            {
                actualEntity = entity ?? new Entity();
                actualEntity.GameWorld = GameWorld;
            }

            actualArray.Add(actualEntity);

            GameWorld.SetComponents(actualEntity.Id,
                ListModule.OfSeq(actualEntity.Components.Select(c => c.GetValue()).Where(v => v is not null)));
        }

        foreach (Eid entity in entitiesToDelete)
        {
            GameWorld.RemoveEntity(entity);
        }

        return actualArray;
    }

    /// <inheritdoc />
    public override Array<Dictionary> _GetPropertyList()
    {
        Array<Dictionary> properties = base._GetPropertyList() ?? new Array<Dictionary>();
        var propertyData = new Dictionary();
        propertyData["name"] = EntitiesName;
        propertyData["type"] = (int)Variant.Type.Array;
        propertyData["usage"] = (int)PropertyUsageFlags.Default;
        propertyData["hint"] = (int)PropertyHint.ResourceType;
        propertyData["hint_string"] = "Entity";
        properties.Add(propertyData);
        return properties;
    }

    public IEnumerable<Entity> GetEntities()
    {
        return entities.Where(e => e is null).Cast<Entity>();
    }
}
