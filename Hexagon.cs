using System.Collections.Generic;
using System.ComponentModel;
using System.Runtime.CompilerServices;
using Godot;

namespace Strategy;

[GlobalClass]
[Tool]
public partial class Hexagon : Resource
{
    private FSharp.Hexagon.Hexagon internalValue = FSharp.Hexagon.Hexagon.Zero;

    public FSharp.Hexagon.Hexagon InternalValue
    {
        get => internalValue;
        private set => SetField(ref internalValue, value);
    }

    [Export]
    public int Q
    {
        get => InternalValue.Q;
        set => InternalValue = FSharp.Hexagon.Hexagon.NewAxial(value, InternalValue.R);
    }

    [Export]
    public int R
    {
        get => InternalValue.R;
        set => InternalValue = FSharp.Hexagon.Hexagon.NewAxial(InternalValue.Q, value);
    }

    public event PropertyChangedEventHandler? PropertyChanged;

    protected virtual void OnPropertyChanged([CallerMemberName] string? propertyName = null)
    {
        PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
    }

    private void SetField<T>(ref T field, T value, [CallerMemberName] string? propertyName = null)
    {
        if (EqualityComparer<T>.Default.Equals(field, value))
        {
            return;
        }

        field = value;
        OnPropertyChanged(propertyName);
    }
}
